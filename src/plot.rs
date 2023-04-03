use plotly::{
    common::{Font, Line, Mode, Pad, Title},
    layout::{Axis, Legend, TicksPosition},
};
use plotly::{layout::Margin, Plot, Scatter};
use serde::ser::Serialize;

// testing
use itertools_num::linspace;
use plotly::color::{NamedColor, Rgb, Rgba};
use plotly::common::{ColorScale, ColorScalePalette, DashType, Fill, LineShape, Marker};
use plotly::layout::{BarMode, Layout, TicksDirection};
use rand_distr::{Distribution, Normal, Uniform};

#[derive(Copy, Clone)]
pub enum DisplayMode {
    Light,
    Dark,
}
// TODO: Use DisplayMode for light mode/dark mode. For light mode, we will also want to consider decreasing from the top when using a list of colors since that will be darker
#[derive(Debug)]
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
    Dashed,
}

pub struct Labels {
    pub x_label: String,
    pub y_label: String,
}

pub const MAIN_COLOR_SLOT: usize = 5;

pub const PRIMITIVE_GREENS: [&str; 10] = [
    "E1FDEA", "BCF2CD", "95E8AF", "6CDD90", "45D471", "2BBA58", "1F9143", "136730", "063F1A",
    "001703",
];

pub const PRIMITIVE_BLUES: [&str; 10] = [
    "C2E8FF", "AED4FF", "9AC0FF", "86ACFF", "7298EB", "5E84D7", "4A70C3", "365CAF", "22489B",
    "002073",
];

pub const PRIMITIVE_PURPLES: [&str; 10] = [
    "FEF4FF", "EAE0FF", "D6CCFF", "C2B8FF", "AEA4FF", "9A90F2", "867CDE", "7268CA", "5E54B6",
    "362C8E",
];

pub const PRIMITIVE_GREYS: [&str; 10] = [
    "F1F1FC", "D6D8DF", "BBBEC3", "A2A4AA", "878A91", "6D7077", "55575E", "3D3E44", "23252B",
    "0B0B15",
];

pub const PRIMITIVE_BLACK: &str = "151718";
pub const PRIMITIVE_WHITE: &str = "FFFFFF";

pub fn transparent_plot<T: Serialize + Clone + 'static>(
    curves: (Vec<Vec<T>>, Vec<Vec<T>>),
    bounds: (Vec<f64>, Vec<f64>),
    plot_name: String,
    legend_names: Option<Vec<String>>,
    colors: Vec<(Color, usize, Emphasis, bool)>,
    (transparent, display_mode, show): (bool, DisplayMode, bool),
    labels: Labels,
) {
    let mut plot = Plot::new();
    for i in 0..curves.0.len() {
        let color = match &colors[0].3 {
            true => &colors[0],
            false => &colors[i],
        };
        let line = match color.0 {
            Color::Green => Line::new().color(PRIMITIVE_GREENS[color.1]),
            Color::Blue => Line::new().color(PRIMITIVE_BLUES[color.1]),
            Color::Purple => Line::new().color(PRIMITIVE_PURPLES[color.1]),
            Color::Grey => Line::new().color(PRIMITIVE_GREYS[color.1]),
            Color::Black => Line::new().color(PRIMITIVE_BLACK),
            Color::White => Line::new().color(PRIMITIVE_WHITE),
        };
        let line = match &color.2 {
            Emphasis::Light => line.width(2.0),
            Emphasis::Heavy => line.width(4.0),
            Emphasis::Dashed => line.width(2.0).dash(plotly::common::DashType::Dash),
        };
        let trace = Scatter::new(curves.0[i].clone(), curves.1[i].clone())
            .mode(Mode::Lines)
            .line(line)
            .name(&match &legend_names {
                Some(names) => format!(" {} {} {}", "$\\Large{", names[i].clone(), "}$"),
                None => "".to_string(),
            });
        plot.add_trace(trace);
    }
    let x_label = format!("{} {} {}", "$\\LARGE{", labels.x_label.clone(), "}$");
    let x_axis = Axis::new()
        .title(Title::new(&x_label).font(Font::new().size(24)))
        .show_grid(true)
        .grid_color(PRIMITIVE_GREYS[MAIN_COLOR_SLOT])
        .zero_line(false)
        .color(PRIMITIVE_WHITE)
        .line_color(PRIMITIVE_WHITE)
        .tick_prefix("$\\LARGE{")
        .tick_suffix("}$")
        .tick_font(Font::new().size(24))
        .auto_margin(false)
        .range(bounds.0)
        .ticks(plotly::layout::TicksDirection::Outside);
    let y_label = format!("{} {} {}", "$\\LARGE{", labels.y_label.clone(), "}$");
    let y_axis = Axis::new()
        .title(Title::new(&y_label).font(Font::new().size(48)))
        .show_grid(true)
        .grid_color(PRIMITIVE_GREYS[MAIN_COLOR_SLOT])
        .zero_line(false)
        .show_line(true)
        .tick_prefix("$\\LARGE{")
        .tick_suffix("}$")
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
    let plot_name = format!("{} {} {}", "$\\huge{", plot_name, "}$");
    let layout = plotly::Layout::new()
        .title(plotly::common::Title::new(plot_name.as_str()))
        .x_axis(x_axis)
        .y_axis(y_axis)
        .width(1600)
        .height(900)
        .margin(Margin::new().bottom(100).left(160).top(100).right(100));
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
    let layout = match legend_names {
        Some(_) => match display_mode {
            DisplayMode::Dark => layout
                .show_legend(true)
                .legend(
                    Legend::new()
                        .font(Font::new().color(PRIMITIVE_WHITE).size(24))
                        .x(0.7)
                        .y(0.7),
                )
                .font(Font::new().color(PRIMITIVE_WHITE)),
            DisplayMode::Light => layout
                .legend(
                    Legend::new()
                        .font(Font::new().color(PRIMITIVE_BLACK).size(24))
                        .x(0.7)
                        .y(0.7),
                )
                .font(Font::new().color(PRIMITIVE_BLACK)),
        },
        None => layout.show_legend(false),
    };

    plot.set_layout(layout);
    plot.write_html(plot_name.as_str().to_owned() + ".html");
    if show {
        plot.show();
    }
}

