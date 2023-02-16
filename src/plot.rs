use std::fmt::format;

use plotly::{ImageFormat, Plot, Scatter};
use rand::Rng;
use serde::ser::Serialize;

use itertools_num::linspace;
use plotly::color::{NamedColor, Rgb, Rgba};
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title, Pad,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use rand_distr::{Distribution, Normal, Uniform};

pub const PRIMITIVE_GREEN: &str = "rgba(43,186,88,255)";
pub const PRIMITIVE_BLACK: &str = "rgba(21,23,24,255)";
pub const PRIMITIVE_WHITE: &str = "rgba(255,255,255,255)";
pub const PRIMITIVE_GRAY: &str = "rgba(108,112,119,255)";
pub const PRIMITIVE_BLUE: &str = "rgba(94,132,215,255)";
pub const PRIMITIVE_PURPLE: &str = "rgba(154,144,242,255)";

pub fn transparent_plot<T: Serialize + Clone + 'static>(x: Vec<T>, y: Vec<T>,  name: String, transparent: bool, show: bool) {
    let mut plot = Plot::new();
    let trace = Scatter::new(x, y).mode(Mode::Lines)
    .marker(
        Marker::new()
            .color(PRIMITIVE_GREEN)
            .size(12)
            .line(Line::new().color(PRIMITIVE_GREEN).width(3.0)),
    ).name("$\\varphi(x,y;K=10,\\sigma=0.6,\\tau=3.5)$");
    plot.add_trace(trace);

    let x_axis = Axis::new()
        .title(Title::new("$\\text{Reserves } x$").font(Font::new().size(24)))
        .show_grid(true)
        .grid_color(PRIMITIVE_GRAY)
        .zero_line(false)
        .color(PRIMITIVE_WHITE)
        .line_color(PRIMITIVE_WHITE)
        .tick_prefix(r"$")
        .tick_suffix(r"$")
        .tick_font(Font::new().size(24))
        .auto_margin(false);

    let y_axis = Axis::new()
        .title(Title::new("$\\text{Reserves } y$").font(Font::new().size(24)))
        .show_grid(true)
        .grid_color(PRIMITIVE_GRAY)
        .zero_line(false)
        .show_line(true)
        .color(PRIMITIVE_WHITE)
        .line_color(PRIMITIVE_WHITE)
        .tick_prefix(r"$")
        .tick_suffix(r"$")
        .tick_font(Font::new().size(24))
        .auto_margin(false);

        let title_text = format!("$\\text{{RMM Trading Function}} $");
        if transparent{
            let layout = plotly::Layout::new()
            .title(plotly::common::Title::new(name.as_str()))
            .x_axis(x_axis)
            .y_axis(y_axis)
            .width(1600)
            .height(900)
            .plot_background_color("rgba(0,0,0,0)")
            .paper_background_color("rgba(0,0,0,0)")
            .show_legend(true);
            plot.set_layout(layout);
        } else{
            let layout = plotly::Layout::new()
            .title(plotly::common::Title::new(&title_text).font(Font::new().color(PRIMITIVE_WHITE)))
            .x_axis(x_axis)
            .y_axis(y_axis)
            .width(1600)
            .height(900)
            .plot_background_color(PRIMITIVE_BLACK)
            .paper_background_color(PRIMITIVE_BLACK)
            .show_legend(true)
            .legend(Legend::new().font(Font::new().color(PRIMITIVE_WHITE).size(24)).x(0.50).y(0.50).background_color("rgba(0,0,0,0)"));
            plot.set_layout(layout);
        }

    
    plot.write_html(name.as_str().to_owned()+".html");
    if show {
        plot.show();
    }    
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