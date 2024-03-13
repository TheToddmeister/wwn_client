use gloo_net::http::Response;
use leptos::{IntoView, Resource, Serializable, SignalGet};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wwn_shared_utils::DataTable;

use crate::error::HttpError;
use crate::error::HttpError::{ClientError, ServerError};
use crate::State::*;

pub mod error;
pub mod data;
pub mod tables;
pub mod style;
pub async fn fetch_data<T: DataTable + DeserializeOwned + Serializable>(filter: T::FilterType) -> Result<Vec<T>, crate::error::Error> {
    let url = format!("http://127.0.0.1:3033{}", T::ENDPOINT);
    let response = gloo_net::http::Request::post(&url)
        .json(&filter)?
        .send()
        .await?;
    handle_response_status(&response)?;
    let data = response.json::<Vec<T>>().await?;
    Ok(data)
}

pub fn get_awaiting_resource_task<T: Serialize + Clone>(data: Resource<(), Result<Vec<T>, String>>) -> State<T> {
    match data.get() {
        None => { Waiting("Awaiting data".to_string()) }
        Some(r) =>
            match r {
                Ok(v) => { Finished(v) }
                Err(e) => { Failed(e) }
            }
    }
}

pub enum State<T> {
    Failed(String),
    Finished(Vec<T>),
    Waiting(String),
}

pub fn handle_response_status(response: &Response) -> Result<(), HttpError> {
    let s = response.status();
    if response.ok() {
        Ok(())
    } else if 400 <= s && s < 500 {
        Err(HttpError::from(
            ClientError(response.status(), http::StatusCode::from_u16(s)?)))
    } else if 500 <= s && s < 600 {
        Err(HttpError::from(
            ServerError(response.status(), http::StatusCode::from_u16(s)?)))
    } else {
        Err(HttpError::from(
            ServerError(response.status(), http::StatusCode::from_u16(s)?)))
    }
}

