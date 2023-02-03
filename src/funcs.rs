pub(crate) mod calculations {
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub(crate) struct ComputationError;

    impl Display for ComputationError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "There was a computation error")
        }
    }

    impl std::error::Error for ComputationError {}

    pub(crate) fn add(x: Option<f32>, y: Option<f32>) -> Result<f32, ComputationError> {
        Ok(x.expect("What the crap") + y.expect("Wholly crap"))
    }

    pub(crate) fn subtract(x: Option<f32>, y: Option<f32>) -> Result<f32, ComputationError> {
        Ok(x.expect("What the crap") - y.expect("Wholly crap"))
    }

    pub(crate) fn multiply(x: Option<f32>, y: Option<f32>) -> Result<f32, ComputationError> {
        Ok(x.expect("What the crap") * y.expect("Wholly crap"))
    }

    pub(crate) fn divide(x: Option<f32>, y: Option<f32>) -> Result<f32, ComputationError> {
        assert_ne!(y.unwrap(), 0.0);
        Ok(x.expect("What the crap") / y.expect("Wholly crap"))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn check_int_add() {
            assert_eq!(add(Some(5.0), Some(5.0)).unwrap(), 10.0);
            assert_eq!(add(Some(5.0), Some(-5.0)).unwrap(), 0.0);
            assert_eq!(add(Some(-5.0), Some(-5.0)).unwrap(), -10.0);
        }

        #[test]
        fn check_divide() {
            assert_eq!(divide(Some(5.0), Some(2.0)).unwrap(), 2.5);
            assert_eq!(divide(Some(10.0), Some(10.0)).unwrap(), 1.0);
            assert_eq!(divide( Some(-5.0), Some(5.0)).unwrap(), -1.0);
        }
    }
}

pub(crate) fn check_for_floats(one: &String, two: &String, three: &String) -> bool {
    let compound: Vec<&String> = vec![one, two, three];

    for test in compound {
        let check = test.parse::<f32>();
        if check.is_ok() && check.expect("What").fract() != 0.0 {
            return true;
        }
    }
    false
}

pub(crate) fn get_operand(one: &String, two: &String, three: &String) -> char {
    let compound: Vec<&String> = vec![one, two, three];

    for test in compound {
        let check = test.parse::<char>();
        if check.is_ok() {
            let char_check = check.unwrap();

            match char_check {
                '+' | '-' | '/' | '*' => return char_check,
                _ => continue,
            }
        }
    }

    'N'
}
