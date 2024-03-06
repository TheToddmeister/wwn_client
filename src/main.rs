#![allow(warnings)]

use std::num::NonZeroU16;

use chrono::NaiveDate;
use gloo_net;
use gloo_net::http::Response;
use http::status::InvalidStatusCode;
use itertools::Itertools;
use leptos::*;
use leptos::html::{data, table};
use leptos::tracing::info;
use leptos_struct_table::*;
use log::log;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wwn_shared_utils;
use wwn_shared_utils::DataTable;
use wwn_shared_utils::mapping::Nation::Norway;
use wwn_shared_utils::station::{Station, StationFilter};

use crate::HttpError::{ClientError, ServerError};
use crate::State::{Failed, Finished, Waiting};
use crate::uuid::Uuid;

/// This generates the component MinimalStationTable
#[derive(TableRow, Clone)]
#[table(impl_vec_data_provider)]
pub struct MinimalStation {
    pub id: String,
    pub river: String,
    pub parameters: String,
}

impl MinimalStation {
    pub fn from_station(station: &Station) -> Self {
        Self {
            id: "a".to_string(),
            river: "a".to_string(),
            parameters: "a".to_string(),
        }
    }
}

pub async fn fetch_data<T: DataTable + DeserializeOwned + Serializable>(filter: T::FilterType) -> Result<Vec<T>, crate::Error> {
    let response = gloo_net::http::Request::post("http://127.0.0.1:3033/station")
        .json(&filter)?
        .send()
        .await?;
    handle_response_status(&response)?;
    let data = response.json::<Vec<T>>().await?;
    Ok(data)
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum HttpError {
    #[error("{0} {1}")]
    ServerError(u16, http::StatusCode),
    #[error("{0} {1}")]
    ClientError(u16, http::StatusCode),
    #[error("{0} {1}")]
    UndefinedHttpError(u16, http::StatusCode),
    #[error("{0}")]
    InvalidStatusCode(#[from] InvalidStatusCode),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    HttpError(#[from] HttpError),
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("{0}")]
    GlooError(#[from] gloo_net::Error),
}

#[component]
impl IntoView for Error {
    fn into_view(self) -> View {
        let message = "Error".to_string() + &self.to_string();
        let html_error = view! {
            <p> "Error: ". </p>
        };
        html_error.into_view()
    }
}

pub fn handle_response_status(response: &Response) -> Result<(), HttpError> {
    let s = response.status();
    if response.ok() {
        Ok(())
    } else if (400 <= s && s < 500) {
        Err(HttpError::from(
            ClientError(response.status(), http::StatusCode::from_u16(s)?)))
    } else if (500 <= s && s < 600) {
        Err(HttpError::from(
            ServerError(response.status(), http::StatusCode::from_u16(s)?)))
    } else {
        Err(HttpError::from(
            ServerError(response.status(), http::StatusCode::from_u16(s)?)))
    }
}

pub async fn fetch_stations() -> Result<Vec<Station>, String> {
    let filter = StationFilter::default(vec![Norway]);
    let data = fetch_data::<Station>(filter).await.map_err(|e| e.to_string())?;
    Ok(data)
}

pub enum State<T> {
    Failed(String),
    Finished(Vec<T>),
    Waiting(String),
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

#[component]
pub fn MinimalStationList() -> impl IntoView {
    let stations_resource = create_local_resource(|| (), |_| async move { fetch_stations().await });

    let data = move || match stations_resource.get() {
        None => view! { <p>"Loading..."</p> }.into_view(),
        Some(r) => match r {
            Ok(v) => {
                let rows = v.iter().map(|s| MinimalStation::from_station(s)).collect_vec();
                let view = view! {
                    <table>
                        <TableContent rows/>
                     </table>
                };
                view.into_view()
            },
            Err(e) => {
                let error_message = "ErrorMessage: ".to_string() + &e;
                let view = view! {<p> {error_message} </p>};
                view.into_view()
            }
        }
    };

    view! {
        <div>
                <div>
                    <h1> "something" </h1>
                    {data}
                </div>
        </div>
        }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    info!("it works");

    mount_to_body(|| {
        view! {
        <MinimalStationList/>
        }
    })
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;
}