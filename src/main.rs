use reqwest;
//use scraper::Html;
use tokio;

#[tokio::main]
async fn main() {
    let url = "https:allo.ua";
    let html = fetch_url(url).await.unwrap();
    println!("{:?}", html);
}

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;

    Ok(response)
}
