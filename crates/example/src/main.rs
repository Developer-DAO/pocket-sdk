use std::collections::HashMap;

use shannon_protos::{
    cosmos_sdk_proto::cosmos::bank::v1beta1::{
        QueryBalanceRequest, query_client::QueryClient as BankQueryClient,
    },
    pocket::supplier::{QueryAllSuppliersRequest, query_client::QueryClient as SupplierClient},
};
use tonic::transport::{Channel, ClientTlsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all suppliers on shannon and count how many there is of each type
    let mut client =
        SupplierClient::connect("https://shannon-grove-grpc.mainnet.poktroll.com").await?;
    let req = QueryAllSuppliersRequest {
        pagination: None,
        filter: None,
        dehydrated: false,
    };

    let res = client
        .all_suppliers(req)
        .await?
        .into_inner()
        .supplier
        .into_iter()
        .fold(HashMap::new(), |mut acc, e| {
            e.services
                .into_iter()
                .for_each(|s| match acc.get_mut(&s.service_id) {
                    Some(v) => *v += 1,
                    None => {
                        acc.insert(s.service_id, 1);
                    }
                });
            acc
        });

    println!("{res:#?}\n");

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
