use crate::constants::*;

pub struct Complex
{
    re: f64,
    im: f64
}

#[allow(dead_code)]
impl Complex
{
    pub fn new(re: f64, im: f64) -> Self
    {
        return Complex{ re: re, im: im };
    }

    pub fn re(&mut self) -> f64
    {
        return self.re;
    }

    pub fn im(&mut self) -> f64
    {
        return self.im;
    }

    pub fn add(&mut self, other: &mut Complex)
    {
        self.re += other.re;
        self.im += other.im;
    }

    pub fn mul(&mut self, other: &mut Complex)
    {
        // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        let re: f64 = self.re * other.re - self.im * other.im;
        let im: f64 = self.re * other.im + self.im * other.re;

        self.re = re;
        self.im = im;
    }

    pub fn div_fl(&mut self, divisor: f64)
    {
        self.re /= divisor;
        self.im /= divisor;
    }

    pub fn mag(&mut self) -> f64
    {
        return f64::sqrt(self.re * self.re + self.im * self.im);
    }

    pub fn atan(&mut self) -> f64
    {
        return f64::atan2(self.im, self.re);
    }
}

pub fn dft(signal: &mut Vec<Complex>) -> Vec<[f64; 3]>
{
    let mut transform: Vec<[f64; 3]> = vec![[0.0; 3]; signal.len()];

    // [freq, amp, phase]
    let mut x: [f64; 3] = [0.0; 3];

    for i in 0..signal.len()
    {
        let mut sum: Complex = Complex::new(0.0, 0.0);

        for j in 0..signal.len()
        {
            let phi: f64 = 2.0 * PI * i as f64 * j as f64 / signal.len() as f64;
            signal[j].mul(&mut Complex::new(f64::cos(phi), -f64::sin(phi)));
            sum.add(&mut signal[j]);
        }

        sum.div_fl(signal.len() as f64);

        x[0] = i as f64;
        x[1] = sum.mag();
        x[2] = sum.atan();

        transform[i] = x;
    }

    transform.sort_by(|a, b| std::cmp::min(a[1], b[1]));

    return transform;
}