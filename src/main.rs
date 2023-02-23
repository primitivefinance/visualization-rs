use std::ops::Div;

use itertools_num::linspace;
use statrs::consts;

mod functions;
mod plot;
use plot::{Color, DisplayMode, Emphasis};

fn main() {
    // Global Toggle Variables
    let transparent = true;
    let display_mode = DisplayMode::Light;
    let show = true;

    // Plot RMM trading curve for multiple taus from a list of prices
    let strike = 3_f64;
    let sigma = 0.5_f64;
    let taus: Vec<f64> = linspace(1.0, 1.0, 1).collect::<Vec<f64>>();
    let plot_name = format!(
        "{} {} {} {} {} {} {}",
        "$\\text{RMM Trading Curves with K=}",
        strike,
        "\\text{, }\\sigma=",
        sigma,
        "\\text{, }\\tau=",
        taus[0],
        "$"
    );
    let p_0 = 0.0_f64;
    let p_1 = 100.0_f64;
    let n = 1000;
    let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
    let mut x: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<Vec<f64>> = Vec::new();
    for tau in taus.iter() {
        let (x_tau, y_tau) = functions::rmm_trading_curve(prices.clone(), strike, sigma, *tau, None);
        x.push(x_tau);
        y.push(y_tau);
    }
    let x_bounds = vec![0_f64, 1_f64];
    let y_bounds = vec![0_f64, strike];
    let colors = vec![(Color::Green, plot::MAIN_SLOT, Emphasis::Heavy)];
    plot::transparent_plot(
        (x, y),
        (x_bounds, y_bounds),
        plot_name,
        None,
        colors,
        (transparent, display_mode, show),
    );
}
