

use serde::{Deserialize, Serialize};
use rand::Rng;
use std::collections::HashMap;
use rusoto_dynamodb::AttributeValue;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    id: String,
    username: String,
    email: String,
    active: bool,
    password: String
}

impl User {
    pub fn new(username: String, email: String, password: String) -> User {
        let number_id:u64 = rand::thread_rng().gen();
        let active = true;
        let id = number_id.to_string();
        User {
            id,
            username,
            email,
            active,
            password
        }
    }

    pub fn new_complete(id:String, username:String, email: String, password: String, active:bool) -> User {
        User {
            id,
            username,
            email,
            active,
            password
        }
    }

    pub fn to_hashmap(&self) -> HashMap<String, AttributeValue> {
        let mut hashmap: HashMap<String, AttributeValue> = HashMap::new();

        let mut username = AttributeValue::default();
        username.s = Some(self.get_username().to_string());
        let mut id = AttributeValue::default();
        id.s = Some(self.get_id().to_string());
        let mut email = AttributeValue::default();
        email.s = Some(self.get_email().to_string());
        let mut password = AttributeValue::default();
        password.s = Some(self.get_password().to_string());
        let mut active = AttributeValue::default();
        active.bool = Some(self.get_active().clone());

        hashmap.insert("id".to_string(), id);
        hashmap.insert("username".to_string(), username);    
        hashmap.insert("password".to_string(), password);
        hashmap.insert("email".to_string(), email);
        hashmap.insert("active".to_string(), active);
        hashmap
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn get_active(&self) -> bool {
        self.active
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
    id: String,
    brand: String,
    model: String,
    class: VehicleClass,
    price: u32,
    rented: bool,
    user_id: Option<u64>,
}