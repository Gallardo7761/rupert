use mathengine::{Value, evaluate_expression};

use crate::error::CalcError;

pub const MAX_EXPR_LEN: usize = 200;

pub fn calc(expr: &str) -> Result<f64, CalcError> {
    let expr = expr.trim();

    if expr.is_empty() {
        return Err(CalcError::Empty);
    }

    if expr.chars().count() > MAX_EXPR_LEN {
        return Err(CalcError::TooLong { max: MAX_EXPR_LEN });
    }

    let number = match evaluate_expression(expr)? {
        Value::Number(n) => n.0,
        Value::UnitValue(_) => return Err(CalcError::UnitsNotSupported),
    };

    if number.is_finite() {
        Ok(number)
    } else {
        Err(CalcError::NotFinite)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn basic_arithmetic() {
        assert!(approx(calc("2+2").unwrap(), 4.0));
        assert!(approx(calc("2+3*4").unwrap(), 14.0));
        assert!(approx(calc("2*(3+4)").unwrap(), 14.0));
        assert!(approx(calc("2^3").unwrap(), 8.0));
    }

    #[test]
    fn empty_expression() {
        assert_eq!(calc(""), Err(CalcError::Empty));
        assert_eq!(calc("   "), Err(CalcError::Empty));
    }

    #[test]
    fn too_long_expression() {
        let expr = "1+".repeat(MAX_EXPR_LEN) + "1";
        assert_eq!(calc(&expr), Err(CalcError::TooLong { max: MAX_EXPR_LEN }));
    }

    #[test]
    fn deep_nesting_is_cut_before_the_parser() {
        let expr = "(".repeat(10_000) + "1" + &")".repeat(10_000);
        assert_eq!(calc(&expr), Err(CalcError::TooLong { max: MAX_EXPR_LEN }));
    }

    #[test]
    fn garbage_is_an_error() {
        assert!(calc("abc").is_err());
        assert!(calc("2 +").is_err());
    }

    #[test]
    fn division_by_zero_is_never_ok() {
        assert!(matches!(
            calc("1/0"),
            Err(CalcError::Evaluation { .. } | CalcError::NotFinite)
        ));
    }

    #[test]
    fn huge_power_is_never_ok() {
        assert!(matches!(
            calc("2^99999"),
            Err(CalcError::Evaluation { .. } | CalcError::NotFinite)
        ));
    }

    #[test]
    fn units_are_rejected() {
        assert_eq!(calc("10m + 5"), Err(CalcError::UnitsNotSupported));
    }
}
