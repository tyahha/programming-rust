fn main() {
    // checked_overflow();
    checked_calc();
}

fn checked_overflow() {
    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}

fn checked_calc() {
    println!("checked 10_u8 + 20 = {:?}", 10_u8.checked_add(30));
    println!("checked 100_u8 + 200 = {:?}", 100_u8.checked_add(200));
}