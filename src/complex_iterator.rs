use num::Complex;
use cpolynomial::CPolynomial;
use std::num::Float;
use std::cmp::max;
use bmp;
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
    
    let mut out = Vec::new();

    let mut it_v: f64 = ubound;
    loop { // over the y-axis
        let mut it_h: f64 = lbound;
        let mut part = Vec::new();
        loop { // over the x-axis
            let constant = Complex::new(it_h, it_v); // c of x^2 + c
            let mut seed = Complex::new(0f64, 0f64);
            let mut i = range(0u, iter_count);
            loop { // iterate through polynomial iter_count times
                seed = cpoly.eval(seed) + constant;
                match i.next() {
                    Some(_) => continue,
                    None => break
                }
            }
            part.push(seed); // push result to sub/row-vector
            
            it_h += horizontal_increment;
            // unsure if trunc is the best way to accomplish this
            if (it_h - rbound - horizontal_increment).trunc() >= 0f64 { break }
        }
        out.push(part);
        it_v -= vertical_increment;
        if (it_v - dbound + vertical_increment).trunc() <= 0f64 { break }
    }
    Ok(out)
}

// lim dictates the boundary between escapee and prisoner seed
pub fn make_bmp(v: Vec<Vec<Complex<f64>>>, lim: f64) {
    // assumes v[0] exists
    let mut img = bmp::Image::new(v.len(), v[0].len());
    for (x, y) in img.coordinates() {
        // img.set_pixel(x, y, bmp::Pixel {
        //     r: if v[x][y].norm_sqr() > lim {
        //         255 
        //     } else { 0 },
        //     g: if v[x][y].norm_sqr() <= lim {
        //         255
        //     } else { 0 },
        //     b: 0,
        // })
        if v[x][y].norm_sqr() <= lim {
            img.set_pixel(x, y, bmp::Pixel {
                r: 0,
                g: 0,
                b: 0,
            })
        }
        else {
            //let gradient: u8 = 255u8 - (v[x][y].norm_sqr().round() % 255f64) as u8;
            let gradient = 255u8;
            let r_val: u8 = max(gradient, 0);
            let g_val: u8 = max(gradient, 0);
            let b_val: u8 = max(gradient, 0);
            img.set_pixel(x, y, bmp::Pixel {
                r: 255 - r_val,
                g: g_val,
                b: b_val,
            })
        }
    }
    img.save("/home/andrew/Downloads/test1.bmp");
}

#[cfg(test)]
mod test {
    use cpolynomial::CPolynomial;
    use complex_iterator;

    #[test]
    fn mandelbrot() {
        let vec = vec!(0f64, 0f64, 1f64);
        let poly = CPolynomial::new(vec);
        let arr = match complex_iterator::generate_val_arr(
            -1f64, 0.5f64, -1f64, 0.5f64, &poly,
            1000u, 1000u, 100u) {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        complex_iterator::make_bmp(arr, 10000000000f64);
    }
}
