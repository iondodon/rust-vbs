use rocket::serde::Serialize;

use crate::{dto::vehicle_dto::VehicleDto, domain::entity::vehicle::Vehicle};

#[derive(Serialize)]
pub struct AvailableForHireResponse {
    pub vehicles: Vec<VehicleDto>
}

impl AvailableForHireResponse {
    pub fn new(vehicles: Vec<Vehicle>) -> Self {
        let vehicles_dto = vehicles
            .into_iter()
            .map(VehicleDto::from)
            .collect();
        Self { vehicles: vehicles_dto }
    }

    // You can add methods here to access or manipulate the vehicles, if needed
}