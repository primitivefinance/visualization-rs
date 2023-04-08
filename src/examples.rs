#![warn(missing_docs)]
use std::ops::Div;

use itertools_num::linspace;
use statrs::consts;

use crate::design::*;
use crate::design::*;
use crate::functions::*;
use crate::plot::*;

/// Plot of different types of approximations to the Gaussian PDF
pub fn compare_approximation_types(display: Display) {
    let title = String::from("\\text{Comparing Types of Approximations}");

    // Use a parameterization
    let t_start = -5.0;
    let t_end = 5.0;
    let number_of_points = 1000;
    let t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>();

    // Build the polynomial approximation
    let polynomial_approximation_design = ElementDesign {
        color: Color::Purple,
        color_slot: MAIN_COLOR_SLOT,
        emphasis: Emphasis::Light,
    };
    let polynomial_approximation = Curve {
        x_coordinates: t.clone(),
        y_coordinates: t.iter().map(|x| 1.0 - x * x).collect(),
        design: polynomial_approximation_design,
        name: Some(String::from("1-x^2")),
    };

    // Build the rational approximation
    let rational_approximation_design = ElementDesign {
        color: Color::Blue,
        color_slot: MAIN_COLOR_SLOT,
        emphasis: Emphasis::Light,
    };
    let rational_approximation = Curve {
        x_coordinates: t.clone(),
        y_coordinates: t.iter().map(|x| 1.0 / (1.0 + x * x)).collect(),
        design: rational_approximation_design,
        name: Some(String::from("(1-x^2)^{-1}")),
    };

    // Build the Gaussian PDF
    let gaussian_pdf_design = ElementDesign {
        color: Color::Green,
        color_slot: MAIN_COLOR_SLOT,
        emphasis: Emphasis::Heavy,
    };
    let gaussian_pdf = Curve {
        x_coordinates: t.clone(),
        y_coordinates: standard_gaussian_pdf(
            t.clone().iter().map(|x| 2.0_f64.sqrt() * x).collect(),
        )
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
        .into_iter()
        .map(|n| match n % 2 {
            0 => (((-1.0) as f64).powi(n.div(2))) / (factorial(n.div(2) as u32) as f64),
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
            design: ElementDesign {
                color: Color::Purple,
                color_slot: degree.clone() as usize,
                emphasis: Emphasis::Light,
            },
            name: Some(format!("{} {}", "\\text{Degree }", degree)),
        };
        curves.push(curve);
    }

    // Build the Gaussian PDF
    let gaussian_pdf_design = ElementDesign {
        color: Color::Green,
        color_slot: MAIN_COLOR_SLOT,
        emphasis: Emphasis::Heavy,
    };
    let gaussian_pdf = Curve {
        x_coordinates: t.clone(),
        y_coordinates: standard_gaussian_pdf(
            t.clone().iter().map(|x| 2.0_f64.sqrt() * x).collect(),
        )
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

    transparent_plot(Some(curves), None, axes, title, display);
}

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
            design: ElementDesign {
                color: Color::Green,
                color_slot: index,
                emphasis: Emphasis::Light,
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

    transparent_plot(Some(curves), None, axes, title, display);
}

// /// Plot RMM trading curve for multiple rescalings
// pub fn rmm_trading_curve_rescaling(transparent: bool, display_mode: DisplayMode, show: bool) {
//     let strike = 3_f64;
//     let sigma = 0.5_f64;
//     let tau = 2.0;
//     let plot_name = format!(
//         "{} {} {} {} {} {} {}",
//         "$\\text{Fractional LPTs with K=}",
//         strike,
//         "\\text{, }\\sigma=",
//         sigma,
//         "\\text{, }\\tau=",
//         tau,
//         "$"
//     );
//     let p_0 = 0.0_f64;
//     let p_1 = 100.0_f64;
//     let n = 1000;
//     let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
//     let mut x: Vec<Vec<f64>> = Vec::new();
//     let mut y: Vec<Vec<f64>> = Vec::new();
//     let scale_factors = linspace(0.1, 1.0, 10).collect::<Vec<f64>>();
//     for scale_factor in scale_factors.iter() {
//         let (x_scale, y_scale) =
//             functions::rmm_trading_curve(prices.clone(), strike, sigma, tau, Some(*scale_factor));
//         x.push(x_scale);
//         y.push(y_scale);
//     }
//     let x_bounds = vec![0_f64, 1_f64];
//     let y_bounds = vec![0_f64, strike];
//     let single_color = true;
//     let colors = vec![(Color::Green, MAIN_COLOR_SLOT, Emphasis::Heavy, single_color)];
//     let labels = Labels {
//         x_label: "$R_x$".to_string(),
//         y_label: "$R_y$".to_string(),
//     };
//     transparent_plot(
//         (x, y),
//         (x_bounds, y_bounds),
//         plot_name,
//         None,
//         colors,
//         (transparent, display_mode, show),
//         labels,
//     );
// }

// /// Plot RMM liquidity distribution for multiple taus
// pub fn rmm_liquidity_distribution(transparent: bool, display_mode: DisplayMode, show: bool) {
//     let plot_name = "$\\text{RMM Liquidity Distribution}$".to_string();
//     let strike = 3_f64;
//     let sigma = 0.5_f64;
//     let taus: Vec<f64> = vec![2.0_f64, 1.5_f64, 1.0_f64, 0.5_f64, 0.1_f64];
//     let p_0 = 0.0_f64;
//     let p_1 = 10.0_f64;
//     let n = 1000;
//     let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
//     let mut x: Vec<Vec<f64>> = Vec::new();
//     let mut y: Vec<Vec<f64>> = Vec::new();
//     for tau in &taus {
//         x.push(prices.clone());
//         let temp =
//             functions::standard_gaussian_pdf(functions::d_one(prices.clone(), strike, sigma, *tau));
//         let temp = temp
//             .iter()
//             .map(|x| x / (sigma * tau.sqrt()))
//             .collect::<Vec<f64>>();
//         let mut temp1: Vec<f64> = Vec::new();
//         for (i, y_val) in temp.iter().enumerate() {
//             temp1.push(y_val / prices[i]);
//         }
//         y.push(temp1);
//     }
//     let x_bounds = vec![0_f64, 10_f64];
//     let y_bounds = vec![0_f64, 1_f64];
//     let single_color = false;
//     let colors = vec![
//         (Color::Green, 0, Emphasis::Light, single_color),
//         (Color::Green, 1, Emphasis::Light, single_color),
//         (Color::Green, 2, Emphasis::Light, single_color),
//         (Color::Green, 3, Emphasis::Light, single_color),
//         (Color::Green, MAIN_COLOR_SLOT, Emphasis::Heavy, single_color),
//     ];
//     let legend_names = vec![
//         "$\\tau=2.0$".to_string(),
//         "$\\tau=1.5$".to_string(),
//         "$\\tau=1.0$".to_string(),
//         "$\\tau=0.5$".to_string(),
//         "$\\tau=0.0$".to_string(),
//     ];
//     let labels = Labels {
//         x_label: "$R_x$".to_string(),
//         y_label: "$R_y$".to_string(),
//     };
//     transparent_plot(
//         (x, y),
//         (x_bounds, y_bounds),
//         plot_name,
//         Some(legend_names),
//         colors,
//         (transparent, display_mode, show),
//         labels,
//     );
// }

// /// Plot RMM portfolio value for multiple taus
// pub fn rmm_portfolio_value(transparent: bool, display_mode: DisplayMode, show: bool) {
//     let plot_name = "\\text{RMM Portfolio Value}".to_string();
//     let strike = 3_f64;
//     let sigma = 0.5_f64;
//     let taus: Vec<f64> = vec![2.0_f64, 1.5_f64, 1.0_f64, 0.5_f64, 0_f64];
//     let p_0 = 0.0_f64;
//     let p_1 = 10.0_f64;
//     let n = 1000;
//     let prices = linspace(p_0, p_1, n).collect::<Vec<f64>>();
//     let mut x: Vec<Vec<f64>> = Vec::new();
//     let mut y: Vec<Vec<f64>> = Vec::new();
//     for tau in &taus {
//         x.push(prices.clone());
//         let temp1 = prices
//             .iter()
//             .zip(
//                 functions::standard_gaussian_cdf(
//                     functions::d_one(prices.clone(), strike, sigma, *tau)
//                         .iter()
//                         .map(|d1| -d1)
//                         .collect(),
//                 )
//                 .iter(),
//             )
//             .map(|(&x, &y)| x * y)
//             .collect::<Vec<f64>>();
//         let temp2 =
//             functions::standard_gaussian_cdf(functions::d_two(prices.clone(), strike, sigma, *tau));
//         y.push(
//             temp1
//                 .iter()
//                 .zip(temp2.iter())
//                 .map(|(&x, &y)| x + strike * y)
//                 .collect(),
//         );
//         // y.push(functions::d_one(prices.clone(), strike, sigma, *tau));
//     }
//     let mut x_single = Vec::new();
//     for _i in 0..prices.len() {
//         x_single.push(strike);
//     }
//     let y_single = linspace(0_f64, 5_f64, n);
//     x.push(x_single);
//     y.push(y_single.collect());
//     let x_bounds = vec![0_f64, 10_f64];
//     let y_bounds = vec![0_f64, 5_f64];
//     let single_color = false;
//     let colors = vec![
//         (Color::Green, 0, Emphasis::Light, single_color),
//         (Color::Green, 1, Emphasis::Light, single_color),
//         (Color::Green, 2, Emphasis::Light, single_color),
//         (Color::Green, 3, Emphasis::Light, single_color),
//         (Color::Green, MAIN_COLOR_SLOT, Emphasis::Heavy, single_color),
//         (Color::Grey, MAIN_COLOR_SLOT, Emphasis::Dashed, single_color),
//     ];
//     let legend_names = vec![
//         "\\tau=2.0".to_string(),
//         "\\tau=1.5".to_string(),
//         "\\tau=1.0".to_string(),
//         "\\tau=0.5".to_string(),
//         "\\tau=0.0".to_string(),
//         "\\text{Strike}".to_string(),
//     ];
//     let labels = Labels {
//         x_label: "S".to_string(),
//         y_label: "V(S)".to_string(),
//     };
//     transparent_plot(
//         (x, y),
//         (x_bounds, y_bounds),
//         plot_name,
//         Some(legend_names),
//         colors,
//         (transparent, display_mode, show),
//         labels,
//     );
// }

// /// Leverage zones plot
// pub fn leverage_zones(transparent: bool, display_mode: DisplayMode, show: bool) {
//     let plot_name = "\\text{Leverage Zones}".to_string();
//     let (mut x, mut y, mut x1, mut y1, mut x2, mut y2) =
//         (vec![], vec![], vec![], vec![], vec![], vec![]);
//     let x_bounds = vec![0.0, 5.0];
//     let y_bounds = vec![0.0, 5.0];
//     let t_start = 0.0;
//     let t_end = 1.0;
//     let number_of_points = 1000;
//     let t = linspace(t_start, t_end, number_of_points).collect::<Vec<f64>>(); // Parameter for curves
//                                                                               // BUILD CURVES //
//                                                                               // y=x^2 curve //
//     x.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
//     y.push(t.iter().map(|t| 25.0 * t * t).collect::<Vec<f64>>());

//     // BUILD REGIONS //
//     // y=x line and above (to y=5) //
//     x1.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
//     y1.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
//     // y=10 line
//     x2.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
//     y2.push(t.iter().map(|_| 5.0).collect::<Vec<f64>>());

//     // y=x line and below (to y=0) //
//     x1.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
//     y1.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
//     // y=0 line
//     x2.push(t.iter().map(|t| 5.0 * t).collect::<Vec<f64>>());
//     y2.push(t.iter().map(|_| 0.0).collect::<Vec<f64>>());

//     // Get the plot
//     let single_color = false;
//     let curve_colors = vec![(Color::Green, MAIN_COLOR_SLOT, Emphasis::Heavy, single_color)];
//     let curve_legend_names = vec!["V(S)=S^2".to_string()];

//     let region_colors = vec![
//         (
//             Color::Purple,
//             MAIN_COLOR_SLOT,
//             Emphasis::Light,
//             single_color,
//         ),
//         (Color::Blue, MAIN_COLOR_SLOT, Emphasis::Light, single_color),
//     ];

//     let region_legend_names = vec![
//         "\\text{Over Levered}".to_string(),
//         "\\text{Under Levered}".to_string(),
//     ];
//     let labels = Labels {
//         x_label: "S".to_string(),
//         y_label: "V(S)".to_string(),
//     };

//     transparent_plot(
//         Some((x, y, curve_colors, Some(curve_legend_names))),
//         Some(((x1, y1), (x2, y2), region_colors, Some(region_legend_names))),
//         (x_bounds, y_bounds),
//         plot_name,
//         (transparent, display_mode, show),
//         labels,
//     );
// }
