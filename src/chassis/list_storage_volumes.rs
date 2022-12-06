use std::time::Duration;
use reqwest::{Client, Error};
use reqwest;
use serde::{Serialize, Deserialize};
use crate::Settings;


#[derive(Debug, Serialize, Deserialize)]
struct StorageVolumeInfo {
    #[serde(rename="@odata.id")]
    name: String,
    #[serde(rename="Members")]
    members: Vec<StorageVolumeMember>
}

#[derive(Debug, Serialize, Deserialize)]
struct StorageVolumeMember {
    #[serde(rename="@odata.id")]
    name: String
}

#[tokio::main]
pub async fn list_storage_volumes(storage_controller: &Option<String>, settings: Settings) -> Result<(), Error> {
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
        .get(format!("https://{}/redfish/v1/Systems/System.Embedded.1/Storage/{}/Volumes", settings.host.to_owned(), storage_controller.as_ref().unwrap()))
        .basic_auth(settings.user, Some(settings.password))
        .send()
        .await
        .unwrap();

    let response_json: StorageVolumeInfo = match response.json().await {
        Ok(r) => r,
        Err(e) => panic!("Could not introspect the token. Error was:\n {:?}", e),
    };

    println!("Found {} storage volume(s):", response_json.members.len());
    for volume in response_json.members {
        println!("- {}", &volume.name.replace("/redfish/v1/Systems/System.Embedded.1/Storage/Volumes/", ""))
    }

    Ok(())
}