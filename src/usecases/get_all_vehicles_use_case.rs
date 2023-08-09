use rocket::info;

use crate::{domain::entity::vehicle::Vehicle, repository::vehicle_repository::VehicleRepository};

pub struct GetAllVehiclesUseCase<'a> {
    vehicle_repository: &'a dyn VehicleRepository,
}

impl <'a> GetAllVehiclesUseCase<'a> {
    pub fn new(vehicle_repository: &'a dyn VehicleRepository) -> Self {
        Self { vehicle_repository }
    }

    pub fn exec(&self) -> Vec<Vehicle> {
        info!("Getting all vehicles");
        self.vehicle_repository.find_all()
    }
}
