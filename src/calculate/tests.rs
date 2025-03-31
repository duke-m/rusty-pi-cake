use super::*;

#[test]
fn test_calculate() {
    let precision = 1;
    let (result, duration) = calculate(precision, None::<&fn(&str)>);
    assert_eq!(result, 4.0);
    assert!(duration < 1.0);
}

#[test]
fn test_calculate_with_logger() {
    let precision = 1;
    let (result, duration) = calculate(precision, Some(&logger));
    assert_eq!(result, 4.0);
    assert!(duration < 1.0);
}

#[test]
fn test_calculate_with_precision() {
    let precision = 5;
    let (result, duration) = calculate(precision, None::<&fn(&str)>);
    assert!(result > 3.14159);
    assert!(result < 3.14160);
    assert!(duration > 0.0001);
}
