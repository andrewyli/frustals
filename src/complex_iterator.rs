use std::cmp;
use num::Complex;
use cpolynomial::CPolynomial;
use bmp;

/// xbound represents the bounds of the picture to be created, for x = l, r, u, d.
/// ubound > lbound, rbound > lbound
/// cpoly is the polynomial to be iterated on, img_l and img_w dictate the size
/// of the array/img.
/// iter_count is the number of iterations to perform before completion.
pub fn generate_val_arr(lbound: f64, rbound: f64, dbound: f64, ubound: f64,
                        cpoly: &CPolynomial, img_l: u64, img_w: u64,
                        iter_count: u64) -> Result<Vec<Vec<isize>>, &str> {
    assert!(rbound > lbound); assert!(ubound > dbound);
    let horizontal_increment: f64 = (rbound - lbound) / (img_l as f64);
    let vertical_increment: f64 = (ubound - dbound) / (img_w as f64);

    let mut out = Vec::new();

    let mut it_v: f64 = ubound;
    loop { // over the y-axis
        let mut it_h: f64 = lbound;
        let mut part = Vec::new();
        loop { // over the x-axis
            let constant = Complex::new(it_h, it_v); // c of x^2 + c
            let mut seed = Complex::new(0f64, 0f64);
            let mut i = 0;
            for _j in 0..iter_count { // iterate through polynomial iter_count times
                seed = cpoly.eval(seed) + constant;
                if seed.norm() > 2f64 {
                    break
                }
                i += 1;
            }
            part.push(i); // push result to sub/row-vector

            it_h += horizontal_increment;
            if it_h - rbound + horizontal_increment >= 0f64 { break }
        }
        out.push(part);
        it_v -= vertical_increment;
        if it_v - dbound - vertical_increment <= 0f64 { break }
    }
    Ok(out)
}

/// x-base: the starting value, default should be 255
/// x-mod: multiplier of v[x][y], changes rate of color change.
/// inverted if true will invert all RGB pixel values

pub fn make_bmp(v: Vec<Vec<isize>>,
                rbase: isize, gbase: isize, bbase: isize,
                rmod: isize, gmod: isize, bmod: isize,
                inverted: bool) -> bmp::Image {

    // create img array of pixels (bmp format)
    assert!(v.len() > 0);
    let mut img = bmp::Image::new(v.len() as u32, v[0].len() as u32);

    let modifier = match inverted {
        true => 1,
        false => 0,
    };

    // iterate and calculate rgb values for all escape periods
    for (a, b) in img.coordinates() {
        let x = a as usize;
        let y = b as usize;

        // inverted will either be 1 or 0, so the 255 * modifier term is optional
        img.set_pixel(a, b, bmp::Pixel {
            r: (255 * modifier - (rbase - v[x][y]) * rmod).abs() as u8,
            b: (255 * modifier - (bbase - v[x][y]) * bmod).abs() as u8,
            g: (255 * modifier - (gbase - v[x][y]) * gmod).abs() as u8,
        })
    }
    img
}

#[cfg(test)]
mod test {
    use cpolynomial::CPolynomial;
    use complex_iterator;

    #[test]
    fn array_gen() {
        let vec = vec!(0f64, 0f64, 2f64);
        let poly = CPolynomial::new(vec);
        let arr = match complex_iterator::generate_val_arr(
            -5f64, 5f64, -5f64, 5f64, &poly,
            20u64, 20u64, 255u64) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };
        assert_eq!(arr[0][0], 0);
    }

    #[test]
    fn mandelbrot() {
        let vec = vec!(0f64, 0f64, 1f64);
        let poly = CPolynomial::new(vec);
        let arr = match complex_iterator::generate_val_arr(
            -2.5f64, 0.5f64, -1.5f64, 1.5f64, &poly,
            600u64, 600u64, 25u64) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        let img = complex_iterator::make_bmp(arr, 150isize, 50isize,
                                             255isize, 1isize, 7isize,
                                             4isize, false);
        img.save("/home/andrew/Downloads/mandelbrot.bmp");
    }

    #[test]
    fn tribrot() {
        let vec = vec!(0f64, 0f64, 0f64, 1f64);
        let poly = CPolynomial::new(vec);
        let arr = match complex_iterator::generate_val_arr(
            -1.5f64, 1.5f64, -1.5f64, 1.5f64, &poly,
            600u64, 600u64, 25u64) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        let img = complex_iterator::make_bmp(arr, 150isize, 200isize,
                                             100isize, 10isize, 2isize,
                                             4isize, false);
        img.save("/home/andrew/Downloads/tribrot.bmp");
    }

    #[test]
    fn something() {
        let vec = vec!(2f64, -1f64, 0f64, 1f64);
        let poly = CPolynomial::new(vec);
        let arr = match complex_iterator::generate_val_arr(
            -1.5f64, 1.5f64, -1.5f64, 1.5f64, &poly,
            600u64, 600u64, 50u64) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };
        let img = complex_iterator::make_bmp(arr, 0isize, 0isize,
                                             0isize, -10isize, -2isize,
                                             -4isize, true);
        img.save("/home/andrew/Downloads/something.bmp");
    }
}
