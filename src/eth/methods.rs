use core::fmt;

use serde::{Serialize, Deserialize};

use super::request::Params;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockParameter {
    Earliest,
    Latest,
    Pending,
    Number(u32),
}

impl fmt::Display for BlockParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockParameter::Earliest => write!(f, "earliest"),
            BlockParameter::Latest => write!(f, "latest"),
            BlockParameter::Pending => write!(f, "pending"),
            BlockParameter::Number(x) => write!(f, "{:x}", x)
        }
    }
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

impl RpcMethod {
    pub fn get_method_name(&self) -> String {
        match self {
            RpcMethod::ChainId => "eth_chainId".to_string(),
            RpcMethod::GetBlockByNumber { block: _, include_transactions: _ } => "eth_getBlockByNumber".to_string(),
        }
    }

    pub fn get_parameters(&self) -> Option<Params> {
        match self {
            RpcMethod::ChainId => None,
            RpcMethod::GetBlockByNumber
                { block, include_transactions }
                => Some(Params::BlockParams(
                    (format!("{}", block), *include_transactions)
                ))
            
        }
    }
}