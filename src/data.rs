use std::rc::Rc;
use wwn_shared_utils::location::{Location, LocationFilter};
use wwn_shared_utils::river::{River, RiverFilter};
use wwn_shared_utils::station::{Station, StationFilter};
use wwn_shared_utils::timeseries::{TimeSeries, TimeSeriesFilter};

use crate::fetch_data;

pub struct StaticData{
    pub stations: Rc<Vec<Station>>,
    pub locations: Rc<Vec<leptos_router::Location>>,
    pub rivers: Rc<Vec<River>>,
    pub observations: Rc<Vec<TimeSeries>>,
}

pub async fn fetch_stations(filter: StationFilter) -> Result<Vec<Station>, String> {
    let filter = StationFilter::default();
    let data = fetch_data::<Station>(filter).await.map_err(|e| e.to_string())?;
    Ok(data)
}

pub async fn fetch_locations() -> Result<Vec<Location>, String> {
    let filter = LocationFilter::default();
    let data = fetch_data::<Location>(filter).await.map_err(|e| e.to_string())?;
    Ok(data)
}

pub async fn fetch_rivers() -> Result<Vec<River>, String> {
    let filter = RiverFilter::default();
    let data = fetch_data::<River>(filter).await.map_err(|e| e.to_string())?;
    Ok(data)
}

pub async fn fetch_timeseries() -> Result<Vec<TimeSeries>, String> {
    let filter = TimeSeriesFilter::default();
    let data = fetch_data::<TimeSeries>(filter).await.map_err(|e| e.to_string())?;
    Ok(data)
}
