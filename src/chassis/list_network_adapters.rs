use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::Settings;

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdapterList {
    Name: String,
    Members: Vec<NetworkAdaptersMember>
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdaptersMember {
    #[serde(rename="@odata.id")]
    Name: String
}

#[tokio::main]
pub async fn list_network_adapters(settings: Settings) -> Result<(), Error> {
    println!("Retrieving list of network interfaces.. (this may take a while)");

    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/NetworkAdapters", settings.host.to_owned()))
        .basic_auth(settings.user, Some(settings.password))
        .send()
        .await
        .unwrap();

    let response_json: NetworkAdapterList = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    println!("Network adapters: {:?}", response_json);
    Ok(())
}