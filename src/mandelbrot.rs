use crate::complex::Complex;
use crate::image::{Image, Pixel};
use std::ops::{Add, Mul};

/// Return a new [`Image`] with the input dimensions `width` and `height`,
/// which portrays a mandelbrot plot with a maximum of `max_iter` iterations
/// for testing set membership.
pub fn generate_image(width: usize, height: usize, max_iter: usize) -> Image {
    let mut image = Image::new(width, height);

    for y in 0..height {
        for x in 0..width {
            // Map the current pixel to coordinates in the complex plane
            let cx = (x as f64 / width as f64 - 0.75) * 3.5;
            let cy = (y as f64 / height as f64 - 0.5) * 2.0;
            let coord = Complex::new(cx, cy);

            let pixel = image.get_mut(x, y).unwrap();

            *pixel = match check_pixel(coord, max_iter) {
                Some(_) => Pixel::white(),
                None => Pixel::black(),
            }
        }
    }

    image
}

/// Check if the complex number `c` belongs to the Mandelbrot set.
pub fn check_pixel(c: Complex<f64>, max_iter: usize) -> Option<usize> {
    let mut z = Complex::default();

    for i in 0..max_iter {
        if z.mag() > 4.0 {
            return Some(i);
        }

        z = z.mul(z).add(c);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::complex::Complex;
    use crate::{client, mandelbrot};

    #[test]
    fn check_pixel_v1() {
        let c = Complex {
            re: -0.875,
            im: 0.0,
        };

        let pixel_is_in_the_set = mandelbrot::check_pixel(c, 1024);

        assert_eq!(pixel_is_in_the_set, None);
    }

    #[test]
    fn check_pixel_v2() {
        let c = Complex { re: -0.1, im: 0.9 };

        let pixel_is_in_the_set = mandelbrot::check_pixel(c, 1024);

        assert_eq!(pixel_is_in_the_set.is_some(), true);
    }

    #[test]
    fn mandelbrot_generate_and_test_pixel_count_v1() {
        let image = mandelbrot::generate_image(525, 300, 1024);

        let pixels_in_the_set = image.get_mandelbrot_pixels();
        println!("Pixels in the set: {}", pixels_in_the_set);

        assert_eq!(pixels_in_the_set, 34062);

        client::save_to_file(&image, "test1.ppm");
    }

    #[test]
    fn mandelbrot_generate_and_test_pixel_count_v2() {
        let image = mandelbrot::generate_image(300, 525, 1024);

        let pixels_in_the_set = image.get_mandelbrot_pixels();
        println!("Pixels in the set: {}", pixels_in_the_set);

        assert_eq!(pixels_in_the_set, 33965);

        client::save_to_file(&image, "test2.ppm");
    }
}
