use reqwest;
use select::document::Document;
use select::predicate::{Name, Attr};
use serde::Deserialize;
use std::fs;
use select::predicate::Predicate;

#[derive(Deserialize)]
struct Config {
    client_id: String,
    client_secret: String,
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct Meta {
    status: i32,
}

#[derive(Debug, Deserialize)]
struct HitResult {
    result: SongResult
}

#[derive(Debug, Deserialize)]
struct Response {
    hits: Vec<HitResult>,
}

#[derive(Debug, Deserialize)]
struct SongResult {
    url: String
    // Здесь можно добавить другие поля, если они вам нужны
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    meta: Meta,
    response: Response,
}


fn load_config() -> Config {
    let contents = fs::read_to_string("config.toml").expect("Error reading the config file");
    toml::from_str(&contents).expect("Error parsing the config file")
}

pub fn get_lyrics_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    let document = Document::from_read(resp.as_bytes())?;

    let song_lyrics = document.find(Name("div").and(Attr("class", "Lyrics__Container-sc-1ynbvzw-1 kUgSbL"))).next();

    match song_lyrics {
        Some(lyrics) => Ok(lyrics.text()),
        None => Err(Box::from("Не удалось найти текст песни")),
    }

}

pub fn get_lyrics(performer:&str, title:&str) -> Result<String, Box<dyn std::error::Error>> {
    
    //тут конкатенация и обработка отправленных слов
    let performer_lower = performer.to_lowercase();
    let title_lower = title.to_lowercase();
    let performer_dashed = performer_lower.replace(" ", "-");
    let title_dashed = title_lower.replace(" ", "-");
    let url = format!("https://genius.com/{}-{}-lyrics", performer_dashed, title_dashed);
    
    //отправка HTTP-запроса к женису для поиска текста песни по заданному исполнителю и названию
    let client = reqwest::blocking::Client::new();
    let config = load_config(); //не забыть про конфиг 
    let access_token = config.access_token;
    
    let search_query = format!("{} {}", performer, title);

    let res = client.get("https://api.genius.com/search")
        .query(&[("q", &search_query)])
        .header("Authorization", format!("Bearer {}", access_token))
        .send()?;
    
    let res_text = res.text()?;  // Получение текста ответа
    let api_response: ApiResponse = serde_json::from_str(&res_text)?;  // Разбор JSON

    // Извлечение URL из api_response (этот шаг будет зависеть от структуры вашего JSON)
    let song_url = api_response.response.hits[0].result.url.clone();

    // Теперь можно использовать этот URL для получения текста песни
    let lyrics = get_lyrics_url(&song_url)?;

    Ok(lyrics)

}

