use std::cmp::PartialEq;

enum Currency {
    Dollar,
    Franc
}

trait Money {
    fn currency(&self) -> &Currency;
}

#[derive(PartialEq, Debug)]
struct Dollar {
    amount: u32,
    currency: Currency
}

impl Dollar {
    fn new(amount: u32) -> Dollar {
        Dollar { amount, currency: Currency::Dollar }
    }

    fn times(&self, multiplier: u32) -> Dollar {
        return Dollar::new( &self.amount * multiplier );
    }
}

impl Money for Dollar {
    fn currency(&self) -> &Currency {
        &self.currency
    }
}

#[derive(PartialEq, Debug)]
struct Franc {
    amount: u32,
    currency: Currency
}

impl Franc {
    fn new(amount: u32) -> Franc {
        Franc { amount, currency: Currency::Franc }
    }

    fn times(&self, multiplier: u32) -> Franc {
        return Franc::new( &self.amount * multiplier );
    }
}

impl Money for Franc {
    fn currency(&self) -> &Currency {
        &self.currency
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
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5) == Dollar::new(5));
        assert!(Dollar::new(5) != Dollar::new(6));
        assert!(Franc::new(5) == Franc::new(5));
        assert!(Franc::new(5) != Franc::new(6));
    }
}