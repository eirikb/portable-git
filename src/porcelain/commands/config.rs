use anyhow::Result;
use std::path::PathBuf;

pub fn run(
    _repository: &PathBuf,
    key: Option<String>,
    value: Option<String>,
    _global: bool,
    _local: bool,
) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    if let (Some(_key), Some(_value)) = (key.as_ref(), value.as_ref()) {
        println!("fatal: setting configuration values not yet implemented");
        return Ok(());
    }

    if let Some(key) = key {
        let config = repo.config_snapshot();
        if let Some(value) = config.string(&key) {
            println!("{}", value);
        } else {
            println!("fatal: config key '{}' not found", key);
        }
        return Ok(());
    }

    println!("fatal: config listing not yet fully implemented");
    println!("Use 'git config <key>' to get specific values");

    Ok(())
}
