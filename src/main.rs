mod errors;
use errors::AppError;

use bytes::Bytes;
use futures::future::join_all;
use log::{info, warn, error, debug};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_reader;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader};
use std::time::{Duration, Instant};
use tempfile::NamedTempFile;
use m3u8_rs::Playlist;
use tokio::time;
use tokio::sync::mpsc;

// Структура для XML
#[derive(Debug, Serialize, Deserialize)]
struct PlaylistData {
    #[serde(rename = "playlist")]
    playlists: Vec<PlaylistEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PlaylistEntry {
    url: String,
    name: Option<String>,
}

#[derive(Debug, Clone)]
struct StreamQuality {
    url: String,
    bandwidth: u64,
    response_time: Duration,
}

async fn setup_logger() -> Result<(), AppError> {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("playlist_processor.log")?;
    
    let logger_config = simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .to_stdout()
        .to_file(log_file);
    
    logger_config.init()?;
    Ok(())
}

async fn download_file(url: &str) -> Result<NamedTempFile, AppError> {
    info!("Downloading file from: {}", url);
    let client = Client::new();
    let response = client.get(url).send().await?;
    
    let mut tmp_file = NamedTempFile::new()?;
    let content = response.bytes().await?;
    tmp_file.write_all(&content)?;
    
    info!("File saved to temporary location: {:?}", tmp_file.path());
    Ok(tmp_file)
}

fn parse_playlists(xml_file: &NamedTempFile) -> Result<PlaylistData, AppError> {
    info!("Parsing XML file");
    let file = File::open(xml_file.path())?;
    let reader = BufReader::new(file);
    
    from_reader(reader).map_err(|e| {
        error!("XML parsing failed: {}", e);
        AppError::from(e)
    })
}

async fn process_m3u8(url: &str) -> Result<Playlist, AppError> {
    info!("Processing M3U8 playlist: {}", url);
    let client = Client::new();
    let response = client.get(url).send().await;
    
    let response = match response {
        Ok(r) => r,
        Err(e) => {
            warn!("Failed to download playlist {}: {}", url, e);
            return Err(AppError::ReqwestError(e));
        }
    };
    
    let content = response.text().await?;
    let parsed = m3u8_rs::parse_playlist(&content);
    
    match parsed {
        Ok(Playlist::MasterPlaylist(pl)) => Ok(Playlist::MasterPlaylist(pl)),
        Ok(Playlist::MediaPlaylist(pl)) => Ok(Playlist::MediaPlaylist(pl)),
        Err(e) => {
            error!("M3U8 parsing failed for {}: {}", url, e);
            Err(AppError::M3u8Error(e.to_string()))
        }
    }
}

async fn check_stream_quality(stream_url: String) -> Result<StreamQuality, AppError> {
    let client = Client::new();
    let start_time = Instant::now();
    
    // Устанавливаем таймаут 5 секунд
    let response = time::timeout(
        Duration::from_secs(5),
        client.get(&stream_url).send()
    ).await??;
    
    let bandwidth = response
        .headers()
        .get("bandwidth")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    let response_time = start_time.elapsed();
    
    Ok(StreamQuality {
        url: stream_url,
        bandwidth,
        response_time,
    })
}

async fn process_playlist(playlist: Playlist) -> Vec<StreamQuality> {
    let mut streams = Vec::new();
    
    match playlist {
        Playlist::MasterPlaylist(pl) => {
            for variant in pl.variants {
                streams.push(variant.uri);
            }
        },
        Playlist::MediaPlaylist(pl) => {
            for segment in pl.segments {
                streams.push(segment.uri);
            }
        }
    }
    
    // Проверяем все потоки параллельно
    let check_futures = streams.into_iter().map(|url| {
        check_stream_quality(url)
    });
    
    let results = join_all(check_futures).await;
    
    results.into_iter()
        .filter_map(|res| {
            match res {
                Ok(quality) => {
                    debug!("Stream {}: bandwidth={}, response_time={:?}", 
                        quality.url, quality.bandwidth, quality.response_time);
                    Some(quality)
                },
                Err(e) => {
                    warn!("Failed to check stream: {}", e);
                    None
                }
            }
        })
        .collect()
}

async fn merge_playlists(playlists: Vec<Playlist>) -> Playlist {
    let (tx, mut rx) = mpsc::channel(32);
    
    // Обрабатываем плейлисты параллельно
    for playlist in playlists {
        let tx = tx.clone();
        tokio::spawn(async move {
            let qualities = process_playlist(playlist).await;
            tx.send(qualities).await.unwrap();
        });
    }
    
    drop(tx); // Закрываем канал после отправки всех задач
    
    let mut all_streams = Vec::new();
    while let Some(qualities) = rx.recv().await {
        all_streams.extend(qualities);
    }
    
    // Группируем потоки по URL и выбираем самый быстрый
    use std::collections::HashMap;
    let mut streams_map: HashMap<String, StreamQuality> = HashMap::new();
    
    for stream in all_streams {
        if let Some(existing) = streams_map.get_mut(&stream.url) {
            if existing.bandwidth < stream.bandwidth {
                *existing = stream;
            }
        } else {
            streams_map.insert(stream.url.clone(), stream);
        }
    }
    
    // Создаем объединенный плейлист
    let mut variants = streams_map.into_iter()
        .map(|(_, quality)| {
            m3u8_rs::VariantStream {
                uri: quality.url,
                bandwidth: quality.bandwidth as u32,
                codecs: None,
                resolution: None,
                frame_rate: None,
                audio: None,
                video: None,
                subtitles: None,
                closed_captions: None,
                alternatives: Vec::new(),
            }
        })
        .collect();
    
    Playlist::MasterPlaylist(m3u8_rs::MasterPlaylist {
        version: Some(6),
        variants,
        session_data: None,
        session_key: None,
        start: None,
        independent_segments: false,
    })
}

async fn save_merged_playlist(playlist: &Playlist) -> Result<(), AppError> {
    let mut file = File::create("merged_playlist.m3u8")?;
    let content = match playlist {
        Playlist::MasterPlaylist(pl) => m3u8_rs::MasterPlaylist::to_string(pl)?,
        Playlist::MediaPlaylist(pl) => m3u8_rs::MediaPlaylist::to_string(pl)?,
    };
    
    file.write_all(content.as_bytes())?;
    info!("Merged playlist saved to merged_playlist.m3u8");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    setup_logger().await?;
    info!("Starting playlist processor");
    
    // 1. Скачиваем XML
    let xml_file = download_file("http://bit.ly/liwizard").await?;
    
    // 2. Парсим XML
    let playlists_data = parse_playlists(&xml_file)?;
    info!("Found {} playlists in XML", playlists_data.playlists.len());
    
    // 3. Обрабатываем каждый плейлист
    let mut playlist_futures = Vec::new();
    
    for entry in playlists_data.playlists {
        info!("Processing playlist: {:?}", entry.name);
        playlist_futures.push(process_m3u8(&entry.url));
    }
    
    let results = join_all(playlist_futures).await;
    let mut valid_playlists = Vec::new();
    
    for result in results {
        match result {
            Ok(playlist) => valid_playlists.push(playlist),
            Err(e) => warn!("Skipping playlist: {}", e),
        }
    }
    
    // 4. Объединяем плейлисты
    info!("Merging {} valid playlists", valid_playlists.len());
    let merged = merge_playlists(valid_playlists).await;
    
    // 5. Сохраняем результат
    save_merged_playlist(&merged).await?;
    
    info!("Processing completed successfully");
    Ok(())
}