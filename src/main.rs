use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    
    let url = "https://doc.rust-lang.ru/stable/rust-by-example/meta/doc.html";
    let _response = fetch_url(url).await;
    //println!("{:?}", response);
    
}

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    println!("{}", response);
    Ok(response)
}








//https://www.thetimes.co.uk/