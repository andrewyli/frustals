/// CPolynomial is a complex-valued polynomial with real (f64) coefficients

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

        // uses Horner's rule for fast evaluation
        for &i in self.coeff.iter().rev() {
            c = (c + Complex::new(i, 0f64)) * x;
        }
        c / x // cannot have x == 0 + 0i, for good reason
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

    #[inline]
    pub fn mul(&self, other: &CPolynomial) -> CPolynomial {
        let mut new_coeff: Vec<f64> = Vec::new();
        //to multiply polynomials, add the exponents and multiply coefficients
        //note: resulting polynimial will have a higher degree, but this struct
        // doesnt keep track of degrees?
        let bigger_len = max(self.cef.len(), other.coef.len());
        let smaller_len = min(self.cef.len(), other.coef.len());

        for i in 0..smaller_len as usize {
            new_coeff.push(self.coeff[i] * other.coeff[i]);
        }
        for i in smaller_len..bigger_len as usize{
            new_coeff.push(0 as f64);
        }
        
        CPolynomial::new(new_coeff) 
    }

    //TODO: divide polynomials
    
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

    #[test]
    fn op_test() {
        let p1 = CPolynomial::new(vec!(0f64, 1f64, 2f64, 7f64));
        let p2 = CPolynomial::new(vec!(4f64, 4f64, 4f64, 3f64));
        assert_eq!(p1.add(&p2).coeff, vec!(4f64, 5f64, 6f64, 10f64));
        assert_eq!(p1.sub(&p2).coeff, vec!(-4f64, -3f64, -2f64, 4f64));
    }
    #[test]
    fn mul_test() {
        let p1 = CPolynomial::new(vec!(0f64, 1f64, 2f64, 7f64));
        let p2 = CPolynomial::new(vec!(4f64, 4f64, 4f64, 3f64));

        assert_eq!(p1.mul(&p2).coeff, vec!(0f64, 4f64, 8f64, 21f64));

        let p1 = CPolynomial::new(vec!(1f64, 2f64, 3f64));
        let p2 = CPolynomial::new(vec!(1f64, 1f64, 1f64, 1f64, 1f64));

        assert_eq!(p1.mul(&p2).coeff, vec!(1f64, 2f64, 3f64, 0f64, 0f64));
    }

    // TODO tests for divide


}
