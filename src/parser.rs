extern crate toml;
use std::fs;

#[derive(Debug)]
struct Config {
    products: Vec[str],
}

fn parser() -> Config {
    let file = fs::read_to_string("prices.toml")?.parse()?;
}