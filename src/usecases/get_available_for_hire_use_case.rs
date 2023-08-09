use rocket::info;
use chrono::NaiveDate;

use crate::{domain::entity::vehicle::Vehicle, repository::vehicle_repository::VehicleRepository};

pub struct GetAvailableForHireUseCase<'a> {
    vehicle_repository: &'a dyn VehicleRepository,
}

impl <'a> GetAvailableForHireUseCase<'a> {
    pub fn new(vehicle_repository: &'a dyn VehicleRepository) -> Self {
        Self { vehicle_repository }
    }

    pub fn get_by_date(&self, date: NaiveDate) -> Vec<Vehicle> {
        info!("Get all available vehicles on date {}", date);
        self.vehicle_repository.get_available_for_hire_on_date(date)
    }
}
