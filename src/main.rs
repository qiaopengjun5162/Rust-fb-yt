use std::collections::HashMap;

use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Debug, Deserialize)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    let user = User {
        name: "John".to_string(),
        age: 20,
        email: "john@example.com".to_string(),
    };

    let firebase = Firebase::new("https://fb-rust-c9a1a-default-rtdb.firebaseio.com").unwrap();

    let response = set_user(&firebase, &user).await;

    let mut user = get_user(&firebase, &response.name).await;
    println!("get_user, user: {user:?}");

    let users = get_users(&firebase).await;
    println!("get_users, users: {users:?}");

    user.email = "johnupdate@example.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    println!("update_user, updated_user: {updated_user:?}");

    delete_user(&firebase, &response.name).await;
    println!("User deleted");
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_response(&_users.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    return users.unwrap();
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

// convert a string to a response
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// convert a string to a user
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}