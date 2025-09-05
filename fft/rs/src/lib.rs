use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            real: self.real * scalar,
            imag: self.imag * scalar,
        }
    }
}
pub fn fft(arr: &mut [Complex]) {
    fn _fft(arr: &mut [Complex], buf: &mut [Complex]) {
        let n = arr.len();
        if n == 1 {
            return;
        }

        // 只用 buf[..n/2]
        for i in 0..n / 2 {
            buf[i] = arr[2 * i]; // 偶数
            arr[i] = arr[2 * i + 1]; // 奇数
        }
        arr[n / 2..].copy_from_slice(&buf[..n / 2]);

        let (a0, a1) = arr.split_at_mut(n / 2);
        _fft(a0, &mut buf[..n / 2]);
        _fft(a1, &mut buf[..n / 2]);

        let ang = -2.0 * PI / n as f64;
        let mut w = Complex::new(1.0, 0.0);
        let wn = Complex::new(ang.cos(), ang.sin());

        for i in 0..n / 2 {
            let p = a0[i];
            let q = w * a1[i];
            a0[i] = p + q;
            a1[i] = p - q;
            w = w * wn;
        }
    }

    let mut buf = vec![Complex::new(0., 0.); arr.len() / 2];
    _fft(arr, &mut buf);
    let factor = 1.0 / (arr.len() as f64).sqrt();
    for it in arr {
        *it = *it * factor;
    }
}
