mod ecr;
mod aws_profile;
mod aws_client;

use structopt::StructOpt;
use inquire::Select;
use crate::aws_client::setup_aws_client_with_user_selection;
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
    let (client, selected_profile) = setup_aws_client_with_user_selection().await?;

    // Get AWS account ID and region
    let account_id = aws_client::get_account_id(&client).await?;
    let region = aws_client::get_region(&client)?;

    // Docker login for private repository
    ecr::authenticate_with_ecr(&account_id, &region, &selected_profile).await?;

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
        .map_err(|e| format!("Failed to select repository: {}", e))?;

    // List images in the selected repository
    let images = list_images_in_repository(&client, selected_repo_name).await?;
    if images.is_empty() {
        println!("No images found in repository");
        return Ok(());
    }

    // Fetch image details
    let image_details = match ecr::describe_images(&client, selected_repo_name).await {
        Ok(details) => details,
        Err(err) => {
            eprintln!("Error describing images: {:?}", err);
            return Ok(());
        }
    };

    let mut image_options = Vec::new();
    for detail in &image_details {
        let pushed_at = detail.image_pushed_at();
        let pushed_at_str = pushed_at.map_or_else(
            || "N/A".to_string(),
            |dt| dt.to_string()
        );

        let size = detail.image_size_in_bytes().unwrap_or_default();
        let digest = detail.image_digest().unwrap_or_default();
        let option = format!(
            "{}\t{}\t{} MB\t{}",
            detail.image_tags.clone().unwrap_or_default().first().cloned().unwrap_or_default(),
            pushed_at_str,
            size / 1024 / 1024,
            digest
        );
        image_options.push(option);
    }

    if image_options.is_empty() {
        println!("No images found in repository");
        return Ok(());
    }

    let selected_image = Select::new("Select an image to pull:", image_options)
        .prompt()
        .map_err(|e| format!("Failed to select image: {}", e))?;
    let selected_image_tag = selected_image.split('\t').next().unwrap_or_default().to_string();

    // Pull the selected image
    let docker_pull_command = format!(
        "docker pull {}.dkr.ecr.{}.amazonaws.com/{}:{}",
        selected_profile, aws_client::get_region(&client)?, selected_repo_name, selected_image_tag
    );

    println!("Running command: {}", docker_pull_command);
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(docker_pull_command)
        .status()
        .map_err(|e| format!("Failed to execute docker pull command: {}", e))?;

    if !status.success() {
        return Err("Docker pull command failed".into());
    }

    Ok(())
}
