use aws_sdk_ecr::Client;
use aws_sdk_ecr::Error;
use aws_sdk_ecr::types::ImageDetail;
use std::process::Command;

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
        .max_results(100)  
        .send()
        .await?;
    
    // Convert Option<&[ImageDetail]> to Vec<ImageDetail>
    let mut image_details = result.image_details().to_vec();
    
    // Sort by pushed date (newest first)
    image_details.sort_by(|a, b| {
        let a_date = a.image_pushed_at();
        let b_date = b.image_pushed_at();
        
        match (b_date, a_date) {
            (Some(b_dt), Some(a_dt)) => b_dt.cmp(&a_dt), 
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
    
    // Take only the latest 30 images
    let latest_images = if image_details.len() > 30 {
        image_details[0..30].to_vec()
    } else {
        image_details
    };
    
    Ok(latest_images)
}

/// Authenticates with AWS ECR
///
/// # Arguments
///
/// * `account_id` - The AWS account ID
/// * `region` - The AWS region
/// * `profile` - The AWS profile name
///
/// # Returns
///
/// Returns a Result indicating success or failure
pub async fn authenticate_with_ecr(account_id: &str, region: &str, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting ECR authentication process...");

    let account_url = format!("{}.dkr.ecr.{}.amazonaws.com", account_id, region);

    let status = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "AWS_PROFILE={} aws ecr get-login-password --region {} | docker login --username AWS --password-stdin {}",
            profile, region, account_url
        ))
        .status()?;

    if !status.success() {
        return Err("Failed to authenticate with ECR".into());
    }

    println!("Successfully authenticated with ECR");
    Ok(())
}

