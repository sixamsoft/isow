#![allow(unused_imports)]
use clap::{Clap};

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Options {
    #[clap(short, long)]
    pub utc: bool,
    #[clap(short, long)]
    pub year: bool,
    #[clap(short, long)]
    pub day: bool,
    #[clap(short, long)]
    pub week: bool,
    #[clap(short, long)]
    pub time: bool,
    #[clap(subcommand)]
    pub patcher: Patcher,
}

#[derive(Clap, Debug)]
pub enum Patcher {
    Update(Updater)
}

#[derive(Clap, Debug)]
pub struct Updater {
    #[clap(short, long)]
    pub list: bool,
}