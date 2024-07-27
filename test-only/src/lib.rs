use std::num::ParseIntError;

/// test
///
/// ```
/// assert_eq!(test_only::add(1,2), 3);
/// ```
///
/// test no_run
///
/// ```no_run
/// assert_eq!(test_only::add(1,2), 4);
/// ```
///
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero_error() {
    1/ 0;
}

#[test]
fn explicit_radix() -> Result<(), ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}