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
    let display_mode = DisplayMode::Light;
    let show = true;

    // ------------------ Plotting Plot 7 ------------------ //
    let plot_name = "\\text{Leverage Zones}".to_string();
    let (mut x1, mut y1, mut x2, mut y2) = (vec![], vec![], vec![], vec![]);
    let x_bounds = vec![0.0, 10.0];
    let y_bounds = vec![0.0, 10.0];
    let t_start = 0.0;
    let t_end = 1.0;
    let number_of_points = 1000;
    let t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>(); // Parameter for curves

    // y=x line and above (to y=10) //
    x1.push(t.iter().map(|t| 10.0 * t).collect::<Vec<f64>>());
    y1.push(t.iter().map(|t| 10.0 * t).collect::<Vec<f64>>());
    // y=10 line
    x2.push(t.iter().map(|t| 10.0 * t).collect::<Vec<f64>>());
    y2.push(t.iter().map(|_| 10.0).collect::<Vec<f64>>());

    // y=x line and below (to y=0) //
    x1.push(t.iter().map(|t| 10.0 * t).collect::<Vec<f64>>());
    y1.push(t.iter().map(|t| 10.0 * t).collect::<Vec<f64>>());
    // y=0 line
    x2.push(t.iter().map(|t| 10.0 * t).collect::<Vec<f64>>());
    y2.push(t.iter().map(|_| 0.0).collect::<Vec<f64>>());

    // Get the plot
    let single_color = false;
    let colors = vec![
        (
            Color::Purple,
            plot::MAIN_COLOR_SLOT,
            Emphasis::Light,
            single_color,
        ),
        (
            Color::Blue,
            plot::MAIN_COLOR_SLOT,
            Emphasis::Light,
            single_color,
        ),
    ];

    // TODO: Add legend positioning here
    let legend_names = vec![
        "\\text{Over Levered}".to_string(),
        "\\text{Under Levered}".to_string(),
    ];
    let labels = Labels {
        x_label: "S".to_string(),
        y_label: "V(S)".to_string(),
    };

    plot::fill_between_curves(
        ((x1, y1), (x2, y2)),
        (x_bounds, y_bounds),
        plot_name,
        Some(legend_names),
        colors,
        (transparent, display_mode, show),
        labels,
    );
}
