#[allow(unused_imports)]
use xivapi_rust::{builder::XIVAPIBuilder, enums::Language, WAKINGSANDS_URL, XIVAPI};

#[tokio::main]
async fn main() {
    let api_key = "your_api_key".to_string();
    let builder = XIVAPIBuilder::new(api_key)
        .language(Language::CN)
        .url(WAKINGSANDS_URL)
        .build();
    let mut client = XIVAPI::new(builder);
    let res = client.endpoint("item").send_request("1675").await;
    let response = res.expect("Failed to get response");
    let json = response.json::<serde_json::Value>().await;
    println!("{:?}", json);
}
