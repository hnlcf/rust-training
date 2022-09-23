use anyhow::Result;
use lou_grep::Config;

fn main() -> Result<()> {
    let args = std::env::args();
    let config = Config::try_parse_config(args)?;
    lou_grep::run(config)?;

    Ok(())
}
