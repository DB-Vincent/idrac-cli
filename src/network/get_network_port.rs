use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::Settings;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkAdapterList {
    #[serde(rename="ActiveLinkTechnology")]
    technology: String,
    #[serde(rename="AssociatedNetworkAddresses")]
    addresses: Vec<String>,
    #[serde(rename="LinkStatus")]
    link_status: String,
    #[serde(rename="PhysicalPortNumber")]
    physical_port_number: String,
    #[serde(rename="SupportedEthernetCapabilities")]
    supported_ethernet_capabilities: Vec<String>,
    #[serde(rename="SupportedLinkCapabilities")]
    supported_link_capabilities: Vec<LinkCapability>,
    #[serde(rename="WakeOnLANEnabled")]
    wol_enabled: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct LinkCapability {
    #[serde(rename="LinkNetworkTechnology")]
    technology: String,
    #[serde(rename="LinkSpeedMbps")]
    speed_mbps: u64,
}

#[tokio::main]
pub async fn retrieve_port_info(network_adapter: &String, port: &String, settings: &Settings) -> Result<NetworkAdapterList, Error> {
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/NetworkAdapters/{}/NetworkPorts/{}", settings.host.to_owned(), network_adapter, port))
        .basic_auth(&settings.user, Some(&settings.password))
        .send()
        .await
        .unwrap();

    let response_json: NetworkAdapterList = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    // println!("{:?}", response_json);
    Ok(response_json)
}

pub fn get_network_port(network_adapter: &String, port: &String, settings: &Settings) {
    match retrieve_port_info(network_adapter, port, settings) {
        Ok(response) => println!("{:?}", response),
        Err(err) => println!("Error: {}", err),
    }
}