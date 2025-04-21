mod message;
use message::chatter_message::chat_client::ChatClient;
use message::chatter_list::list_users_request_service_client;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatClient::connect("http://[::1]:50051").await?;
    let username  = "random1";
    let request = tonic::Request::new(list_users_request_service_client {
        username: username,
    });

    let response = client.send_message(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}