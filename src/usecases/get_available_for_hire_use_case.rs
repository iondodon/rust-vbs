use rocket::info;
use chrono::NaiveDate;

use crate::{
    domain::entity::vehicle::Vehicle, 
    repository::vehicle_repository
};


pub fn get_by_date(date: NaiveDate) -> Vec<Vehicle> {
    info!("Get all available vehicles on date {}", date);
    vehicle_repository::get_available_for_hire_on_date(date)
}
