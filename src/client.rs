mod message;

use std::time::Duration;
use message::chatter_list::list_users_request_service_client::ListUsersRequestServiceClient;
use rand::prelude::*;
use tokio::time::interval;
use tonic::transport::Channel;
use crate::message::chatter_list::ListUsersRequest;

async fn initialize_chat_service(channel: Channel) ->  Result<(), Box<dyn std::error::Error>> {
    let mut list_user_client = ListUsersRequestServiceClient::new(channel.clone());
    let mut rng = rand::rng();
    let username = format!("user-{}", rng.random::<u32>());
    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await; // Wait for next tick
        let list_user_request = tonic::Request::new(ListUsersRequest{
            username: username.clone(),
        });
        match list_user_client.send_request(list_user_request).await {
            Ok(response) => {
                println!("Received: {:?}", response.into_inner());
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;
    let _ = initialize_chat_service(channel).await;
    Ok(())
}