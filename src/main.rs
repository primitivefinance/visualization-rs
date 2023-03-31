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

    // ------------------ Plotting Plot 0 ------------------ //
    // Plot of different types of approximations to the Gaussian PDF
    let plot_name = "".to_string();
    let (mut x, mut y) = (vec![], vec![]);
    let x_bounds = vec![-5.0, 5.0];
    let y_bounds = vec![-0.5, 1.5];
    let number_of_points = 1000;
    let x_input = linspace(x_bounds[0], x_bounds[1], number_of_points).collect::<Vec<f64>>();
    x.push(x_input.clone());
    y.push(x_input.iter().map(|x| 1.0 - x * x).collect::<Vec<f64>>());
    x.push(x_input.clone());
    y.push(
        x_input
            .iter()
            .map(|x| 1.0 / (1.0 + x * x))
            .collect::<Vec<f64>>(),
    );
    x.push(x_input.clone());
    let y_temp = functions::standard_gaussian_pdf(
        x_input
            .iter()
            .map(|x| x * 2.0_f64.sqrt())
            .collect::<Vec<f64>>(),
    );
    y.push(
        y_temp
            .iter()
            .map(|y| consts::SQRT_2PI * y)
            .collect::<Vec<f64>>(),
    );
    let single_color = false;
    let colors = vec![
        (Color::Purple, plot::MAIN_COLOR_SLOT, Emphasis::Light, single_color),
        (Color::Blue, plot::MAIN_COLOR_SLOT, Emphasis::Light, single_color),
        (Color::Green, plot::MAIN_COLOR_SLOT, Emphasis::Heavy, single_color),
    ];

    // TODO: Add legend positioning here
    let legend_names = vec![
        "1-x^2".to_string(),
        "(1-x^2)^{-1}".to_string(),
        "\\exp\\left(-x^2\\right)".to_string(),
    ];
    let labels = Labels {
        x_label: "x".to_string(),
        y_label: "f(x)".to_string(),
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
