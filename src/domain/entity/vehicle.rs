use uuid::Uuid;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use crate::domain::types::fuel_type::FuelType;
use crate::dto::vehicle_dto::VehicleDto;

use super::vehicle_category::VehicleCategory;

#[derive(Clone, Debug)]
pub struct Booking {
    // Fields from the Booking class go here...
}

#[derive(Clone, Debug)]
pub struct Vehicle {
    pub id: i64,
    pub uuid: Uuid,
    pub registration_number: String,
    pub make: String,
    pub model: String,
    pub fuel_type: FuelType,
    pub category: VehicleCategory,
    pub bookings: HashSet<Booking>,
}

impl PartialEq for Vehicle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Vehicle {}

impl Hash for Vehicle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl From<VehicleDto> for Vehicle {
    fn from(dto: VehicleDto) -> Self {
        Self {
            id: 0, // You'll need to set this appropriately based on your logic
            uuid: dto.uuid,
            registration_number: dto.registration_number,
            make: dto.make,
            model: dto.model,
            fuel_type: dto.fuel_type,
            category: VehicleCategory::from(dto.vehicle_category),
            bookings: HashSet::new(),
        }
    }
}