
pub mod minimal_station;

use leptos::{component, Signal, SignalGet, SignalSet, WriteSignal};
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use wwn_shared_utils::DataTable;
use wwn_shared_utils::location::{Location, LocationFilter};
use wwn_shared_utils::mapping::Nation::Norway;
use wwn_shared_utils::station::{Station, StationFilter};
use crate::HttpError;
use crate::HttpError::{HttpClientError, HttpServerError};

pub fn http_response_validate(response: &Response) -> Result<(), HttpError> {
    let status = &response.status();
    if status.is_server_error() {
        return Err(HttpServerError(status.as_u16(), *status));
    }
    if status.is_client_error() {
        return Err(HttpClientError(status.as_u16(), *status));
    }
    Ok(())
}

pub async fn fetch_remote_data<T: DataTable + Serialize + DeserializeOwned>(filter: T::FilterType) -> crate::Result<Vec<T>> {
    let url = "http://127.0.0.1:3033/station";
    let response = reqwest::Client::new()
        .post(url)
        .json(&filter)
        .send().await?;
    http_response_validate(&response)?;
    let result = response.json::<Vec<T>>().await?;
    Ok(result)
}

pub async fn init_local_static_data<T: DataTable + Serialize + DeserializeOwned + PartialEq + Clone>(filter: T::FilterType)
                                                                                                     -> crate::Result<(Signal<Vec<T>>, WriteSignal<Vec<T>>, impl Fn() + Clone + Sized)> {
    let (r, w, func) = use_local_storage::<Vec<T>, JsonCodec>(T::ENDPOINT);
    let local_data = r.get();
    if local_data.is_empty() {
        let data = fetch_remote_data::<T>(filter).await?;
        w.set(data)
    }
    Ok((r, w, func))
}

pub async fn init_local_stations(filter: StationFilter)->crate::Result<Vec<Station>> {
    let (r, _, _) = init_local_static_data::<Station>(filter).await?;
    let stations = r.get();
    Ok(stations)
}

pub async fn fetch_static_stations(filter: StationFilter) -> crate::Result<Vec<Station>> {
    let data = fetch_remote_data::<Station>(filter).await?;
    Ok(data)
}

pub async fn fetch_locations(filter: LocationFilter) -> crate::Result<Vec<Location>> {
    let data = fetch_remote_data::<Location>(filter).await?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use tokio;

    #[tokio::test]
    #[ignore]
    pub async fn test_fetch_stations() {
        let filter = StationFilter::default(vec![Norway]);
        let string = serde_json::to_string(&filter).unwrap();
        let a = "";
        let s = fetch_static_stations(filter).await.unwrap();
        let stations_ids = s.into_iter()
            .map(|a| a.station_id).collect_vec();
        assert!(!stations_ids.is_empty());
        assert!(stations_ids.contains(&"1.200.0".to_string()))
    }

    #[tokio::test]
    #[ignore]
    pub async fn test_fetch_locations() {
        let filter = LocationFilter::default(vec![Norway]);
        let string = serde_json::to_string(&filter).unwrap();
        let a = "";
        let s = fetch_locations(filter).await.unwrap();
        let stations_ids = s.into_iter()
            .map(|a| a.name).collect_vec();
        assert!(!stations_ids.is_empty());
        assert!(stations_ids.contains(&"1.42.0".to_string()))
    }
}
