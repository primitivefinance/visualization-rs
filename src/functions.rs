use rand_distr::{Distribution, Normal};
use statrs::consts;
use statrs::distribution::{ContinuousCDF, Normal as NormalDist};

pub fn _sample_normal(mean: f64, std_dev: f64, n: usize) -> Vec<f64> {
    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(normal.sample(&mut rng));
    }
    v
}

pub fn d_one(x: Vec<f64>, strike: f64, sigma: f64, tau: f64) -> Vec<f64> {
    let mut v = Vec::with_capacity(x.len());
    for x_val in &x {
        v.push((x_val / strike).ln() / (sigma * tau.sqrt()) + 0.5 * sigma * tau.sqrt());
    }
    v
}

pub fn d_two(x: Vec<f64>, strike: f64, sigma: f64, tau: f64) -> Vec<f64> {
    let mut v = Vec::with_capacity(x.len());
    for x_val in &x {
        v.push((x_val / strike).ln() / (sigma * tau.sqrt()) - 0.5 * sigma * tau.sqrt());
    }
    v
}

// TODO: Rewrite as R_x and R_y functions so that we can use closures the same way in main.rs
pub fn rmm_trading_curve(
    prices: Vec<f64>,
    strike: f64,
    sigma: f64,
    tau: f64,
    scaling: Option<f64>
) -> (Vec<f64>, Vec<f64>) {
    let n = prices.len();
    let (mut x, mut y) = (Vec::with_capacity(n), Vec::with_capacity(n));
    let d1 = d_one(prices.clone(), strike, sigma, tau);
    let d2 = d_two(prices, strike, sigma, tau);
    let normal = NormalDist::new(0.0, 1.0).unwrap();
    for i in 0..n {
        x.push(scaling.unwrap_or(1.0)*(1.0 - normal.cdf(d1[i])));
        y.push(scaling.unwrap_or(1.0)*strike * normal.cdf(d2[i]));
    }
    (x, y)
}

pub fn _standard_gaussian_cdf(x: Vec<f64>) -> Vec<f64> {
    let normal = NormalDist::new(0.0, 1.0).unwrap();
    let mut y = Vec::with_capacity(x.len());
    for x_val in &x {
        y.push(normal.cdf(*x_val));
    }
    y
}

pub fn standard_gaussian_pdf(x: Vec<f64>) -> Vec<f64> {
    let mut y = Vec::with_capacity(x.len());
    for x_val in &x {
        y.push((-0.5 * x_val * x_val).exp() / (consts::SQRT_2PI));
    }
    y
}

pub fn polynomial_approx(x: Vec<f64>, coeffs: Vec<f64>) -> Vec<f64> {
    let mut y = Vec::with_capacity(x.len());
    for x_val in &x {
        let mut y_val = 0.0;
        for (i, coeff) in coeffs.iter().enumerate() {
            y_val += coeff * x_val.powi(i as i32);
        }
        y.push(y_val);
    }
    y
}

pub fn factorial(n: u32) -> u32 {
    let mut f = 1;
    for i in 1..=n {
        f *= i;
    }
    f
}

pub fn parametric_line(t: Vec<f64>, a: f64, b: f64, x_0: f64, y_0: f64) -> (Vec<f64>,Vec<f64>) {
    let mut x = Vec::with_capacity(t.len());
    let mut y = Vec::with_capacity(t.len());
    for t_val in &t {
        x.push(a * t_val + x_0);
        y.push(b * t_val + y_0);
    }
    (x,y)
}