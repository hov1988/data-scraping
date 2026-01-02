#[derive(Debug, Clone)]
pub struct ItemLink {
    pub url: String,
}

#[derive(Debug)]
pub struct HouseDetails {
    pub condition: Option<String>,
    pub rooms: Option<u8>,
    pub house_area_m2: Option<f32>,
    pub construction_type: Option<String>,
    pub floors: Option<u8>,
    pub bathrooms: Option<u8>,
    pub garage: Option<String>,
    pub renovation: Option<String>,
    pub appliances: Vec<String>,
    pub service_lines: Vec<String>,
    pub facilities: Vec<String>,
    pub furniture: Option<String>,
    pub land_area_m2: Option<f32>,
    pub price_history: Vec<PriceHistory>,
    pub description: String,
    pub location: Option<String>,
}

#[derive(Debug)]
pub struct PriceHistory {
    pub date: String,
    pub price: String,
    pub diff: Option<String>,
}
