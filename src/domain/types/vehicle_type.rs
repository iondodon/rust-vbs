use rocket::serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VehicleType {
    SmallCar,
    EstateCar,
    Van,
}
