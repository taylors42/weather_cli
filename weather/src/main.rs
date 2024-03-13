use reqwest;
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://ipinfo.io/json?token=917f90adf26853";
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Error in the request: {}", response.status());
    }
    Ok(())
}

