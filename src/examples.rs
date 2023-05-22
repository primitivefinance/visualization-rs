#![warn(missing_docs)]
use std::error::Error;
use std::ops::Div;

use itertools_num::linspace;
use mentat::MonotonicCubicSpline;
use statrs::consts;

use visualize::{design::*, file_handler::*, functions::*, plot::*};

#[allow(unused)]
/// Plot of different types of approximations to the Gaussian PDF
pub fn compare_approximation_types(display: Display) {
    let title = String::from("\\text{Comparing Types of Approximations}");
    // Use a parameterization
    let t_start = -5.0;
    let t_end = 5.0;
    let number_of_points = 1000;
    let t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>();
    // Build the polynomial approximation
    let polynomial_approximation_design = CurveDesign {
        color: Color::Purple,
        color_slot: MAIN_COLOR_SLOT,
        style: Style::Lines(LineEmphasis::Light),
    };
    let polynomial_approximation = Curve {
        x_coordinates: t.clone(),
        y_coordinates: t.iter().map(|x| 1.0 - x * x).collect(),
        design: polynomial_approximation_design,
        name: Some(String::from("1-x^2")),
    };
    // Build the rational approximation
    let rational_approximation_design = CurveDesign {
        color: Color::Blue,
        color_slot: MAIN_COLOR_SLOT,
        style: Style::Lines(LineEmphasis::Light),
    };
    let rational_approximation = Curve {
        x_coordinates: t.clone(),
        y_coordinates: t.iter().map(|x| 1.0 / (1.0 + x * x)).collect(),
        design: rational_approximation_design,
        name: Some(String::from("(1-x^2)^{-1}")),
    };
    // Build the Gaussian PDF
    let gaussian_pdf_design = CurveDesign {
        color: Color::Green,
        color_slot: MAIN_COLOR_SLOT,
        style: Style::Lines(LineEmphasis::Heavy),
    };
    let gaussian_pdf = Curve {
        x_coordinates: t.clone(),
        y_coordinates: standard_gaussian_pdf(t.iter().map(|x| 2.0_f64.sqrt() * x).collect())
            .iter()
            .map(|y| consts::SQRT_2PI * y)
            .collect(),
        design: gaussian_pdf_design,
        name: Some(String::from("\\exp\\left(-x^2\\right)")),
    };
    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("x"),
        y_label: String::from("f(x)"),
        bounds: (vec![t_start, t_end], vec![-0.5, 1.5]),
    };
    //plot
    transparent_plot(
        Some(vec![
            polynomial_approximation,
            rational_approximation,
            gaussian_pdf,
        ]),
        None,
        axes,
        title,
        display,
    );
}
#[allow(unused)]
/// Plots of polynomial approximations to the Gaussian PDF
pub fn polynomial_approximations(display: Display) {
    let title = String::from("\\text{Polynomial Approximations}");
    // Use a parameterization
    let t_start = -5.0;
    let t_end = 5.0;
    let number_of_points = 1000;
    let t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>();
    // Build the approximation coefficients
    let top_degree = 8;
    let coefficient_range = 0..top_degree + 1;
    let coefficients = coefficient_range
        .clone()
        .map(|n| match n % 2 {
            0 => ((-1.0_f64).powi(n.div(2))) / (factorial(n.div(2) as u32) as f64),
            _ => 0.0,
        })
        .collect::<Vec<f64>>();
    // Build the polynomial approximations
    let mut curves = vec![];
    for degree in coefficient_range.step_by(2) {
        let curve = Curve {
            x_coordinates: t.clone(),
            y_coordinates: polynomial_approx(
                t.clone(),
                coefficients[0..1 + degree as usize].to_vec(),
            ),
            design: CurveDesign {
                color: Color::Purple,
                color_slot: degree as usize,
                style: Style::Lines(LineEmphasis::Light),
            },
            name: Some(format!("{} {}", "\\text{Degree }", degree)),
        };
        curves.push(curve);
    }
    // Build the Gaussian PDF
    let gaussian_pdf_design = CurveDesign {
        color: Color::Green,
        color_slot: MAIN_COLOR_SLOT,
        style: Style::Lines(LineEmphasis::Heavy),
    };
    let gaussian_pdf = Curve {
        x_coordinates: t.clone(),
        y_coordinates: standard_gaussian_pdf(t.iter().map(|x| 2.0_f64.sqrt() * x).collect())
            .iter()
            .map(|y| consts::SQRT_2PI * y)
            .collect(),
        design: gaussian_pdf_design,
        name: Some(String::from("\\exp\\left(-x^2\\right)")),
    };
    curves.push(gaussian_pdf);
    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("x"),
        y_label: String::from("f(x)"),
        bounds: (vec![t_start, t_end], vec![-0.5, 1.5]),
    };
    // plot
    transparent_plot(Some(curves), None, axes, title, display);
}
#[allow(unused)]
/// Plot RMM trading curve for multiple taus from a list of prices
pub fn rmm_trading_curve_multiple_taus(display: Display) {
    let title = String::from("\\text{RMM Trading Curve}");
    // Define the relavant RMM-CC parameters with multiple taus
    let strike = 3_f64;
    let sigma = 0.5_f64;
    let taus: Vec<f64> = linspace(2.0, 0.0, 5).collect::<Vec<f64>>();
    // Create a list of prices that we will compute the reserves from
    let price_start = 0.0_f64;
    let price_end = 100.0_f64;
    let number_of_prices = 1000;
    let prices = linspace(price_start, price_end, number_of_prices).collect::<Vec<f64>>();
    // Build the curves
    let mut curves = vec![];
    for (index, tau) in taus.iter().enumerate() {
        let (reserves_x_tau, reserves_y_tau) =
            rmm_trading_curve(prices.clone(), strike, sigma, *tau, None);
        let curve = Curve {
            x_coordinates: reserves_x_tau,
            y_coordinates: reserves_y_tau,
            design: CurveDesign {
                color: Color::Green,
                color_slot: index,
                style: Style::Lines(LineEmphasis::Light),
            },
            name: Some(format!("{} {}", "\\tau=", tau)),
        };
        curves.push(curve);
    }
    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("R_x"),
        y_label: String::from("R_y"),
        bounds: (vec![0.0, 1.0], vec![0.0, 3.0]),
    };
    // plot
    transparent_plot(Some(curves), None, axes, title, display);
}
#[allow(unused)]
/// Plot RMM trading curve for multiple rescalings
pub fn rmm_trading_curve_rescaling(display: Display) {
    // Define the RMM-CC parameters
    let strike = 3_f64;
    let sigma = 0.5_f64;
    let tau = 2.0;
    let title = format!(
        "{} {} {} {} {} {} {}",
        "$\\text{Fractional LPTs with K=}",
        strike,
        "\\text{, }\\sigma=",
        sigma,
        "\\text{, }\\tau=",
        tau,
        "$"
    );
    // Define the range of prices
    let price_start = 0.0_f64;
    let price_end = 100.0_f64;
    let number_of_prices = 1000;
    let prices = linspace(price_start, price_end, number_of_prices).collect::<Vec<f64>>();
    // Choose the rescaling factors and build the curves
    let scale_factors = linspace(0.1, 1.0, 10);
    let mut curves = vec![];
    for scale_factor in scale_factors {
        let (x_scale, y_scale) =
            rmm_trading_curve(prices.clone(), strike, sigma, tau, Some(scale_factor));
        let curve = Curve {
            x_coordinates: x_scale,
            y_coordinates: y_scale,
            design: CurveDesign {
                color: Color::Green,
                color_slot: MAIN_COLOR_SLOT,
                style: Style::Lines(LineEmphasis::Light),
            },
            name: Some(format!("{} {}", "\\text{Scale }", scale_factor)),
        };
        curves.push(curve);
    }
    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("R_x"),
        y_label: String::from("R_y"),
        bounds: (vec![0.0, 1.0], vec![0.0, 3.0]),
    };
    // plot
    transparent_plot(Some(curves), None, axes, title, display);
}

