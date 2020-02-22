fn add_one(a : &mut u32) -> &mut u32 {
    *a = *a + 1;
    a
}


fn main() {
    let mut i = 12_u32;
    let b = add_one(&mut i);
    let c = add_one(b);
    *c = *c + 1;
    println!("{}", c);
    println!("{}", b);
    println!("{}", i);
}
