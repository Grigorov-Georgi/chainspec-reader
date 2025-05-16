use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

const KEY_NAMES_JSON: &str = include_str!("../key_names.json");
const OUTPUT_JSON: &str = "output.json";

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    chainspec: PathBuf,

    #[arg(short, long)]
    raw: bool,
}

#[derive(Debug, Deserialize)]
struct KeyName {
    name: String,
    key: String,
}

#[derive(Debug, Serialize)]
struct MergedKey {
    key: String,
    name: Option<String>,
    value: String,
}

fn load_key_names() -> Result<HashMap<String, String>> {
    let key_names: Vec<KeyName> = serde_json::from_str(KEY_NAMES_JSON)?;
    Ok(key_names.into_iter().map(|k| (k.key, k.name)).collect())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let chainspec_file = File::open(&args.chainspec)?;
    let chainspec: Value = serde_json::from_reader(BufReader::new(chainspec_file))?;

    let key_names_map = load_key_names()?;

    let top_pointer = if args.raw {
        "/genesis/raw/top"
    } else {
        "/genesis/top"
    };

    let top = chainspec
        .pointer(top_pointer)
        .context("Missing /genesis/top in chainspec")?
        .as_object()
        .context("/genesis/top is not an object")?;

    let code = String::from("0x3a636f6465");
    let merged: Vec<MergedKey> = top
        .iter()
        .filter(|(k, _)| *k != &code)
        .map(|(k, v)| MergedKey {
            key: k.clone(),
            name: key_names_map.get(&k.to_lowercase()).cloned(),
            value: v.as_str().unwrap_or_default().to_string(),
        })
        .collect();

    let output_file = File::create(OUTPUT_JSON)?;
    serde_json::to_writer_pretty(output_file, &merged)?;

    Ok(())
}
