#![allow(deprecated)]
use config::Config;
use lazy_static::lazy_static;
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::builder()
    .add_source(config::Environment::with_prefix("APP"))
    .build()
    .unwrap());
}