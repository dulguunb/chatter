mod message;
mod service;

use tonic::{transport::Server, Request, Response, Status};
use message::chatter_message::chat_server::Chat;
use crate::message::chatter_message::chat_server::ChatServer;
use service::api::Chatter;
use service::api::ListUsersRequestServiceChatter;
use message::chatter_list::list_users_request_service_server::ListUsersRequestServiceServer;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let chatter = Chatter::default();
    let chat_requester = ListUsersRequestServiceChatter::new();
    Server::builder()
        .add_service(ChatServer::new(chatter))
        .add_service(ListUsersRequestServiceServer::new(chat_requester))
        .serve(addr)
        .await?;
    Ok(())
}