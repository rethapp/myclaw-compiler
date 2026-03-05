use std::collections::VecDeque;
use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    let mut stream = TcpStream::connect("repubblica.it").expect("Failed to connect to Repubblica.it");
    let mut buffer = VecDeque::new();

    stream.read_to_end(&mut buffer).expect("Failed to read from Repubblica.it");

    let html = String::from_utf8(buffer).expect("Failed to convert buffer to string");

    // Simple HTML parsing to find the main news section
    let start_index = html.find("<div class=\"news-list\">").expect("Failed to find news list");
    let end_index = html.find("</div>").expect("Failed to find closing news list tag");

    let news_section = &html[start_index..end_index];

    let mut news = Vec::new();
    let news_items = news_section.split("<article>");

    for item in news_items {
        if item.contains("<h3>") {
            let title_start = item.find("<h3>").expect("Failed to find news title");
            let title_end = item.find("</h3>").expect("Failed to find closing news title tag");
            let title = &item[title_start..title_end];
            news.push(title);
        }
    }

    // Limit to 3 news
    let news = news.into_iter().take(3).collect();

    for news in news {
        println!("{}", news);
    }
}