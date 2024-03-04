fn main() {
    // checked_overflow();
    checked_calc();
    wrapped_calc();
    saturated_calc();
    overflowed_calc();
    float_research();
    boolean_research();
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

fn wrapped_calc() {
    println!("wrapped 10_u8 + 20 = {:?}", 10_u8.wrapping_add(30));
    println!("wrapped 100_u8 + 200 = {:?}", 100_u8.wrapping_add(200));
}

fn saturated_calc() {
    println!("saturated 10_u8 + 20 = {:?}", 10_u8.saturating_add(30));
    println!("saturated 100_u8 + 200 = {:?}", 100_u8.saturating_add(200));
}

fn overflowed_calc() {
    println!("overflowed 10_u8 + 20 = {:?}", 10_u8.overflowing_add(30));
    println!("overflowed 100_u8 + 200 = {:?}", 100_u8.overflowing_add(200));
}

fn float_research() {
    println!("(-1. / f32::INFINITY).is_sign_negative()) = {}", (-1. / f32::INFINITY).is_sign_negative());
    println!("-f32::MIN == f32::MAX = {}", -f32::MIN == f32::MAX);
}

fn boolean_research() {
    println!("false as i32 = {}", false as i32);
    println!("true as i32 = {}", true as i32);
}