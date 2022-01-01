#[cfg(test)]
mod tests {
    use super::Poly;
    

    #[test]
    fn test_evaluation() {
        let p = Poly::new(vec![1,2,3]);
        assert_eq!(p.eval(10,0), 321);
    }
    
    #[test]
    fn test_formatting() {
        let x = Poly::new(vec![1,2,3]);
        assert_eq!(format!("{}", x), "(1)^0 + (2)^1 + (3)^2");
    }
}

use std::ops::{Mul, Add};
use std::fmt;

#[derive(Debug)]
pub struct Poly <T>{
    coeff : Vec<T>
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Poly<T>{
    pub fn new(coeff : Vec<T>) -> Poly<T>{
        Poly{ coeff:coeff }
    }
    pub fn eval(self : &Poly<T>, x : T, zero : T) -> T {
        self.coeff.iter()
            .rev()
            .fold(zero, |acc, c| acc * x + *c)
    }
}

impl<T: fmt::Debug> fmt::Display for Poly<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result : Vec<String> = self.coeff.iter()
            .enumerate()
            .map(|(i, c)| format!("({:?})^{}",c,i))
            .collect();
        write!(f, "{}", result.join(" + "))
    }
}

use std::ops::Fn;
impl<T> Fn<(T, )> for Poly<T>{
    type Output = T;
    extern "rust-call" fn call(&self, args : (T,)){
        self.eval(args[0])
    }
}
