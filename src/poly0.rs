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
    use std::fmt::Write as FmtWrite;
    #[test]
    fn creation_and_debug() {
        let x = Poly::new(vec![1,2,3]);
        let mut s = String::new();
        write!(&mut s, "{:?}", x);
        assert_eq!(s, "Poly { coeff: [1, 2, 3] }");
    }
}
