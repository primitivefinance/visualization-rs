use itertools_num::linspace;

mod plot;
mod functions;

fn main() {

    // Plot RMM trading curve from a list of prices
    let name = "rmm_trading_curve".to_string();
    let K = 10_f64;
    let sigma = 0.6_f64;
    let tau = 3.5_f64;
    let p_0 = 0.0_f64;
    let p_1 = 100.0_f64;
    let n = 100;
    let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
    let (x,y) = functions::rmm_trading_curve(prices, K, sigma, tau);
    plot::transparent_plot(x, y, name, false, true);

    // Other examples
    // plot::test_plot(false);
    // plot::line_and_scatter_plots(false);
}
