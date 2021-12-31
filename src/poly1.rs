#[cfg(test)]
mod tests {
    use super::Poly;
    use std::fmt::Write as FmtWrite;
    #[test]
    fn creation_and_debug() {
        let x = Poly::new(vec![1,2,3]);
        let mut s = String::new();
        write!(&mut s, "{:?}", x);
        assert_eq!(s, "Poly { coeff: [1, 2, 3] }");
    }

    #[test]
    fn test_evaluation() {
        let p = Poly::new(vec![1,2,3]);
        assert_eq!(p.eval(10,0), 321);
    }
}

use std::ops::{Mul, Add};

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
