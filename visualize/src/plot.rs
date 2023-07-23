//! A module for plotting curves and regions.

#![warn(missing_docs)]
use plotly::{
    color::NamedColor,
    common::{Fill, Font, Line, Marker, Mode, Title},
    layout::{Axis, Legend, Margin},
    Plot, Scatter,
};

use crate::design::*;

/// A struct to hold the data for a curve.
pub struct Curve {
    /// A vector of x_coordinates.
    pub x_coordinates: Vec<f64>,
    /// A vector of y_coordinates.
    pub y_coordinates: Vec<f64>,
    /// A struct to hold the design of the curve.
    pub design: CurveDesign,
    /// An optional name for the curve that appears in the legend.
    pub name: Option<String>,
}

/// A struct to hold the data for a filled in region.
pub struct Region {
    /// A tuple of x_coordinates.
    /// The two sets will provide bounds for a region.
    pub x_coordinates: (Vec<f64>, Vec<f64>),
    /// A tuple of y_coordinates.
    /// The two sets will provide bounds for a region.
    pub y_coordinates: (Vec<f64>, Vec<f64>),
    /// A struct to hold the design of the region.
    pub design: RegionDesign,
    /// An optional name for the region that appears in the legend.
    pub name: Option<String>,
}

/// A struct to hold the data for a the axes around curves and regions.
pub struct Axes {
    /// A string for the title of the x-axis.
    pub x_label: String,
    /// A string for the title of the y-axis.
    pub y_label: String,
    /// A tuple for the x and y limits of the plot.
    pub bounds: (Vec<f64>, Vec<f64>),
}

#[derive(Debug, Copy, Clone)]
/// A struct that holds high level visualization data for the plot.
pub struct Display {
    /// A boolean to determine if the plot background should be transparent.
    pub transparent: bool,
    /// An enum to determine if we are using light or dark mode.
    pub mode: DisplayMode,
    /// A boolean to determine if the plot should be shown or just generated and saved.
    pub show: bool,
}

/// The main plotting function for curves and regions.
pub fn transparent_plot(
    curves: Option<Vec<Curve>>,
    regions: Option<Vec<Region>>,
    axes: Axes,
    title: String,
    display: Display,
    file_name: Option<String>,
) {
    let mut plot = Plot::new();
    // TODO: Below should be put into a helper function

    if let Some(regions) = regions {
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
                .show_legend(region.name.is_some());
            plot.add_trace(trace);
        }
    }
    if let Some(curves) = curves {
        for curve in curves.iter() {
            let color_slot = curve.design.color_slot;
            let trace = match &curve.design.style {
                Style::Lines(line_emphasis) => {
                    let line = match curve.design.color {
                        Color::Green => Line::new().color(PRIMITIVE_GREENS[color_slot]),
                        Color::Blue => Line::new().color(PRIMITIVE_BLUES[color_slot]),
                        Color::Purple => Line::new().color(PRIMITIVE_PURPLES[color_slot]),
                        Color::Grey => Line::new().color(PRIMITIVE_GREYS[color_slot]),
                        Color::Black => Line::new().color(PRIMITIVE_BLACK),
                        Color::White => Line::new().color(PRIMITIVE_WHITE),
                    };
                    let line = match line_emphasis {
                        LineEmphasis::Light => line.width(2.0),
                        LineEmphasis::Heavy => line.width(4.0),
                        LineEmphasis::Dashed => {
                            line.width(2.0).dash(plotly::common::DashType::Dash)
                        }
                    };
                    Scatter::new(curve.x_coordinates.clone(), curve.y_coordinates.clone())
                        .mode(Mode::Lines)
                        .line(line)
                        .name(&match curve.name.clone() {
                            Some(name) => format!(" {} {} {}", "$\\Large{", name, "}$"),
                            None => "".to_string(),
                        })
                        .show_legend(curve.name.is_some())
                }
                Style::Markers(marker_emphasis) => {
                    let marker = match curve.design.color {
                        Color::Green => Marker::new().color(PRIMITIVE_GREENS[color_slot]),
                        Color::Blue => Marker::new().color(PRIMITIVE_BLUES[color_slot]),
                        Color::Purple => Marker::new().color(PRIMITIVE_PURPLES[color_slot]),
                        Color::Grey => Marker::new().color(PRIMITIVE_GREYS[color_slot]),
                        Color::Black => Marker::new().color(PRIMITIVE_BLACK),
                        Color::White => Marker::new().color(PRIMITIVE_WHITE),
                    };
                    let marker = match marker_emphasis {
                        MarkerEmphasis::Light => marker.size(10),
                        MarkerEmphasis::Heavy => marker.size(20),
                    };
                    Scatter::new(curve.x_coordinates.clone(), curve.y_coordinates.clone())
                        .mode(Mode::Markers)
                        .marker(marker)
                        .name(&match curve.name.clone() {
                            Some(name) => format!(" {} {} {}", "$\\Large{", name, "}$"),
                            None => "".to_string(),
                        })
                        .show_legend(curve.name.is_some())
                }
            };
            plot.add_trace(trace);
        }
    }

    let x_label = format!("{} {} {}", "$\\LARGE{", axes.x_label, "}$");
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
    let y_label = format!("{} {} {}", "$\\LARGE{", axes.y_label, "}$");
    let y_axis = Axis::new()
        .title(Title::new(&y_label).font(Font::new().size(60)))
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
        .margin(Margin::new().bottom(100).left(180).top(100).right(100));
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
                    .x(0.8)
                    .y(0.9),
            )
            .font(Font::new().color(PRIMITIVE_WHITE)),
        DisplayMode::Light => layout
            .legend(
                Legend::new()
                    .font(Font::new().color(PRIMITIVE_BLACK).size(24))
                    .x(0.8)
                    .y(0.9),
            )
            .font(Font::new().color(PRIMITIVE_BLACK)),
    };

    plot.set_layout(layout);
    let file = match file_name {
        Some(file_name) => file_name,
        None => "plot.html".to_string(),
    };
    plot.write_html(file);
    if display.show {
        plot.show();
    }
}
