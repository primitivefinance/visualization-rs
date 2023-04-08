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
        mode: DisplayMode::Light,
        show: true,
    };
    leverage_zones_with_pvf(display);
}
