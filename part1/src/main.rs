use std::cmp::PartialEq;


#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
struct Franc {
    amount: u32
}

impl Franc {
    fn new(amount: u32) -> Franc {
        Franc { amount }
    }

    fn times(&self, multiplier: u32) -> Franc {
        return Franc::new( &self.amount * multiplier );
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
    }
}