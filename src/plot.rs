use plotly::{
    common::{Font, Line, Mode, Title},
    layout::{Axis, Legend},
};
use plotly::{layout::Margin, Plot, Scatter};
use serde::ser::Serialize;

#[derive(Copy, Clone)]
pub enum DisplayMode {
    Light,
    Dark,
}
// TODO: Use DisplayMode for light mode/dark mode. For light mode, we will also want to consider decreasing from the top when using a list of colors since that will be darker
pub enum Color {
    Green,
    Blue,
    Purple,
    Grey,
    Black,
    White,
}

pub enum Emphasis {
    Light,
    Heavy,
}

pub const MAIN_SLOT: usize = 5;

pub const PRIMITIVE_GREEN: &str = "2BBA58"; // slot 5 of PRIMITIVE_GREENS
pub const PRIMITIVE_GREENS: [&str; 10] = [
    "E1FDEA", "BCF2CD", "95E8AF", "6CDD90", "45D471", "2BBA58", "1F9143", "136730", "063F1A",
    "001703",
];

pub const PRIMITIVE_BLUE: &str = "5E84D7"; // slot 5 of PRIMITIVE_BLUES
pub const PRIMITIVE_BLUES: [&str; 10] = [
    "C2E8FF", "AED4FF", "9AC0FF", "86ACFF", "7298EB", "5E84D7", "4A70C3", "365CAF", "22489B",
    "002073",
];

pub const PRIMITIVE_PURPLE: &str = "9A90F2"; // slot 5 of PRIMITIVE_PURPLES
pub const PRIMITIVE_PURPLES: [&str; 10] = [
    "FEF4FF", "EAE0FF", "D6CCFF", "C2B8FF", "AEA4FF", "9A90F2", "867CDE", "7268CA", "5E54B6",
    "362C8E",
];

pub const PRIMITIVE_GREY: &str = "55575E"; // slot 5 of PRIMITIVE_GREYS
pub const PRIMITIVE_GREYS: [&str; 10] = [
    "F1F1FC", "D6D8DF", "BBBEC3", "A2A4AA", "878A91", "6D7077", "55575E", "3D3E44", "23252B",
    "0B0B15",
];

pub const PRIMITIVE_BLACK: &str = "rgba(21,23,24,255)"; //"151718";
pub const PRIMITIVE_WHITE: &str = "FFFFFF";

pub fn transparent_plot<T: Serialize + Clone + 'static>(
    curves: (Vec<Vec<T>>, Vec<Vec<T>>),
    bounds: (Vec<f64>, Vec<f64>),
    plot_name: String,
    legend_names: Vec<String>,
    colors: Vec<(Color, usize, Emphasis)>,
    (transparent, display_mode, show): (bool, DisplayMode, bool),
) {
    let mut plot = Plot::new();
    for i in 0..curves.0.len() {
        let line = match &colors[i].0 {
            Color::Green => Line::new().color(PRIMITIVE_GREENS[colors[i].1]),
            Color::Blue => Line::new().color(PRIMITIVE_BLUES[colors[i].1]),
            Color::Purple => Line::new().color(PRIMITIVE_PURPLES[colors[i].1]),
            Color::Grey => Line::new().color(PRIMITIVE_GREYS[colors[i].1]),
            Color::Black => Line::new().color(PRIMITIVE_BLACK),
            Color::White => Line::new().color(PRIMITIVE_WHITE),
        };
        let line = match &colors[i].2 {
            Emphasis::Light => line.width(2.0),
            Emphasis::Heavy => line.width(4.0),
        };
        let trace = Scatter::new(curves.0[i].clone(), curves.1[i].clone())
            .mode(Mode::Lines)
            .line(line)
            .name(&legend_names[i]);
        plot.add_trace(trace);
    }

    let x_axis = Axis::new()
        .title(Title::new("$x$").font(Font::new().size(24)))
        .show_grid(true)
        .grid_color(PRIMITIVE_GREY)
        .zero_line(false)
        .color(PRIMITIVE_WHITE)
        .line_color(PRIMITIVE_WHITE)
        .tick_prefix(r"$")
        .tick_suffix(r"$")
        .tick_font(Font::new().size(24))
        .auto_margin(false)
        .range(bounds.0)
        .ticks(plotly::layout::TicksDirection::Outside);

    let y_axis = Axis::new()
        .title(Title::new("$y$").font(Font::new().size(24)))
        .show_grid(true)
        .grid_color(PRIMITIVE_GREY)
        .zero_line(false)
        .show_line(true)
        .tick_prefix(r"$")
        .tick_suffix(r"$")
        .tick_font(Font::new().size(24))
        .auto_margin(false)
        .range(bounds.1)
        .ticks(plotly::layout::TicksDirection::Outside);

    let (x_axis, y_axis) = match display_mode {
        DisplayMode::Light => (
            x_axis.color(PRIMITIVE_BLACK).line_color(PRIMITIVE_BLACK),
            y_axis.color(PRIMITIVE_BLACK).line_color(PRIMITIVE_BLACK),
        ),
        DisplayMode::Dark => (
            x_axis.color(PRIMITIVE_WHITE).line_color(PRIMITIVE_WHITE),
            y_axis.color(PRIMITIVE_WHITE).line_color(PRIMITIVE_WHITE),
        ),
    };

    let layout = plotly::Layout::new()
        .title(plotly::common::Title::new(plot_name.as_str()))
        .x_axis(x_axis)
        .y_axis(y_axis)
        .width(1600)
        .height(900)
        .show_legend(true)
        .margin(Margin::new().bottom(100).left(100).top(100).right(100));
    let layout = match transparent {
        true => layout
            .plot_background_color("rgba(0,0,0,0)")
            .paper_background_color("rgba(0,0,0,0)"),
        false => match display_mode {
            DisplayMode::Dark => layout
                .plot_background_color(PRIMITIVE_BLACK)
                .paper_background_color(PRIMITIVE_BLACK),
            DisplayMode::Light => layout
                .plot_background_color(PRIMITIVE_WHITE)
                .paper_background_color(PRIMITIVE_WHITE),
        },
    };
    let layout = match display_mode {
        DisplayMode::Dark => layout
            .legend(
                Legend::new()
                    .font(Font::new().color(PRIMITIVE_WHITE).size(24))
                    .x(0.75)
                    .y(0.75),
            )
            .font(Font::new().color(PRIMITIVE_WHITE)),
        DisplayMode::Light => layout
            .legend(
                Legend::new()
                    .font(Font::new().color(PRIMITIVE_BLACK).size(24))
                    .x(0.75)
                    .y(0.75),
            )
            .font(Font::new().color(PRIMITIVE_BLACK)),
    };
    plot.set_layout(layout);
    plot.write_html(plot_name.as_str().to_owned() + ".html");
    if show {
        plot.show();
    }
}
