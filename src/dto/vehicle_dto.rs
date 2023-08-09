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

impl From<Vehicle> for VehicleDto {
    fn from(vehicle: Vehicle) -> Self {
        VehicleDto {
            uuid: vehicle.uuid,
            registration_number: vehicle.registration_number,
            make: vehicle.make,
            model: vehicle.model,
            fuel_type: vehicle.fuel_type,
            vehicle_category: VehicleCategoryDto::from(vehicle.category),
        }
    }
}