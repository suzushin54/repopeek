use structopt::StructOpt;
use aws_config::BehaviorVersion;
use aws_sdk_ecr::Client;
use inquire::Select;

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
        let repo_names: Vec<&str> = repositories.iter()
            .map(|repo| repo.repository_name.as_deref().unwrap_or("Unknown"))
            .collect();

        // Prompt the user to select a repository
        let selected_repo_name = Select::new("Select a repository:", repo_names)
            .prompt()
            .unwrap();

        // List images in the selected repository
        println!("Selected repository: {}", selected_repo_name);
        let images_resp = client
            .list_images()
            .repository_name(selected_repo_name)
            .send()
            .await?;

        if let Some(image_ids) = images_resp.image_ids {
            if image_ids.is_empty() {
                println!("No images found in repository");
            } else {
                println!("Images in repository '{}':", selected_repo_name);
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
