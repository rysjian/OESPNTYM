use reqwest;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL для скачивания (bit.ly ссылка ведёт на liwizard)
    let url = "http://bit.ly/liwizard";
    
    // Создаём клиента reqwest
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;
    
    // Отправляем GET запрос
    let response = client.get(url).send().await?;
    
    // Получаем имя файла из URL или используем имя по умолчанию
    let file_name = response.url()
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("downloaded_file");
    
    println!("Скачивание файла: {}", file_name);
    
    // Создаём файл для сохранения
    let mut dest = {
        let mut path = PathBuf::from(".");
        path.push(file_name);
        File::create(path)?
    };
    
    // Копируем содержимое ответа в файл
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;
    
    println!("Файл успешно скачан!");
    
    Ok(())
}