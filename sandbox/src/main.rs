fn main() {
    // checked_overflow();
    checked_calc();
    wrapped_calc();
    saturated_calc();
    overflowed_calc();
    float_research();
    boolean_research();
    char_research();
    array_research();
    vector_sandbox();
    slice_sandbox();
    string_sandbox();
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

fn char_research() {
    println!("--- char research");
    println!("  {}", '\u{CA0}');
    println!("'\\x2A' = {}", '\x2A');
    println!("'ಠ' as u16 = {}", 'ಠ' as u16);
    println!("42_u8 as char = {}", 42_u8 as char);
    println!("std::char::from_u32(3232) = {:?}", std::char::from_u32(3232));
}

fn array_research() {
    let mut a = [ 3, 1, 4, 100, 5, 8];
    println!("before sort = {:?}", a);
    a.sort();
    println!("after sort = {:?}", a);
}

fn vector_sandbox() {
    println!("----- vector sandbox");

    fn print_vector(v: &Vec<i32>) {
        println!("------- print vector");
        for x in v {
            println!("{}", x);
        }
    }

    let mut primes = vec![1,2,3,5,7,11];
    print_vector(&primes);

    primes.push(13);
    print_vector(&primes);

    let new_vec = vec![0; 3];
    print_vector(&new_vec);

    let mut from_range: Vec<i32> = (0..5).collect();
    print_vector(&from_range);

    from_range.reverse();
    print_vector(&from_range);

    let mut v: Vec<i32> = Vec::with_capacity(2);
    println!("before push");
    println!("len() = {}", v.len());
    println!("capacity() = {}", v.capacity());

    v.push(1);
    v.push(2);
    println!("after push");
    println!("len() = {}", v.len());
    println!("capacity() = {}", v.capacity());

    v.push(3);
    println!("after extra push");
    println!("len() = {}", v.len());
    println!("capacity() = {}", v.capacity());
}

fn slice_sandbox() {
    println!("----- slice sandbox");

    let v = vec![0.0, -0.7097, 1., 2., 3., 4.];
    let a = [0.0, -0.707];
    println!("v = {:?}", v);
    println!("a = {:?}", a);

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;
    println!("sv = {:?}", sv);
    println!("sa = {:?}", sa);

    fn print(n: &[f64]) {
        for e in n {
            println!("{}", e);
        }
    }

    print(&a);
    print(&v);

    print(&v[0..2]);
    print(&a[1..])
}

fn string_sandbox() {
    println!("----- string sandbox");
    println!("multi line string
             multiline string");
    println!("multi line string2\
             multiline string");
    println!(r"row string c:\Program Files\Gorillas");
    println!(r###"b"GET" == &[b'G', b'E', b'T'] = {}"###, b"GET" == &[b'G', b'E', b'T']);

    println!(r###""ಠ_ಠ".len() = {}"###, "ಠ_ಠ".len());
    println!(r###""ಠ_ಠ".chars().count() = {}"###, "ಠ_ಠ".chars().count());

    let str = "&str";
    let string = str.to_string();
    let string2 = str.to_owned();
    println!("str = {}, string = {}, string2 = {}", str, string, string2);

    let bits = vec!["veni", "vidi", "vici"];
    println!("bits.concat() = {}", bits.concat());
    println!("bits.join(\", \") = {}", bits.join(","));
}