use async_trait::async_trait;

use crate::XIVAPI;

pub enum StringAlgo {
    /// wildcard + fuzzy
    Custom,
    /// wildcard
    Wildcard,
    WildcardPlus,
    Fuzzy,
    Term,
    Prefix,
    Match,
    MatchPrase,
    MatchPrasePrefix,
    MultiMatch,
    QueryString,
}

pub struct SearchParams {
    /// Search a specific series of indexes separated by commas.
    pub indexes: Option<String>,
    /// The string to search for. The results for this are affected by string_column and string_algo.
    /// Default: wildcard
    pub string: String,
    /// The search algorithm to use for string matching.
    pub string_algo: Option<StringAlgo>,
}


// 添加泛型约束, 使得可以自由的传入不同数量的参数
// 但是 string 和 filter 是必须的
#[async_trait]
trait Search {
    async fn search(&mut self, params: &str) -> Result<reqwest::Response, reqwest::Error>;
}


#[async_trait]
impl Search for XIVAPI {
    async fn search(&mut self, _params: &str) -> Result<reqwest::Response, reqwest::Error>{
        todo!()
    }
}