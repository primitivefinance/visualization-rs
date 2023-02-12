use itertools_num::linspace;
use rand_distr::{Normal,Distribution};

pub fn sample_normal<F>(mean: F, n: usize) -> Vec<F>
where
    F: From<f64>,
{
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(F::from(normal.sample(&mut rng)));
    }
    v
}

// pub fn standard_normal<F>(x: Vec<F>) -> Vec<F>
// where
//     F: From<f64>,
// {
//     let normaldist = Normal::new(0, 1)?;
//     let mut v = Vec::with_capacity(n);
//     for i in 0..n {
//         v.push(x[i] - mean);
//     }
//     let std = v.iter().map(|x| x * x).sum::<F>().sqrt();
//     for i in 0..n {
//         v[i] /= std;
//     }
//     v
// }
