use std::path::PathBuf;
use ini::ini;

/// Reads the AWS config file and returns a list of profile names
///
/// # Returns
///
/// Returns a Result with a vector of profile names or an error
pub fn get_profile_names() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let config_file_path = PathBuf::from(shellexpand::tilde("~/.aws/config").to_string());
    let conf = ini!(&config_file_path.to_str().unwrap());

    let mut profiles = Vec::new();
    for (section_name, _prop) in &conf {
        if section_name == "default" {
            profiles.push("default".to_string());
        } else if section_name.starts_with("profile ") {
            profiles.push(section_name.trim_start_matches("profile ").to_string());
        }
    }

    Ok(profiles)
}