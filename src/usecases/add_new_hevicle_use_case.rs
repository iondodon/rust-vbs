use crate::{
    repository::vehicle_repository, 
    domain::entity::vehicle::Vehicle
};


pub fn exec(vehicle: Vehicle) {
    info!("Getting all vehicles");
    vehicle_repository::add_vehicle(vehicle);
}
