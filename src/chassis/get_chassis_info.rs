use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::Settings;

#[derive(Debug, Serialize, Deserialize)]
struct ChassisInfo {
    #[serde(rename="IndicatorLED")]
    indicator_led: String,
    #[serde(rename="Manufacturer")]
    manufacturer: String,
    #[serde(rename="Model")]
    model: String,
    #[serde(rename="PartNumber")]
    part_number: String,
    #[serde(rename="PowerState")]
    power_state: String,
    #[serde(rename="SKU")]
    sku: String,
    #[serde(rename="SerialNumber")]
    serial_number: String,
    #[serde(rename="Status")]
    status: Status
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    #[serde(rename="Health")]
    health: String,
    #[serde(rename="State")]
    state: String
}

#[tokio::main]
pub async fn get_chassis_info(settings: Settings) -> Result<(), Error> {
    println!("Retrieving chassis info.. (this may take a while)");

    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Chassis/System.Embedded.1", settings.host.to_owned()))
        .basic_auth(settings.user, Some(settings.password))
        .send()
        .await
        .unwrap();

    let response_json: ChassisInfo = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    println!("Chassis info: {:?}", response_json);
    Ok(())
}