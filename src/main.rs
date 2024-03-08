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
use wwn_shared_utils::mapping::ParameterDefinitions;
use wwn_shared_utils::station::{Station, StationFilter};

use frontend::tables::minimal_station::MinimalStationList;

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