use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::Settings;

#[derive(Debug, Serialize, Deserialize)]
struct StorageControllerInfo {
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="Drives")]
    drives: Vec<Drive>,
    #[serde(rename="Status")]
    status: Status,
    #[serde(rename="StorageControllers")]
    storage_controllers: Vec<StorageController>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Drive {
    #[serde(rename="@odata.id")]
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    #[serde(rename="Health")]
    health: String,
    #[serde(rename="State")]
    state: String
}

#[derive(Debug, Serialize, Deserialize)]
struct StorageController {
    #[serde(rename="@odata.id")]
    name: String,
    #[serde(rename="FirmwareVersion")]
    firmware_version: String,
    #[serde(rename="Manufacturer")]
    manufacturer: String,
    #[serde(rename="Model")]
    model: String,
    #[serde(rename="SpeedGbps")]
    speed_gbps: u8,
    #[serde(rename="Status")]
    status: Status,
    #[serde(rename="SupportedControllerProtocols")]
    controller_protocols: Vec<String>,
    #[serde(rename="SupportedDeviceProtocols")]
    device_protocols: Vec<String>
}

#[tokio::main]
pub async fn get_storage_controller(storage_controller: &Option<String>, settings: Settings) -> Result<(), Error> {
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/Storage/{}", settings.host.to_owned(), storage_controller.as_ref().unwrap()))
        .basic_auth(settings.user, Some(settings.password))
        .send()
        .await
        .unwrap();

    let response_json: StorageControllerInfo = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    println!("Device name: {}", response_json.name);
    println!("Storage controller:");
    for storage_controller in response_json.storage_controllers {
        println!("- Name:             {}", &storage_controller.name.replace("/redfish/v1/Systems/System.Embedded.1/StorageControllers/", ""));
        println!("  Firmware version: {}", storage_controller.firmware_version);
        println!("  Device type:      {} {}", storage_controller.manufacturer, storage_controller.model);
        println!("  Speed:            {} Gbps", storage_controller.speed_gbps);
        if storage_controller.status.state == "Enabled" { println!("  Status: {}", storage_controller.status.health) }
        println!("  Controller protocols:");
        for protocol in storage_controller.controller_protocols {
            println!("    - {}", protocol);
        }
        println!("  Device protocols:");
        for protocol in storage_controller.device_protocols {
            println!("    - {}", protocol);
        }
    }
    println!("Attached drives:");
    for drive in response_json.drives {
        let long_name = &drive.name;
        let short_name = long_name.replace("/redfish/v1/Systems/System.Embedded.1/Storage/Drives/", "");

        println!("- {}", short_name)
    }

    Ok(())
}