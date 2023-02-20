use itertools_num::linspace;
use statrs::function;
use statrs::consts;

mod functions;
mod plot;

fn main() {
    let name = "Comparing Types of Approximation".to_string();
    let x_0 = -5.0_f64;
    let x_1 = 5.0_f64;
    let n = 1000;
    let x_input = linspace(x_0, x_1, n).collect::<Vec<f64>>();
    let (x1, y1) = (x_input.clone(), functions::standard_gaussian_pdf(x_input.clone().iter().map(|x| x*2.0_f64.sqrt()).collect::<Vec<f64>>()));
    let y1 = y1.iter().map(|y| consts::SQRT_2PI * y).collect::<Vec<f64>>();
    let (x2, y2) = (x_input.clone(), x_input.iter().map(|x| 1.0 / (1.0+x*x)).collect::<Vec<f64>>());
    let (x3, y3) = (x_input.clone(), x_input.iter().map(|x| 1.0-x*x).collect::<Vec<f64>>());
    let x = vec![x1, x2, x3];
    let y = vec![y1, y2, y3];
    let x_bounds = vec![x_0, x_1];
    let y_bounds = vec![-1.0, 1.5];
    plot::transparent_plot(x, y, x_bounds, y_bounds, name, true, true);

    let name = "Polynomial Approximations".to_string();
    let x_0 = -5.0_f64;
    let x_1 = 5.0_f64;
    let n = 1000;
    let x_input = linspace(x_0, x_1, n).collect::<Vec<f64>>();
    let (x0, y0) = (x_input.clone(), x_input.iter().map(|x| 1.0).collect::<Vec<f64>>());
    let (x1, y1) = (x_input.clone(), x_input.iter().map(|x| 1.0-x*x).collect::<Vec<f64>>());
    let (x2, y2) = (x_input.clone(), x_input.iter().map(|x| 1.0-x*x+x*x*x*x/2.0).collect::<Vec<f64>>());
    let (x3, y3) = (x_input.clone(), x_input.iter().map(|x| 1.0-x*x+x*x*x*x/2.0 - x*x*x*x*x*x/6.0).collect::<Vec<f64>>());
    let (x4, y4) = (x_input.clone(), x_input.iter().map(|x| 1.0-x*x+x*x*x*x/2.0 - x*x*x*x*x*x/6.0 + x*x*x*x*x*x*x*x/24.0).collect::<Vec<f64>>());
    let (x5, y5) = (x_input.clone(), functions::standard_gaussian_pdf(x_input.clone().iter().map(|x| x*2.0_f64.sqrt()).collect::<Vec<f64>>()));
    let y5 = y5.iter().map(|y| consts::SQRT_2PI * y).collect::<Vec<f64>>();
    let x = vec![x0, x1, x2, x3, x4, x5];
    let y = vec![y0, y1, y2, y3, y4, y5];
    let names = vec![format!("${} {} {}", "\\text{", "Degree 0", "}$").to_string(), format!("${} {} {}", "\\text{", "Degree 2", "}$").to_string(),
    format!("${} {} {}", "\\text{", "Degree 4", "}$").to_string(), format!("${} {} {}", "\\text{", "Degree 6", "}$").to_string(), format!("${} {} {}", "\\text{", "Degree 8", "}$").to_string(), "$\\exp\\left(-x^2\\right)\\quad .$".to_string()];
    let x_bounds = vec![x_0, x_1];
    let y_bounds = vec![-1.0, 1.5];
    plot::transparent_plot_again(x, y, x_bounds, y_bounds, name, names, true, true);

    // Plot RMM trading curve from a list of prices
    let name = "RMM Trading Curve".to_string();
    let K = 3_f64;
    let sigma = 0.5_f64;
    let taus = vec![2.0_f64, 1.5_f64, 1.0_f64, 0.5_f64, 0.0_f64];
    let p_0 = 0.0_f64;
    let p_1 = 100.0_f64;
    let n = 1000;
    let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
    let mut x: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<Vec<f64>> = Vec::new();
    for tau in taus.iter() {
        let (x_tau, y_tau) = functions::rmm_trading_curve(prices.clone(), K, sigma, *tau);
        x.push(x_tau);
        y.push(y_tau);
    }
    let x_bounds = vec![0_f64,1_f64];
    let y_bounds = vec![0_f64,K];
    let names = vec!["$\\tau=2.0\\qquad .$".to_string(), "$\\tau=1.5$".to_string(), "$\\tau=1.0$".to_string(), "$\\tau=0.5$".to_string(), "$\\tau=0.0$".to_string()];
    plot::transparent_plot_again(x, y, x_bounds, y_bounds, name, names, true, true);
    // plot::trading_curve_plot(x, y, name, false,true);

    // Other examples
    // plot::test_plot(false);
    // plot::line_and_scatter_plots(false);
}
