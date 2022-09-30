use crate::profile::xdiff::DiffProfile;
use std::collections::HashMap;

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

impl DiffConfig {
    pub fn new(profiles: HashMap<String, DiffProfile>) -> Self {
        Self { profiles }
    }

    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    pub fn from_yaml(content: &str) -> Result<Self> {
        let config: DiffConfig = serde_yaml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }

    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }

    pub fn validate(&self) -> Result<()> {
        for (name, profile) in &self.profiles {
            profile
                .validate()
                .context(format!("falied to validate profile: {}", name))?;
        }
        Ok(())
    }
}
