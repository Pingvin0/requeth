use serde::Serialize;
use serde_json::{Value, Map};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum RpcParams {
    Array(Vec<Value>),
    Map(Map<String, Value>),
    None(())
}

#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<RpcParams>,
    pub id: i32
}