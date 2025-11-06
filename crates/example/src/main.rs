use std::collections::HashMap;

use shannon_protos::{
    pocket::{
        supplier::{QueryAllSuppliersRequest, query_client::QueryClient as SupplierClient},
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all suppliers on shannon and count how many there is of each type
    let mut client =
        SupplierClient::connect("https://shannon-grove-grpc.mainnet.poktroll.com").await?;
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

    Ok(())
}
