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