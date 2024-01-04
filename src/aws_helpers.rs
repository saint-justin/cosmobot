//! A set of AWS helper functions.

use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::{Client, Error};
const SECRET_ID: &str = "bot_token";

/// Retreives the Discord bot API token from AWS SecretsManager, returns
/// a result or any error that prevented token retrieval
pub async fn get_bot_token() -> Result<String, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&config);
    let res = client
        .get_secret_value()
        .secret_id(SECRET_ID)
        .send()
        .await?;
    
    println!("Bot secret token successfully retreived");
    Ok(res.secret_string().unwrap().to_owned())
}
