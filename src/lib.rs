use builder::XIVAPIBuilder;

pub mod builder;
pub mod enums;

pub const XIVAPI_URL: &'static str = "https://xivapi.com";

/// This is the chinese version of the API
/// more info: https://github.com/thewakingsands/cafemaker/wiki
pub const WAKINGSANDS_URL: &'static str = "https://cafemaker.wakingsands.com";

/// The main struct for interacting with the XIVAPI.
/// ```rust
/// use xivapi_rust::XIVAPI;
/// use xivapi_rust::enums::Language;
/// use xivapi_rust::builder::XIVAPIBuilder;
/// use xivapi_rust::WAKINGSANDS_URL;
///
/// #[tokio::main]
/// async fn main() {
///     let api_key = "your_api_key".to_string();
///     let language = Language::CN;
///     let builder = XIVAPIBuilder::new(api_key)
///     .language(Language::CN)
///     .url(WAKINGSANDS_URL)
///     .build();
///     let mut client = XIVAPI::new(builder);
///     let res = client.endpoint("item").send_request("1675").await;
///     let response = res.expect("Failed to get response");
///     let json = response.json::<serde_json::Value>().await;
///     println!("{:?}", json);
/// }
/// ```
/// the client should always be mutable
/// so you can use the same client for multiple requests through change the endpoint
/// This will help you get started with the XIVAPI.
pub struct XIVAPI {
    pub builder: XIVAPIBuilder,
    pub endpoint: String,
}

impl XIVAPI {
    pub fn new(builder: XIVAPIBuilder) -> Self {
        Self {
            builder,
            endpoint: String::default(),
        }
    }
    pub fn endpoint(&mut self, endpoint: &str) -> &mut Self {
        self.endpoint = endpoint.to_string();
        self
    }
    pub async fn send_request(&self, params: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/{}?{}", self.builder.api_url, self.endpoint, params);
        self.builder
            .client
            .request(reqwest::Method::GET, url)
            .send()
            .await
    }
}

#[cfg(test)]
mod test {
    use crate::{builder::XIVAPIBuilder, enums::Language, WAKINGSANDS_URL, XIVAPI};

    #[tokio::test]
    async fn test_basic() {
        let api_key = "your_api_key".to_string();
        let language = Language::CN;
        let builder = XIVAPIBuilder::new(api_key)
            .language(language)
            .url(WAKINGSANDS_URL)
            .build();
        let mut client = XIVAPI::new(builder);
        let res = client.endpoint("item").send_request("1675").await;
        let response = res.expect("Failed to get response");
        let json = response.json::<serde_json::Value>().await;
        println!("{:?}", json);
    }

    #[tokio::test]
    async fn test_action() {
        let api_key = "your_api_key".to_string();
        let language = Language::CN;
        let builder = XIVAPIBuilder::new(api_key)
            .language(language)
            .url(WAKINGSANDS_URL)
            .build();
        let mut client = XIVAPI::new(builder);
        let res = client.endpoint("Action").send_request("127").await;
        let response = res.expect("Failed to get response");
        let json = response.json::<serde_json::Value>().await;
        println!("{:?}", json);
    }
}
