use polars::prelude::*;
use std::error::Error;

#[allow(unused)]
/// Import CSV file of price data
/// # Arguments
/// * `file_path` - path to csv file of price data (&str)
/// * `column_name` - name of column to read from csv file (&str)
/// # Returns
/// * `Result<Vec<f4>, Box<dyn Error>>` - Vector of values. (Vec<f64>)
pub fn read_column_from_csv(
    file_path: &str,
    column_name: &str,
) -> Result<Vec<f64>, Box<dyn Error>> {
    let df = CsvReader::from_path(file_path)?
        .infer_schema(None)
        .with_ignore_errors(true)
        .has_header(true)
        .finish()?;
    let column_as_f64 = df
        .column(column_name)?
        .cast(&DataType::Float64)?
        .f64()?
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<f64>>();

    Ok(column_as_f64)
}
