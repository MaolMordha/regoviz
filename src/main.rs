#![feature(assert_matches)]
use std::{error::Error, fs, path::PathBuf, str::from_utf8};

use clap::Parser;
use policy::Policy;

mod cli;
mod policy;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = cli::Cli::parse();
    let file = fs::read(PathBuf::from(cli.policy_path))?;
    let content = from_utf8(&file)?;
    let policy = Policy::from_json(content)?;

    println!("{}", policy.r#static().string(0).value());
    Ok(())
}
