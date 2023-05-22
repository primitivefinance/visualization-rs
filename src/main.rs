mod examples;
use examples::*;
#[allow(unused)]
use visualize::{design::DisplayMode, file_handler::*, plot::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Global visualization variables
    let display = Display {
        transparent: false,
        mode: DisplayMode::Light,
        show: true,
    };
    simulation_fee_growth(display)?;

    Ok(())
}