/// Plot RMM liquidity distribution for multiple taus
#[allow(unused)]
pub fn rmm_liquidity_distribution(display: Display) {
    let title = String::from("$\\text{RMM Liquidity Distribution}$");
    // Define the relavant RMM-CC parameters with multiple taus
    let strike = 3_f64;
    let sigma = 0.5_f64;
    let taus: Vec<f64> = vec![1.0_f64, 0.5_f64, 0.3_f64, 0.2_f64, 0.1_f64];
    // Create a list of prices that we will compute the reserves from
    let price_start = 0.0_f64;
    let price_end = 10.0_f64;
    let number_of_prices = 1000;
    let prices = linspace(price_start, price_end, number_of_prices).collect::<Vec<f64>>();
    // Build the curves
    let mut curves = vec![];
    for (index, tau) in taus.iter().enumerate() {
        let pdf_of_d_one = standard_gaussian_pdf(d_one(prices.clone(), strike, sigma, *tau));
        let temp = pdf_of_d_one
            .iter()
            .map(|output| output / (sigma * tau.sqrt()))
            .collect::<Vec<f64>>();
        let mut after_divide: Vec<f64> = Vec::new();
        for (i, y_val) in temp.iter().enumerate() {
            after_divide.push(y_val / prices[i]);
        }

        let curve = Curve {
            x_coordinates: prices.clone(),
            y_coordinates: after_divide,
            design: CurveDesign {
                color: Color::Green,
                color_slot: index,
                style: Style::Lines(LineEmphasis::Light),
            },
            name: Some(format!("{} {}", "\\tau=", tau)),
        };
        curves.push(curve);
    }
    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("S"),
        y_label: String::from("L(S)"),
        bounds: (vec![price_start, price_end], vec![0.0, 1.0]),
    };
    // plot
    transparent_plot(Some(curves), None, axes, title, display);
}

