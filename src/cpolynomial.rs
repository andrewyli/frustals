/// CPolynomial is a complex-valued polynomial with real (f64) coefficients

/// To-do: add in mul and div
use num::Complex;

/// the first element of the vector should be the constant term!
pub struct CPolynomial {
    coeff: Vec<f64>
}

impl CPolynomial {
    pub fn new(v: Vec<f64>) -> CPolynomial {
        CPolynomial { coeff: v }
    }
    #[inline]
    pub fn eval(&self, x: Complex<f64>) -> Complex<f64> {
        if x.re == 0f64 && x.im == 0f64 { return Complex::new(self.coeff[0], 0f64) }
        let mut c = Complex::new(0f64, 0f64);
        // uses Horner's rule
        for &i in self.coeff.iter().rev() {
            c = (c + Complex::new(i, 0f64)) * x;
        }
        c / x // cannot have x == 0 + 0i
    }
    #[inline]
    pub fn add(&self, other: &CPolynomial) -> CPolynomial {
        let mut new_coeff: Vec<f64> = Vec::new();
        if self.coeff.len() < other.coeff.len() {
            for i in 0..self.coeff.len() as usize {
                new_coeff.push(self.coeff[i] + other.coeff[i]);
            }
            for i in self.coeff.len()..other.coeff.len() as usize {
                new_coeff.push(other.coeff[i]);
            }
        }
        else {
            for i in 0..other.coeff.len() as usize {
                new_coeff.push(self.coeff[i] + other.coeff[i]);
            }
            for i in other.coeff.len()..self.coeff.len() as usize {
                new_coeff.push(self.coeff[i]);
            }
        }
        CPolynomial::new(new_coeff)
    }
    #[inline]
    pub fn sub(&self, other: &CPolynomial) -> CPolynomial {
        let mut new_coeff: Vec<f64> = Vec::new();
        if self.coeff.len() < other.coeff.len() {
            for i in 0..self.coeff.len() as usize {
                new_coeff.push(self.coeff[i] - other.coeff[i]);
            }
            for i in self.coeff.len()..other.coeff.len() as usize{
                new_coeff.push(-1f64 * other.coeff[i]);
            }
        }
        else {
            for i in 0..other.coeff.len() as usize {
                new_coeff.push(self.coeff[i] - other.coeff[i]);
            }
            for i in other.coeff.len()..self.coeff.len() as usize {
                new_coeff.push(self.coeff[i]);
            }
        }
        CPolynomial::new(new_coeff)
    }

    // Multiplcation and Division to-do
}

#[cfg(test)]
mod test {
    use num::Complex;
    use cpolynomial::CPolynomial;

    #[test]
    fn eval_test() {
        let p1 = CPolynomial::new(vec!(0f64, 1f64, 2f64, 7f64));
        let p2 = CPolynomial::new(vec!(4f64, 4f64, 4f64, 3f64));
        assert_eq!(p1.eval(Complex::new(2.5f64, 0f64)).re, 124.375f64);
        assert_eq!(p1.eval(Complex::new(2.5f64, 0f64)).im, 0f64);
        assert_eq!(p2.eval(Complex::new(1f64, -2f64)).re, -37f64);
        assert_eq!(p2.eval(Complex::new(1f64, -2f64)).im, -18f64);
    }

    // need to include divide, multiply
    #[test]
    fn op_test() {
        let p1 = CPolynomial::new(vec!(0f64, 1f64, 2f64, 7f64));
        let p2 = CPolynomial::new(vec!(4f64, 4f64, 4f64, 3f64));
        assert_eq!(p1.add(&p2).coeff, vec!(4f64, 5f64, 6f64, 10f64));
        assert_eq!(p1.sub(&p2).coeff, vec!(-4f64, -3f64, -2f64, 4f64));
    }
}
