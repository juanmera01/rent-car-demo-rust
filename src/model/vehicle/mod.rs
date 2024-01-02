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

impl ToString for VehicleClass {
    fn to_string(&self) -> String {
        match *self {
            VehicleClass::Car => String::from("car"),
            VehicleClass::Van => String::from("van"),
            VehicleClass::Motorcycle => String::from("motorcycle"),
            VehicleClass::Truck => String::from("truck"),
        }
    }
}

pub trait Vehicle {
    fn calculate_rent(&self, days: u32, user: &User) -> u32;

    fn to_hashmap(&self) -> HashMap<String, AttributeValue>;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CarToCreate {
    pub brand: String,
    pub model: String,
    pub price_per_hour: u32
}

impl Car {

    pub fn new_complete(id: String, brand: String, model: String, class: VehicleClass, price_per_hour: u32, rented:bool, user_id:String) -> Car {
        Car {
            id,
            brand,
            model,
            class,
            price_per_hour,
            rented,
            user_id: Some(user_id),
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

    pub fn to_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut hashmap: HashMap<String, AttributeValue> = HashMap::new();

        let mut id = AttributeValue::default();
        id.s = Some(self.get_id().to_string());
        hashmap.insert(String::from("id"), id);

        let mut brand = AttributeValue::default();
        brand.s = Some(self.get_brand().to_string());
        hashmap.insert(String::from("brand"), brand);

        let mut model = AttributeValue::default();
        model.s = Some(self.get_model().to_string());
        hashmap.insert(String::from("model"), model);

        let mut class = AttributeValue::default();
        class.s = Some(self.get_class().to_string());
        hashmap.insert(String::from("class"), class);
        
        let mut price_per_hour = AttributeValue::default();
        price_per_hour.n = Some(self.get_price_per_hour().to_string());
        hashmap.insert(String::from("price_per_hour"), price_per_hour);
        
        let mut rented = AttributeValue::default();
        rented.bool = Some(self.get_rented().to_owned());
        hashmap.insert(String::from("rented"), rented);
        
        let mut user_id = AttributeValue::default();
        user_id.s = self.get_user_id().to_owned();
        hashmap.insert(String::from("user_id"), user_id);

        hashmap

    }
}
