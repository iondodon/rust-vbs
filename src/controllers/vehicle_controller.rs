use rocket::get;
use rocket::serde::json::Json;

use crate::controllers::response::all_vehicles_response::AllVehiclesResponse;
use crate::controllers::response::available_for_hire_response::AvailableForHireResponse;
use crate::controllers::response::cost_response::CostResponse;
use crate::usecases::get_all_vehicles_use_case::GetAllVehiclesUseCase;
use crate::usecases::get_available_for_hire_use_case::GetAvailableForHireUseCase;
use crate::usecases::get_cost_of_hiring_use_case::GetCostOfHiringUseCase;

pub struct VehicleController<'a> {
    get_all_vehicles_use_case: &'a GetAllVehiclesUseCase<'a>,
    get_available_for_hire_use_case: &'a GetAvailableForHireUseCase<'a>,
    get_cost_of_hiring_use_case: &'a GetCostOfHiringUseCase<'a>,
}

impl <'a> VehicleController<'a> {
    pub fn new(
        get_all_vehicles_use_case: &'a GetAllVehiclesUseCase,
        get_available_for_hire_use_case: &'a GetAvailableForHireUseCase,
        get_cost_of_hiring_use_case: &'a GetCostOfHiringUseCase,
    ) -> Self {
        Self {
            get_all_vehicles_use_case,
            get_available_for_hire_use_case,
            get_cost_of_hiring_use_case,
        }
    }

    pub fn get_all_vehicles(&self) -> AllVehiclesResponse {
        self.get_all_vehicles_use_case.exec()
    }

    pub fn get_available_for_hire(&self, date: NaiveDate) -> AvailableForHireResponse {
        self.get_available_for_hire_use_case.get_by_date(date)
    }

    pub fn get_cost_of_hiring(&self, vehicle_uuid: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> Result<CostResponse, VbsError> {
        let period_inclusive = to_date.signed_duration_since(from_date) + Duration::days(1);
        self.get_cost_of_hiring_use_case.by_period(vehicle_uuid, period_inclusive)
    }
}

#[get("/")]
pub fn get_all_vehicles() -> Json<AllVehiclesResponse> {
    Json(AllVehiclesResponse { vehicles: Vec::new() })
}

#[get("/available?<date>")]
pub fn get_available_by_date(date: String) -> Json<AvailableForHireResponse> {
    Json(AvailableForHireResponse { vehicles: todo!() })
}

#[get("/<vehicle_uuid>/cost?<from_date>&<to_date>")]
pub fn get_cost_by_period(vehicle_uuid: String, from_date: String, to_date: String) -> Json<CostResponse> {
    Json(CostResponse { cost: todo!() })
}
