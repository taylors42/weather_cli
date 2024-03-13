use reqwest;
use serde_json::Value;
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://ipinfo.io/json?token=917f90adf26853";
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let client = reqwest::Client::new();
        let body = response.text().await?;
        let body_json: Value = serde_json::from_str(&body).unwrap();
        let json_loc = body_json["loc"].as_str().unwrap();
        let replace_string = json_loc.replace(",", "%2C");
        let geolocation_url = format!("https://weatherapi-com.p.rapidapi.com/current.json?q={}",replace_string);
        let geolocation_request = client.get(geolocation_url)
        .header("X-RapidAPI-Key", "6af0087817msh11552228f1f7680p194fc6jsn88e02f9db09e")
        .header("X-RapidAPI-Host", "weatherapi-com.p.rapidapi.com")
        .send()
        .await?;
        let geolocation_body = geolocation_request.text().await?;
        let geolocation_json: Value = serde_json::from_str(&geolocation_body).unwrap();
        println!("Your data:");
        println!("IP -> {}", body_json["ip"]);
        println!("Hum -> {}", geolocation_json["current"]["humidity"]);
        println!("TEMP -> {}", geolocation_json["current"]["temp_c"]);
    } else {
        println!("Error in the request: {}", response.status());
    }
    Ok(())
}

