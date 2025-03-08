use aws_config::BehaviorVersion;
use aws_sdk_ecr::Client;
use inquire::Select;
use crate::aws_profile::get_profile_names;
use std::future::Future;

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
/// Returns a Result with a tuple containing the AWS client and the selected profile name
pub fn setup_aws_client_with_user_selection() -> impl Future<Output = Result<(Client, String), Box<dyn std::error::Error>>> {
    async move {
        let profiles = get_profile_names()?;
        if profiles.is_empty() {
            return Err("No AWS profiles found".into());
        }

        let selected_profile = Select::new("Select an AWS profile:", profiles).prompt()?;
        let client = setup_aws_client(&selected_profile).await?;
        Ok((client, selected_profile))
    }
}

/// Retrieves the AWS account ID using the ECR client
pub async fn get_account_id(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let result = client.describe_registry().send().await?;
    let Some(registry_id) = result.registry_id() else {
        return Err("Failed to retrieve account ID".into());
    };
    Ok(registry_id.to_string())
}

/// Retrieves the configured region from the AWS client configuration
pub fn get_region(client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let Some(region) = client.config().region() else {
        return Err("Region not found".into());
    };
    Ok(region.to_string())
}
