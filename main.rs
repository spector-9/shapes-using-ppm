mod ppm;

use crate::ppm::Image;
// I don't know what this pattern is called.
fn starshape_pattern(grid_size: u16) {
    let mut x = Image::new(1920, 1080);
    for m in 0..x.get_height() {
        for n in 0..x.get_width() {
            if (n * m / grid_size) % 2 == 0 {
                x.fill(0xFF0000, m, n);            
            }
        }
    }
    x.write_file().expect("File cannot be created");
}

fn checkered_pattern(grid_size: u16) {
    let mut x = Image::new(1920, 1080);
    for m in 0..x.get_height() {
        for n in 0..x.get_width() {
        // You can change the logic gate operator to draw different types of grid. Try &, | and ^
            if ((n / grid_size)  % 2 == 0) ^ ((m / grid_size)  % 2 == 0) {
                x.fill(0xFF0000, m, n);            
            }
        }
    }
    x.write_file().expect("File cannot be created");
}

fn main() {
    checkered_pattern(100);
    //starshape_pattern(530);
}

