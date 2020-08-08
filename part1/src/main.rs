use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug, PartialEq)]
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
    fn times(&self, multiplier: u32) -> Dollar {
        MoneyImpl::dollar( &self.amount * multiplier )
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
    fn times(&self, multiplier: u32) -> Franc {
        MoneyImpl::franc( &self.amount * multiplier )
    }
}

impl Money for Franc {
    fn currency(&self) -> &Currency {
        &self.currency
    }
}

struct MoneyImpl {}

impl MoneyImpl {
    fn franc(amount: u32) -> Franc {
        Franc { amount, currency: Currency::Franc }
    }

    fn dollar(amount: u32) -> Dollar {
        Dollar { amount, currency: Currency::Dollar }
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = MoneyImpl::dollar(5);
        assert_eq!(MoneyImpl::dollar(10), five.times(2));
        assert_eq!(MoneyImpl::dollar(15), five.times(3));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = MoneyImpl::franc(5);
        assert_eq!(MoneyImpl::franc(10), five.times(2));
        assert_eq!(MoneyImpl::franc(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(MoneyImpl::dollar(5) == MoneyImpl::dollar(5));
        assert!(MoneyImpl::dollar(5) != MoneyImpl::dollar(6));
        assert!(MoneyImpl::franc(5) == MoneyImpl::franc(5));
        assert!(MoneyImpl::franc(5) != MoneyImpl::franc(6));
    }
}