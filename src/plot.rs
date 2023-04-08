#![warn(missing_docs)]
use plotly::{
    common::{Font, Line, Mode, Pad, Title},
    layout::{Axis, Legend, TicksPosition},
    Configuration,
};
use plotly::{layout::Margin, Plot, Scatter};
use serde::ser::Serialize;

// testing
use itertools_num::linspace;
use plotly::color::{NamedColor, Rgb, Rgba};
use plotly::common::{ColorScale, ColorScalePalette, DashType, Fill, LineShape, Marker};
use plotly::layout::{BarMode, Layout, TicksDirection};
use rand_distr::{Distribution, Normal, Uniform};

use crate::design::*;

pub struct Curve {
    pub x_coordinates: Vec<f64>,
    pub y_coordinates: Vec<f64>,
    pub design: ElementDesign, // TODO: Edit this
    pub name: Option<String>,
}

pub struct Region {
    /// A tuple of x_coordinates.
    /// The two sets will provide bounds for a region.
    pub x_coordinates: (Vec<f64>, Vec<f64>),
    /// A tuple of y_coordinates.
    /// The two sets will provide bounds for a region.
    pub y_coordinates: (Vec<f64>, Vec<f64>),
    pub design: ElementDesign,
    pub name: Option<String>,
}

pub struct Axes {
    pub x_label: String,
    pub y_label: String,
    pub bounds: (Vec<f64>, Vec<f64>),
}

pub struct Display {
    pub transparent: bool,
    pub mode: DisplayMode,
    pub show: bool,
}

