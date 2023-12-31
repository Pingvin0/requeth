use eth::client;

mod eth;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let conn = client::EthRpcClient::new(
        "eth.llamarpc.com".to_string(),
        443,
        client::ConnectionProtocol::HTTPS,
        client::Authentication::None
    );
    
    let r = conn.make_request(eth::methods::RpcMethod::GetBlockByNumber { block: eth::methods::BlockParameter::Latest, include_transactions: false }).await?;

    println!("{}", r.text().await?);
    

    Ok(())
}
