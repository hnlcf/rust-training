use anyhow::Result;
use xdiff_core::config::xdiff::DiffConfig;

fn main() -> Result<()> {
    let content = include_str!("../fixtures/test.yaml");
    let config = DiffConfig::from_yaml(content)?;

    println!("{:#?}", config);
    Ok(())
}
