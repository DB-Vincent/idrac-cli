use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::lib::convert;
use crate::Settings;


#[derive(Debug, Serialize, Deserialize)]
struct StorageVolumeInfo {
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="Description")]
    description: String,
    #[serde(rename="BlockSizeBytes")]
    block_size_bytes: u64,
    #[serde(rename="CapacityBytes")]
    capacity_bytes: u64,
    #[serde(rename="Encrypted")]
    encrypted: bool,
    #[serde(rename="Links")]
    links: Drives,
    #[serde(rename="Status")]
    status: Status,
    #[serde(rename="VolumeType")]
    volume_type: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Drives {
    #[serde(rename="Drives")]
    drives: Vec<Drive>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Drive {
    #[serde(rename="@odata.id")]
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    #[serde(rename="Health")]
    health: String,
    #[serde(rename="State")]
    state: String
}

#[tokio::main]
pub async fn get_storage_volume(volume: &Option<String>, settings: Settings) -> Result<(), Error> {
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/Storage/Volumes/{}", settings.host.to_owned(), volume.as_ref().unwrap()))
        .basic_auth(settings.user, Some(settings.password))
        .send()
        .await
        .unwrap();

    let response_json: StorageVolumeInfo = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    println!("Volume name: {}", response_json.name);
    println!("Description: {}", response_json.description);
    println!("Block size:  {} bytes", response_json.block_size_bytes);
    println!("Capacity:    {}", convert(response_json.capacity_bytes as f64));
    println!("Encrypted:   {}", response_json.encrypted);
    if response_json.status.state == "Enabled" { println!("Status:      {}", response_json.status.health) }
    println!("Drives:");
    for drive in response_json.links.drives {
        println!("- {}", &drive.name.replace("/redfish/v1/Systems/System.Embedded.1/Storage/Drives/", ""))
    }

    Ok(())
}