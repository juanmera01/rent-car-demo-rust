use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, DeleteItemInput, ScanInput, PutItemInput, QueryInput, AttributeValue};
use crate::model::User;
use maplit::hashmap;
use std::collections::HashMap;

const TABLE_NAME:&str = "rentcar_users_table";

pub async fn save_user(user: User) -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::PutItemError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let input = PutItemInput {
        table_name: TABLE_NAME.to_string(),
        item: user.to_hashmap(),
        ..Default::default()
    };

    client.put_item(input).await.map(|_| ())
}

pub async fn list_users() -> Result<Vec<User>, rusoto_core::RusotoError<rusoto_dynamodb::ScanError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let scan_input = ScanInput {
        table_name: TABLE_NAME.to_string(),
        ..Default::default()
    };
    let mut users: Vec<User> = Vec::new();
    
    let result = client.scan(scan_input.clone()).await?;
    if let Some(scanned_items) = result.items {
        let mut s: Vec<User> = Vec::new();
        for item in scanned_items {
            let user: User = serde_json::from_str(&item.into_iter().next().unwrap().0)?;
            s.push(user);
        }
        users.extend_from_slice(&s);
    }
    
    Ok(users)
}

pub async fn delete_user(id: u64) -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::DeleteItemError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);
    
    let delete_input = DeleteItemInput {
        key: hashmap! {
            "id".to_string() => rusoto_dynamodb::AttributeValue {
                s: Some(id.to_string()),
                ..Default::default()
            },
        },
        table_name: TABLE_NAME.to_string(),
        ..Default::default()
    };

    client.delete_item(delete_input).await.map(|_| ())
}

pub async fn update_user(user: User) -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::PutItemError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let serialized = serde_json::to_string(&user)?;
    let input = PutItemInput {
        table_name: TABLE_NAME.to_string(),
        item: serde_json::from_str(&serialized)?,
        ..Default::default()
    };

    client.put_item(input).await.map(|_| ())
}

pub async fn get_user(id: u64) -> Result<User, rusoto_core::RusotoError<rusoto_dynamodb::QueryError>> {
    let client = DynamoDbClient::new(Region::EuCentral1);

    let query_input = QueryInput {
        table_name: TABLE_NAME.to_string(),
        key_condition_expression: Some(format!("{} = :val", "id")),
        expression_attribute_values: Some(hashmap! {
            ":val".to_string() => rusoto_dynamodb::AttributeValue {
                s: Some(id.to_string()),
                ..Default::default()
            },
        }),
        ..Default::default()
    };
    let result = client.query(query_input).await?.items.unwrap().into_iter().next().unwrap();
    let str_result = serde_json::to_string(&(result)).unwrap();

    let user: User = serde_json::from_str(&str_result).unwrap();
    Ok(user)
}