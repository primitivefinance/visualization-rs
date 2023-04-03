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
    let plot_name = "\\text{Convex Portfolio Value Function}".to_string();
    let (mut x, mut y, mut x1, mut y1, mut x2, mut y2) = (vec![], vec![], vec![], vec![], vec![], vec![]);
    let x_bounds = vec![0.0, 5.0];
    let y_bounds = vec![0.0, 5.0];
    let t_start = 0.0;
    let t_end = 1.0;
    let number_of_points = 1000;
    let t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>(); // Parameter for curves
    // BUILD CURVES //
    // y=x^2 curve //
    x.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
    y.push(t.iter().map(|t| 25.0*t*t).collect::<Vec<f64>>());

    // BUILD REGIONS //
    // y=x line and above (to y=5) //
    x1.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
    y1.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
    // y=10 line
    x2.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
    y2.push(t.iter().map(|_| 5.0).collect::<Vec<f64>>());

    // y=x line and below (to y=0) //
    x1.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
    y1.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
    // y=0 line
    x2.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
    y2.push(t.iter().map(|_| 0.0).collect::<Vec<f64>>());

    // Get the plot
    let single_color = false;
    let curve_colors = vec![
        (
            Color::Green,
            plot::MAIN_COLOR_SLOT,
            Emphasis::Heavy,
            single_color,
        ),
    ];
    let curve_legend_names = vec![
        "V(S)=S^2".to_string(),
    ];

    let region_colors = vec![
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
    let region_legend_names = vec![
        "\\text{Over Levered}".to_string(),
        "\\text{Under Levered}".to_string(),
    ];
    let labels = Labels {
        x_label: "S".to_string(),
        y_label: "V(S)".to_string(),
    };

    plot::transparent_plot(
        Some((x, y, curve_colors, Some(curve_legend_names))),
        Some(((x1, y1), (x2, y2), region_colors, None)),
        (x_bounds, y_bounds),
        plot_name,
        (transparent, display_mode, show),
        labels,
    );
}
