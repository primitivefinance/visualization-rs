use itertools_num::linspace;

mod functions;
mod plot;

fn main() {
    let name = "Comparing Types of Approximation".to_string();
    let x_0 = -10.0_f64;
    let x_1 = 10.0_f64;
    let n = 100;
    let x = linspace(x_0, x_1, n).collect::<Vec<f64>>();
    let (x, y) = functions::standard_gaussian_pdf(x);
    plot::transparent_plot(x, y, name, false, true);

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
    // plot::trading_curve_plot(x, y, name, true, false);

    // Other examples
    // plot::test_plot(false);
    // plot::line_and_scatter_plots(false);
}
