struct Dollar {
    amount: u32
}

impl Dollar {
    fn times(&mut self, multiplier: u32) {
        self.amount = 10;
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testMultiplication() {
        let mut five = Dollar {amount: 5};
        five.times(2);
        assert_eq!(10, five.amount)
    }
}