use structopt::StructOpt;
use aws_config::BehaviorVersion;
use aws_sdk_ecr::Client;

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
        for repo in repositories {
            let repo_name = repo.repository_name.unwrap_or_default();
            println!("Repository: {}", repo_name);

            // Listing docker images
            let images = client
                .list_images()
                .repository_name(&repo_name)
                .send()
                .await?;

            if let Some(image_ids) = images.image_ids {
                for image_id in image_ids {
                    println!(" Image: {:?}", image_id.image_tag.unwrap_or_default());
                }
            } else {
                println!(" No images found in repository");
            }
        }
    } else {
        println!("No repositories found");
    }

    Ok(())
}
