use itertools_num::linspace;

mod plot;
mod functions;

fn main() {
    let x_0 = -3.0_f64;
    let x_1 = 3.0_f64;
    let n = 1000;
    let t = linspace(x_0, x_1, n).collect::<Vec<f64>>();
    let x = t.iter().map(|x| (-x.powf(2.0)).exp()).collect();
    plot::transparent_plot(t, x);
    plot::test_plot(false);
    plot::line_and_scatter_plots(false);
}
