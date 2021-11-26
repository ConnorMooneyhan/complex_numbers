use std::{ fmt, ops };

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
}
