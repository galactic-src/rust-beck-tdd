use std::cmp::PartialEq;

#[derive(Clone, Debug, PartialEq)]
enum Currency {
    Dollar,
    Franc
}

trait Money {
    fn currency(&self) -> &Currency;
    fn times(&self, multiplier: u32) -> Self;
}

#[derive(PartialEq, Debug)]
struct Dollar {
    amount: u32,
    currency: Currency
}

impl Money for Dollar {
    fn currency(&self) -> &Currency {
        &self.currency
    }
    fn times(&self, multiplier: u32) -> Dollar {
        MoneyImpl::dollar( &self.amount * multiplier )
    }
}

#[derive(PartialEq, Debug)]
struct Franc {
    amount: u32,
    currency: Currency
}

impl Franc {
}

impl Money for Franc {
    fn currency(&self) -> &Currency {
        &self.currency
    }
    fn times(&self, multiplier: u32) -> Franc {
        MoneyImpl::franc( &self.amount * multiplier )
    }
}

#[derive(PartialEq, Debug)]
struct MoneyImpl {
    amount: u32,
    currency: Currency
}

impl MoneyImpl {
    fn franc(amount: u32) -> Franc {
        Franc { amount, currency: Currency::Franc }
    }

    fn dollar(amount: u32) -> Dollar {
        Dollar { amount, currency: Currency::Dollar }
    }
}

impl Money for MoneyImpl{
    fn currency(&self) -> &Currency {
        &self.currency
    }

    fn times(&self, multiplier: u32) -> MoneyImpl {
        MoneyImpl { amount: &self.amount * multiplier, currency: self.currency.clone() }
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