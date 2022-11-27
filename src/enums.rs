pub enum Language {
    /// English
    EN,
    /// Japanese
    JA,
    /// German
    DE,
    /// French
    FR,
    /// Chinese
    CN,
}

impl Language {
    pub fn to_string(&self) -> String {
        match self {
            Language::EN => "en".to_string(),
            Language::JA => "ja".to_string(),
            Language::DE => "de".to_string(),
            Language::FR => "fr".to_string(),
            Language::CN => "cn".to_string(),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::EN => write!(f, "EN"),
            Language::JA => write!(f, "JA"),
            Language::DE => write!(f, "DE"),
            Language::FR => write!(f, "FR"),
            Language::CN => write!(f, "CN"),
        }
    }
}
