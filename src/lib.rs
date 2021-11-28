use std::{ fmt, ops, cmp::Ordering };

#[derive(PartialEq, Clone, Copy, Debug)]
/// Complex Number
pub struct C64 {
    re: f64,
    im: f64
}

impl C64 {
    pub fn new<T, U>(a: T, b: U) -> C64
        where f64: From<T>,
              f64: From<U>
    {   
        C64 {
            re: f64::from(a),
            im: f64::from(b)
        }
    }
}

impl fmt::Display for C64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

impl ops::Add for C64 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Self {
            re: self.re + other.re,
            im: self.im + other.im
        }
    }
}

impl ops::Sub for C64 {
    type Output = C64;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            re: self.re - other.re,
            im: self.im - other.im
        }
    }
}

impl ops::Mul for C64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            re: subtract_f64(multiply_f64(self.re, other.re), multiply_f64(self.im, other.im)),
            im: add_f64(multiply_f64(self.re, other.im), multiply_f64(self.im, other.re))
        }
    }
}

impl ops::Div for C64 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let numerator = self * Self::new(other.re, -other.im);
        let denominator = other.re.powi(2) + other.im.powi(2);
        
        // Uses precision of five decimal places
        Self {
            re: (numerator.re / denominator * 100000.0).round() / 100000.0,
            im: (numerator.im / denominator * 100000.0).round() / 100000.0
        }
    }
}

/// Takes an f64 as input and returns its number of decimal places
fn decimal_places(num: f64) -> usize {
    if num.fract() == 0.0 { return 0 }
    
    let num_str = num.to_string();
    let split_str: Vec<&str> = num_str.split('.').collect();
    split_str[1].len()
}

/// Adds two f64 values with *supreme exactitude*
fn add_f64(a: f64, b: f64) -> f64 {
    let a_dec = decimal_places(a) as u32;
    let a_shifted = a * 10_u64.pow(a_dec) as f64;
    let b_dec = decimal_places(b) as u32;
    let b_shifted = b * 10_u64.pow(b_dec) as f64;
    match a_dec.cmp(&b_dec) {
        Ordering::Less => (a_shifted * 10_u64.pow(b_dec - a_dec) as f64 + b_shifted) / 10_u64.pow(b_dec) as f64,
        Ordering::Equal => (a_shifted + b_shifted) / 10_u64.pow(a_dec) as f64,
        Ordering::Greater => (a_shifted + b_shifted * 10_u64.pow(a_dec - b_dec) as f64) / 10_u64.pow(a_dec) as f64
    }
}

/// Subtracts two f64 values with *supreme exactitude*
fn subtract_f64(a: f64, b: f64) -> f64 {
    add_f64(a, -b)
}

