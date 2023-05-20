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
        transparent: true,
        mode: DisplayMode::Dark,
        show: true,
    };
    plot_forced_rebalance(display)?;

    Ok(())
}
