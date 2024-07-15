use aws_config::BehaviorVersion;
use aws_sdk_ecr::Client;

/// Sets up the AWS client
///
/// # Returns
///
/// Returns a Result with the AWS client or an error
pub async fn setup_aws_client() -> Result<Client, Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    Ok(Client::new(&config))
}