/// Plot RMM portfolio value for multiple taus
#[allow(unused)]
pub fn rmm_portfolio_value(display: Display) {
    let title = "\\text{RMM Portfolio Value}".to_string();
    // Define the relavant RMM-CC parameters with multiple taus
    let strike = 3_f64;
    let sigma = 0.5_f64;
    let taus = vec![2.0_f64, 1.5_f64, 1.0_f64, 0.5_f64, 0_f64];
    // Create a list of prices that we will compute the reserves from
    let price_start = 0.0_f64;
    let price_end = 10.0_f64;
    let number_of_prices = 1000;
    let prices = linspace(price_start, price_end, number_of_prices).collect::<Vec<f64>>();
    // Build the curves
    let mut curves = vec![];
    for (index, tau) in taus.iter().enumerate() {
        let temp1 = prices
            .iter()
            .zip(
                standard_gaussian_cdf(
                    d_one(prices.clone(), strike, sigma, *tau)
                        .iter()
                        .map(|d1| -d1)
                        .collect(),
                )
                .iter(),
            )
            .map(|(&x, &y)| x * y)
            .collect::<Vec<f64>>();
        let temp2 = standard_gaussian_cdf(d_two(prices.clone(), strike, sigma, *tau));
        let curve = Curve {
            x_coordinates: prices.clone(),
            y_coordinates: temp1
                .iter()
                .zip(temp2.iter())
                .map(|(&x, &y)| x + strike * y)
                .collect(),
            design: CurveDesign {
                color: Color::Green,
                color_slot: index,
                style: Style::Lines(LineEmphasis::Light),
            },
            name: Some(format!("{} {}", "\\tau=", tau)),
        };

        curves.push(curve);
    }
    // Make a dashed line at the strike price
    let strike_price_curve = Curve {
        x_coordinates: prices.iter().map(|_| strike).collect(),
        y_coordinates: linspace(0.0, 5.0, number_of_prices).collect(),
        design: CurveDesign {
            color: Color::Grey,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines(LineEmphasis::Dashed),
        },
        name: Some(String::from("Strike")),
    };
    curves.push(strike_price_curve);
    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("S"),
        y_label: String::from("V(S)"),
        bounds: (vec![price_start, price_end], vec![0.0, 5.0]),
    };
    //plot
    transparent_plot(Some(curves), None, axes, title, display);
}

