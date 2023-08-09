#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;
mod presenters;
mod dto;
mod domain;
mod usecases;
mod repository;
mod error;

use controllers::vehicle_controller::{
    create_new_vehicle,
    get_all_vehicles, 
    get_available_by_date, 
    get_cost_by_period
};


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/vehicles", routes![
        create_new_vehicle,
        get_all_vehicles, 
        get_available_by_date, 
        get_cost_by_period
    ])
}