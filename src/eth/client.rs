use core::fmt;
use std::future::Future;

use super::{methods::{BlockParameter, RpcMethod}, request::RpcRequest};

#[derive(Debug)]
pub enum ConnectionProtocol {
    HTTP,
    HTTPS
}

impl fmt::Display for ConnectionProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionProtocol::HTTP => write!(f, "http"),
            ConnectionProtocol::HTTPS => write!(f, "https")
        }
    }
}

#[derive(Debug)]
pub enum Authentication {
    None,
    Basic {
        username: String,
        password: String
    }
}

#[derive(Debug)]
pub struct EthRpcClient {
    pub host: String,
    pub port: u16,
    pub protocol: ConnectionProtocol,
    pub authentication: Authentication,
    uri: String
}



impl EthRpcClient {
    fn get_uri(protocol: &ConnectionProtocol, host: &String, port: &u16) -> String {
        return format!("{}://{}:{}/", protocol, host, port);
    }

    pub fn new(host: String, port: u16, protocol: ConnectionProtocol, authentication: Authentication) -> Self {
        return Self {
            uri: Self::get_uri(&protocol, &host, &port),
            authentication: authentication,
            host: host,
            port: port,
            protocol: protocol,
        };
    }
 //curl -H "Content-Type: application/json" -X POST --data '{"jsonrpc":"2.0","method":"web3_clientVersion","params":[],"id":67}' 127.0.0.1:8545
    pub fn make_request(&self, method: RpcMethod) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
        let client = reqwest::Client::new();
        

        let mut requestBuilder = client.post(self.uri.clone());

        if let Authentication::Basic { username, password } = &self.authentication {
            requestBuilder = requestBuilder
            .basic_auth(username, Some(password));
        }

        match method {
            RpcMethod::ChainId => {
                let rpc_request = RpcRequest {
                    jsonrpc: "2".to_string(),
                    method: "eth_chainId".to_string(),
                    params: None,
                    id: 1
                };

                requestBuilder = requestBuilder.json(&rpc_request);
            },
            RpcMethod::GetBlockByNumber { block, include_transactions } => ()
        }

        requestBuilder.send()
    }

    pub async fn get_block(&self, param: &BlockParameter) {

    }
}