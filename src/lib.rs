//! # kd library
//! Base business logic for the kd project
pub mod korean;
pub mod music;
pub mod config;
pub mod models;

pub trait DisplayMoreInfo {
    fn more_info(&self) -> String;
}