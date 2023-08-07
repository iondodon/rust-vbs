use rocket::get;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use crate::dto::vehicle_dto::VehicleDto;

#[derive(Serialize, Deserialize)]
pub struct AllVehiclesResponse {
    vehicles: Vec<VehicleDto>,
}

#[derive(Serialize)]
pub struct AvailableForHireResponse {

}

#[derive(Serialize)]
pub struct CostResponse {
    
}

#[get("/")]
pub fn get_all_vehicles() -> Json<AllVehiclesResponse> {
    Json(AllVehiclesResponse { vehicles: Vec::new() })
}

#[get("/available?<date>")]
pub fn get_available_by_date(date: String) -> Json<AvailableForHireResponse> {
    Json(AvailableForHireResponse {})
}

#[get("/<vehicle_uuid>/cost?<from_date>&<to_date>")]
pub fn get_cost_by_period(vehicle_uuid: String, from_date: String, to_date: String) -> Json<CostResponse> {
    Json(CostResponse {})
}
