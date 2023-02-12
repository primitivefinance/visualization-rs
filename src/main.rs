mod plot;

fn main() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![2.0, 3.0, 2.0];
    plot::transparent_plot(x, y);

    plot::test_plot(true);
    plot::line_and_scatter_plots(false);
}
