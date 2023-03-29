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

    // Plot 1: Portfolio Value Function
    let s_values: Vec<f64> = (0..=100).map(|x| x as f64).collect();
    let v_values: Vec<f64> = s_values.iter().map(|&s| s.sqrt()).collect();
    let curves = (vec![s_values], vec![v_values]);
    

    let x_bounds = vec![0.0, 100.0];
    let y_bounds = vec![0.0, 10.0]; // Update the y-axis upper bound to match the maximum value of V(S) = √S
    let bounds = (x_bounds, y_bounds);
    
    let plot_name = "Portfolio Value Function: V(S) = √S".to_string();
    
    let legend_names = Some(vec!["V(S) = √S".to_string()]);
    let colors = vec![(Color::Green, plot::MAIN_COLOR_SLOT, Emphasis::Heavy, true)];

    let labels = Labels {
        x_label: "$S$".to_string(),
        y_label: "$V(S)$".to_string(),
    };

    plot::transparent_plot(
        (curves),
        (bounds.0, bounds.1),
        plot_name,
        legend_names,
        colors,
        (transparent, display_mode, show),
        labels,
    );

}