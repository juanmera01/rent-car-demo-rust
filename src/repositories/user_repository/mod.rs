use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, DeleteItemInput, ScanInput, PutItemInput, QueryInput, AttributeValue, UpdateItemInput};
use crate::model::user::User;
use std::collections::HashMap;

const TABLE_NAME:&str = "rentcar_users_table";

pub async fn save_user(user: User) -> Result<User, rusoto_core::RusotoError<rusoto_dynamodb::PutItemError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let input = PutItemInput {
        table_name: TABLE_NAME.to_string(),
        item: user.to_hashmap(),
        ..Default::default()
    };

    client.put_item(input).await.map(|_| (user))
}

pub async fn list_users() -> Result<Vec<User>, rusoto_core::RusotoError<rusoto_dynamodb::ScanError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let scan_input = ScanInput {
        table_name: TABLE_NAME.to_string(),
        ..Default::default()
    };
    let mut users: Vec<User> = Vec::new();
    
    match client.scan(scan_input).await?.items {
        Some(items) => {
            for item in items.iter().enumerate() {
                users.push(parse_hashmap_to_user(item.1));
            }
            Ok(users)
        }
        None => {
            Ok(users)
        }
    }
}

pub async fn delete_user(id: &String) -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::DeleteItemError>> {
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

pub async fn update_user(user: User) -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::UpdateItemError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let input = UpdateItemInput {
        table_name: TABLE_NAME.to_string(),
        key: parse_string_to_hashmap(&"id".to_string(), &user.get_id().to_string()),
        update_expression: Some("SET #username = :username, #email = :email, #active = :active, #password = :password".to_string()),
        expression_attribute_names: Some({
            let mut attribute_names = std::collections::HashMap::new();
            attribute_names.insert("#username".to_string(), "username".to_string());
            attribute_names.insert("#email".to_string(), "email".to_string());
            attribute_names.insert("#active".to_string(), "active".to_string());
            attribute_names.insert("#password".to_string(), "password".to_string());
            attribute_names
        }),
        expression_attribute_values: Some({
            let mut attribute_values = std::collections::HashMap::new();
            attribute_values.insert(
                ":username".to_string(),
                AttributeValue {
                    s: Some(user.get_username().to_string()),
                    ..Default::default()
                },
            );
            attribute_values.insert(
                ":email".to_string(),
                AttributeValue {
                    s: Some(user.get_email().to_string()),
                    ..Default::default()
                },
            );
            attribute_values.insert(
                ":active".to_string(),
                AttributeValue {
                    s: Some(user.get_active().to_string()),
                    ..Default::default()
                },
            );
            attribute_values.insert(
                ":password".to_string(),
                AttributeValue {
                    s: Some(user.get_password().to_string()),
                    ..Default::default()
                },
            );
            attribute_values
        }),
        ..Default::default()
    };

    client.update_item(input).await.map(|_| ())
}

pub async fn get_user(id: &String) -> Result<Option<User>, rusoto_core::RusotoError<rusoto_dynamodb::QueryError>> {
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

            let user = parse_hashmap_to_user(item);
            Ok(Some(user))
        },
        None => Ok(None)
    }
}

fn parse_hashmap_to_user(item: &HashMap<String, AttributeValue>) -> User {
    let id:String = item.get("id").unwrap().s.clone().unwrap(); 
    let username:String = item.get("username").unwrap().s.clone().unwrap();
    let email:String = item.get("email").unwrap().s.clone().unwrap();
    let password:String = item.get("password").unwrap().s.clone().unwrap();
    let active:bool = item.get("active").unwrap_or(&AttributeValue::default()).bool.unwrap_or(true);

    User::new_complete(id, username, email, password, active)
}

fn parse_string_to_hashmap(key:&String, string: &String) -> HashMap<String, AttributeValue> {
    let mut hashmap: HashMap<String, AttributeValue> = HashMap::new();
    hashmap.insert(key.to_string(), AttributeValue {
        s: Some(string.to_string()),
        ..Default::default()
    });
    hashmap
}
