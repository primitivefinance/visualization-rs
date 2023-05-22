mod design;
mod examples;
mod file_handler;
mod functions;
mod plot;
use design::DisplayMode;
use examples::*;
#[allow(unused)]
use file_handler::*;
use plot::*;

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
