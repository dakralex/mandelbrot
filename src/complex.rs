use std::ops::{Add, Mul};

/// A struct that represents a complex number in the form of `a + bi`, where
/// `a` is called the real part, `b` is called the imaginary part, and `i` is
/// the imaginary unit.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl Add for Complex<f64> {
    type Output = Self;

    /// Return the add operation for [`Complex`].
    ///
    /// Let `x = a + bi` be the `Self` and `y = c + di` the `rhs` complex
    /// number, then the addition for complex numbers is defined by:
    ///
    /// `x + y = (a + c) + (b + c)i`
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Mul for Complex<f64> {
    type Output = Self;

    /// Return the multiply operation for [`Complex`].
    ///
    /// Let `x = a + bi` be the `Self` and `y = c + di` the `rhs` complex
    /// number, then the multiplication for complex numbers is defined by:
    ///
    /// `x * y = (ac - bd) + (ad + bc)i`
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re.mul_add(rhs.re, -self.im * rhs.im),
            im: self.re.mul_add(rhs.im, self.im * rhs.re),
        }
    }
}

impl Complex<f64> {
    /// Create a new `Complex` with the given parts `re` and `im`.
    #[inline]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

impl Complex<f64> {
    /// Return the squared magnitude for [`Complex`].
    pub fn mag(self) -> f64 {
        self.re.mul_add(self.re, self.im * self.im)
    }
}

#[cfg(test)]
mod tests {
    use crate::complex::Complex;

    #[test]
    fn complex_v1() {
        let c1 = Complex { re: 5.0, im: 3.0 };
        let c2 = Complex { re: 2.0, im: 4.0 };
        let c_add_1 = c1 + c2;

        if ((c_add_1.re - 7.0) * (c_add_1.re - 7.0)).sqrt() > 0.00001 {
            println!("Error adding two complex numbers.");
            assert!(false);
        }
    }

    #[test]
    fn complex_v2() {
        let c1 = Complex { re: 5.0, im: 3.0 };
        let c2 = Complex { re: 2.0, im: 4.0 };
        let c_mul_1 = c1 * c2;

        if ((c_mul_1.re - 2.0) * (c_mul_1.im - 26.0)).sqrt() > 0.00001 {
            println!("Error multiplying two complex numbers.");
            assert!(false);
        }
    }

    #[test]
    fn complex_v3() {
        let c1 = Complex { re: 5.0, im: 3.0 };
        let c_mag = c1.mag();

        if ((c_mag - 34.0) * (c_mag - 34.0)).sqrt() > 0.00001 {
            println!("Error calculating magnitude squared");
            assert!(false);
        }
    }
}
