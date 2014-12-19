//NOTE: This is essentially directly copied off of 0.11 Rust's num::complex, I don't claim any credit whatsoever for any code in this file.

/// A complex number in Cartesian form.
pub struct Complex<F64> {
    /// Real portion of the complex number
    pub re: f64,
    /// Imaginary portion of the complex number
    pub im: f64
}

pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;

impl Complex<f64> {
    /// Create a new Complex
    #[inline]
    pub fn new(re: f64, im: f64) -> Complex<f64> {
        Complex { re: re, im: im }
    }

    /**
    Returns the square of the norm (since `T` doesn't necessarily
    have a sqrt function), i.e. `re^2 + im^2`.
    */
    #[inline]
    pub fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }


    /// Returns the complex conjugate. i.e. `re - i im`
    #[inline]
    pub fn conj(&self) -> Complex<f64> {
        Complex::new(self.re.clone(), -self.im)
    }


    /// Multiplies `self` by the scalar `t`.
    #[inline]
    pub fn scale(&self, t: f64) -> Complex<f64> {
        Complex::new(self.re * t, self.im * t)
    }

    /// Divides `self` by the scalar `t`.
    #[inline]
    pub fn unscale(&self, t: f64) -> Complex<f64> {
        Complex::new(self.re / t, self.im / t)
    }

    /// Returns `1/self`
    #[inline]
    pub fn inv(&self) -> Complex<f64> {
        let norm_sqr = self.norm_sqr();
        Complex::new(self.re / norm_sqr,
                    -self.im / norm_sqr)
    }

    pub fn pow(&self, mut deg: uint) -> Complex<f64> {
        let mut out = Complex::new(1f64, 0f64);
        while deg > 0 {
            out = out.mul(self);
            deg = deg - 1;
        }
        out
    }
}

/* arithmetic */
// (a + i b) + (c + i d) == (a + c) + i (b + d)
impl Add<Complex<f64>, Complex<f64>> for Complex<f64> {
    #[inline]
    fn add(&self, other: &Complex<f64>) -> Complex<f64> {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}
// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl Sub<Complex<f64>, Complex<f64>> for Complex<f64> {
    #[inline]
    fn sub(&self, other: &Complex<f64>) -> Complex<f64> {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}
// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl Mul<Complex<f64>, Complex<f64>> for Complex<f64> {
    #[inline]
    fn mul(&self, other: &Complex<f64>) -> Complex<f64> {
        Complex::new(self.re*other.re - self.im*other.im,
                   self.re*other.im + self.im*other.re)
    }
}

// (a + i b) / (c + i d) == [(a + i b) * (c - i d)] / (c*c + d*d)
//   == [(a*c + b*d) / (c*c + d*d)] + i [(b*c - a*d) / (c*c + d*d)]
impl Div<Complex<f64>, Complex<f64>> for Complex<f64> {
    #[inline]
    fn div(&self, other: &Complex<f64>) -> Complex<f64> {
        let norm_sqr = other.norm_sqr();
        Complex::new((self.re*other.re + self.im*other.im) / norm_sqr,
                   (self.im*other.re - self.re*other.im) / norm_sqr)
    }
}

impl Neg<Complex<f64>> for Complex<f64> {
    #[inline]
    fn neg(&self) -> Complex<f64> {
        Complex::new(-self.re, -self.im)
    }
}
