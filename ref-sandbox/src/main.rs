static mut STASH: &i32 = &128;

fn main() {
    f(&1);

    unsafe {
        println!("STASH value = {}", STASH);
    }

    let a = [10, 1, 30, 5];
    let s = smallest(&a);
    println!("smallest is {}", *s);
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < * s { s = r; }
    }
    s
}