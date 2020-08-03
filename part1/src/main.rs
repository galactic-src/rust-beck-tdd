use std::cmp::PartialEq;


#[derive(PartialEq)]
struct Dollar {
    amount: u32
}

impl Dollar {
    fn new(amount: u32) -> Dollar {
        Dollar { amount }
    }

    fn times(&self, multiplier: u32) -> Dollar {
        return Dollar::new( &self.amount * multiplier );
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5) == Dollar::new(5));
        assert!(Dollar::new(5) != Dollar::new(6));
    }
}