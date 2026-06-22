use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CliArguments {
    #[arg(long)]
    pub output: Option<PathBuf>,
}
#[derive(Clone)]
pub struct Post {
    pub title: String,
    pub posted: String,
    pub author: String,
    pub path: String,
    pub description: String,
    pub estimated_reading_time: u64,
    pub markdown: String,
}

#[derive(Deserialize)]
pub struct PostDescription {
    pub title: String,
    pub description: String,
    pub posted: String,
    pub author: String,
}
