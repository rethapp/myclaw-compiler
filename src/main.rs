use std::fs;
use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("repubblica.it:80").await?;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    let request = format!(
        "GET / HTTP/1.1\r\nHost: repubblica.it\r\nConnection: close\r\n\r\n"
    );

    stream.write_all(request.as_bytes()).await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    let mut news = Vec::new();
    let mut lines = response.lines();

    while let Some(line) = lines.next() {
        if line.starts_with("<title>") {
            let title = line.replace("<title>", "").replace("</title>", "");
            news.push(title);
        }
        if news.len() == 3 {
            break;
        }
    }

    for (i, news) in news.iter().enumerate() {
        println!("{}. {}", i + 1, news);
    }

    Ok(())
}