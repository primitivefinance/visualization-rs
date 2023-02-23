use std::ops::Div;

use itertools_num::linspace;
use statrs::consts;

mod functions;
mod plot;
use plot::{Color, Emphasis};

fn main() {
    // ------------------ Plotting Plot 1 ------------------ //
    // Plot of different types of approximations to the Gaussian PDF
    let plot_name = "Comparing Types of Approximation".to_string();
    let (mut x, mut y) = (vec![], vec![]);
    let x_bounds = vec![-5.0, 5.0];
    let y_bounds = vec![-0.5, 1.5];
    let number_of_points = 1000;
    let x_input = linspace(x_bounds[0], x_bounds[1], number_of_points).collect::<Vec<f64>>();
    x.push(x_input.clone());
    y.push(x_input.iter().map(|x| 1.0 - x * x).collect::<Vec<f64>>());
    x.push(        x_input.clone());
    y.push( x_input
            .iter()
            .map(|x| 1.0 / (1.0 + x * x))
            .collect::<Vec<f64>>(),
    );
    x.push(x_input.clone());
    let y_temp =functions::standard_gaussian_pdf(
            x_input
                .iter()
                .map(|x| x * 2.0_f64.sqrt())
                .collect::<Vec<f64>>(),
    );
    y.push(y_temp
        .iter()
        .map(|y| consts::SQRT_2PI * y)
        .collect::<Vec<f64>>());
    let curves = (x,y);
    let bounds = (x_bounds, y_bounds);
    let colors = vec![
        (Color::Purple, plot::MAIN_SLOT, Emphasis::Light),
        (Color::Blue, plot::MAIN_SLOT, Emphasis::Light),
        (Color::Green, plot::MAIN_SLOT, Emphasis::Heavy),
    ];
    let legend_names = vec![
        "$1-x^2$".to_string(),
        "$(1-x^2)^{-1}$".to_string(),
        "$\\exp\\left(-x^2\\right)$".to_string(),
    ];
    plot::transparent_plot(
        curves,
        bounds,
        plot_name,
        legend_names,
        colors,
        false,
        true,
    );

    // ------------------ Plotting Plot 2 ------------------ //
    // Plots of polynomial approximations to the Gaussian PDF
    let plot_name = "Polynomial Approximations".to_string();
    let mut x = vec![];
    let mut y = vec![];
    let x_bounds = vec![-5.0, 5.0];
    let number_of_points = 1000;
    let top_degree = 8;
    let x_input = linspace(x_bounds[0], x_bounds[1], number_of_points).collect::<Vec<f64>>();
    let coeff_range: Vec<i32> = (0..top_degree + 1).collect();
    let coeffs = coeff_range
        .iter()
        .map(|n| match n % 2 {
            0 => (((-1.0) as f64).powi(n.div(2))) / (functions::factorial(n.div(2) as u32) as f64),
            _ => 0.0,
        })
        .collect::<Vec<f64>>();
    println!("{:#?}", coeffs);
    for i in (0..top_degree + 1).step_by(2) {
        x.push(x_input.clone());
        y.push(functions::polynomial_approx(
            x_input.clone(),
            coeffs[0..1 + i as usize].to_vec(),
        ));
    }
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
    let curves = (x, y);
    let y_bounds = vec![-1.0, 1.5];
    let bounds = (x_bounds, y_bounds);
    let colors = vec![
        (Color::Purple, 0, Emphasis::Light),
        (Color::Purple, 1, Emphasis::Light),
        (Color::Purple, 2, Emphasis::Light),
        (Color::Purple, 3, Emphasis::Light),
        (Color::Purple, 4, Emphasis::Light),
        (Color::Green, plot::MAIN_SLOT, Emphasis::Heavy),
    ];
    let legend_names = vec![
        format!("${} {} {}", "\\text{", "Degree 0", "}$"),
        format!("${} {} {}", "\\text{", "Degree 2", "}$"),
        format!("${} {} {}", "\\text{", "Degree 4", "}$"),
        format!("${} {} {}", "\\text{", "Degree 6", "}$"),
        format!("${} {} {}", "\\text{", "Degree 8", "}$"),
        "$\\exp\\left(-x^2\\right)$".to_string(),
    ];
    plot::transparent_plot(curves, bounds, plot_name, legend_names, colors, false, true);

    // ------------------ Plotting Plot 3 ------------------ //
    // // Plot RMM trading curve for multiple taus from a list of prices
    // let plot_name = "RMM Trading Curve".to_string();
    // let strike = 3_f64;
    // let sigma = 0.5_f64;
    // let taus = vec![2.0_f64, 1.5_f64, 1.0_f64, 0.5_f64, 0.0_f64];
    // let p_0 = 0.0_f64;
    // let p_1 = 100.0_f64;
    // let n = 1000;
    // let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
    // let mut x: Vec<Vec<f64>> = Vec::new();
    // let mut y: Vec<Vec<f64>> = Vec::new();
    // for tau in taus.iter() {
    //     let (x_tau, y_tau) = functions::rmm_trading_curve(prices.clone(), strike, sigma, *tau);
    //     x.push(x_tau);
    //     y.push(y_tau);
    // }
    // let x_bounds = vec![0_f64, 1_f64];
    // let y_bounds = vec![0_f64, strike];
    // let colors = vec![
    //     (Color::Green, 0, Emphasis::Light),
    //     (Color::Green, 1, Emphasis::Light),
    //     (Color::Green, 2, Emphasis::Light),
    //     (Color::Green, 3, Emphasis::Light),
    //     (Color::Green, plot::MAIN_SLOT, Emphasis::Heavy),
    // ];
    // let legend_names = vec![
    //     "$\\tau=2.0$".to_string(),
    //     "$\\tau=1.5$".to_string(),
    //     "$\\tau=1.0$".to_string(),
    //     "$\\tau=0.5$".to_string(),
    //     "$\\tau=0.0$".to_string(),
    // ];
    // plot::transparent_plot(
    //     x,
    //     y,
    //     x_bounds,
    //     y_bounds,
    //     plot_name,
    //     legend_names,
    //     colors,
    //     false,
    //     true,
    // );
}
