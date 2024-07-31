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
}
