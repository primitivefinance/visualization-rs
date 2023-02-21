use itertools_num::linspace;
use statrs::consts;

mod functions;
mod plot;
use plot::{Color, Emphasis};

fn main() {
    // Plot of different types of approximations to the Gaussian PDF
    let plot_name = "Comparing Types of Approximation".to_string();
    let x_0 = -5.0_f64;
    let x_1 = 5.0_f64;
    let n = 1000;
    let x_input = linspace(x_0, x_1, n).collect::<Vec<f64>>();
    let (x1, y1) = (
        x_input.clone(),
        x_input.iter().map(|x| 1.0 - x * x).collect::<Vec<f64>>(),
    );
    let (x2, y2) = (
        x_input.clone(),
        x_input
            .iter()
            .map(|x| 1.0 / (1.0 + x * x))
            .collect::<Vec<f64>>(),
    );
    let (x3, y3) = (
        x_input.clone(),
        functions::standard_gaussian_pdf(
            x_input
                .iter()
                .map(|x| x * 2.0_f64.sqrt())
                .collect::<Vec<f64>>(),
        ),
    );
    let y3 = y3
        .iter()
        .map(|y| consts::SQRT_2PI * y)
        .collect::<Vec<f64>>();
    let x = vec![x1, x2, x3];
    let y = vec![y1, y2, y3];
    let x_bounds = vec![x_0, x_1];
    let y_bounds = vec![-1.0, 1.5];
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
        x,
        y,
        x_bounds,
        y_bounds,
        plot_name,
        legend_names,
        colors,
        false,
        true,
    );

    // Plots of polynomial approximations to the Gaussian PDF
    let plot_name = "Polynomial Approximations".to_string();
    let x_0 = -5.0_f64;
    let x_1 = 5.0_f64;
    let n = 1000;
    let x_input = linspace(x_0, x_1, n).collect::<Vec<f64>>();
    let (x0, y0) = (
        x_input.clone(),
        x_input.iter().map(|_x| 1.0).collect::<Vec<f64>>(),
    );
    let (x1, y1) = (
        x_input.clone(),
        x_input.iter().map(|x| 1.0 - x * x).collect::<Vec<f64>>(),
    );
    let (x2, y2) = (
        x_input.clone(),
        x_input
            .iter()
            .map(|x| 1.0 - x * x + x * x * x * x / 2.0)
            .collect::<Vec<f64>>(),
    );
    let (x3, y3) = (
        x_input.clone(),
        x_input
            .iter()
            .map(|x| 1.0 - x * x + x * x * x * x / 2.0 - x * x * x * x * x * x / 6.0)
            .collect::<Vec<f64>>(),
    );
    let (x4, y4) = (
        x_input.clone(),
        x_input
            .iter()
            .map(|x| {
                1.0 - x * x + x * x * x * x / 2.0 - x * x * x * x * x * x / 6.0
                    + x * x * x * x * x * x * x * x / 24.0
            })
            .collect::<Vec<f64>>(),
    );
    let (x5, y5) = (
        x_input.clone(),
        functions::standard_gaussian_pdf(
            x_input
                .iter()
                .map(|x| x * 2.0_f64.sqrt())
                .collect::<Vec<f64>>(),
        ),
    );
    let y5 = y5
        .iter()
        .map(|y| consts::SQRT_2PI * y)
        .collect::<Vec<f64>>();
    let x = vec![x0, x1, x2, x3, x4, x5];
    let y = vec![y0, y1, y2, y3, y4, y5];
    let x_bounds = vec![x_0, x_1];
    let y_bounds = vec![-1.0, 1.5];
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
    plot::transparent_plot(
        x,
        y,
        x_bounds,
        y_bounds,
        plot_name,
        legend_names,
        colors,
        false,
        true,
    );

    // Plot RMM trading curve for multiple taus from a list of prices
    let plot_name = "RMM Trading Curve".to_string();
    let strike = 3_f64;
    let sigma = 0.5_f64;
    let taus = vec![2.0_f64, 1.5_f64, 1.0_f64, 0.5_f64, 0.0_f64];
    let p_0 = 0.0_f64;
    let p_1 = 100.0_f64;
    let n = 1000;
    let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
    let mut x: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<Vec<f64>> = Vec::new();
    for tau in taus.iter() {
        let (x_tau, y_tau) = functions::rmm_trading_curve(prices.clone(), strike, sigma, *tau);
        x.push(x_tau);
        y.push(y_tau);
    }
    let x_bounds = vec![0_f64, 1_f64];
    let y_bounds = vec![0_f64, strike];
    let colors = vec![
        (Color::Green, 0, Emphasis::Light),
        (Color::Green, 1, Emphasis::Light),
        (Color::Green, 2, Emphasis::Light),
        (Color::Green, 3, Emphasis::Light),
        (Color::Green, plot::MAIN_SLOT, Emphasis::Heavy),
    ];
    let legend_names = vec![
        "$\\tau=2.0$".to_string(),
        "$\\tau=1.5$".to_string(),
        "$\\tau=1.0$".to_string(),
        "$\\tau=0.5$".to_string(),
        "$\\tau=0.0$".to_string(),
    ];
    plot::transparent_plot(
        x,
        y,
        x_bounds,
        y_bounds,
        plot_name,
        legend_names,
        colors,
        false,
        true,
    );
}
