use rocket::serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum VehicleType {
    SmallCar,
    EstateCar,
    Van,
}
