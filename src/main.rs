use color_eyre::eyre::Result;

pub mod examples;

fn main() -> Result<()> {
    // sshy::examples::hello_world::run()
    examples::app_counter::run()
}
