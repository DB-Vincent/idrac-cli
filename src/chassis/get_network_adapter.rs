use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::Settings;

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdapterInfo {
    #[serde(rename="Manufacturer")]
    manufacturer: String,
    #[serde(rename="Model")]
    model: String,
    #[serde(rename="PartNumber")]
    part_number: String,
    #[serde(rename="SerialNumber")]
    serial_number: String,
    #[serde(rename="Controllers")]
    controllers: Vec<NetworkAdapterController>
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdapterController {
    #[serde(rename="FirmwarePackageVersion")]
    firmware_package_version: String,
    #[serde(rename="Links")]
    links:NetworkAdapterControllerLink
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdapterControllerLink {
    #[serde(rename="NetworkPorts@odata.count")]
    port_count: u8,
    #[serde(rename="NetworkPorts")]
    network_ports: Vec<NetworkAdapterControllerPort>
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdapterControllerPort {
    #[serde(rename="@odata.id")]
    name: String
}

#[tokio::main]
pub async fn get_network_adapter(network_adapter: &Option<String>, settings: Settings) -> Result<(), Error> {
    println!("Retrieving network interface info.. (this may take a while)");

    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/NetworkAdapters/{}", settings.host.to_owned(), network_adapter.as_ref().unwrap()))
        .basic_auth(settings.user, Some(settings.password))
        .send()
        .await
        .unwrap();
    let response_json: NetworkAdapterInfo = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    println!("NIC:           {}", network_adapter.as_ref().unwrap());
    println!("Manufacturer:  {}", response_json.manufacturer);
    println!("Model:         {}", response_json.model);
    println!("Part number:   {}", response_json.part_number);
    println!("Serial number: {}", response_json.serial_number);

    println!("\n");

    for controller in response_json.controllers.iter(){
        for link in &controller.links.network_ports {
            println!("{}", link.name)
        }
    }

    Ok(())
}