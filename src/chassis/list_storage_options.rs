use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::Settings;

#[derive(Debug, Serialize, Deserialize)]
struct StorageControllerList {
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="Members")]
    members: Vec<StorageControllerMember>
}

#[derive(Debug, Serialize, Deserialize)]
struct StorageControllerMember {
    #[serde(rename="@odata.id")]
    name: String
}

#[tokio::main]
pub async fn list_storage_controllers(settings: Settings) -> Result<(), Error> {
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/Storage", settings.host.to_owned()))
        .basic_auth(settings.user, Some(settings.password))
        .send()
        .await
        .unwrap();

    let response_json: StorageControllerList = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    println!("Found {} storage controller(s):", response_json.members.len());
    for storage_controller in response_json.members {
        let long_name = storage_controller.name;
        let short_name = long_name.replace("/redfish/v1/Systems/System.Embedded.1/Storage/", "");

        println!("- {}", short_name)
    }

    Ok(())
}