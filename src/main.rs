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
    plot::transparent_plot(x, y, x_bounds, y_bounds, name, false, true);

    // let name = "Polynomial Approximations".to_string();
    // let x_0 = -5.0_f64;
    // let x_1 = 5.0_f64;
    // let n = 1000;
    // let x_input = linspace(x_0, x_1, n).collect::<Vec<f64>>();
    // let (x0, y0) = (x_input.clone(), x_input.iter().map(|x| 1.0).collect::<Vec<f64>>());
    // let (x1, y1) = (x_input.clone(), x_input.iter().map(|x| 1.0-x*x).collect::<Vec<f64>>());
    // let (x2, y2) = (x_input.clone(), x_input.iter().map(|x| 1.0-x*x+x*x*x*x/2.0).collect::<Vec<f64>>());
    // let (x3, y3) = (x_input.clone(), x_input.iter().map(|x| 1.0-x*x+x*x*x*x/2.0 - x*x*x*x*x*x/6.0).collect::<Vec<f64>>());
    // let (x4, y4) = (x_input.clone(), x_input.iter().map(|x| 1.0-x*x+x*x*x*x/2.0 - x*x*x*x*x*x/6.0 + x*x*x*x*x*x*x*x/24.0).collect::<Vec<f64>>());
    // let (x5, y5) = (x_input.clone(), functions::standard_gaussian_pdf(x_input.clone().iter().map(|x| x*2.0_f64.sqrt()).collect::<Vec<f64>>()));
    // let y5 = y5.iter().map(|y| consts::SQRT_2PI * y).collect::<Vec<f64>>();
    // let x = vec![x0, x1, x2, x3, x4, x5];
    // let y = vec![y0, y1, y2, y3, y4, y5];
    // let names = vec!["Degree 0".to_string(), "Degree 2".to_string(), "Degree 4".to_string(), "Degree 6".to_string(), "Degree 8".to_string()];
    // let x_bounds = vec![x_0, x_1];
    // let y_bounds = vec![-1.0, 1.5];
    // plot::transparent_plot_again(x, y, x_bounds, y_bounds, name, names, false, true);

    // Plot RMM trading curve from a list of prices
    // let name = "RMM Trading Curve".to_string();
    // let K = 10_f64;
    // let sigma = 0.6_f64;
    // let tau = 3.5_f64;
    // let p_0 = 0.0_f64;
    // let p_1 = 100.0_f64;
    // let n = 100;
    // let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
    // let (x,y) = functions::rmm_trading_curve(prices, K, sigma, tau);
    // plot::trading_curve_plot(x, y, name, false,true);

    // Other examples
    // plot::test_plot(false);
    // plot::line_and_scatter_plots(false);
}
