use reqwest;
use serde_json::Value;
use std::collections::HashMap;

fn main() {
    let url = "https://www.repubblica.it/rss/ultime-notizie.xml";
    let response = reqwest::get(url).unwrap();
    let xml_content = response.text().unwrap();

    let mut news = Vec::new();
    let mut parser = xmltree::XMLTree::parse(&xml_content).unwrap();
    let channel = parser.descendants().find(|node| node.tag.name == "channel").unwrap();

    for item in channel.descendants().filter(|node| node.tag.name == "item") {
        let title = item.descendants().find(|node| node.tag.name == "title").unwrap().text().unwrap();
        let link = item.descendants().find(|node| node.tag.name == "link").unwrap().text().unwrap();
        news.push((title, link));
    }

    let mut top_news: Vec<_> = news.into_iter().take(3).collect();
    top_news.sort_by(|a, b| a.0.cmp(&b.0));

    for (title, link) in top_news {
        println!("Title: {}", title);
        println!("Link: {}", link);
        println!();
    }
}