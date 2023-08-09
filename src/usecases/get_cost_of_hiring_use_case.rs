use rocket::info;
use uuid::Uuid;
use chrono::Duration;

use crate::{repository::vehicle_repository::VehicleRepository, error::vbs_error::VbsError};

pub struct GetCostOfHiringUseCase<'a> {
    vehicle_repository: &'a dyn VehicleRepository,
}

impl<'a> GetCostOfHiringUseCase<'a> {
    pub fn new(vehicle_repository: &'a dyn VehicleRepository) -> Self {
        Self { vehicle_repository }
    }

    pub fn by_period(&self, vehicle_uuid: Uuid, period: Duration) -> Result<f64, VbsError> {
        info!("Get price to hire vehicle with UUID {} for period {:?}", vehicle_uuid, period);
        let vehicle = self.vehicle_repository.find_by_uuid(vehicle_uuid)
            .ok_or(VbsError::ResourceNotFound(format!("Vehicle with UUID {} not found", vehicle_uuid)))?;
        let price_per_day = vehicle.category.price_per_day;
        let number_of_days = period.num_days() as f64;
        let cost = price_per_day * number_of_days;
        Ok(cost)
    }
}
