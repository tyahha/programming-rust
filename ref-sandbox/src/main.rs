static mut STASH: &i32 = &128;

fn main() {
    f(&1);

    unsafe {
        println!("STASH value = {}", STASH);
    }
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
