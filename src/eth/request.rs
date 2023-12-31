use serde::Serialize;
use serde_json::{Value, Map};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum Params {
    Array(Vec<Value>),
    Map(Map<String, Value>),
    BlockParams((String, bool)),
    None(())
}

#[derive(Clone, PartialEq, Debug, Serialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Params>,
    pub id: i32
}