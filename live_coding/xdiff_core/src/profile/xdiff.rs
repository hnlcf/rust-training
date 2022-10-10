use super::{req::RequestProfile, res::ResponseProfile};
use crate::utils::diff_text;
use crate::{cli::OverrideArgs, config::ValidateConfig};

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    #[serde(skip_serializing_if = "is_default", default)]
    pub res: ResponseProfile,
}

fn is_default<T>(v: &T) -> bool
where
    T: Default + PartialEq,
{
    v == &T::default()
}

impl ValidateConfig for DiffProfile {
    fn validate(&self) -> Result<()> {
        self.req1.validate().context("req1 failed to validate")?;
        self.req2.validate().context("req2 failed to validate")?;
        Ok(())
    }
}

impl DiffProfile {
    pub fn new(req1: RequestProfile, req2: RequestProfile, res: ResponseProfile) -> Self {
        Self { req1, req2, res }
    }

    pub async fn diff(&self, args: OverrideArgs) -> Result<String> {
        let res1 = self.req1.send(&args).await?;
        let res2 = self.req2.send(&args).await?;

        let text1 = res1.get_text(&self.res).await?;
        let text2 = res2.get_text(&self.res).await?;

        diff_text(&text1, &text2)
    }
}
