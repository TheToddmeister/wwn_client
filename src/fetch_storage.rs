use leptos::{create_effect, create_resource, Signal, SignalGet, SignalSet, WriteSignal};
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use wwn_shared_utils::DataTable;
use wwn_shared_utils::location::LocationFilter;
use wwn_shared_utils::river::RiverFilter;
use wwn_shared_utils::station::{Station, StationFilter};
use wwn_shared_utils::timeseries::TimeSeriesFilter;

use crate::data::fetch_stations;
use crate::error::Error;

pub struct Filters{
    pub station: StationFilter,
    pub river: RiverFilter,
    pub location: LocationFilter,
    pub timeseries: TimeSeriesFilter,
}

pub struct StaticLocalData{
    station: Signal<Result<Vec<Station>, String>>,
}
impl Filters{
    pub fn get_local()->Self{
        let (r,w,d) = use_local_storage::<StationFilter, JsonCodec>("StationFilter");
        let station = r.get();
        let (r,w,d) = use_local_storage::<RiverFilter, JsonCodec>("RiverFilter");
        let river = r.get();
        let (r,w,d) = use_local_storage::<LocationFilter, JsonCodec>("LocationFilter");
        let location = r.get();
        let (r,w,d) = use_local_storage::<TimeSeriesFilter, JsonCodec>("TimeSeriesFilter");
        let timeseries = r.get();
        Self{
            station,
            river,
            location,
            timeseries,
        }
    }
}
pub fn store_local_filter<T:DataTable>(filter: T::FilterType)-> Result<Signal<T>, wwn_shared_utils::Error>{
    let filter_name = format!("filter{}", T::ENDPOINT);
    let (r, w, _) = use_local_storage::<T::DataType, JsonCodec>(filter_name);
    w.set(filter);
    Ok(r)
}
