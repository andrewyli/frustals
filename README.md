frustals
========

A library for fractals made by iteration in Rust.
This will use polynomial iteration with seeds to color a picture. Simply choose your polynomial, specify parameters for image size and graphing window, and edit color scheme. See test ```mandlebrot``` for more information.

```cargo test``` will run all tests, generating a BMP of the Mandelbrot set.
Make sure the path where the BMP is to be written to is changed to the desired path
(ln 82 of complex_iterator.rs, it is currently set to ```/home/andrew/Downloads/test1.bmp```)

Pictures:

![Picture 1](https://raw.github.com/EchoAce/frustals/master/test1.bmp)
![Picture 2](https://raw.github.com/EchoAce/frustals/master/test2.bmp)
![Mandelbrot](https://raw.github.com/EchoAce/frustals/master/mandelbrot.bmp)
(tri-mandlebrot)
![Tribrot](https://raw.github.com/EchoAce/frustals/master/tribrot.bmp)
