use anyhow::Result;
use xdiff::config::DiffConfig;

fn main() -> Result<()> {
    let content = include_str!("../fixtures/test.yaml");
    let config = DiffConfig::from_yaml(content)?;

    println!("{:#?}", config);
    Ok(())
}
