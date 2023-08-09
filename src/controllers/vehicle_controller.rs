use chrono::NaiveDate;
use rocket::get;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::controllers::response::all_vehicles_response::AllVehiclesResponse;
use crate::controllers::response::available_for_hire_response::AvailableForHireResponse;
use crate::controllers::response::cost_response::CostResponse;

use crate::dto::vehicle_dto::VehicleDto;
use crate::error::vbs_error::VbsError;
use crate::presenters::vehicle_presenter;


#[post("/", format = "json", data = "<new_vehicle>")]
pub fn create_new_vehicle(new_vehicle: Json<VehicleDto>) -> Json<Result<(), VbsError>> {
    let result = vehicle_presenter::create_new(new_vehicle.0);
    Json(result)
}

#[get("/")]
pub fn get_all_vehicles() -> Json<AllVehiclesResponse> {
    let response = vehicle_presenter::get_all_vehicles();
    Json(response)
}

#[get("/available?<date>")]
pub fn get_available_by_date(date: String) -> Json<AvailableForHireResponse> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
    let response = vehicle_presenter::get_available_for_hire_by_date(date);
    Json(response)
}

#[get("/<vehicle_uuid>/cost?<from_date>&<to_date>")]
pub fn get_cost_by_period(vehicle_uuid: String, from_date: String, to_date: String) -> Json<CostResponse> {
    let vehicle_uuid = Uuid::parse_str(&vehicle_uuid).unwrap();
    let from_date = NaiveDate::parse_from_str(&from_date, "%Y-%m-%d").unwrap();
    let to_date = NaiveDate::parse_from_str(&to_date, "%Y-%m-%d").unwrap();
    let response = vehicle_presenter::get_cost_by_period(vehicle_uuid, from_date, to_date).unwrap();
    Json(response)
}
