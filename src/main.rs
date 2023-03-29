use std::ops::Div;

use functions::standard_gaussian_cdf;
use itertools_num::linspace;
use statrs::{consts, function};

mod functions;
mod plot;
use plot::{Color, DisplayMode, Emphasis, Labels};

fn main() {
    // Global Toggle Variables
    let transparent = true;
    let display_mode = DisplayMode::Dark;
    let show = true;

    // ------------------ Plotting Plot 6 ------------------ //
    // Plot RMM portfolio value for multiple taus
    let plot_name = "\\text{RMM Portfolio Value}".to_string();
    let strike = 3_f64;
    let sigma = 0.5_f64;
    let taus: Vec<f64> = vec![2.0_f64, 1.5_f64, 1.0_f64, 0.5_f64, 0_f64];
    let p_0 = 0.0_f64;
    let p_1 = 10.0_f64;
    let n = 1000;
    let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
    let mut x: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<Vec<f64>> = Vec::new();
    for tau in &taus {
        x.push(prices.clone());
        let temp1 = prices.iter().zip(functions::standard_gaussian_cdf(functions::d_one(prices.clone(), strike, sigma, *tau).iter().map(|d1| -d1).collect()).iter()).map(|(&x, &y)| x * y).collect::<Vec<f64>>();
        let temp2=  functions::standard_gaussian_cdf(functions::d_two(prices.clone(), strike, sigma, *tau));
        y.push(temp1.iter().zip(temp2.iter()).map(|(&x, &y)| x + strike * y).collect()); 
        // y.push(functions::d_one(prices.clone(), strike, sigma, *tau));
    };
    let mut x_single = Vec::new();
    for _i in 0..prices.len() {
        x_single.push(strike);
    }
    let y_single = linspace(0_f64, 5_f64, n);
    x.push(x_single);
    y.push(y_single.collect());
    let x_bounds = vec![0_f64, 10_f64];
    let y_bounds = vec![0_f64, 5_f64];
    let single_color = false;
    let colors = vec![
        (Color::Green, 0, Emphasis::Light, single_color),
        (Color::Green, 1, Emphasis::Light, single_color),
        (Color::Green, 2, Emphasis::Light, single_color),
        (Color::Green, 3, Emphasis::Light, single_color),
        (Color::Green, plot::MAIN_COLOR_SLOT, Emphasis::Heavy, single_color),
        (Color::Grey, plot::MAIN_COLOR_SLOT, Emphasis::Dashed, single_color),
    ];
    let legend_names = vec![
        "\\tau=2.0".to_string(),
        "\\tau=1.5".to_string(),
        "\\tau=1.0".to_string(),
        "\\tau=0.5".to_string(),
        "\\tau=0.0".to_string(),
        "\\text{Strike}".to_string(),
    ];
    let labels = Labels {
        x_label: "S".to_string(),
        y_label: "V(S)".to_string(),
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
