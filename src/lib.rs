use gloo_net::http::Response;
use leptos::{IntoView, Resource, Serializable, SignalGet};
use leptos_query::{create_query, QueryOptions, QueryScope};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wwn_shared_utils::DataTable;

use crate::error::HttpError;
use crate::error::HttpError::{ClientError, ServerError};
use crate::State::*;

pub mod error;
pub mod tables;
pub mod style;
pub mod status;
pub mod filter;
pub mod components;
pub mod home;



pub fn table_query<T:DataTable + 'static>() -> QueryScope<T::FilterType, Result<Vec<T>, String>> {
    let data = create_query(
        |f: T::FilterType| async move { serializable_fetch_data(f).await },
        QueryOptions::default(),
    );
    data
}
pub async fn serializable_fetch_data<T: DataTable>(filter: T::FilterType) -> Result<Vec<T>, String>{
    fetch_data(filter).await.map_err(|e| e.to_string())
}

pub async fn fetch_data<T: DataTable>(filter: T::FilterType) -> Result<Vec<T>, crate::error::Error> {
    let url = format!("http://127.0.0.1:3033{}", T::ENDPOINT);
    let response = gloo_net::http::Request::post(&url)
        .json(&filter)?
        .send()
        .await?;
    handle_response_status(&response)?;
    let data = response.json::<Vec<T>>().await?;
    Ok(data)
}

pub enum State<T> {
    NotRequested(String),
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
            ClientError(response.status(), http::StatusCode::from_u16(s)?, response.status_text())))
    } else if 500 <= s && s < 600 {
        Err(HttpError::from(
            ServerError(response.status(), http::StatusCode::from_u16(s)?, response.status_text())))
    } else {
        Err(HttpError::from(
            ServerError(response.status(), http::StatusCode::from_u16(s)?, response.status_text())))
    }
}

