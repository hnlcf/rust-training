use super::{LoadConfig, ValidateConfig};
use crate::profile::xdiff::DiffProfile;
use std::collections::HashMap;

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

impl DiffConfig {
    pub fn new(profiles: HashMap<String, DiffProfile>) -> Self {
        Self { profiles }
    }

    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}

impl LoadConfig for DiffConfig {}

impl ValidateConfig for DiffConfig {
    fn validate(&self) -> Result<()> {
        for (name, profile) in &self.profiles {
            profile
                .validate()
                .context(format!("falied to validate profile: {}", name))?;
        }
        Ok(())
    }
}
