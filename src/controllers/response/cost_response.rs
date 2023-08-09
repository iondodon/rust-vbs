use rocket::serde::Serialize;

use crate::error::vbs_error::VbsError;

#[derive(Serialize)]
pub struct CostResponse {
    pub cost: Result<f64, VbsError>,
}

impl CostResponse {
    pub fn new(cost: Result<f64, VbsError>) -> Self {
        Self { cost }
    }
}