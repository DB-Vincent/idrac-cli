use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::lib::convert;
use crate::Settings;

#[derive(Debug, Serialize, Deserialize)]
struct DiskInfo {
    #[serde(rename="BlockSizeBytes")]
    block_size_bytes: u64,
    #[serde(rename="CapableSpeedGbs")]
    capable_speed_gbps: u8,
    #[serde(rename="CapacityBytes")]
    capacity_bytes: u64,
    #[serde(rename="Description")]
    description: String,
    #[serde(rename="FailurePredicted")]
    failure_predicted: bool,
    #[serde(rename="HotspareType")]
    hotspare_type: String,
    #[serde(rename="Links")]
    links: Links,
    #[serde(rename="Manufacturer")]
    manufacturer: String,
    #[serde(rename="MediaType")]
    media_type: String,
    #[serde(rename="Model")]
    model: String,
    #[serde(rename="NegotiatedSpeedGbs")]
    negotaited_speed_gbps: u8,
    #[serde(rename="Protocol")]
    protocol: String,
    #[serde(rename="RotationSpeedRPM")]
    rotation_speed_rpm: u64,
    #[serde(rename="Status")]
    status: Status
}

#[derive(Debug, Serialize, Deserialize)]
struct Links {
    #[serde(rename="Chassis")]
    chassis: Chassis,
    #[serde(rename="Volumes")]
    volumes: Vec<Volume>
}

#[derive(Debug, Serialize, Deserialize)]
struct Chassis {
    #[serde(rename="@odata.id")]
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Volume {
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

#[tokio::main]
pub async fn get_storage_disk(disk: &Option<String>, settings: Settings) -> Result<(), Error> {
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/Storage/Drives/{}", settings.host.to_owned(), disk.as_ref().unwrap()))
        .basic_auth(settings.user, Some(settings.password))
        .send()
        .await
        .unwrap();

    let response_json: DiskInfo = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    println!("{:?}", response_json);

    Ok(())
}