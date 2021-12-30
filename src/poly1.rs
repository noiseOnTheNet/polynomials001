#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
pub struct Poly <T>{
    coeff : Vec<T>
}

impl<T> Poly<T>{
    pub fn new(coeff : Vec<T>) -> Poly<T>{
        Poly{ coeff:coeff }
    }
    pub fn eval(self : &Poly<T>, x : T) -> T {
        x
    }
}
