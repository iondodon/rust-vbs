use rocket::serde::{Serialize, Deserialize};
use crate::domain::types::vehicle_type::VehicleType;
use crate::domain::entity::vehicle_category::VehicleCategory;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehicleCategoryDto {
    pub category: VehicleType,
    pub price_per_day: f64,
}

impl From<VehicleCategory> for VehicleCategoryDto {
    // The translation of the 'from' method will depend on the corresponding
    // translation of the 'VehicleCategory' domain entity.
    // For now, I've included a placeholder, and this can be filled out later.
    fn from(vehicle_category: VehicleCategory) -> Self {
        VehicleCategoryDto {
            category: vehicle_category.category,
            price_per_day: vehicle_category.price_per_day
        }
    }
}