/// Leverage zones plot with S^2 pvf
#[allow(unused)]
pub fn leverage_zones_with_pvf(display: Display) {
    let title = "\\text{Leverage Zones}".to_string();
    // Use a parameterization of the curves to build them
    let t_start = 0.0;
    let t_end = 1.0;
    let number_of_points = 1000;
    let t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>(); // Parameter for curves
                                                                              // Build the S^2 pvf
    let curve = Curve {
        x_coordinates: t.iter().map(|t| 5.0 * t).collect(),
        y_coordinates: t.iter().map(|t| 25.0 * t * t).collect(),
        design: CurveDesign {
            color: Color::Green,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines(LineEmphasis::Heavy),
        },
        name: Some(String::from("V(S)=S^2")),
    };
    // BUILD REGIONS
    // y=x line and above (to y=5)
    let over_levered = Region {
        x_coordinates: (
            t.iter().map(|t| 5.0 * t).collect(),
            t.iter().map(|t| 5.0 * t).collect(),
        ),
        y_coordinates: (
            t.iter().map(|t| 5.0 * t).collect(),
            t.iter().map(|_| 5.0).collect(),
        ),
        design: RegionDesign {
            color: Color::Purple,
            color_slot: MAIN_COLOR_SLOT,
        },
        name: Some(String::from("Over-levered")),
    };
    // y=x line and below (to y=0)
    let under_levered = Region {
        x_coordinates: (
            t.iter().map(|t| 5.0 * t).collect(),
            t.iter().map(|t| 5.0 * t).collect(),
        ),
        y_coordinates: (
            t.iter().map(|t| 5.0 * t).collect(),
            t.iter().map(|_| 0.0).collect(),
        ),
        design: RegionDesign {
            color: Color::Blue,
            color_slot: MAIN_COLOR_SLOT,
        },
        name: Some(String::from("Under-levered")),
    };
    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("S"),
        y_label: String::from("V(S)"),
        bounds: (vec![0.0, 5.0], vec![0.0, 5.0]),
    };
    // plot
    transparent_plot(
        Some(vec![curve]),
        Some(vec![over_levered, under_levered]),
        axes,
        title,
        display,
    );
}
#[allow(unused)]
/// plot brownian bridge
pub fn brownian_bridge_plotter(display: Display, start_price: f64, end_price: f64) {
    let title = "\\text{Example Price Paths}".to_string();
    // Use a parameterization of the curves to build them
    let t_start = 0.0;
    let t_end = 1.0;
    let number_of_points = 1000;
    let mut t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>(); // Parameter for curves
                                                                                  // Build brownian bridge curve
    let brownian1 =
        brownian_bridge_generator(start_price, end_price, t_end, number_of_points, 1.0, 4);
    let brownian2 =
        brownian_bridge_generator(start_price, end_price, t_end, number_of_points, 1.0, 33);
    let curve1 = Curve {
        x_coordinates: t.clone(),
        y_coordinates: brownian1,
        design: CurveDesign {
            color: Color::Green,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines(LineEmphasis::Light),
        },
        name: Some(String::from("\\text{High Volatility}")),
    };
    let curve2 = Curve {
        x_coordinates: t,
        y_coordinates: brownian2,
        design: CurveDesign {
            color: Color::Green,
            color_slot: 2,
            style: Style::Lines(LineEmphasis::Light),
        },
        name: Some(String::from("\\text{Low Volatility}")),
    };
    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("t"),
        y_label: String::from("P(t)"),
        bounds: (vec![0.0, 1.0], vec![0.0, 3000.0]),
    };
    //plot
    transparent_plot(Some(vec![curve1, curve2]), None, axes, title, display);
}

