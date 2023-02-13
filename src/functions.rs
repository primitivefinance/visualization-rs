use itertools_num::linspace;
use rand_distr::{Normal,Distribution};
use statrs::distribution::{Normal as NormalDist,ContinuousCDF};

pub fn sample_normal(mean: f64, n: usize) -> Vec<f64>{
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(f64::from(normal.sample(&mut rng)));
    }
    v
}

pub fn d_one(x: Vec<f64>, K: f64, sigma: f64, tau: f64) -> Vec<f64> {
    let n = x.len();
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push((x[i]/K).ln()/(sigma*tau.sqrt()) + 0.5*sigma*tau.sqrt());
    }
    v
}

pub fn d_two(x: Vec<f64>, K: f64, sigma: f64, tau: f64) -> Vec<f64> {
    let n = x.len();
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push((x[i]/K).ln()/(sigma*tau.sqrt()) - 0.5*sigma*tau.sqrt());
    }
    v
}

pub fn rmm_trading_curve(prices: Vec<f64>, K: f64, sigma: f64, tau: f64) -> (Vec<f64>,Vec<f64>) {
    let n = prices.len();
    let (mut x,mut y) = (Vec::with_capacity(n),Vec::with_capacity(n));
    let d1 = d_one(prices.clone(), K, sigma, tau);
    let d2 = d_two(prices.clone(), K, sigma, tau);
    let normal = NormalDist::new(0.0, 1.0).unwrap();
    for i in 0..n {
        x.push(1.0 - normal.cdf(d1[i]));
        y.push(K*normal.cdf(d2[i]));
    }
    (x,y)
}
