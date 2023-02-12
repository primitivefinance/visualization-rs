use plotly::{ImageFormat, Plot, Scatter};
use rand::Rng;
use serde::ser::Serialize;

use itertools_num::linspace;
use plotly::color::{NamedColor, Rgb, Rgba};
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use rand_distr::{Distribution, Normal, Uniform};

pub fn transparent_plot<T: Serialize + Clone + 'static>(x: Vec<T>, y: Vec<T>) {
    let mut plot = Plot::new();
    let trace = Scatter::new(x, y);
    plot.add_trace(trace);

    let layout = plotly::Layout::new()
        .title(plotly::common::Title::new("Test Plot"))
        .width(800)
        .height(600)
        .plot_background_color("rgba(0,0,0,0)")
        .paper_background_color("rgba(0,0,0,0)");
    plot.set_layout(layout);
    plot.write_image("out.png", ImageFormat::PNG, 800, 600, 1.0);
}

pub fn test_plot(show: bool) {
    let trace1 = Scatter::new(vec![52698, 43117], vec![53, 31])
        .mode(Mode::Markers)
        .name("North America")
        .text_array(vec!["United States", "Canada"])
        .marker(
            Marker::new()
                .color(Rgb::new(164, 194, 244))
                .size(12)
                .line(Line::new().color(NamedColor::White).width(0.5)),
        );
    let trace2 = Scatter::new(
        vec![
            39317, 37236, 35650, 30066, 29570, 27159, 23557, 21046, 18007,
        ],
        vec![33, 20, 13, 19, 27, 19, 49, 44, 38],
    )
    .mode(Mode::Markers)
    .name("Europe")
    .text_array(vec![
        "Germany",
        "Britain",
        "France",
        "Spain",
        "Italy",
        "Czech Rep.",
        "Greece",
        "Poland",
    ])
    .marker(Marker::new().color(Rgb::new(255, 217, 102)).size(12));
    let trace3 = Scatter::new(
        vec![42952, 37037, 33106, 17478, 9813, 5253, 4692, 3899],
        vec![23, 42, 54, 89, 14, 99, 93, 70],
    )
    .mode(Mode::Markers)
    .name("Asia/Pacific")
    .text_array(vec![
        "Australia",
        "Japan",
        "South Korea",
        "Malaysia",
        "China",
        "Indonesia",
        "Philippines",
        "India",
    ])
    .marker(Marker::new().color(Rgb::new(234, 153, 153)).size(12));
    let trace4 = Scatter::new(
        vec![19097, 18601, 15595, 13546, 12026, 7434, 5419],
        vec![43, 47, 56, 80, 86, 93, 80],
    )
    .mode(Mode::Markers)
    .name("Latin America")
    .text_array(vec![
        "Chile",
        "Argentina",
        "Mexico",
        "Venezuela",
        "Venezuela",
        "El Salvador",
        "Bolivia",
    ])
    .marker(Marker::new().color(Rgb::new(142, 124, 195)).size(12));

    let x_axis = Axis::new()
        .title(Title::new("GDP per Capita"))
        .show_grid(false)
        .zero_line(false)
        .color(NamedColor::Red)
        .line_color(NamedColor::Blue);
    let y_axis = Axis::new().title(Title::new("Percent")).show_line(false);

    let layout = Layout::new()
        .title(Title::new("$\\text{Quarter } Q=2023 \\text{ Growth}$"))
        .x_axis(x_axis)
        .y_axis(y_axis);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("colored_and_styled_scatter_plot"))
    );

}

pub fn line_and_scatter_plots(show: bool) {
    let n: usize = 100;
    let mut rng = rand::thread_rng();
    let random_x: Vec<f64> = linspace(0., 1., n).collect();
    let random_y0: Vec<f64> = Normal::new(5., 1.)
        .unwrap()
        .sample_iter(rng.clone())
        .take(n)
        .collect();
    let random_y1: Vec<f64> = Normal::new(0., 1.)
        .unwrap()
        .sample_iter(rng.clone())
        .take(n)
        .collect();
    let random_y2: Vec<f64> = Normal::new(-5., 1.)
        .unwrap()
        .sample_iter(rng.clone())
        .take(n)
        .collect();

    let trace1 = Scatter::new(random_x.clone(), random_y0)
        .mode(Mode::Markers)
        .name("markers");
    let trace2 = Scatter::new(random_x.clone(), random_y1)
        .mode(Mode::LinesMarkers)
        .name("linex+markers");
    let trace3 = Scatter::new(random_x, random_y2)
        .mode(Mode::Lines)
        .name("lines");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("line_and_scatter_plots")));
}