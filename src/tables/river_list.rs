use wwn_shared_utils::river::River;

pub struct RiverList{
    pub river: String,
    pub alias: Vec<String>,
    pub tributary_hierarchy: Vec<String>,
    pub drainage_baisin: Option<String>,
    pub catchment_area: Option<u64>,
    pub nation: String,

}
impl RiverList {
    pub fn from_river(r: &River)->Self{
        Self{
            river: r.name.to_string(),
            alias: r.alias.to_owned(),
            tributary_hierarchy: r.tributary_hierarchy.to_owned(),
            drainage_baisin: r.drainage_basin.to_owned(),
            nation: r.origin.to_nation().to_string(),
            catchment_area: r.catchment_area.to_owned(),
        }
    }
}
