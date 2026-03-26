use std::collections::HashMap;

use shannon_protos::{
    cosmos::base::v1beta1::Coin,
    pocket::{
        application::{
            QueryGetApplicationRequest,
            query_client::QueryClient as ApplicationClient,
        },
        supplier::{QueryAllSuppliersRequest, query_client::QueryClient as SupplierClient},
    },
};

const URL: &'static str = "https://sauron-rpc.infra.pocket.network";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // Get all suppliers on shannon and count how many there is of each type
    let mut client = SupplierClient::connect(URL).await?;
    let req = QueryAllSuppliersRequest {
        pagination: None,
        dehydrated: false,
        service_id: String::from(""),
        operator_address: String::from(""),
        owner_address: String::from(""),
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

    println!("{res:#?}");

    let mut client = ApplicationClient::connect(URL).await?;

    let req = QueryGetApplicationRequest {
        address: String::new(),
    };
    let res = client.application(req).await.expect("");
    let qga = res.into_inner();
    if let Some(app) = qga.application {
        println!(
            "App Stake Amount: {}upokt",
            app.stake
                .unwrap_or(Coin {
                    denom: String::from("upokt"),
                    amount: String::from("0")
                })
                .amount
        );
    }

    Ok(())
}
