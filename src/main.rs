use std::ops::Div;

use itertools_num::linspace;
use statrs::{consts, function};

mod functions;
mod plot;
use plot::{Color, DisplayMode, Emphasis, Labels};

fn main() {
    // Global Toggle Variables
    let transparent = true;
    let display_mode = DisplayMode::Light;
    let show = true;

    // ------------------ Plotting Plot 3 ------------------ //
    // Plot RMM trading curve for multiple taus from a list of prices
    let plot_name = "$\\text{RMM Liquidity Distribution}$".to_string();
    let strike = 3_f64;
    let sigma = 0.5_f64;
    let taus: Vec<f64> = linspace(2.0, 0.1, 5).collect::<Vec<f64>>();
    let p_0 = 0.0_f64;
    let p_1 = 10.0_f64;
    let n = 1000;
    let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
    let mut x: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<Vec<f64>> = Vec::new();
    for tau in &taus {
        x.push(prices.clone());
        let temp = functions::standard_gaussian_pdf(functions::d_one(prices.clone(), strike, sigma, *tau));
        let temp = temp.iter().map(|x| x / (sigma * tau.sqrt())).collect::<Vec<f64>>();
        let mut temp1: Vec<f64> = Vec::new();
        for (i, y_val) in temp.iter().enumerate() {
            temp1.push(y_val / prices[i]);
        }
        y.push(temp1);
    };
    let x_bounds = vec![0_f64, 10_f64];
    let y_bounds = vec![0_f64, 1_f64];
    let single_color = false;
    let colors = vec![
        (Color::Green, 0, Emphasis::Light, single_color),
        (Color::Green, 1, Emphasis::Light, single_color),
        (Color::Green, 2, Emphasis::Light, single_color),
        (Color::Green, 3, Emphasis::Light, single_color),
        (Color::Green, plot::MAIN_COLOR_SLOT, Emphasis::Heavy, single_color),
    ];
    let legend_names = vec![
        "$\\tau=2.0$".to_string(),
        "$\\tau=1.5$".to_string(),
        "$\\tau=1.0$".to_string(),
        "$\\tau=0.5$".to_string(),
        "$\\tau=0.1$".to_string(),
    ];
    let labels = Labels {
        x_label: "$R_x$".to_string(),
        y_label: "$R_y$".to_string(),
    };
    plot::transparent_plot(
        (x, y),
        (x_bounds, y_bounds),
        plot_name,
        Some(legend_names),
        colors,
        (transparent, display_mode, show),
        labels,
    );
}
