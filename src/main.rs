#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;
mod dto;
mod domain;

use controllers::vehicle_controller::{
    get_all_vehicles, 
    get_available_by_date, 
    get_cost_by_period
};


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        get_all_vehicles, 
        get_available_by_date, 
        get_cost_by_period
    ])
}