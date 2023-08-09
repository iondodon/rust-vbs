use uuid::Uuid;

use chrono::{NaiveDate, Duration};

use crate::{
    usecases::{
        get_all_vehicles_use_case::GetAllVehiclesUseCase, 
        get_available_for_hire_use_case::GetAvailableForHireUseCase, 
        get_cost_of_hiring_use_case::GetCostOfHiringUseCase
    }, 
    controllers::response::{
        all_vehicles_response::AllVehiclesResponse, 
        available_for_hire_response::AvailableForHireResponse, cost_response::CostResponse
    }, 
    error::vbs_error::VbsError
};

pub struct VehiclePresenter<'a> {
    get_all_vehicles_use_case: &'a GetAllVehiclesUseCase<'a>,
    get_available_for_hire_use_case: &'a GetAvailableForHireUseCase<'a>,
    get_cost_of_hiring_use_case: &'a GetCostOfHiringUseCase<'a>,
}

impl <'a> VehiclePresenter<'a> {
    pub fn new(
        get_all_vehicles_use_case: &'a GetAllVehiclesUseCase,
        get_available_for_hire_use_case: &'a GetAvailableForHireUseCase,
        get_cost_of_hiring_use_case: &'a GetCostOfHiringUseCase,
    ) -> Self {
        Self {
            get_all_vehicles_use_case: &get_all_vehicles_use_case,
            get_available_for_hire_use_case: &get_available_for_hire_use_case,
            get_cost_of_hiring_use_case: &get_cost_of_hiring_use_case,
        }
    }

    pub fn get_all_vehicles(&self) -> AllVehiclesResponse {
        let vehicle_dtos = self.get_all_vehicles_use_case.exec();
        AllVehiclesResponse::new(vehicle_dtos)
    }

    pub fn get_available_for_hire_by_date(&self, date: NaiveDate) -> AvailableForHireResponse {
        let available_vehicles = self.get_available_for_hire_use_case.get_by_date(date);
        AvailableForHireResponse::new(available_vehicles)
    }

    pub fn get_cost_by_period(&self, vehicle_uuid: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> Result<CostResponse, VbsError> {
        if from_date > to_date {
            return Err(VbsError::InvalidPeriod);
        }
        let period_inclusive = to_date.signed_duration_since(from_date) + Duration::days(1);
        let cost = self.get_cost_of_hiring_use_case.by_period(vehicle_uuid, period_inclusive);
        Ok(CostResponse::new(cost))
    }
}
