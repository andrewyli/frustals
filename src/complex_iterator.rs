use num::Complex;
use cpolynomial::CPolynomial;
use std::num::Float;
// xbound represents the bounds of the picture to be created, for x = l, r, u, d.
// ubound > lbound, rbound > lbound
// cpoly is the polynomial to be iterated on, img_l and img_w dictate the size
// of the array/img.
// iter_count is the number of iterations to perform before completion.

pub fn generate_val_arr(lbound: f64, rbound: f64, dbound: f64, ubound: f64,
                        cpoly: &CPolynomial, img_l: uint, img_w: uint,
                        iter_count: uint) -> Result<Vec<Vec<Complex<f64>>>, &str> {
    assert!(rbound > lbound); assert!(ubound > dbound);
    let horizontal_increment: f64 = (rbound - lbound) / (img_l as f64);
    let vertical_increment: f64 = (ubound - dbound) / (img_w as f64);
    
    let mut it_h: f64 = lbound;
    let mut it_v: f64 = ubound;
    
    let mut out = Vec::new();

    loop {
        let mut part = Vec::new();
        loop {
            let mut seed = Complex::new(it_h, it_v);
            let mut i = range(0u, iter_count);
            loop {
                seed = cpoly.eval(seed);
                match i.next() {
                    Some(_) => continue,
                    None => {
                        break
                    }
                }
            }
            part.push(seed);
            
            it_h += horizontal_increment;
            // fix the comparison here!
            if (it_h - ubound - 1f64).round() <= 0.0001 { break }
        }
        out.push(part);
        it_v -= vertical_increment;
        if (it_v - lbound + 1f64).round() <= 0.0001 { break }
    }
    Ok(out)
}

#[cfg(test)]
mod test {
    use cpolynomial::CPolynomial;
    use complex_iterator;

    #[test]
    fn arr_gen_1() {
        let vec = vec!(3f64, 2.1f64, -1.4f64);
        let poly = CPolynomial::new(vec);
        let arr = match complex_iterator::generate_val_arr(
            -5f64, 5f64, -5f64, 5f64, &poly,
            20u, 20u, 1u) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        // WARNING: An NaN is generated here!
        for a in arr.iter() {
            for b in a.iter() {
                print!("{} ", b);
            }
        }
    }
}
