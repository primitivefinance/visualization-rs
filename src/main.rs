mod design;
mod examples;
mod functions;
mod plot;
mod file_handler;
#[allow(unused)]
use file_handler::*;
use design::DisplayMode;
use examples::*;
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
