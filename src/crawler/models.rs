#[derive(Debug, Clone)]
pub struct ItemLink {
    pub url: String,
}

#[derive(Debug)]
pub struct PhoneContact {
    pub raw: String,
    pub display: String,
    pub source: PhoneSource,
}

#[derive(Debug)]
pub enum PhoneSource {
    Direct,
    Viber,
    WhatsApp,
}

#[derive(Debug, Clone)]
pub struct ContactPhone {
    pub raw: String,
    pub display: String,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct ContactInfo {
    pub seller_name: Option<String>,
    pub phones: Vec<ContactPhone>,
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
    pub amenities: Option<String>,
    pub comfort: Option<String>,
    pub ceiling_height: Option<String>,
    pub prepayment: Option<String>,
    pub utility_payments: Option<String>,
    pub lease_type: Option<String>,
    pub minimum_rental_period: Option<String>,
    pub sewerage: Option<String>,
    pub parking: Option<String>,
    pub entrance: Option<String>,
    pub location_from_street: Option<String>,
    pub elevator: Option<String>,
    pub floor_area: Option<String>,
}
