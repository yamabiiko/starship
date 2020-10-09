// Lib is present to allow for benchmarking
pub mod bug_report;
pub mod config;
pub mod configs;
pub mod configure;
pub mod context;
pub mod formatter;
pub mod git;
pub mod logger;
pub mod module;
mod modules;
pub mod print;
pub mod segment;
mod utils;

#[cfg(test)]
mod test;
