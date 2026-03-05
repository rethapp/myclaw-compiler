use reqwest;
use serde_json::Value;

#[derive(serde::Deserialize)]
struct NewsItem {
    title: String,
    link: String,
    pub_date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://repubblica.it/api/v2/news";
    let response = reqwest::get(url).await?;
    let news: Vec<NewsItem> = response.json().await?;

    println!("Latest News:");
    for (i, news_item) in news.iter().enumerate().take(3) {
        println!("#{} {} - {}", i + 1, news_item.title, news_item.pub_date);
        println!("Link: {}", news_item.link);
        println!();
    }

    Ok(())
}