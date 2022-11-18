use builder::XIVAPIBuilder;

pub mod builder;
pub mod enums;

pub const XIVAPI_URL: &'static str = "https://xivapi.com";

/// This is the chinese version of the API
/// more info: https://github.com/thewakingsands/cafemaker/wiki
pub const WAKINGSANDS_URL: &'static str = "https://cafemaker.wakingsands.com";

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
        let builder = XIVAPIBuilder::new(
            "370234866b404d39a27f195d75c18766e06c06d3ac264cfe8a7621b39ee6a9a1".to_string(),
        )
        .language(Language::CN)
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
        let builder = XIVAPIBuilder::new(
            "370234866b404d39a27f195d75c18766e06c06d3ac264cfe8a7621b39ee6a9a1".to_string(),
        )
        .language(Language::CN)
        .url(WAKINGSANDS_URL)
        .build();
        let mut client = XIVAPI::new(builder);
        let res = client.endpoint("Action").send_request("127").await;
        let response = res.expect("Failed to get response");
        let json = response.json::<serde_json::Value>().await;
        println!("{:?}", json);
    }
}
