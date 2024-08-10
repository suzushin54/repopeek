use aws_config::BehaviorVersion;
use aws_sdk_ecr::Client;
use inquire::Select;
use crate::aws_profile::get_profile_names;


/// Sets up the AWS client with the specified profile
///
/// # Arguments
///
/// * `profile` - Profile name
///
/// # Returns
///
/// Returns a Result with the AWS client or an error
pub async fn setup_aws_client(profile: &str) -> Result<Client, Box<dyn std::error::Error>> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(aws_config::Region::new("ap-northeast-1"))
        .profile_name(profile)
        .load()
        .await;
    Ok(Client::new(&config))
}

/// Prompts the user to select an AWS profile and sets up the AWS client
///
/// # Returns
///
/// Returns a Result with the AWS client or an error
pub async fn setup_aws_client_with_user_selection() -> Result<Client, Box<dyn std::error::Error>> {
    let profiles = get_profile_names()?;
    if profiles.is_empty() {
        return Err("No AWS profiles found".into());
    }

    let selected_profile = Select::new("Select an AWS profile:", profiles).prompt()?;
    setup_aws_client(&selected_profile).await
}

/// Retrieves the AWS account ID using the ECR client
pub async fn get_account_id(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let result = client.describe_registry().send().await?;
    result.registry_id()
        .map(String::from)
        .ok_or_else(|| "Failed to retrieve account ID".into())
}

/// Retrieves the configured region from the AWS client configuration
pub fn get_region(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    client.config().region().map(|r| r.to_string()).ok_or_else(|| "Region not found".into())
}
