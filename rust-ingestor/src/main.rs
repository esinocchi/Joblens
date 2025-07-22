use axum::{
    extract::Json, // Extracts JSON data from the request body
    routing::{post, get}, // function to handle POST requests
    Router,       // Creates URL routing
    http::StatusCode,
};

use serde::{Deserialize, Serialize}; // Serde is a library for serializing and deserializing JSON


/*
    PubSubMessage is the main struct that contains the message and subscription that we receive from the webhook
    PubSubData is the struct that contains the data, message id, and publish time
    GmailNotification is the struct that contains the email address and history id after we deserialize the data
*/


// Debug makes the struct printable
// Deserialize makes the struct creatable from JSON
#[derive(Debug, Deserialize)]
struct GmailNotification {
    email_address: String,
    history_id: String,
};


#[derive(Debug, Deserialize)]
struct PubSubData {
    data: String,
    #[serde(rename = "messageId")]
    message_id: String,
    #[serde(rename = "publishTime")]
    publish_time: String,
};


#[derive(Debug, Deserialize)]
struct PubSubMessage {
    message: PubSubData,
    subscription: String,
};


async fn handle_webhook() -> StatusCode {
    println!("Webhook recieved!");
    StatusCode::OK // returns HTTP status code 200 OK
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