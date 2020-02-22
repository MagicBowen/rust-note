#![feature(test)]

extern crate test;

use test::Bencher;
use str2num;

#[bench]
fn execute_str_to_uint_10000(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..10000 {
            str2num::str_to_big_int("12345");
        }
    });
}