/// Multiplies two f64 values with *supreme exactitude*
fn multiply_f64(a: f64, b: f64) -> f64 {
    let a_dec = decimal_places(a) as u32;
    let a_shifted = a * 10_u64.pow(a_dec) as f64;
    let b_dec = decimal_places(b) as u32;
    let b_shifted = b * 10_u64.pow(b_dec) as f64;
    let downshift = 10_u64.pow(a_dec + b_dec) as f64;
    (a_shifted.trunc() * b_shifted.trunc()) / downshift
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    /// Tests proper implementation of Add for C64
    fn adds_c64() {        
        assert_eq!(
            C64::new(1, 2) + C64::new(5, 8), 
            C64::new(6, 10)
        );

        assert_eq!(
            C64::new(5, -22) + C64::new(5, 8), 
            C64::new(10, -14)
        );
        assert_eq!(
            C64::new(-180, -42) + C64::new(-51, -82), 
            C64::new(-231, -124)
        );

        assert_eq!(
            C64::new(1.86, 2) + C64::new(5, 8.789), 
            C64::new(6.86, 10.789)
        );

        assert_eq!(
            C64::new(-546.3, -4002.57) + C64::new(5.5464, 8.21), 
            C64::new(-540.7536, -3994.36)
        );
    }

    #[test]
    /// Tests proper implementation of Sub for C64
    fn subtracts_c64() {        
        assert_eq!(
            C64::new(1, 2) - C64::new(5, 8), 
            C64::new(-4, -6)
        );

        assert_eq!(
            C64::new(5, -22) - C64::new(5, 8), 
            C64::new(0, -30)
        );
        assert_eq!(
            C64::new(-180, -42) - C64::new(-51, -82), 
            C64::new(-129, 40)
        );

        assert_eq!(
            C64::new(1.02, 2) - C64::new(5, 8.789), 
            C64::new(-3.98, -6.789)
        );

        assert_eq!(
            C64::new(-546.3, -4002.57) - C64::new(5.546, 8.21), 
            C64::new(-551.846, -4010.78)
        );
    }

    #[test]
    /// Tests proper implementation of Sub for C64
    fn multiplies_c64() {        
        assert_eq!(C64::new(1, 2) * C64::new(5, 8), C64::new(-11, 18));
        assert_eq!(C64::new(5, -22) * C64::new(5, 8), C64::new(201, -70));
        assert_eq!(C64::new(-180, -42) * C64::new(-51, -82), C64::new(5736, 16902));
        assert_eq!(C64::new(1.3, 2) * C64::new(-5, 8.7), C64::new(-23.9, 1.31));
        assert_eq!(C64::new(-546.3, -4002.57) * C64::new(5.546, 8.21), C64::new(29831.3199, -26683.37622));
    }

    #[test]
    /// Tests proper implementation of Div for C64 to five decimal places
    fn divides_c64() {
        assert_eq!(C64::new(1, 2) / C64::new(5, 8), C64::new(0.23596, 0.02247));
        assert_eq!(C64::new(5, -22) / C64::new(5, 8), C64::new(-1.69663, -1.68539));
        assert_eq!(C64::new(-180, -42) / C64::new(-51, -82), C64::new(1.35378, -1.35314));
        assert_eq!(C64::new(1.3, 2) / C64::new(-5, 8.7), C64::new(0.10825, -0.21164));
        assert_eq!(C64::new(-546.3, -4002.57) / C64::new(5.546, 8.21), C64::new(-365.62825, -180.44754));
    }
    
    #[test]
    /// Tests decimal_places
    fn calculates_decimal_places() {
        assert_eq!(decimal_places(2.93), 2);
        assert_eq!(decimal_places(2.123456789), 9);
        assert_eq!(decimal_places(28489.502), 3);
        assert_eq!(decimal_places(0.93), 2);
        assert_eq!(decimal_places(2.0), 0);
        assert_eq!(decimal_places(0.0), 0);
        assert_eq!(decimal_places(-546.3), 1);
        assert_eq!(decimal_places(5.546), 3);
        assert_eq!(decimal_places(-4002.57), 2);
        assert_eq!(decimal_places(8.21), 2);
    }

    #[test]
    /// Tests add_f64
    fn adds_f64() {
        assert_eq!(add_f64(137.2901, 738.298), 875.5881);
        assert_eq!(add_f64(5.05, 6.05), 11.1);
        assert_eq!(add_f64(-5.05, 6.05), 1.0);
        assert_eq!(add_f64(5.05, -6.05), -1.0);
        assert_eq!(add_f64(-5.05, -6.05), -11.1);
        assert_eq!(add_f64(2.02, 1.7), 3.72);
        assert_eq!(add_f64(137.2901, 0.0), 137.2901);
        assert_eq!(add_f64(0.0, 738.298), 738.298);
        assert_eq!(add_f64(0.0, 0.0), 0.0);
    }

    #[test]
    /// Tests add_f64
    fn subtracts_f64() {
        assert_eq!(subtract_f64(137.2901, 738.298), -601.0079);
        assert_eq!(subtract_f64(5.05, 6.05), -1.0);
        assert_eq!(subtract_f64(-5.05, 6.05), -11.1);
        assert_eq!(subtract_f64(5.05, -6.05), 11.1);
        assert_eq!(subtract_f64(-5.05, -6.05), 1.0);
        assert_eq!(subtract_f64(2.02, 1.7), 0.32);
        assert_eq!(subtract_f64(137.2901, 0.0), 137.2901);
        assert_eq!(subtract_f64(0.0, 738.298), -738.298);
        assert_eq!(subtract_f64(0.0, 0.0), 0.0);
        assert_eq!(subtract_f64(-3029.7798, -32861.0997), 29831.3199);
    }

    #[test]
    /// Tests multiply_f64
    fn multiplies_f64() {
        assert_eq!(multiply_f64(2.0, 5.0), 10.0);
        assert_eq!(multiply_f64(2.56, 5.82), 14.8992);
        assert_eq!(multiply_f64(2.2, 1.2), 2.64);
        assert_eq!(multiply_f64(4.5, 0.0), 0.0);
        assert_eq!(multiply_f64(0.0, 8.2), 0.0);
        assert_eq!(multiply_f64(0.0, -8.2), 0.0);
        assert_eq!(multiply_f64(-0.0, 8.2), 0.0);
        assert_eq!(multiply_f64(-3.0, 5.2), -15.6);
        assert_eq!(multiply_f64(3.0, -5.2), -15.6);
        assert_eq!(multiply_f64(-3.0, -5.2), 15.6);
        assert_eq!(multiply_f64(3.0, 5.2), 15.6);
        assert_eq!(multiply_f64(-546.3, 5.546), -3029.7798);
        assert_eq!(multiply_f64(-4002.57, 8.21), -32861.0997);
    }

}
