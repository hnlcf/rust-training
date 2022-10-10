pub mod xdiff;
pub mod xreq;

use anyhow::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use tokio::fs;

#[async_trait]
pub trait LoadConfig
where
    Self: Sized + ValidateConfig + DeserializeOwned,
{
    /// Load config from yaml file
    async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }
    /// Load config from yaml string
    fn from_yaml(content: &str) -> Result<Self> {
        let config: Self = serde_yaml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }
}

pub trait ValidateConfig {
    fn validate(&self) -> Result<()>;
}
