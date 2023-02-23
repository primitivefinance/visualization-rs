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
    let tau = 2.0;
    let plot_name = format!(
        "{} {} {} {} {} {} {}",
        "$\\text{RMM Trading Curves with K=}",
        strike,
        "\\text{, }\\sigma=",
        sigma,
        "\\text{, }\\tau=",
        tau,
        "$"
    );
    let p_0 = 0.0_f64;
    let p_1 = 100.0_f64;
    let n = 1000;
    let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
    let mut x: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<Vec<f64>> = Vec::new();
    let scale_factors = linspace(0.1, 2.0, 6).collect::<Vec<f64>>();
    for scale_factor in scale_factors.iter() {
        let (x_scale, y_scale) = functions::rmm_trading_curve(prices.clone(), strike, sigma, tau, Some(*scale_factor));
        x.push(x_scale);
        y.push(y_scale);
    }
    let t = linspace(0.0, 1.0, 1000).collect::<Vec<f64>>();
    let slopes = vec![(2.0,1.0), (1.0, 1.0), (1.0,2.0)];
    for slope in &slopes {
        let curve = functions::parametric_line(t.clone(), slope.0, slope.1, 0.0, 0.0);
        x.push(curve.0);
        y.push(curve.1);
    }
    // let curve = functions::parametric_line(t, 1.0, 1.0, 0.0, 0.0);
    // x.push(curve.0);
    // y.push(curve.1);
    let x_bounds = vec![0_f64, 1_f64];
    let y_bounds = vec![0_f64, strike];
    let single_color = true;
    let colors = vec![(Color::Green, plot::MAIN_COLOR_SLOT, Emphasis::Heavy, single_color)];
    plot::transparent_plot(
        (x, y),
        (x_bounds, y_bounds),
        plot_name,
        None,
        colors,
        (transparent, display_mode, show),
    );
}
