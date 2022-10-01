use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};
use std::io::Write;

use xdiff_core::{
    cli::{Action, Args, OverrideArgs, RunArgs},
    config::{xdiff::DiffConfig, xreq::RequestConfig},
    profile::{req::RequestProfile, res::ResponseProfile, xdiff::DiffProfile},
    utils::highlight_text,
    LoadConfig,
};

const DEFAULT_CONFIG_PATH: &str = "~/.xreq";

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Run(arg) => run(arg).await?,
        Action::Parse => parse().await?,
        _ => panic!("Not implemented action"),
    };

    Ok(())
}

async fn run(arg: RunArgs) -> Result<()> {
    let config_file = arg
        .config
        .unwrap_or_else(|| DEFAULT_CONFIG_PATH.to_string());
    let config = RequestConfig::load_yaml(&config_file).await?;
    let profile = config.get_profile(&arg.profile).ok_or_else(|| {
        anyhow!(
            "Profile {} not found in config file {}",
            arg.profile,
            config_file
        )
    })?;

    let override_args = arg.override_args.into();
    let res = profile.send(&override_args).await?;
    write!(std::io::stdout().lock(), "{:?}", res)?;

    Ok(())
}

async fn parse() -> Result<()> {
    let theme = ColorfulTheme::default();
    let profile_name: String = Input::with_theme(&theme)
        .with_prompt("Profile name")
        .interact_text()?;
    let url1: String = Input::with_theme(&theme)
        .with_prompt("Url1")
        .interact_text()?;
    let url2: String = Input::with_theme(&theme)
        .with_prompt("Url2")
        .interact_text()?;
    let req1: RequestProfile = url1.parse()?;
    let req2: RequestProfile = url2.parse()?;

    let headers = req1.send(&OverrideArgs::default()).await?.get_header_keys();
    let skip_headers = MultiSelect::with_theme(&theme)
        .with_prompt("Select items to skip")
        .items(&headers)
        .interact()?
        .iter()
        .map(|&idx| headers.get(idx).unwrap().to_string())
        .collect();

    let res = ResponseProfile::new(skip_headers, vec![]);
    let profile = DiffProfile::new(req1, req2, res);
    let config = DiffConfig::new(vec![(profile_name, profile)].into_iter().collect());

    let result = serde_yaml::to_string(&config)?;
    write!(
        std::io::stdout().lock(),
        "---\n{}",
        highlight_text(&result, "yaml")?
    )?;

    Ok(())
}
