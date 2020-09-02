use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Hash, Eq, Clone, Debug, PartialEq)]
enum Currency {
    Dollar,
    Franc
}

type Amount = u32;

#[derive(PartialEq, Debug, Clone)]
struct Money {
    amount: Amount,
    currency: Currency
}

impl Money {
    fn franc(amount: Amount) -> Money {
        Money { amount, currency: Currency::Franc }
    }

    fn dollar(amount: Amount) -> Money {
        Money { amount, currency: Currency::Dollar }
    }

    fn times(&self, multiplier: Amount) -> Money {
        Money { amount: &self.amount * multiplier, currency: self.currency.clone() }
    }

    fn plus(&self, addend: &Money) -> Expression {
        Expression::Sum {
            augend: self.clone(),
            addend: addend.clone()
        }
    }
}

enum Expression {
    Money {money: Money},
    Sum {augend: Money, addend: Money}
}

impl Expression {
    fn reduce(&self, bank: &Bank, to: &Currency) -> Money {
        match self {
            Expression::Money {money} => {
                let rate = bank.rate(&money.currency, to);
                return Money { amount: money.amount.clone() / rate, currency: to.clone()}
            },
            Expression::Sum { augend, addend} =>
                Money { amount: &augend.amount + &addend.amount, currency: Currency::Dollar }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Pair {
    from: Currency,
    to: Currency
}

struct Bank {
    rates: HashMap<Pair, Amount>
}

impl Bank {
    fn new() -> Bank {
        Bank {
            rates: HashMap::new()
        }
    }

    fn reduce(&self, source: &Expression, to: &Currency) -> Money {
        source.reduce(self, to)
    }

    fn add_rate(&mut self, from: Currency, to: Currency, rate: Amount) {
        let p = Pair { from: from, to: to };
        self.rates.insert(p, rate);
    }

    fn rate(&self, from: &Currency, to: &Currency) -> Amount {
        if from == to {
            1
        } else {
            *self.rates
                .get(&Pair { from: from.clone(), to: to.clone() })
                .unwrap()
        }
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
        let bank = Bank::new();
        let reduced = bank.reduce(&expression, &Currency::Dollar);
        assert_eq!(Money::dollar(10), reduced);
    }

    #[test]
    fn test_reduce_sum() {
        let sum = Expression::Sum { augend: Money::dollar(3), addend: Money::dollar(4) };
        let bank = Bank::new();
        let result = bank.reduce(&sum, &Currency::Dollar);
        assert_eq!(Money::dollar(7), result);
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

    #[test]
    fn test_identity_rate() {
        assert_eq!(1, Bank::new().rate(&Currency::Dollar, &Currency::Dollar));
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);
        let result = bank.reduce( &Expression::Money { money: Money::franc(2) }, &Currency::Dollar);
        assert_eq!(Money::dollar(1), result);
    }
}