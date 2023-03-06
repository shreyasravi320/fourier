use crate::constants::*;
use std::collections::VecDeque;

pub fn dft(signal: &mut VecDeque<[f64; 2]>) -> Vec<[f64; 3]>
{
    let mut transform: Vec<[f64; 3]> = Vec::new();

    // [freq, amp, phase]
    let mut x: [f64; 3] = [0.0; 3];

    for i in 0..signal.len()
    {
        let mut sum: [f64; 2] = [0.0, 0.0];

        for j in 0..signal.len()
        {
            let phi: f64 = 2.0 * PI * i as f64 * j as f64 / signal.len() as f64;
            sum[0] += signal[j][0] * f64::cos(phi);
            sum[1] -= signal[j][1] * f64::sin(phi);
        }

        sum[0] /= signal.len() as f64;
        sum[1] /= signal.len() as f64;

        x[0] = i as f64;
        x[1] = f64::sqrt(sum[0] * sum[0] + sum[1] * sum[1]);
        x[2] = f64::atan2(sum[1], sum[0]);

        transform.push(x);
    }

    transform.sort_by(|a, b| b[1].partial_cmp(&a[1]).unwrap());

    return transform;
}