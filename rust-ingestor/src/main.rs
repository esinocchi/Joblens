use axum::{
    extract::Json, // Extracts JSON data from the request body
    routing::post, // function to handle POST requests
    Router,       // Creates URL routing
    http::StatusCode,
};

use serde::{Deserialize, Serialize}; // Serde is a library for serializing and deserializing JSON

// imports the str and base64 types
use std::str;
use base64::{Engine as _, engine::general_purpose};




/*
    - PubSubMessage is the main struct that contains the message and subscription that we receive from the webhook
    - PubSubData is the struct that contains the data, message id, and publish time
    - GmailNotification is the struct that contains the email address and history id after we deserialize the data

    - Debug makes the struct printable
    - Deserialize makes the struct creatable from JSON
*/
#[derive(Debug, Deserialize)]
struct GmailNotification {
    email_address: String,
    history_id: String,
}


#[derive(Debug, Deserialize)]
struct PubSubData {
    data: String,
    #[serde(rename = "messageId")]
    message_id: String,
    #[serde(rename = "publishTime")]
    publish_time: String,
}


/*
Example of PubSubMessage:
{
  "message": {
    "data": "eyJlbWFpbF9hZGRyZXNzIjoidXNlckBleGFtcGxlLmNvbSIsImhpc3RvcnlfaWQiOiIxMjM0NTYifQ==",
    "messageId": "unique_message_id",
    "publishTime": "2021-05-05T12:00:00.000Z"
  },
  "subscription": "projects/my-project/subscriptions/my-subscription"
}
*/
#[derive(Debug, Deserialize)]
struct PubSubMessage {
    message: PubSubData,
    subscription: String,
}

// ? is a propogation shorthand that tells Rust:
// Error -> return the error
// Ok -> Keep going

// Decodes the base64 data into a byte array
fn decode_base64_data(base64: &str) -> Result<Vec<u8>, String> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(base64)
        .map_err(|e| format!("Error decoding base64 data: {}", e))?;
    Ok(bytes)
}

// Converts the byte array into a string
fn bytes_to_json(bytes: Vec<u8>) -> Result<String, String> {
    let json_str = String::from_utf8(bytes)
        .map_err(|e| format!("Error converting to JSON: {}", e))?;
    Ok(json_str)
}

// Converts the string into a GmailNotification struct
fn json_to_gmail_notification(json: &str) -> Result<GmailNotification, String> {
    let gmail_notification: GmailNotification = serde_json::from_str(&json)
        .map_err(|e|  format!("Error deserializing JSONL: {}", e))?;
    Ok(gmail_notification)
}


// Parses the GmailNotification struct from the PubSubMessage
fn parse_gmail_notifcation(payload: &PubSubMessage) -> Result<GmailNotification, String> {
    let base_64_data = &payload.message.data;
    let bytes = decode_base64_data(base_64_data)?;
    let json_str = bytes_to_json(bytes)?;
    let gmail_notification = json_to_gmail_notification(&json_str)?;    
    Ok(gmail_notification)
}

// Json(payload): Json<PubSubMessage> converts the JSON payload into a PubSubMessage struct
async fn handle_webhook(Json(payload): Json<PubSubMessage>) -> StatusCode {
    StatusCode::OK
}


#[tokio::main]
async fn main() {
    /*
    Creates a variable called app that holds a router object
    ::new() creates a new router object
    .route() is used to define the route for the webhook
    When URL path is "/gmail-event" and the method is POST -> call handle_webhook()
    */
    let app = Router::new()
        .route("/gmail-event", post(handle_webhook));

    /*
    Creates a variable called listener that holds a TCP listener object
    .bind() is used to bind the listener to the port
    .await is used to wait for the listener to be created
    .expect() is used to handle any errors that occur
    */
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind port 8080");

    // Prints the server is running on the port
    println!("Server is running on http://127.0.0.1:8080");
    println!("Ready to receive webhooks");

    /*
    Starts the server
    .serve() is used to start the server
    .await is used to wait for the server to start
    .expect() is used to handle any errors that occur
    */
    axum::serve(listener,app)
        .await
        .expect("Failed to start server");
}