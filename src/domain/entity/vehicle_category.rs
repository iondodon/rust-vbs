use std::hash::{Hash, Hasher};
use rocket::serde::{Serialize, Deserialize};
use crate::domain::types::vehicle_type::VehicleType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehicleCategory {
    pub id: i64,
    pub category: VehicleType,
    pub price_per_day: f64,
}

impl PartialEq for VehicleCategory {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for VehicleCategory {}

impl Hash for VehicleCategory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
