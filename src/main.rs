// use color_eyre::eyre::Result;

use std::error::Error;

pub mod examples;

//fn main() -> Result<()> {
fn main() -> Result<(), Box<dyn Error>> {
    // examples::hello_world::run()
    // examples::app_counter::run()
    examples::json_editor::run()
}
