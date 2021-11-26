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

#[cfg(test)]
mod tests {
    use super::*;
    
}