#[allow(unused)]
pub fn cubic_spline_plotter(display: Display) {
    let number_of_points = 7;
    let title = format!(
        "{}{}{}",
        "\\text{Monotonic Cubic Spline Approximation for ", number_of_points, " Points}"
    );

    // Use a parameterization of the curves to build them
    let x_start = -3.0;
    let x_end = 3.0;

    let x_coordinates = linspace(x_start, x_end, number_of_points).collect::<Vec<f64>>(); // Parameter for curves

    // Get the CDF points
    let y_coordinates = standard_gaussian_cdf(x_coordinates.clone());
    let curve = Curve {
        x_coordinates: x_coordinates.clone(),
        y_coordinates: y_coordinates.clone(),
        design: CurveDesign {
            color: Color::Green,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Markers(MarkerEmphasis::Heavy),
        },
        name: Some(String::from("\\text{CDF Points}")), // TODO: Make this just discrete points
    };

    // Get the cubic spline
    let mut spline = MonotonicCubicSpline::new(&x_coordinates, &y_coordinates);
    let x_spline_coordinates = linspace(x_start, x_end, 1000).collect::<Vec<f64>>(); // Parameter for curves
    let y_spline_coordinates = x_spline_coordinates
        .clone()
        .into_iter()
        .map(|x| spline.interpolate(x))
        .collect::<Vec<f64>>();
    let spline_curve = Curve {
        x_coordinates: x_spline_coordinates,
        y_coordinates: y_spline_coordinates,
        design: CurveDesign {
            color: Color::Blue,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines(LineEmphasis::Heavy),
        },
        name: Some(String::from("\\text{CDF Spline}")),
    };

    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("x"),
        y_label: String::from("\\Phi(x)"),
        bounds: (vec![-3.0, 3.0], vec![0.0, 1.0]),
    };

    transparent_plot(Some(vec![curve, spline_curve]), None, axes, title, display);
}
#[allow(unused)]
/// Plot imported csv data for single column csv's
pub fn csv_plotter(display: Display) -> Result<(), Box<dyn Error>> {
    // Get the file information
    let file_path = "test_prices.csv";
    let column_name = "liquid_exchange_prices";

    // Import the data from the csv file
    let prices = read_column_from_csv(file_path, column_name)?;

    // Use a parameterization of the curves to build them
    let number_of_points = prices.len();
    let t_start = 0.0;
    let t_end = number_of_points as f64;
    let t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>();

    // Build curve
    let curve = Curve {
        x_coordinates: t,
        y_coordinates: prices,
        design: CurveDesign {
            color: Color::Green,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines(LineEmphasis::Light),
        },
        name: Some(String::from("\\text{Liquid Exchange Prices}")),
    };
    // build plot axes and title
    let title = "\\text{csv Data}".to_string();
    let axes = Axes {
        x_label: String::from("\\text{Trade Number}"),
        y_label: String::from("\\text{Prices}"),
        bounds: (vec![0.0, t_end], vec![0.95, 1.05]),
    };
    //plot
    transparent_plot(Some(vec![curve]), None, axes, title, display);
    Ok(())
}
#[allow(unused)]
/// Plot for Perpetual Put and Covered Call
pub fn pp_and_cc_plotter(display: Display) -> Result<(), Box<dyn Error>> {
    let number_of_points = 1000;
    let title = "\\text{Perpetual Put and Covered Call}".to_string();

    let x_start = 0.1;
    let x_end = 15.0;
    let x_coordinates = linspace(x_start, x_end, number_of_points).collect::<Vec<f64>>(); // Parameter for curves

    let strike = 7.0;
    let sigma = 0.5;
    let rate = 0.04;
    let tau = 0.1;
    // generate curves
    let cc = rmm_cc_payoff(x_coordinates.clone(), strike, sigma, tau).1;
    let pp = rmm_pp_payoff(x_coordinates.clone(), strike, sigma, tau).1;
    let cc_curve = Curve {
        x_coordinates: x_coordinates.clone(),
        y_coordinates: cc.clone(),
        design: CurveDesign {
            color: Color::Green,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines(LineEmphasis::Light),
        },
        name: Some(String::from("\\text{Covered Call}")),
    };
    let pp_curve = Curve {
        x_coordinates: x_coordinates.clone(),
        y_coordinates: pp.clone(),
        design: CurveDesign {
            color: Color::Blue,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines(LineEmphasis::Light),
        },
        name: Some(String::from("\\text{Perpetual Put}")),
    };
    let both_curve = Curve {
        x_coordinates,
        y_coordinates: cc
            .iter()
            .zip(pp.iter())
            .map(|(&x, &y)| x + y)
            .collect::<Vec<_>>(),
        design: CurveDesign {
            color: Color::Purple,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines(LineEmphasis::Light),
        },
        name: Some(String::from("\\text{Covered Call + Perpetual Put}")),
    };
    // Build the plot's axes
    let axes = Axes {
        x_label: String::from("S"),
        y_label: String::from("V(S)"),
        bounds: (vec![x_start, x_end], vec![0.0, 10.0]),
    };
    //plot
    transparent_plot(
        Some(vec![cc_curve, pp_curve, both_curve]),
        None,
        axes,
        title,
        display,
    );
    Ok(())
}
#[allow(unused)]
/// Plot for Forced-Re-balance on RMM-CC
pub fn plot_forced_rebalance(display: Display) -> Result<(), Box<dyn Error>> {
    let number_of_points = 1000;
    let x_start = 0.001;
    let x_end = 0.999;
    let x = linspace(x_start, x_end, number_of_points).collect::<Vec<f64>>(); // Parameter for curves

    let strike = 5.0;
    let sigma = 0.5;
    let ratio = 1.5;
    let tau = 0.8;
    let inv = 0.0;

    let (reserves, value) = forced_rebalance(x, strike, sigma, tau, ratio, inv);
    // Build curve
    let curve = Curve {
        x_coordinates: reserves,
        y_coordinates: value,
        design: CurveDesign {
            color: Color::Purple,
            color_slot: MAIN_COLOR_SLOT,
            style: Style::Lines(LineEmphasis::Light),
        },
        name: Some(String::from("\\text{Forced Rebalance}")),
    };
    // build plot axes and title
    let title = "\\text{Forced Rebalance}".to_string();
    let axes = Axes {
        x_label: String::from("R_x"),
        y_label: String::from("V(R_x)"),
        bounds: (vec![0.0, 1.0], vec![-1.0, 0.6]),
    };
    //plot
    transparent_plot(Some(vec![curve]), None, axes, title, display);
    Ok(())
}
