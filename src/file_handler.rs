use csv::ReaderBuilder;
use itertools_num::linspace;
use std::{error::Error, fs::File};

use crate::design::*;
use crate::plot::*;

#[allow(unused)]
/// Import CSV file of price data
/// # Arguments
/// * `file_path` - path to csv file of price data (String)
/// # Returns
/// * `price_data` - Vector of prices. (Vec<f64>)
pub fn import_price_from_csv(file_path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut price_data: Vec<f64> = Vec::new();
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in reader.deserialize() {
        let num: f64 = result?;
        price_data.push(num);
    }

    Ok(price_data)
}

#[allow(unused)]
/// Plot imported csv data for single column csv's
pub fn csv_plotter(display: Display, file_path: &str) {
    let title = "\\text{csv Data}".to_string();
    // Import the data from the csv file
    let value = import_price_from_csv(file_path).unwrap();
    // Use a parameterization of the curves to build them
    let t_start = 0.0;
    let number_of_points = value.len();
    let t_end = number_of_points as f64;
    let t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>();
    // build curve
    let curve = Curve {
        x_coordinates: t.clone(),
        y_coordinates: value,
        design: CurveDesign {
            color: Color::Green,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines,
            emphasis: Emphasis::Light,
        },
        name: Some(String::from("\\text{csv data}")),
    };
    // build plot axes
    let axes = Axes {
        x_label: String::from("event number"),
        y_label: String::from("value"),
        bounds: (vec![0.0, t_end], vec![0.0, 3000.0]),
    };
    //plot
    transparent_plot(Some(vec![curve]), None, axes, title, display);
}
