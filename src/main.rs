use reqwest;
use serde_json::Value;

fn get_latest_news() -> Result<Vec<String>, reqwest::Error> {
    let url = "https://repubblica.it/rss/ultimi-news/";
    let response = reqwest::get(url)?.text()?;
    let json: Value = serde_json::from_str(&response)?;

    let mut news = Vec::new();
    for item in json["rss"]["channel"]["item"].as_array().unwrap() {
        news.push(item["title"].as_str().unwrap().to_string());
        if news.len() == 3 {
            break;
        }
    }

    Ok(news)
}

fn main() {
    match get_latest_news() {
        Ok(news) => {
            for news in news {
                println!("{}", news);
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch news: {}", e);
        }
    }
}