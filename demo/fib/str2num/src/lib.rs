use num_bigint::BigUint;
use std::str::FromStr;

/// Function for transfor a string to big unsigned integer.
///
/// Just for test document test.
///
/// ```
/// use num_bigint::ToBigUint;
/// 
/// let result = str2num::str_to_big_int("123");
/// assert_eq!(ToBigUint::to_biguint(&123), Some(result));
/// ```
pub fn str_to_big_int(s: &str) -> BigUint {
    return BigUint::from_str(s).unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn unit_test_big_uint() {
        let s = "12345";
        let result = BigUint::from_str(s).unwrap();
        assert_eq!(ToBigUint::to_biguint(&12345), Some(result));
    }

    #[test]
    fn unit_test_str_to_big_int() {
        let result = str_to_big_int("123");
        assert_eq!(ToBigUint::to_biguint(&123), Some(result));
    }
}
