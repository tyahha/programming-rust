fn main() {
    checked_overflow();
}

fn checked_overflow() {
    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}