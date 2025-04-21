use crate::message::chatter_message::chat_server::Chat;
use tonic::{Request, Response, Status};
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

//TODO: Add authentication
#[derive(Debug, Default)]
pub struct ListUsersRequestServiceChatter{
    active_users: Vec<String>,
    is_available: Vec<bool>
}
impl ListUsersRequestServiceChatter{
    pub(crate) fn new() -> Self {
        ListUsersRequestServiceChatter{
            active_users: vec![],
            is_available: vec![],
        }
    }
}
#[tonic::async_trait]
impl ListUsersRequestService for ListUsersRequestServiceChatter{
    async fn send_request(&mut self, request: Request<ListUsersRequest>) -> Result<Response<ListUsersResponse>, Status> {
        let response = Response::new(ListUsersResponse{
            username: self.active_users.clone(),
            available: self.is_available.clone(),
        });
        self.active_users.push(request.get_ref().clone().username);
        self.is_available.push(true);
        Ok(response)
    }


}
