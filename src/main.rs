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

fn main() {
    // Global visualization variables
    let display = Display {
        transparent: true,
        mode: DisplayMode::Dark,
        show: true,
    };
    brownian_bridge_plotter(display, 1200.0, 2000.0);
}
