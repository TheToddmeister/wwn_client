#![allow(warnings)]

use std::cell::RefCell;
use std::future::Future;
use std::num::NonZeroU16;
use std::rc::Rc;
use std::sync::Arc;

use leptos::*;
use leptos_struct_table::*;
use leptos_router::*;
use leptos::html::{data, table};
use leptos::tracing::info;

use crate::error::Result;
use chrono::NaiveDate;
use gloo_net;
use gloo_net::http::Response;
use http::status::InvalidStatusCode;
use itertools::Itertools;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;

use log::log;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wwn_shared_utils;
use wwn_shared_utils::DataTable;
use wwn_shared_utils::mapping::Nation::Norway;
use wwn_shared_utils::mapping::ParameterDefinitions;
use wwn_shared_utils::river::River;
use wwn_shared_utils::station::{Station, StationFilter};
use wwn_shared_utils::timeseries::TimeSeries;
use frontend::data::fetch_stations;
use frontend::error::Error;
use frontend::error::Error::EmptyDataResponseError;
use frontend::fetch_data;
use frontend::fetch_storage::{Filters, set_local_filters, set_local_stations};

use frontend::tables::minimal_station::{StationListTable, StationList};
use frontend::tables::river_list::{RiverList, RiverListTable};

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    info!("it works");

    mount_to_body(|| {
        view! {
        <App/>
        }
    })
}

pub type ExternalData<T> = (ReadSignal<T>, WriteSignal<T>);

fn get_data<T:DataTable>() -> Result<()> {
    let station_filter = T::FilterType::default();
    let (read_station_filter, set_station_filter) = create_signal(station_filter);

    let external_data = create_resource(move || read_station_filter.set() , |f| async move { fetch_data(f)});
    let v = create_effect(move |_| {
        if let Some(data) = external_data.get(){
            let (read, set, change) = use_local_storage::<Vec<Station>,JsonCodec>(T::ENDPOINT);
            set.set(data);
        }
    });
}


#[component]
pub fn App() -> impl IntoView {
    provide_query_client();
    let filters = Filters::get_local();
    let station = fe

    view! {
    <Router>
      <nav>
      </nav>
      <main>
          <Routes>
          <Route path="/" view=Home/>
          <Route path=Station::ENDPOINT view=StationListTable/>
          <Route path=River::ENDPOINT view=RiverListTable/>
          <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
        </Routes>
      </main>
    </Router>
  }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
    <h1>"Home"</h1>
  }
}


#[cfg(test)]
mod test {
    use tokio;

    use super::*;
}