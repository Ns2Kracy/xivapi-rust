use crate::enums::Language;
use reqwest::Client;

pub struct XIVAPIBuilder {
    pub client: Client,
    pub api_key: String,
    pub api_url: String,
    pub language: String,
}


impl XIVAPIBuilder {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key,
            api_url: "https://xivapi.com".to_string(),
            language: String::default(),
        }
    }
    pub fn url(&mut self, url: &str) -> &mut Self {
        self.api_url = url.to_string();
        self
    }
    pub fn language(&mut self, language: Language) -> &mut Self {
        self.language = language.to_string();
        self
    }
    pub fn build(&self) -> Self {
        Self {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            api_url: self.api_url.clone(),
            language: self.language.clone(),
        }
    }
}
