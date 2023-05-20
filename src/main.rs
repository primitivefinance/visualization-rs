mod examples;
use examples::*;
#[allow(unused)]
use visualize::{design::DisplayMode, file_handler::*, plot::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Global visualization variables
    let display = Display {
        transparent: true,
        mode: DisplayMode::Light,
        show: true,
    };
    plot_forced_rebalance(display)?;

    Ok(())
}
