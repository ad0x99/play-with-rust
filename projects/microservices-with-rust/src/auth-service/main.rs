use std::sync::Mutex;

mod auth;
mod sessions;
mod users;

use auth::*;
use sessions::{Sessions, SessionsImpl};
use users::{Users, UsersImpl};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces. This is needed for Docker to work, which we will add later on.
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
    // Port 50052 is the recommended gRPC port.
    let address = "[::0]:50052"
        .parse()
        .unwrap_or_else(|error| panic!("Failed to listening on 50052 {}", error));

    // Create user service instance
    let users_service: Box<Mutex<dyn Users + Send + Sync + 'static>> =
        Box::new(Mutex::new(UsersImpl::default()));

    // Create session service instance
    let sessions_service: Box<Mutex<dyn Sessions + Send + Sync + 'static>> =
        Box::new(Mutex::new(SessionsImpl::default()));

    let auth_service = AuthService::new(users_service, sessions_service);

    // Initialize gRPC server
    Server::builder()
        .add_service(AuthServer::new(auth_service))
        .serve(address)
        .await?;

    Ok(())
}
