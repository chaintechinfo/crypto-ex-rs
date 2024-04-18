fn main() {
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);
    assert_eq!(x.is_err(), false);
    assert_eq!(x.ok(), Some(-3));
    assert_eq!(x.err(), None);

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);
    assert_eq!(x.is_err(), true);
    assert_eq!(x.ok(), None);
    assert_eq!(x.err(), Some("Some error message"));

    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.unwrap(), -3);

    #[allow(unused_variables)]
        let x: Result<i32, &str> = Err("Some error message");
    // assert_eq!(x.unwrap(), -3); // this will panic

    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.unwrap_or(100), -3);

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.unwrap_or(100), 100);

    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.unwrap_or_else(|e| e.len() as i32), -3);

    let x: Result<i32, &str> = Err("Some error message.");
    assert_eq!(x.unwrap_or_else(|e| e.len() as i32), 19);

    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.expect("The world is ending"), -3);

    // let x: Result<i32, &str> = Err("Some error message");
    // assert_eq!(x.expect("The world is ending"), -3); // panics with `The world is ending: Some error message`

    let x: Result<i32, &str> = Ok(2);
    assert_eq!(x.map(|v| v * 2), Ok(4));

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.map(|v| v * 2), Err("Some error message"));

    let x: Result<Result<i32, &str>, &str> = Ok(Ok(-3));
    assert_eq!(x.unwrap(), Ok(-3));

    checked::op(1.0, 10.0);
}

mod checked {
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MatchResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MatchResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MatchResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MatchResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    fn op_(x: f64, y: f64) -> MatchResult {
        let ratio = div(x, y)?;
        let ln = ln(ratio)?;
        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            // panic!
            Err(why) => println!("{:?}", match why {
                MathError::NonPositiveLogarithm => "Non-positive argument",
                MathError::DivisionByZero => "division by zero",
                MathError::NegativeSquareRoot => "Negative square root",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}