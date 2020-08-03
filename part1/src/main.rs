struct Dollar {
    amount: u32
}

impl Dollar {
    fn times(&self, multiplier: u32) -> Dollar {
        return Dollar { amount: self.amount * multiplier };
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testMultiplication() {
        let five = Dollar {amount: 5};
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }

    #[test]
    fn testEquality() {
        assert!(Dollar { amount: 5 } == Dollar { amount: 5 });
        assert!(Dollar { amount: 5 } != Dollar { amount: 6 });
    }
}