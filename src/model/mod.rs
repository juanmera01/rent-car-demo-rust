
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    id: u64,
    username: String,
    email: String,
    active: bool,
    password: String
}

impl User {
    pub fn new(username: String, email: String, password: String) -> User {
        let id = rand::thread_rng().gen();
        let active = true;
        User {
            id,
            username,
            email,
            active,
            password
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserToCreate {
    pub username: String,
    pub email: String,
    pub password: String,
    pub pass_confirm: String
}

enum VehicleClass {
    Car,
    Van,
    Motorcycle,
    Truck
}

trait Vehicle {
    fn calculate_rent(&self, days: u32, user: &User) -> u32;
}

pub struct Car {
    id: u64,
    brand: String,
    model: String,
    class: VehicleClass,
    price: u32,
    rented: bool,
    user_id: Option<u64>,
}