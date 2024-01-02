use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, DeleteItemInput, ScanInput, PutItemInput, QueryInput, AttributeValue, UpdateItemInput};
use crate::model::{vehicle::{Car, VehicleClass}, user};
use std::collections::HashMap;

const TABLE_NAME:&str = "rentcar_vehicles_table";


pub async fn save_car(car:Car) -> Result<Car, rusoto_core::RusotoError<rusoto_dynamodb::PutItemError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let input = PutItemInput {
        table_name: TABLE_NAME.to_string(),
        item: car.to_hashmap(),
        ..Default::default()
    };

    client.put_item(input).await.map(|_| (car))
}

pub async fn list_cars() -> Result<Vec<Car>, rusoto_core::RusotoError<rusoto_dynamodb::ScanError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let scan_input = ScanInput {
        table_name: TABLE_NAME.to_string(),
        ..Default::default()
    };
    let mut users: Vec<Car> = Vec::new();
    
    match client.scan(scan_input).await?.items {
        Some(items) => {
            for item in items.iter().enumerate() {
                users.push(parse_hashmap_to_car(item.1));
            }
            Ok(users)
        }
        None => {
            Ok(users)
        }
    }
}

pub async fn delete_car(id: &String) -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::DeleteItemError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);
    
    let mut id_av: AttributeValue = AttributeValue::default();
    id_av.s = Some(id.to_string());
    let mut hashmap: HashMap<String, AttributeValue> = HashMap::new();
    hashmap.insert("id".to_string(), id_av);

    let delete_input: DeleteItemInput = DeleteItemInput {
        key: hashmap,
        table_name: TABLE_NAME.to_string(),
        ..Default::default()
    };

    client.delete_item(delete_input).await.map(|_| ())
}

pub async fn update_car(car: Car) -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::UpdateItemError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let input = UpdateItemInput {
        table_name: TABLE_NAME.to_string(),
        key: parse_string_to_hashmap(&"id".to_string(), &car.get_id().to_string()),
        update_expression: Some("SET #brand = :brand, #model = :model, #class = :class, #price_per_hour = :price_per_hour, #rented = :rented, #user_id = :user_id".to_string()),
        expression_attribute_names: Some({
            let mut attribute_names = std::collections::HashMap::new();
            attribute_names.insert("#brand".to_string(), "brand".to_string());
            attribute_names.insert("#model".to_string(), "model".to_string());
            attribute_names.insert("#class".to_string(), "class".to_string());
            attribute_names.insert("#price_per_hour".to_string(), "price_per_hour".to_string());
            attribute_names.insert("#rented".to_string(), "rented".to_string());
            attribute_names.insert("#user_id".to_string(), "user_id".to_string());
            attribute_names
        }),
        expression_attribute_values: Some({
            let mut attribute_values = std::collections::HashMap::new();
            attribute_values.insert(
                ":brand".to_string(),
                AttributeValue {
                    s: Some(car.get_brand().to_string()),
                    ..Default::default()
                },
            );
            attribute_values.insert(
                ":model".to_string(),
                AttributeValue {
                    s: Some(car.get_model().to_string()),
                    ..Default::default()
                },
            );
            attribute_values.insert(
                ":class".to_string(),
                AttributeValue {
                    s: Some(car.get_class().to_string()),
                    ..Default::default()
                },
            );
            attribute_values.insert(
                ":price_per_hour".to_string(),
                AttributeValue {
                    s: Some(car.get_price_per_hour().to_string()),
                    ..Default::default()
                },
            );
            attribute_values.insert(
                ":rented".to_string(),
                AttributeValue {
                    s: Some(car.get_rented().to_string()),
                    ..Default::default()
                },
            );
            let mut user_id = String::from("");
            if car.get_user_id().is_some(){
                user_id = car.get_user_id().clone().unwrap().to_string();
            }
            attribute_values.insert(
                ":user_id".to_string(),
                AttributeValue {
                    s: Some(user_id),
                    ..Default::default()
                },
            );
            attribute_values
        }),
        ..Default::default()
    };

    client.update_item(input).await.map(|_| ())
}

pub async fn get_car(id: &String) -> Result<Option<Car>, rusoto_core::RusotoError<rusoto_dynamodb::QueryError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let mut id_av: AttributeValue = AttributeValue::default();
    id_av.s = Some(id.to_string());
    let mut hashmap: HashMap<String, AttributeValue> = HashMap::new();
    hashmap.insert(":val".to_string(), id_av);

    let query_input = QueryInput {
        table_name: TABLE_NAME.to_string(),
        key_condition_expression: Some(format!("{} = :val", "id")),
        expression_attribute_values: Some(hashmap),
        ..Default::default()
    };
    match client.query(query_input).await?.items {
        Some(items) => {

            let op_item = items.iter().next();
            if op_item.is_none() {
                return Ok(None);
            }
            let item = op_item.unwrap();

            let user = parse_hashmap_to_car(item);
            Ok(Some(user))
        },
        None => Ok(None)
    }
}


fn parse_hashmap_to_car(item: &HashMap<String, AttributeValue>) -> Car {
    
    let id:String = item.get("id").unwrap().s.clone().unwrap(); 
    let brand:String = item.get("brand").unwrap().s.clone().unwrap();
    let model:String = item.get("model").unwrap().s.clone().unwrap();
    let class:VehicleClass = VehicleClass::Car;
    let price_per_hour:u32 = parse_string_to_number(item.get("price_per_hour").unwrap().n.clone().unwrap());
    let rented:bool = item.get("rented").unwrap().bool.clone().unwrap();
    let user_id:String = item.get("user_id").unwrap().s.clone().unwrap();

    Car::new_complete(id, brand, model, VehicleClass::Car, price_per_hour, rented, user_id)
}

fn parse_string_to_number(str:String) -> u32{
    match str.parse::<u32>() {
        Ok(number) => {
            number
        }
        Err(_) => {
            0
        }
    }
}

fn parse_string_to_hashmap(key:&String, string: &String) -> HashMap<String, AttributeValue> {
    let mut hashmap: HashMap<String, AttributeValue> = HashMap::new();
    hashmap.insert(key.to_string(), AttributeValue {
        s: Some(string.to_string()),
        ..Default::default()
    });
    hashmap
}