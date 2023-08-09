use uuid::Uuid;

use chrono::{NaiveDate, Duration};

use crate::{
    usecases::{
        get_all_vehicles_use_case, 
        get_available_for_hire_use_case, 
        get_cost_of_hiring_use_case, add_new_hevicle_use_case
    }, 
    controllers::response::{
        all_vehicles_response::AllVehiclesResponse, 
        available_for_hire_response::AvailableForHireResponse, cost_response::CostResponse
    }, 
    error::vbs_error::VbsError, domain::entity::vehicle::Vehicle, dto::vehicle_dto::VehicleDto
};


pub fn create_new(new_vehicle_dto: VehicleDto) -> Result<(), VbsError> {
    let new_vehicle = Vehicle::from(new_vehicle_dto);
    add_new_hevicle_use_case::exec(new_vehicle);
    Result::Ok(())
}

pub fn get_all_vehicles() -> AllVehiclesResponse {
    let vehicle_dtos = get_all_vehicles_use_case::exec();
    AllVehiclesResponse::new(vehicle_dtos)
}

pub fn get_available_for_hire_by_date(date: NaiveDate) -> AvailableForHireResponse {
    let available_vehicles = get_available_for_hire_use_case::get_by_date(date);
    AvailableForHireResponse::new(available_vehicles)
}

pub fn get_cost_by_period(vehicle_uuid: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> Result<CostResponse, VbsError> {
    if from_date > to_date {
        return Err(VbsError::InvalidPeriod);
    }
    let period_inclusive = to_date.signed_duration_since(from_date) + Duration::days(1);
    let cost = get_cost_of_hiring_use_case::by_period(vehicle_uuid, period_inclusive);
    Ok(CostResponse::new(cost))
}

