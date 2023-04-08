mod design;
mod examples;
mod functions;
mod plot;
use design::DisplayMode;
use examples::compare_approximation_types;
use plot::*;

fn main() {
    // Global visualization variables
    let display = Display {
        transparent: true,
        mode: DisplayMode::Light,
        show: true,
    };
    compare_approximation_types(display);
}
