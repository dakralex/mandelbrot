use std::fmt::{Display, Formatter};

/// A struct that represents an image.
#[derive(Default)]
pub struct Image {
    pub(crate) width: usize,
    pub(crate) height: usize,
    data: Vec<Pixel>,
}

impl Image {
    /// Create a new `Image` with the given `width` and `height`.
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Pixel::default(); width * height],
        }
    }

    /// Return an immutable reference to a [`Pixel`] at a given `x` and `y` position.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.data.get(y * self.width + x)
    }

    /// Return a mutable reference to a [`Pixel`] at a given `x` and `y` position.
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        self.data.get_mut(y * self.width + x)
    }

    /// Return the count of mandelbrot pixels in the [`Image`], that are the
    /// [`Pixel`] elements that have another value than the color white.
    #[inline]
    pub fn get_mandelbrot_pixels(&self) -> usize {
        self.data
            .iter()
            .filter(|pixel| pixel.is_mandelbrot())
            .count()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl Pixel {
    /// Create a new [`Pixel`] given the RGB values `r`, `g`, and `b`.
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a new [`Pixel`] that is set to the color black.
    #[inline]
    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    /// Create a new [`Pixel`] that is set to the color white.
    #[inline]
    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    /// Check whether pixel is mandelbrot, that is it has a color other than
    /// white.
    #[inline]
    pub fn is_mandelbrot(&self) -> bool {
        self.r < 255 && self.g < 255 && self.b < 255
    }
}

#[cfg(test)]
mod tests {
    use crate::image::{Image, Pixel};

    #[test]
    fn pixels() {
        // test pixels and image
        let p1 = Pixel { r: 255, g: 0, b: 0 };
        let p2 = p1;

        println!("{}", p1); // will work only if copy and display traits are implemented

        assert_eq!(p1, p2); // will fail unless traits are set

        let test_pixel_str = format!("{}", p1); // display trait required

        assert_eq!(test_pixel_str, String::from("255 0 0")); // display trait required

        let img1 = Image::new(100, 200);

        let element_0_0 = img1.get(0, 0).unwrap();

        assert_eq!(img1.width, 100);
        assert_eq!(img1.height, 200);
        assert_eq!(element_0_0, &Pixel { r: 0, g: 0, b: 0 });
        assert_eq!(img1.get(0, 1), Some(&Pixel { r: 0, g: 0, b: 0 }));

        let mut img2 = Image::new(200, 100);

        *img2.get_mut(0, 0).unwrap() = Pixel { r: 1, g: 0, b: 0 };

        let element_0_0_mut = img2.get_mut(0, 0).unwrap();

        assert_eq!(element_0_0_mut, &Pixel { r: 1, g: 0, b: 0 });
    }
}
