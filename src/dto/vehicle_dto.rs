use uuid::Uuid;
use rocket::serde::{Serialize, Deserialize};
use crate::domain::{entity::vehicle::Vehicle, types::fuel_type::FuelType};

use super::vehicle_category_dto::VehicleCategoryDto;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehicleDto {
    pub uuid: Uuid,
    pub registration_number: String,
    pub make: String,
    pub model: String,
    pub fuel_type: FuelType,
    pub vehicle_category: VehicleCategoryDto,
}

// The translation of the 'from' method will depend on the corresponding
// translation of the 'Vehicle' domain entity and 'VehicleCategoryDto'.
// For now, I've included a placeholder, and this can be filled out later.
impl VehicleDto {
    pub fn from(vehicle: &Vehicle) -> Self {
        // Placeholder implementation
        VehicleDto {
            uuid: vehicle.uuid,
            registration_number: vehicle.registration_number.clone(),
            make: vehicle.make.clone(),
            model: vehicle.model.clone(),
            fuel_type: vehicle.fuel_type,
            vehicle_category: VehicleCategoryDto::from(vehicle.category),
        }
    }
}