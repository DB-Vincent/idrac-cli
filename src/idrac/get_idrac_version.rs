use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::Settings;

#[derive(Debug, Serialize, Deserialize)]
struct VersionData {
    #[serde(rename="FirmwareVersion")]
    firmware_version: String,
}

#[tokio::main]
pub async fn get_idrac_version(settings: Settings) -> Result<(), Error> {
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Managers/iDRAC.Embedded.1", settings.host.to_owned()))
        .basic_auth(settings.user, Some(settings.password))
        .send()
        .await
        .unwrap();

    let response_json: VersionData = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    println!("iDRAC firmware version: {}", response_json.firmware_version);
    Ok(())
}