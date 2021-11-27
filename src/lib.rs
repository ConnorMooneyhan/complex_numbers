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
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re
        }
    }
}

/// Takes an f64 as input and returns its number of decimal places
fn decimal_places(num: f64) -> usize {
    if num.fract() == 0.0 { return 1 }
    
    let num_str = num.to_string();
    let split_str: Vec<&str> = num_str.split('.').collect();
    split_str[1].len()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    /// Tests proper implementation of Add for C64
    fn adds() {        
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
    fn subtracts() {        
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
    fn multiplies() {        
        assert_eq!(
            C64::new(1, 2) * C64::new(5, 8), 
            C64::new(-11, 18)
        );

        assert_eq!(
            C64::new(5, -22) * C64::new(5, 8), 
            C64::new(201, -70)
        );
        assert_eq!(
            C64::new(-180, -42) * C64::new(-51, -82), 
            C64::new(5736, 16902)
        );

        assert_eq!(
            C64::new(1.3, 2) * C64::new(-5, 8.7), 
            C64::new(-23.9, 1.31)
        );

        assert_eq!(
            C64::new(-546.3, -4002.57) * C64::new(5.546, 8.21), 
            C64::new(35890.8795, -17713.13022)
        );
    }

    #[test]
    fn calculates_decimal_places() {
        assert_eq!(decimal_places(2.93), 2);
        assert_eq!(decimal_places(2.123456789), 9);
        assert_eq!(decimal_places(28489.502), 3);
        assert_eq!(decimal_places(0.93), 2);
        assert_eq!(decimal_places(2.0), 1);
        assert_eq!(decimal_places(0.0), 1);
    }
}
