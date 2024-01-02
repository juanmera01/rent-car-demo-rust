use axum::extract::{Path};
use axum::response::{Html, IntoResponse};
use axum::Json;
use crate::model::vehicle::{CarToCreate, Car};
use crate::repositories::vehicle_repository::{save_car, get_car, update_car, delete_car, list_cars};

pub async fn create_vehicle_handler(data: Json<CarToCreate>) -> impl IntoResponse {
    let car_to_create: CarToCreate = data.0;

    let car = Car::new(car_to_create.brand, car_to_create.model, car_to_create.price_per_hour);

    match save_car(car).await {
        Ok(car) => {
            Html(format!("Car saved successfuly! {:?}", car))
        }
        Err(err) => {
            Html(format!("There was an error saving the new car: {:?}", err))
        }
    }
}

pub async fn get_vehicle_handler(Path(id): Path<String>) -> impl IntoResponse {
    match get_car(&id).await {
        Ok(car) => {
            if car.is_none() {
                return Html(format!("Vehicle not found"));
            }
            Html(format!("Success! {:?}", car.unwrap()))
        }
        Err(err) => {
            Html(format!("There was an error getting the user: {:?}", err))
        }
    }
}

pub async fn update_vehicle_handler(car: Json<Car>) -> impl IntoResponse {
    let vehicle = car.0;
    match get_car(&vehicle.get_id().to_string()).await {
        Ok(vehicle_fetched) => {
            if vehicle_fetched.is_none() {
                return Html(format!("Vehicle not found"));
            } else {
                match update_car(vehicle).await {
                    Ok(vehicle) => {
                        return Html(format!("Vehicle updated successfully: {:?}", vehicle));
                    }
                    Err(err) => {
                        return Html(format!("There was an error updating the vehicle, err: {:?}", err));
                    }
                }
            }
        }
        Err(err) => {
            return Html(format!("There was an error getting the user: {:?}", err));
        }
    };
}

pub async fn delete_vehicle_handler(Path(id): Path<String>) -> impl IntoResponse {
    match delete_car(&id).await {
        Ok(_) => {
            return Html(format!("Vehicle with id {} deleted successfully", &id));
        }
        Err(err) => {
            return Html(format!("There was an error deleting the vehicle with id {}: {:?}", &id, err));
        }
    }
}

pub async fn list_vehicles_handler() -> impl IntoResponse {
    match list_cars().await {
        Ok(vehicles) => {
            return Html(format!("vehicles: {:?}", vehicles));
        }
        Err(err) => {
            return Html(format!("Something went wrong listing the vehicles: {:?}", err));
        }
    }
}