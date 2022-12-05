use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::{NetworkAdapter, Settings};

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdapterInfo {
    Manufacturer: String,
    Model: String,
    PartNumber: String,
    SerialNumber: String,
    Controllers: Vec<NetworkAdapterController>
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdapterController {
    FirmwarePackageVersion: String,
    Links:NetworkAdapterControllerLink
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdapterControllerLink {
    #[serde(rename="NetworkPorts@odata.count")]
    PortCount: u8,
    NetworkPorts: Vec<NetworkAdapterControllerPort>
}

#[derive(Debug, Serialize, Deserialize)]
struct NetworkAdapterControllerPort {
    #[serde(rename="@odata.id")]
    Name: String
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
    println!("Manufacturer:  {}", response_json.Manufacturer);
    println!("Model:         {}", response_json.Model);
    println!("Part number:   {}", response_json.PartNumber);
    println!("Serial number: {}", response_json.SerialNumber);

    println!("\n");

    for controller in response_json.Controllers.iter(){
        for link in &controller.Links.NetworkPorts {
            println!("{}", link.Name)
        }
    }

    Ok(())
}