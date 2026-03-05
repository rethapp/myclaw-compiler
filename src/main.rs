use reqwest::get;
use serde_json::Value;

fn main() {
    let url = "https://www.repubblica.it/rss/news/rss.xml";
    let response = get(url).expect("Failed to fetch the news feed");
    let xml_content = response.text().expect("Failed to read the XML content");

    let news: Vec<Value> = serde_xml_rs::from_str(&xml_content).expect("Failed to parse the XML into JSON");
    let mut top_news = Vec::new();

    for news_item in &news {
        if let Some(title) = news_item.get("title") {
            top_news.push(title.as_str().unwrap().to_string());
            if top_news.len() == 3 {
                break;
            }
        }
    }

    println!("Top 3 news articles:");
    for (index, article) in top_news.iter().enumerate() {
        println!("{}. {}", index + 1, article);
    }
}