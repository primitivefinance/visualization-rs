mod design;
mod examples;
mod functions;
mod plot;
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
    rmm_portfolio_value(display);
}
