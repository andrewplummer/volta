extern crate notion_core;

use notion_core::tool::{Tool, Npm};

/// The entry point for the `npm` shim.
pub fn main() {
    Npm::launch()
}