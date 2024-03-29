use itertools_num::linspace;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use rand_pcg::Pcg64;
use statrs::consts;
use statrs::distribution::{ContinuousCDF, Normal as NormalDist};

#[allow(unused)]
pub fn _sample_normal(mean: f64, std_dev: f64, n: usize) -> Vec<f64> {
    let normal = Normal::new(mean, std_dev).unwrap();
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(normal.sample(&mut rng));
    }
    v
}

#[allow(unused)]
pub fn brownian_bridge_generator(
    start_price: f64,
    end_price: f64,
    end_time: f64,
    steps: usize,
    volatility: f64,
    seed: u64,
) -> Vec<f64> {
    let t = linspace(0.0, end_time, steps).collect::<Vec<f64>>();
    let mut rng = Pcg64::seed_from_u64(seed);
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut prices = vec![start_price];
    let mut current_price = start_price;

    for i in 1..steps {
        let dt = t[i] - t[i - 1];
        let z = rng.sample(normal);
        current_price += z * dt.sqrt();
        prices.push(current_price);
    }
    let bridge = prices
        .iter()
        .map(|x| {
            start_price
                + (end_price - start_price) * (x - prices[steps - 1])
                    / (prices[0] - prices[steps - 1])
        })
        .collect();
    bridge
}

// TODO: May be better to have functions not vectorized and use iterators outside.
#[allow(unused)]
pub fn d_one(x: Vec<f64>, strike: f64, sigma: f64, tau: f64) -> Vec<f64> {
    let mut v = Vec::with_capacity(x.len());
    for x_val in &x {
        v.push((x_val / strike).ln() / (sigma * tau.sqrt()) + 0.5 * sigma * tau.sqrt());
    }
    v
}

#[allow(unused)]
pub fn d_two(x: Vec<f64>, strike: f64, sigma: f64, tau: f64) -> Vec<f64> {
    let mut v = Vec::with_capacity(x.len());
    for x_val in &x {
        v.push((x_val / strike).ln() / (sigma * tau.sqrt()) - 0.5 * sigma * tau.sqrt());
    }
    v
}

#[allow(unused)]
pub fn rmm_trading_curve(
    prices: Vec<f64>,
    strike: f64,
    sigma: f64,
    tau: f64,
    scaling: Option<f64>,
) -> (Vec<f64>, Vec<f64>) {
    let n = prices.len();
    let (mut x, mut y) = (Vec::with_capacity(n), Vec::with_capacity(n));
    let d1 = d_one(prices.clone(), strike, sigma, tau);
    let d2 = d_two(prices, strike, sigma, tau);
    let normal = NormalDist::new(0.0, 1.0).unwrap();
    for i in 0..n {
        x.push(scaling.unwrap_or(1.0) * (1.0 - normal.cdf(d1[i])));
        y.push(scaling.unwrap_or(1.0) * strike * normal.cdf(d2[i]));
    }
    (x, y)
}
#[allow(unused)]
pub fn standard_gaussian_cdf(x: Vec<f64>) -> Vec<f64> {
    let normal = NormalDist::new(0.0, 1.0).unwrap();
    let mut y = Vec::with_capacity(x.len());
    for x_val in &x {
        y.push(normal.cdf(*x_val));
    }
    y
}
#[allow(unused)]
pub fn standard_gaussian_pdf(x: Vec<f64>) -> Vec<f64> {
    let mut y = Vec::with_capacity(x.len());
    for x_val in &x {
        y.push((-0.5 * x_val * x_val).exp() / (consts::SQRT_2PI));
    }
    y
}
#[allow(unused)]
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
#[allow(unused)]
pub fn factorial(n: u32) -> u32 {
    let mut f = 1;
    for i in 1..=n {
        f *= i;
    }
    f
}
#[allow(unused)]
pub fn parametric_line(t: Vec<f64>, a: f64, b: f64, x_0: f64, y_0: f64) -> (Vec<f64>, Vec<f64>) {
    let mut x = Vec::with_capacity(t.len());
    let mut y = Vec::with_capacity(t.len());
    for t_val in &t {
        x.push(a * t_val + x_0);
        y.push(b * t_val + y_0);
    }
    (x, y)
}
#[allow(unused)]
pub fn rmm_cc_payoff(prices: Vec<f64>, strike: f64, sigma: f64, tau: f64) -> (Vec<f64>, Vec<f64>) {
    let n = prices.len();
    let mut v = Vec::with_capacity(n);
    let d1 = d_one(prices.clone(), strike, sigma, tau);
    let d2 = d_two(prices.clone(), strike, sigma, tau);
    let normal = NormalDist::new(0.0, 1.0).unwrap();
    for i in 0..n {
        let g = (1.0 - normal.cdf(d1[i])) * prices[i] + strike * normal.cdf(d2[i]);
        v.push(g);
    }
    (prices, v)
}
#[allow(unused)]
pub fn rmm_pp_payoff(prices: Vec<f64>, strike: f64, sigma: f64, rate: f64) -> (Vec<f64>, Vec<f64>) {
    let ell = 2.0 * rate / (2.0 * rate + sigma.powi(2)) * strike;
    (
        prices.clone(),
        prices
            .iter()
            .map(|price| {
                if price <= &ell {
                    strike - price
                } else {
                    (strike - ell) * (ell / price).powf(2.0 * rate / sigma.powi(2))
                }
            })
            .collect::<Vec<f64>>(),
    )
}
#[allow(unused)]
pub fn forced_rebalance(
    reserves: Vec<f64>,
    strike: f64,
    sigma: f64,
    tau: f64,
    ratio: f64,
    inv: f64,
) -> (Vec<f64>, Vec<f64>) {
    let normal = NormalDist::new(0.0, 1.0).unwrap();
    (
        reserves.clone(),
        reserves
            .iter()
            .map(|reserve| {
                (ratio * reserve - inv) / strike
                    - normal.cdf(normal.inverse_cdf(1.0 - reserve) - sigma * tau.sqrt())
            })
            .collect::<Vec<f64>>(),
    )
}

#[allow(unused)]
pub fn g3m_trading_curve(x_values: Vec<f64>, w: f64, l: f64) -> (Vec<f64>, Vec<f64>) {
    let n = x_values.len();
    let (x, mut y) = (x_values, Vec::with_capacity(n));
    for i in 0..n {
        let pow = 1_f64 / (1_f64 - w);
        y.push((l / x[i].powf(w)).powf(pow));
    }
    (x, y)
}
