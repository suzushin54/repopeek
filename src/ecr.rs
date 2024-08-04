use aws_sdk_ecr::Client;
use aws_sdk_ecr::Error;
use aws_sdk_ecr::types::ImageDetail;

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
/// Returns a Result with a vector of image tags or an error
pub async fn list_images_in_repository(client: &Client, repo_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("Selected repository {}", repo_name);

    let images = client
        .list_images()
        .repository_name(repo_name)
        .send()
        .await?;

    let image_ids = match images.image_ids {
        Some(ids) => ids.into_iter()
            .map(|image_id| image_id.image_tag.unwrap_or("No tags".to_string()))
            .collect(),
        None => vec![],
    };

    Ok(image_ids)
}

pub async fn describe_images(client: &Client, repo_name: &str) -> Result<Vec<ImageDetail>, Error> {
    let result = client.describe_images()
        .repository_name(repo_name)
        .send()
        .await?;

    Ok(result.image_details().to_vec())
}
