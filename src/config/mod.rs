use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct GatewayConfig {
    pub http: HttpConfig,
    pub routes: Vec<RouteConfig>,
    pub pipelines: HashMap<String, Vec<PolicyConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct RouteConfig {
    pub path: String,
    #[serde(default)]
    pub methods: Vec<HttpMethod>,
    pub target: String,
    pub pipeline: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PolicyConfig {
    Named(String), // e.g., just "cors"
    WithConfig(HashMap<String, serde_yaml::Value>),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}