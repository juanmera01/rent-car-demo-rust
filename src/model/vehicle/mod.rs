use serde::{Deserialize, Serialize};
use rand::Rng;
use std::collections::HashMap;
use rusoto_dynamodb::AttributeValue;
use crate::model::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub enum VehicleClass {
    Car,
    Van,
    Motorcycle,
    Truck
}

trait Vehicle {
    fn calculate_rent(&self, days: u32, user: &User) -> u32;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    id: String,
    brand: String,
    model: String,
    class: VehicleClass,
    price_per_hour: u32,
    rented: bool,
    user_id: Option<String>,
}

impl Car {

    pub fn new_complete(id: String, brand: String, model: String, class: VehicleClass, price_per_hour: u32) -> Car {
        Car {
            id,
            brand,
            model,
            class,
            price_per_hour,
            rented: false,
            user_id: None,
        }
    }

    pub fn new(brand:String, model:String, price_per_hour:u32) -> Car {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<u32>().to_string();
        Car {
            id,
            brand,
            model,
            class: VehicleClass::Car,
            price_per_hour,
            rented: false,
            user_id: None,
        }
    }

    pub fn default() -> Car {
        Car::new(String::from("brand"), String::from("model"), 30)
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_brand(&self) -> &String {
        &self.brand
    }
    pub fn get_model(&self) -> &String {
        &self.model
    }
    pub fn get_class(&self) -> &VehicleClass {
        &self.class
    }
    pub fn get_price_per_hour(&self) -> &u32 {
        &self.price_per_hour
    }
    pub fn get_rented(&self) -> &bool {
        &self.rented
    }
    pub fn get_user_id(&self) -> &Option<String> {
        &self.user_id
    }
}