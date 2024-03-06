use std::rc::Rc;

use itertools::Itertools;
use wwn_shared_utils::station::Station;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_struct_table::*;

#[derive(TableRow, Clone)]
#[table(impl_vec_data_provider)]
pub struct MinimalStation {
    pub id: String,
    pub river: String,
    pub parameters: String,
}

impl MinimalStation {
    pub fn from_station(s: &Station) -> MinimalStation {
        let parameters = s.station_parameters.iter()
            .map(|a| a.parameter.to_string()).collect_vec();
        MinimalStation {
            id: s.station_id.to_string(),
            river: "".to_string(),
            parameters: "a".to_string(),
        }
    }
}

/// Default Home Page
#[component]
pub fn StationList(rows: Vec<MinimalStation>) -> impl IntoView {

    view! {
            <Html lang="en" dir="ltr" attr:data-theme="light"/>
            // sets the document title
            <Title text="Welcome to Leptos CSR"/>
            <Meta charset="UTF-8"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

            <h1> "hey there!" </h1>
            <table>
               <TableContent rows=rows/>
            </table>
    }
}
