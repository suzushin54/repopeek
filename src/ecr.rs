use aws_sdk_ecr::Client;

/// Lists all repositories in the ECR
///
/// # Arguments
///
/// * `client` - The AWS ECR client
///
/// # Returns
///
/// Returns a Result with a vector of repository names or an error
pub async fn list_repositories(client: &Client) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let res = client.describe_repositories().send().await?;
    let repositories = match res.repositories {
        Some(repos) => repos.into_iter()
            .map(|repo| repo.repository_name.unwrap_or("Unknown".to_string()))
            .collect(),
        None => vec![],
    };
    Ok(repositories)
}

/// Lists all images in the specified repository and prints the image tags
///
/// # Arguments
///
/// * `client` - The AWS ECR client
/// * `repo_name` - The name of the repository
///
/// # Returns
///
/// A result indicating success or failure. If successful, the function prints the image tags.
pub async fn list_images_in_repository(client: &Client, repo_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Selected repository {}", repo_name);

    let images = client
        .list_images()
        .repository_name(repo_name)
        .send()
        .await?;

    let image_ids = match images.image_ids {
        Some(ids) => ids,
        None => {
            println!("No images found in repository");
            return Ok(());
        }
    };

    if image_ids.is_empty() {
        println!("No images found in repository");
        return Ok(());
    }

    println!("Images in repository '{}':", repo_name);

    for image_id in image_ids {
        println!("  Image: {:?}", image_id.image_tag.as_deref().unwrap_or("No tags"));
    }

    Ok(())
}