#[derive(Debug, Clone)]
pub struct ItemLink {
    pub url: String,
}

#[derive(Debug)]
pub struct ContactInfo {
    pub seller_name: Option<String>,
    pub phone_display: Option<String>,
    pub phone_raw: Option<String>,
}

#[derive(Debug)]
pub struct PriceHistory {
    pub date: String,
    pub price: String,
    pub diff: Option<String>,
}

#[derive(Debug)]
pub struct HouseDetails {
    pub external_id: String,
    pub url: String,  
    pub title: Option<String>,
    pub price: Option<String>,
    pub contact: ContactInfo,
    pub images: Vec<String>,
    pub price_history: Vec<PriceHistory>,
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
    pub description: String,
    pub location: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
