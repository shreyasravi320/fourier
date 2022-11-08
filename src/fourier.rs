use crate::constants::*;

pub fn dft(signal: Vec<f64>) -> Vec<[f64; 2]>
{
    let mut transform: Vec<[f64; 2]> = vec![[0.0, 0.0]; signal.len()];

    // [Re, Im]
    let mut x: [f64; 2] = [0.0, 0.0];

    for i in 0..signal.len()
    {
        for j in 0..signal.len()
        {
            let phi: f64 = 2.0 * PI * i as f64 * j as f64 / signal.len() as f64;
            x[0] += signal[j] * f64::cos(phi);
            x[1] -= signal[j] * f64::sin(phi);
        }

        transform[i] = x;
    }

    return transform;
}