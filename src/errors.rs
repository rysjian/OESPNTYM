use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("HTTP request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("XML parse error: {0}")]
    XmlError(#[from] serde_xml_rs::Error),
    
    #[error("M3U8 parse error: {0}")]
    M3u8Error(String),
    
    #[error("Timeout while checking stream")]
    TimeoutError,
    
    #[error("Invalid playlist structure")]
    InvalidPlaylistStructure,
}