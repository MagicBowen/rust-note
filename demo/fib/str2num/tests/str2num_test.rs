extern crate str2num;
use num_bigint::ToBigUint;
// use rand::prelude::*;

#[test]
fn test_str_to_big_int() {
    let r = rand::random::<u32>();
    println!("generate a rand u32: {}", r);
    let result = str2num::str_to_big_int(&r.to_string());
    assert_eq!(ToBigUint::to_biguint(&r), Some(result));
}