pub fn fill_between_curves<T: Serialize + Clone + 'static>(
    curves: ((Vec<Vec<T>>, Vec<Vec<T>>),(Vec<Vec<T>>, Vec<Vec<T>>)),
    bounds: (Vec<f64>, Vec<f64>),
    plot_name: String,
    legend_names: Option<Vec<String>>,
    colors: Vec<(Color, usize, Emphasis, bool)>,
    (transparent, display_mode, show): (bool, DisplayMode, bool),
    labels: Labels,
) {
    // Initial x vecs
    let x1_coords = curves.0.0[0].clone();
    let x2_coords = curves.1.0[0].clone();
    let x_thing = x2_coords.iter().cloned().rev().collect::<Vec<T>>();
    // Append thing to x1_coords to create a new vector
    let x_combined = x1_coords.into_iter().chain(x_thing.into_iter()).collect::<Vec<T>>();

    // Initial y vecs
    let y1_coords = curves.0.1[0].clone();
    let y2_coords = curves.1.1[0].clone();
    let y_thing = y2_coords.iter().cloned().rev().collect::<Vec<T>>();
    // Append thing to x1_coords to create a new vector
    let y_combined = y1_coords.into_iter().chain(y_thing.into_iter()).collect::<Vec<T>>();

    let trace = Scatter::new(
        x_combined,
        y_combined,
    )
    .fill(Fill::ToNext)
    .fill_color(Rgba::new(0, 176, 246, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Premium")
    .show_legend(true);

    // y=x line
    // let trace_yx = Scatter::new(x1.clone(), x2.clone())
    //     .line(Line::new().color(NamedColor::Black).width(2.0))
    //     .name("No leverage")
    //     .show_legend(true);

    let layout = Layout::new()
        .paper_background_color(Rgb::new(255, 255, 255))
        .plot_background_color(Rgb::new(229, 229, 229))
        .x_axis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .range(vec![0.0, 5.0])
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false),
        )
        .y_axis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false),
        );

    let mut plot = Plot::new();
    plot.set_layout(layout);
    plot.add_trace(trace);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("filled_lines")));
}

// pub fn filled_lines(show: bool) {
//     // Initial vecs
//     // let mut x1 = vec![0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0];
//     let x1 = vec![0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0, 1.0, 0.0];

//     let trace2 = Scatter::new(
//         x1.clone(),
//         vec![
//             10.0, 10.0, 10.0, 10.0, 10.0, 4.0, 3.0, 2.0, 1.0, 0.0,
//         ],
//     )
//     .fill(Fill::ToNext)
//     .fill_color(Rgba::new(0, 176, 246, 0.2))
//     .line(Line::new().color(NamedColor::Transparent))
//     .name("Premium")
//     .show_legend(true);

//     // y=x line
//     // let trace_yx = Scatter::new(x1.clone(), x2.clone())
//     //     .line(Line::new().color(NamedColor::Black).width(2.0))
//     //     .name("No leverage")
//     //     .show_legend(true);

//     let layout = Layout::new()
//         .paper_background_color(Rgb::new(255, 255, 255))
//         .plot_background_color(Rgb::new(229, 229, 229))
//         .x_axis(
//             Axis::new()
//                 .grid_color(Rgb::new(255, 255, 255))
//                 .range(vec![0.0, 5.0])
//                 .show_grid(true)
//                 .show_line(false)
//                 .show_tick_labels(true)
//                 .tick_color(Rgb::new(127, 127, 127))
//                 .ticks(TicksDirection::Outside)
//                 .zero_line(false),
//         )
//         .y_axis(
//             Axis::new()
//                 .grid_color(Rgb::new(255, 255, 255))
//                 .show_grid(true)
//                 .show_line(false)
//                 .show_tick_labels(true)
//                 .tick_color(Rgb::new(127, 127, 127))
//                 .ticks(TicksDirection::Outside)
//                 .zero_line(false),
//         );

//     let mut plot = Plot::new();
//     plot.set_layout(layout);
//     // plot.add_trace(trace1);
//     // plot.add_trace(bottom_trace1);
//     plot.add_trace(trace2);
//     // plot.add_trace(trace3);
//     // plot.add_trace(trace_yx);
//     if show {
//         plot.show();
//     }
//     println!("{}", plot.to_inline_html(Some("filled_lines")));
// }