pub fn transparent_plot(
    curves: Option<Vec<Curve>>,
    regions: Option<Vec<Region>>,
    axes: Axes,
    title: String,
    display: Display,
) {
    let mut plot = Plot::new();
    // TODO: Below should be put into a helper function
    match regions {
        Some(regions) => {
            for region in regions.iter() {
                let color_slot = region.design.color_slot;
                let color = match region.design.color {
                    Color::Green => format!("{}AA", PRIMITIVE_GREENS[color_slot]),
                    Color::Blue => format!("{}AA", PRIMITIVE_BLUES[color_slot]),
                    Color::Purple => format!("{}AA", PRIMITIVE_PURPLES[color_slot]),
                    Color::Grey => format!("{}AA", PRIMITIVE_GREYS[color_slot]),
                    Color::Black => format!("{}AA", PRIMITIVE_BLACK),
                    Color::White => format!("{}AA", PRIMITIVE_WHITE),
                };
                // Combine the two x coordinates for the bounding curves by reversing the second and appending into a longer vector.
                let x1_coords = region.x_coordinates.0.clone();
                let x2_coords = region.x_coordinates.1.clone();
                let x2_reversed = x2_coords.iter().cloned().rev().collect::<Vec<f64>>();
                // Append thing to x1_coords to create a new vector
                let x_combined = x1_coords
                    .into_iter()
                    .chain(x2_reversed.into_iter())
                    .collect::<Vec<f64>>();

                // Combine the two y coordinates for the bounding curves by reversing the second and appending into a longer vector.
                let y1_coords = region.y_coordinates.0.clone();
                let y2_coords = region.y_coordinates.1.clone();
                let y2_reversed = y2_coords.iter().cloned().rev().collect::<Vec<f64>>();
                // Append thing to x1_coords to create a new vector
                let y_combined = y1_coords
                    .into_iter()
                    .chain(y2_reversed.into_iter())
                    .collect::<Vec<f64>>();

                let trace = Scatter::new(x_combined, y_combined)
                    .fill(Fill::ToSelf)
                    .fill_color(color)
                    .line(Line::new().color(NamedColor::Transparent))
                    .name(&match region.name.clone() {
                        Some(name) => format!(" {} {} {}", "$\\Large{", name, "}$"),
                        None => "".to_string(),
                    })
                    .show_legend(match region.name {
                        Some(_) => true,
                        None => false,
                    });
                plot.add_trace(trace);
            }
        }
        None => {}
    }
    match curves {
        Some(curves) => {
            for curve in curves.iter() {
                let color_slot = curve.design.color_slot;
                let line = match curve.design.color {
                    Color::Green => Line::new().color(PRIMITIVE_GREENS[color_slot]),
                    Color::Blue => Line::new().color(PRIMITIVE_BLUES[color_slot]),
                    Color::Purple => Line::new().color(PRIMITIVE_PURPLES[color_slot]),
                    Color::Grey => Line::new().color(PRIMITIVE_GREYS[color_slot]),
                    Color::Black => Line::new().color(PRIMITIVE_BLACK),
                    Color::White => Line::new().color(PRIMITIVE_WHITE),
                };
                let line = match curve.design.emphasis {
                    Emphasis::Light => line.width(2.0),
                    Emphasis::Heavy => line.width(4.0),
                    Emphasis::Dashed => line.width(2.0).dash(plotly::common::DashType::Dash),
                };
                let trace = Scatter::new(curve.x_coordinates.clone(), curve.y_coordinates.clone())
                    .mode(Mode::Lines)
                    .line(line)
                    .name(&match curve.name.clone() {
                        Some(name) => format!(" {} {} {}", "$\\Large{", name, "}$"),
                        None => "".to_string(),
                    })
                    .show_legend(match curve.name {
                        Some(_) => true,
                        None => false,
                    });
                plot.add_trace(trace);
            }
        }
        None => {}
    }
    let x_label = format!("{} {} {}", "$\\LARGE{", axes.x_label.clone(), "}$");
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
        .range(axes.bounds.0)
        .ticks(plotly::layout::TicksDirection::Outside);
    let y_label = format!("{} {} {}", "$\\LARGE{", axes.y_label.clone(), "}$");
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
        .range(axes.bounds.1)
        .ticks(plotly::layout::TicksDirection::Outside);

    let (x_axis, y_axis) = match display.mode {
        DisplayMode::Light => (
            x_axis.color(PRIMITIVE_BLACK).line_color(PRIMITIVE_BLACK),
            y_axis.color(PRIMITIVE_BLACK).line_color(PRIMITIVE_BLACK),
        ),
        DisplayMode::Dark => (
            x_axis.color(PRIMITIVE_WHITE).line_color(PRIMITIVE_WHITE),
            y_axis.color(PRIMITIVE_WHITE).line_color(PRIMITIVE_WHITE),
        ),
    };
    let title = format!("{} {} {}", "$\\huge{", title, "}$");
    let layout = plotly::Layout::new()
        .title(plotly::common::Title::new(title.as_str()))
        .x_axis(x_axis)
        .y_axis(y_axis)
        .width(1600)
        .height(900)
        .margin(Margin::new().bottom(100).left(160).top(100).right(100));
    let layout = match display.transparent {
        true => layout
            .plot_background_color("rgba(0,0,0,0)")
            .paper_background_color("rgba(0,0,0,0)"),
        false => match display.mode {
            DisplayMode::Dark => layout
                .plot_background_color(PRIMITIVE_BLACK)
                .paper_background_color(PRIMITIVE_BLACK),
            DisplayMode::Light => layout
                .plot_background_color(PRIMITIVE_WHITE)
                .paper_background_color(PRIMITIVE_WHITE),
        },
    };
    let layout = match display.mode {
        DisplayMode::Dark => layout
            .show_legend(true)
            .legend(
                Legend::new()
                    .font(Font::new().color(PRIMITIVE_WHITE).size(24))
                    .x(0.7)
                    .y(0.9),
            )
            .font(Font::new().color(PRIMITIVE_WHITE)),
        DisplayMode::Light => layout
            .legend(
                Legend::new()
                    .font(Font::new().color(PRIMITIVE_BLACK).size(24))
                    .x(0.7)
                    .y(0.9),
            )
            .font(Font::new().color(PRIMITIVE_BLACK)),
    };

    plot.set_layout(layout);
    plot.write_html(title.as_str().to_owned() + ".html");
    if display.show {
        plot.show();
    }
}
