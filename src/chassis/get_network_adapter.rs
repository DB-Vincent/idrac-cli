use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};

use crate::chassis::get_network_port::NetworkAdapterList;
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
pub async fn get_network_adapter(network_adapter: &Option<String>, settings: Settings, detailed: bool) -> Result<(), Error> {
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/NetworkAdapters/{}", settings.host.to_owned(), network_adapter.as_ref().unwrap()))
        .basic_auth(&settings.user, Some(&settings.password))
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
    println!("Serial number: {}\n", response_json.serial_number);

    let mut port_list: Vec<String> = Vec::new();

    for controller in response_json.controllers.iter(){
        println!("Found {} ports on controller:", controller.links.port_count);

        for link in &controller.links.network_ports {
            let long_name = &link.name;
            let short_name = long_name.replace(&format!("/redfish/v1/Systems/System.Embedded.1/NetworkAdapters/{}/NetworkPorts/", network_adapter.as_ref().unwrap()), "");

            port_list.push(short_name);
        }
    }

    if detailed {
        for port in port_list {
            println!("- {}", port);

            let response = Client::builder()
                .danger_accept_invalid_certs(true)
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap()
                .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/NetworkAdapters/{}/NetworkPorts/{}", settings.host.to_owned(), network_adapter.as_ref().unwrap(), port))
                .basic_auth(&settings.user, Some(&settings.password))
                .send()
                .await
                .unwrap();

            let response_json: NetworkAdapterList = match response.json().await {
                Ok(r) => r,
                Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
            };
            println!("{:?}", response_json)

        }
    } else {
        for port in port_list {
            println!("- {}", port);
        }
    }

    Ok(())
}