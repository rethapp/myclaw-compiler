use reqwest;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://repubblica.it/rss/ultima-notizia.xml";
    let response = reqwest::get(url)?;
    let news: Value = response.json()?;

    let mut latest_news = vec![];
    for item in news["rss"]["channel"]["item"].as_array().unwrap() {
        let title = item["title"].as_str().unwrap();
        let link = item["link"].as_str().unwrap();
        latest_news.push((title, link));
    }

    for (i, (title, link)) in latest_news.iter().enumerate() {
        println!("{}. {}", i + 1, title);
        println!("Link: {}", link);
        if i == 2 {
            break;
        }
    }

    Ok(())
}