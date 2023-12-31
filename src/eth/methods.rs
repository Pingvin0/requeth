use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockParameter {
    Earliest,
    Latest,
    Pending,
    Number(u32),
}

impl Serialize for BlockParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match *self {
            BlockParameter::Earliest => serializer.serialize_str("earliest"),
            BlockParameter::Latest => serializer.serialize_str("latest"),
            BlockParameter::Pending => serializer.serialize_str("pending"),
            BlockParameter::Number(x) => serializer.serialize_str(&format!("{:x}", x)),
        }
    }
}

#[derive(Debug)]
pub enum RpcMethod {
    ChainId,
    GetBlockByNumber {
        block: BlockParameter,
        include_transactions: bool
    }
}