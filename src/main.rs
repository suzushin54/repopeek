mod aws_client;
mod ecr;

use structopt::StructOpt;
use inquire::Select;
use crate::aws_client::setup_aws_client;
use ecr::{list_repositories, list_images_in_repository};


/// Command-line interface for the application
#[derive(StructOpt)]
struct Cli {}

/// Main function
///
/// This function sets up the AWS client, lists the repositories,
/// prompts the user to select a repository, and then lists the images in the selected repository.
///
/// # Returns
///
/// Returns a Result with the success or error status of the operation.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Cli::from_args();

    // Setup AWS Client
    let client = setup_aws_client().await?;

    let repositories = list_repositories(&client).await?;
    if repositories.is_empty() {
        println!("No repositories found");
        return Ok(());
    }

    let repo_names: Vec<&str> = repositories.iter()
        .map(|s| s.as_str()).collect();

    // Prompt the user to select a repository
    let selected_repo_name = Select::new("Select a repository:", repo_names)
        .prompt()
        .unwrap();

    // List images in the selected repository
    list_images_in_repository(&client, selected_repo_name).await?;

    Ok(())
}
