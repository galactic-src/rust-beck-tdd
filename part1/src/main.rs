use std::cmp::PartialEq;

#[derive(Clone, Debug, PartialEq)]
enum Currency {
    Dollar,
    Franc
}

#[derive(PartialEq, Debug, Clone)]
struct Money {
    amount: u32,
    currency: Currency
}

impl Money {
    fn franc(amount: u32) -> Money {
        Money { amount, currency: Currency::Franc }
    }

    fn dollar(amount: u32) -> Money {
        Money { amount, currency: Currency::Dollar }
    }

    fn times(&self, multiplier: u32) -> Money {
        Money { amount: &self.amount * multiplier, currency: self.currency.clone() }
    }

    fn plus(&self, addend: &Money) -> Sum {
        Sum {
            augend: self.clone(),
            addend: addend.clone()
        }
    }
}

struct Sum {
    augend: Money,
    addend: Money
}

struct Bank {}

impl Bank {
    fn reduce(&self, source: &Sum, to: &Currency) -> Money {
        Money { amount: source.addend.amount.clone() + source.augend.amount.clone(), currency: to.clone() }
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let five = Money::dollar(5);
        let expression = five.plus(&five);
        let bank = Bank{};
        let reduced = bank.reduce(&expression, &Currency::Dollar);
        assert_eq!(Money::dollar(10), reduced);
    }

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Money::dollar(5) == Money::dollar(5));
        assert!(Money::dollar(5) != Money::dollar(6));
        assert!(Money::franc(5) == Money::franc(5));
        assert!(Money::franc(5) != Money::franc(6));
    }
}