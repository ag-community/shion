use crate::adapters::ip_api;
use crate::models::location::LocationInformation;
use std::net::IpAddr;
use tracing::warn;

pub async fn get_location(ip_address: IpAddr) -> LocationInformation {
    match ip_api::get_ip_info(ip_address).await {
        Ok(location) => LocationInformation {
            country: location
                .country_code
                .unwrap_or("xx".to_string())
                .to_lowercase(),
            latitude: location.latitude.unwrap_or(0.0),
            longitude: location.longitude.unwrap_or(0.0),
        },
        Err(e) => {
            warn!(
                ip_address = ip_address.to_string(),
                "Failed getting location for IP address: {e:?}"
            );
            LocationInformation {
                country: "xx".to_string(),
                latitude: 0.0,
                longitude: 0.0,
            }
        }
    }
}
