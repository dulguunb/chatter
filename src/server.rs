mod message;
mod service;

use tonic::{transport::Server, Request, Response, Status};
use message::chatter_message::chat_server::Chat;
use crate::message::chatter_message::chat_server::ChatServer;
use service::api::Chatter;
use service::api::ListUsersRequestServiceChatter;
use message::chatter_list::list_users_request_service_server::ListUsersRequestServiceServer;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::service::api::AvailableUsers;

static AVAILABLE_USERS: Lazy<Mutex<AvailableUsers>> = Lazy::new(|| {
    Mutex::new(AvailableUsers::new())
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let chatter = Chatter::default();
    let chat_requester = ListUsersRequestServiceChatter::default();
    Server::builder()
        .add_service(ChatServer::new(chatter))
        .add_service(ListUsersRequestServiceServer::new(chat_requester))
        .serve(addr)
        .await?;
    Ok(())
}