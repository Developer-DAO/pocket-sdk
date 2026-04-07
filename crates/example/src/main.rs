use rustls::crypto::{CryptoProvider, aws_lc_rs::default_provider};
use shannon_protos::pocket::supplier::{
    QueryAllSuppliersRequest, query_client::QueryClient as SupplierClient,
};
use std::collections::HashMap;
use tonic::transport::{Channel, ClientTlsConfig};

const URL: &'static str = "https://sauron-rpc.infra.pocket.network";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get all suppliers on shannon and count how many there is of each type
    let provider: CryptoProvider = default_provider();
    CryptoProvider::install_default(provider).expect("Failed to install aws_lc_rs tls provider");
    let tls_config = ClientTlsConfig::new().with_native_roots();
    let endpoint = Channel::from_shared(URL.to_string())?;
    let ch = endpoint.tls_config(tls_config)?.connect().await?;
    let mut client = SupplierClient::new(ch);

    let req = QueryAllSuppliersRequest {
        pagination: None,
        dehydrated: false,
        service_id: String::from("sui"),
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
