use rocket::info;
use uuid::Uuid;
use chrono::Duration;

use crate::{
    repository::vehicle_repository, 
    error::vbs_error::VbsError
};


pub fn by_period(vehicle_uuid: Uuid, period: Duration) -> Result<f64, VbsError> {
    info!("Get price to hire vehicle with UUID {} for period {:?}", vehicle_uuid, period);
    let vehicle = vehicle_repository::find_by_uuid(vehicle_uuid)
        .ok_or(VbsError::ResourceNotFound(format!("Vehicle with UUID {} not found", vehicle_uuid)))?;
    let price_per_day = vehicle.category.price_per_day;
    let number_of_days = period.num_days() as f64;
    let cost = price_per_day * number_of_days;
    Ok(cost)
}
