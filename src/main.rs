use structopt::StructOpt;
use aws_config::BehaviorVersion;
use aws_sdk_ecr::Client;
use std::io::{self, Write};

#[derive(StructOpt)]
struct Cli {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Cli::from_args();

    // Setup AWS Client
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let res = client.describe_repositories().send().await?;

    if let Some(repositories) = res.repositories {
        println!("ECR Repositories:");
        for (index, repo) in repositories.iter().enumerate() {
            println!("{}. {}", index + 1, repo.repository_name.as_deref().unwrap_or("Unknown"));
        }

        // Prompt the user to select a repository
        print!("Select a repository by number: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let selected_index: usize = input.trim().parse()?;

        if selected_index == 0 || selected_index > repositories.len() {
            println!("Invalid selection");
            return Ok(());
        }

        let selected_repo = &repositories[selected_index - 1];
        let repo_name = selected_repo.repository_name.as_deref().unwrap_or("Unknown");

        // List images in the selected repository
        println!("Selected repository: {}", repo_name);
        let images_resp = client
            .list_images()
            .repository_name(repo_name)
            .send()
            .await?;

        if let Some(image_ids) = images_resp.image_ids {
            if image_ids.is_empty() {
                println!("No images found in repository");
            } else {
                println!("Images in repository '{}':", repo_name);
                for image_id in image_ids {
                    println!("  Image: {:?}", image_id.image_tag.as_deref().unwrap_or("No tag"));
                }
            }
        } else {
            println!("No images found in repository");
        }
    } else {
        println!("No repositories found");
    }

    Ok(())
}
