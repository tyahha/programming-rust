use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        },
    }
}

#[test]
fn test_parse_pair () {
    assert_eq!(parse_pair::<i32>("",','), None);
    assert_eq!(parse_pair::<i32>("10,",','), None);
    assert_eq!(parse_pair::<i32>(",10",','), None);
    assert_eq!(parse_pair::<i32>("10,100",','), Some((10, 100)));
}