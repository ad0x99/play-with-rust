use std::env;

use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};
use tokio::time::{sleep, Duration};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::authentication::{SignInResponse, SignOutResponse, SignUpResponse, StatusCode};

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AUTH_SERVICE_HOST_NAME will be set to 'auth' when running the health check service in Docker
    // ::0 is required for Docker to work: https://stackoverflow.com/questions/59179831/docker-app-server-ip-address-127-0-0-1-difference-of-0-0-0-0-ip
    let auth_hostname = env::var("AUTH_SERVICE_HOST_NAME").unwrap_or("[::0]".to_owned());

    // Establish connection with the auth service
    let mut client = AuthClient::connect(format!("http://{}:50052", auth_hostname)).await?;

    loop {
        // Generate random username and password using UUID
        let username: String = Uuid::new_v4().to_string();
        let password: String = Uuid::new_v4().to_string();

        // Create a sign-up request
        let request: Request<SignUpRequest> = tonic::Request::new(SignUpRequest {
            username: username.clone(),
            password: password.clone(),
        });

        // Make a sign-up request and get the response
        let response: Response<SignUpResponse> = client.sign_up(request).await?;

        // Log the sign-up response status
        println!(
            "SIGN UP RESPONSE STATUS: {:?}",
            StatusCode::try_from(response.into_inner().status_code).unwrap()
        );

        // ---------------------------------------------

        // Create a sign-in request
        let request: Request<SignInRequest> = tonic::Request::new(SignInRequest {
            username: username.clone(),
            password: password.clone(),
        });

        // Make a sign-in request and get the response
        let response: SignInResponse = client.sign_in(request).await?.into_inner();

        // ---------------------------------------------

        // Create a sign-out request using the session token from the sign-in response
        let request: Request<SignOutRequest> = tonic::Request::new(SignOutRequest {
            session_token: response.session_token,
        });

        // Make a sign-out request and get the response
        let response: Response<SignOutResponse> = client.sign_out(request).await?;

        // Log the sign-out response status
        println!(
            "SIGN OUT RESPONSE STATUS: {:?}",
            StatusCode::try_from(response.into_inner().status_code).unwrap()
        );

        println!("--------------------------------------",);

        // Sleep for 3 seconds before the next iteration
        sleep(Duration::from_secs(3)).await;
    }
}
