mod design;
mod examples;
mod functions;
mod plot;
use design::DisplayMode;
use examples::{
    compare_approximation_types, polynomial_approximations, rmm_trading_curve_multiple_taus,
};
use plot::*;

fn main() {
    // Global visualization variables
    let display = Display {
        transparent: true,
        mode: DisplayMode::Light,
        show: true,
    };
    rmm_trading_curve_multiple_taus(display);
}
