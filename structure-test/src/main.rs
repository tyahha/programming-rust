#[derive(Debug)]
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

impl GrayscaleMap {
    fn getComputedSize(&self) -> usize {
        self.size.0 * self.size.1
    }
}

#[derive(Debug)]
struct Bounds(pub usize, pub usize);

#[derive(Debug)]
struct Onesuch;

#[derive(Debug)]
pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { older: Vec::new(), younger: Vec::new() }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    // ...
}

impl Queue<i32> {
    pub fn print_younger_numbers(&self) {
        println!("young numbers: {:?}", self.younger);
    }
}

struct Extrema<'a> {
    greatest: &'a i32,
    least: &'a i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least    { least = &slice[i]; }
        if slice[i] > *greatest { greatest = &slice[i]; }
    }
    Extrema { greatest, least }
}

fn main() {
    let width = 1024;
    let height = 574;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    println!("image.computedSize = {}", image.getComputedSize());

    let image_bounds = Bounds(1024, 768);
    println!("image_bounds = {:?}", image_bounds);

    let o = Onesuch;
    println!("o = {:?}", o);

    let mut q = Queue::new();
    q.push(1);
    println!("Queue: {:?}", q);
    q.print_younger_numbers();

    let mut q2 = Queue::new();
    q2.push("aaa");
    println!("Queue: {:?}", q2);

    let a = [0, -3, 0, 15, 49];
    let e = find_extrema(&a);
    println!("least: {}", e.least);
    println!("greatest: {}", e.greatest);
}
