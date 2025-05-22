use shannon_protos::{
    cosmos_sdk_proto::cosmos::bank::v1beta1::{
        QueryBalanceRequest, query_client::QueryClient as BankQueryClient,
    },
    pocket::gateway::{QueryAllGatewaysRequest, query_client::QueryClient as GatewayQueryClient},
};
use tonic::transport::{Channel, ClientTlsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // example of interacting with shannon endpoints

    let mut client =
        GatewayQueryClient::connect("https://shannon-testnet-grove-grpc.beta.poktroll.com").await?;
    let req = QueryAllGatewaysRequest { pagination: None };
    let res = client.all_gateways(req).await?.into_inner().gateways;
    println!("{res:?}\n");

    // example of getting balances

    let ch = Channel::from_static("https://shannon-testnet-grove-grpc.beta.poktroll.com")
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;

    let mut client = BankQueryClient::new(ch);

    let req = QueryBalanceRequest {
        address: "pokt1xjylmpd9y257t59z4js5tnxkae7v4d2zr4seh9".to_string(),
        denom: "upokt".to_string(),
    };

    let bal_res = client.balance(req).await?.into_inner();
    println!("{bal_res:?}");

    Ok(())
}
