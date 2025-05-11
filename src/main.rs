use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

// Определяем структуру данных, соответствующую JSON
#[derive(Debug, Serialize, Deserialize)]
struct WizardData {
    // Замените эти поля на реальные из вашего JSON
    name: Option<String>,
    spells: Option<Vec<String>>,
    level: Option<u32>,
    // Добавьте другие поля по мере необходимости
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Скачиваем файл
    let url = "http://bit.ly/lidwizard";
    let response = reqwest::get(url).await?;
    
    // Парсим JSON прямо из ответа
    let wizard_data: WizardData = response.json().await?;
    
    // Выводим результат
    println!("{:#?}", wizard_data);
    
    // Здесь можно добавить обработку данных
    if let Some(name) = wizard_data.name {
        println!("Wizard name: {}", name);
    }
    
    if let Some(spells) = wizard_data.spells {
        println!("Known spells:");
        for spell in spells {
            println!("- {}", spell);
        }
    }

    Ok(())
}