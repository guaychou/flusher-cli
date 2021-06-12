use {
    getset::{Getters, Setters},
    log::info,
    serde::{Deserialize, Serialize},
    std::fmt,
};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename(deserialize = "requestId"))]
    request_id: String,
    redis: RedisData,
}

#[derive(Debug, Deserialize, Getters)]
pub struct FlusherVault {
    #[getset(get = "pub with_prefix")]
    username: String,
    #[getset(get = "pub with_prefix")]
    password: String,
    #[getset(get = "pub with_prefix")]
    #[serde(rename(deserialize = "flusherDefaultPassword"))]
    flusher_default_password: String,
}

#[derive(Debug, Deserialize, Getters)]
pub struct FlusherVaultApps {
    #[serde(rename(deserialize = "flusherPassword"))]
    #[getset(get = "pub with_prefix")]
    flusher_password: String,
}

#[derive(derivative::Derivative, PartialEq, Deserialize, Serialize, Setters, Getters)]
#[derivative(Debug)]
pub struct RedisData {
    /// Operation
    #[getset(get = "pub with_prefix")]
    operation: Operation,
    /// Redis address
    address: String,
    /// Redis Port
    port: i32,
    /// Master group name of redis sentinel
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub with_prefix")]
    master: Option<String>,
    /// Key pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    /// Database
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<i32>,
    /// Redis Password
    #[serde(skip_serializing_if = "Option::is_none")]
    #[derivative(Debug = "ignore")]
    #[getset(set = "pub with_prefix")]
    password: Option<String>,
    /// Redis Sentinel Password
    #[serde(
        rename(deserialize = "sentinelPassword", serialize = "sentinelPassword"),
        skip_serializing_if = "Option::is_none"
    )]
    #[derivative(Debug = "ignore")]
    #[getset(set = "pub with_prefix")]
    sentinel_password: Option<String>,

    /// Check if we need redis auth to connect with, it will applied to all redis password (sentinel and standalone)
    #[getset(get = "pub with_prefix")]
    #[serde(default = "bool::default")]
    auth: bool,
}

/// Operation available
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Operation {
    FlushAll,
    FlushAllAsync,
    FlushDB,
    FlushDbAsync,
    DelKeyByPrefix,
}

impl Operation {
    pub fn suffix(&self) -> &str {
        match self {
            Self::FlushAll => "/api/v1/flush",
            Self::FlushAllAsync => "/api/v1/flushasync",
            Self::FlushDB => "/api/v1/flushdb",
            Self::FlushDbAsync => "/api/v1/flushdbasync",
            Self::DelKeyByPrefix => "/api/v1/prefixdelkey",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlusherResponse {
    success: bool,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct FlusherResponseError {
    success: bool,
    error: String,
}

#[derive(Deserialize, Debug)]
pub struct FlusherHealthyResponse {
    success: bool,
    message: String,
    version: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum FlusherResult {
    Ok(FlusherResponse),
    Err(FlusherResponseError),
}

impl fmt::Display for FlusherResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Success: {} | Message: {}", self.success, self.message)
    }
}

impl fmt::Display for FlusherResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Success: {} | Message: {}", self.success, self.error)
    }
}

impl fmt::Display for FlusherHealthyResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Flusher Health Check Status: {}, Running version: {}",
            self.message, self.version
        )
    }
}

pub fn read_yaml(config_path: &str) -> RedisData {
    let f = std::fs::File::open(config_path).expect("Couldn't found file");
    let d: Data = serde_yaml::from_reader(f).expect("Parse failed");
    info!(
        "Data has been read from config for RequestID : {}",
        d.request_id
    );
    return d.redis;
}
