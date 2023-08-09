use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::entity::vehicle::Vehicle;

static mut VEHICLES: Vec<Vehicle> = Vec::new();


pub fn _add_vehicle(vehicle: Vehicle) {
    unsafe { VEHICLES.push(vehicle); }
}


pub fn find_all() -> Vec<Vehicle> {
    unsafe { VEHICLES.clone() }
}

pub fn get_available_for_hire_on_date(_date: NaiveDate) -> Vec<Vehicle> {
    unsafe { VEHICLES.clone() }
}

pub fn find_by_uuid(uuid: Uuid) -> Option<Vehicle> {
    unsafe { VEHICLES.iter().find(|&v| v.uuid == uuid).cloned() }
}

