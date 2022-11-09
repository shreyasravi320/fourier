use crate::constants::*;

pub fn dft(signal: Vec<f64>) -> Vec<[f64; 3]>
{
    let mut transform: Vec<[f64; 3]> = vec![[0.0; 3]; signal.len()];

    // [freq, amp, phase]
    let mut x: [f64; 3] = [0.0; 3];

    for i in 0..signal.len()
    {
        let mut re: f64 = 0.0;
        let mut im: f64 = 0.0;

        for j in 0..signal.len()
        {
            let phi: f64 = 2.0 * PI * i as f64 * j as f64 / signal.len() as f64;
            re += signal[j] * f64::cos(phi);
            im -= signal[j] * f64::sin(phi);
        }

        re /= signal.len() as f64;
        im /= signal.len() as f64;

        x[0] = i as f64;
        x[1] = f64::sqrt(re * re + im * im);
        x[2] = f64::atan2(im, re);

        transform[i] = x;
    }

    for i in 0..transform.len()
    {
        println!("[ {} {} {} ]", transform[i][0], transform[i][1], transform[i][2]);
    }

    return transform;
}