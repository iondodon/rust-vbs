use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::entity::vehicle::Vehicle;


pub trait VehicleRepository {
    fn find_all(&self) -> Vec<Vehicle>;
    fn get_available_for_hire_on_date(&self, date: NaiveDate) -> Vec<Vehicle>;
    fn find_by_uuid(&self, uuid: Uuid) -> Option<Vehicle>;
}

pub struct InMemoryVehicleRepository {
    vehicles: Vec<Vehicle>,
}

impl InMemoryVehicleRepository {
    pub fn new() -> Self {
        Self {
            vehicles: Vec::new(),
        }
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }
}

impl VehicleRepository for InMemoryVehicleRepository {
    fn find_all(&self) -> Vec<Vehicle> {
        self.vehicles.clone()
    }

    fn get_available_for_hire_on_date(&self, _date: NaiveDate) -> Vec<Vehicle> {
        self.vehicles.clone()
    }

    fn find_by_uuid(&self, uuid: Uuid) -> Option<Vehicle> {
        self.vehicles.iter().find(|&v| v.uuid == uuid).cloned()
    }
}
