use clap::{Parser, Subcommand};
use std::env;

use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};

// Import the generated protobuf code for the authentication service
pub mod authentication {
    tonic::include_proto!("authentication");
}

// Define the command line interface using clap
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // Define the subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

// Define the possible subcommands
#[derive(Subcommand)]
enum Commands {
    SignIn {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignUp {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignOut {
        #[arg(short, long)]
        session_token: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the IP address of the authentication service from environment variables
    let auth_ip = env::var("AUTH_SERVICE_IP").unwrap_or("[::0]".to_owned());
    // Connect to the authentication service
    let mut client = AuthClient::connect(format!("http://{}:50052", auth_ip)).await?;

    // Parse the command line arguments
    let cli = Cli::parse();

    // Match the subcommand and execute the corresponding logic
    match &cli.command {
        Some(Commands::SignIn { username, password }) => {
            // Create a sign-in request
            let request = tonic::Request::new(SignInRequest {
                username: username.clone(),
                password: password.clone(),
            });

            // Send the sign-in request and get the response
            let response = client.sign_in(request).await?.into_inner();

            // Print the response
            println!("{:?}", response);
        }
        Some(Commands::SignUp { username, password }) => {
            // Create a sign-up request
            let request = tonic::Request::new(SignUpRequest {
                username: username.clone(),
                password: password.clone(),
            });

            // Send the sign-up request and get the response
            let response = client.sign_up(request).await?;

            // Print the response
            println!("{:?}", response.into_inner());
        }
        Some(Commands::SignOut { session_token }) => {
            // Create a sign-out request
            let request = tonic::Request::new(SignOutRequest {
                session_token: session_token.clone(),
            });

            // Send the sign-out request and get the response
            let response = client.sign_out(request).await?;

            // Print the response
            println!("{:?}", response.into_inner());
        }
        None => {}
    }

    Ok(())
}
