use std::ops::Deref;
use std::thread::available_parallelism;
use crate::message::chatter_message::chat_server::Chat;
use tonic::{Request, Response, Status};
use crate::AVAILABLE_USERS;
use crate::message::chatter_list::list_users_request_service_server::ListUsersRequestService;
use crate::message::chatter_list::{ListUsersRequest, ListUsersResponse};
use crate::message::chatter_message::Message;

#[derive(Debug, Default)]
pub struct Chatter {}
#[tonic::async_trait]
impl Chat for Chatter {
    async fn send_message(&self, request: Request<Message>) -> Result<Response<Message>, Status> {
        let reply = Message {
            username: "Server".to_string(),
            message: String::from("Hello from the service!"),
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

pub struct AvailableUsers{
    active_users: Vec<String>,
    is_available: Vec<bool>
}
impl AvailableUsers{
    pub fn new() -> Self {
        return AvailableUsers{
            active_users: vec![],
            is_available: vec![],
        }
    }
}

//TODO: Add authentication
#[derive(Debug, Default)]
pub struct ListUsersRequestServiceChatter{}
impl ListUsersRequestServiceChatter{}
#[tonic::async_trait]
impl ListUsersRequestService for ListUsersRequestServiceChatter{
    async fn send_request(&self, request: Request<ListUsersRequest>) -> Result<Response<ListUsersResponse>, Status> {
        {
            let new_user = request.get_ref().clone().username;
            let mut available_users = AVAILABLE_USERS.lock().unwrap();
            let response = Response::new(ListUsersResponse{
                username:  available_users.active_users.clone(),
                available:  available_users.is_available.clone(),
            });
            if !available_users.active_users.contains(&new_user){
                available_users.active_users.push(new_user);
                available_users.is_available.push(true);
            }
            Ok(response)
        }

    }

}
