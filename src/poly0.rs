#[derive(Debug)]
pub struct Poly <T>{
    coeff : Vec<T>
}

impl<T> Poly<T>{
    pub fn new(coeff : Vec<T>) -> Poly<T>{
        Poly{ coeff:coeff }
    }
}

#[cfg(test)]
mod tests {
    use super::Poly;
    #[test]
    fn creation_and_debug() {
        let x = Poly::new(vec![1,2,3]);
        assert_eq!(format!("{:?}", x), "Poly { coeff: [1, 2, 3] }");
    }
}
