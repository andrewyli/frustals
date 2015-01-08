use num::Complex;
use cpolynomial::CPolynomial;
use bmp;

/// xbound represents the bounds of the picture to be created, for x = l, r, u, d.
/// ubound > lbound, rbound > lbound
/// cpoly is the polynomial to be iterated on, img_l and img_w dictate the size
/// of the array/img.
/// iter_count is the number of iterations to perform before completion.
pub fn generate_val_arr(lbound: f64, rbound: f64, dbound: f64, ubound: f64,
                        cpoly: &CPolynomial, img_l: uint, img_w: uint,
                        iter_count: uint) -> Result<Vec<Vec<int>>, &str> {
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
            let mut i = 0i;
            for _j in range(0, iter_count) { // iterate through polynomial iter_count times
                seed = cpoly.eval(seed) + constant;
                if seed.norm() > 2f64 {
                    break
                }
                i += 1i;
            }
            part.push(i); // push result to sub/row-vector
            
            it_h += horizontal_increment;
            // unsure if trunc is the best way to accomplish this
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
pub fn make_bmp(v: Vec<Vec<int>>,
                rbase: u8, gbase: u8, bbase: u8,
                rmod: i8, gmod: i8, bmod: i8) -> bmp::Image {
    // assumes v[0] exists
    let mut img = bmp::Image::new(v.len(), v[0].len());
    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, bmp::Pixel {
            r: rbase - (v[x][y] as u8) * (rmod as u8),
            g: gbase- (v[x][y] as u8) * (gmod as u8),
            b: bbase- (v[x][y] as u8) * (bmod as u8),
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
            20u, 20u, 255u) {
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
            600u, 600u, 100u) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        let img = complex_iterator::make_bmp(arr, 255u8, 255u8, 255u8, 10i8, 7i8, 4i8);
        img.save("/home/andrew/Downloads/mandelbrot.bmp");
    }

    #[test]
    fn tribrot() {
        let vec = vec!(0f64, 0f64, 0f64, 1f64);
        let poly = CPolynomial::new(vec);
        let arr = match complex_iterator::generate_val_arr(
            -1.5f64, 1.5f64, -1.5f64, 1.5f64, &poly,
            600u, 600u, 100u) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        let img = complex_iterator::make_bmp(arr, 0u8, 0u8, 0u8, -10i8, -2i8, -4i8);
        img.save("/home/andrew/Downloads/tribrot.bmp");
    }
}
