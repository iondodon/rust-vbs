use rocket::info;

use crate::{
    domain::entity::vehicle::Vehicle, 
    repository::vehicle_repository
};


pub fn exec() -> Vec<Vehicle> {
    info!("Getting all vehicles");
    vehicle_repository::find_all()
}
