use wwn_shared_utils::location::{Location, LocationFilter};
use wwn_shared_utils::mapping::Nation::Norway;
use wwn_shared_utils::mapping::Origin;
use wwn_shared_utils::mapping::ParameterDefinitions::FLOW;
use wwn_shared_utils::station::{Station, StationFilter};
use wwn_shared_utils::timeseries::{TimeSeries, TimeSeriesFilter};

use crate::fetch_data;

pub async fn fetch_stations() -> Result<Vec<Station>, String> {
    let filter = StationFilter{
        nations: vec![Norway],
        exclude_activity_status_inactive: false,
        station_ids: vec![],
        regulation_status: vec![],
        rivers: vec![],
        must_include_parameters: vec![],
    };
    let data = fetch_data::<Station>(filter).await.map_err(|e| e.to_string())?;
    Ok(data)
}

pub async fn fetch_locations(filter:LocationFilter) -> Result<Vec<Location>, String> {
    let data = fetch_data::<Location>(filter).await.map_err(|e| e.to_string())?;
    Ok(data)
}

pub async fn fetch_timeseries(filter: TimeSeriesFilter) -> Result<Vec<TimeSeries>, String> {
    let data = fetch_data::<TimeSeries>(filter).await.map_err(|e| e.to_string())?;
    Ok(data)
}
