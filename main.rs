use std::fs::File;
use std::io::{Write, BufWriter};

struct Image {
    height: u16,
    width: u16,
    pixel_data: Vec<u32>,
}

impl Image {
    fn new(width: u16, height: u16) -> Self {
        Image {
            height,
            width,
            pixel_data: vec![0x00FF00; width as usize * height as usize],
        }
    }

    fn fill_with_rgb(&mut self, m: u16, n: u16, r: u8, g: u8, b: u8) {
       let value_in_single_digit: u32 = ((r as u32) << (8 * 2))
                                   + ((g as u32) << (8 * 1))
                                   + ((b as u32) << (8 * 0));
        self.fill(value_in_single_digit, m , n);

    }

    fn fill(&mut self, rgb_number: u32, m: u16, n: u16) {
        self.pixel_data[m as usize * self.width as usize+ n as usize] = rgb_number;
    }

    fn value_at(&self, m: u16, n: u16) -> u32{
        self.pixel_data[m as usize * self.width as usize+ n as usize]
    }
    
    fn get_height(&self) -> u16 {
        self.height
    }

    fn get_width(&self) -> u16 {
        self.width
    }
}

struct Pixel {
}

impl Pixel {
    fn from(rgb_value: u32) -> [u8; 3] {
        let rgb_array: [u8; 3] = [
            (rgb_value >> (8 * 2) & 0x0000FF) as u8,
            (rgb_value >> (8 * 1) & 0xFF) as u8,
            (rgb_value >> (8 * 0) & 0xFF) as u8,];
        rgb_array
    }
}


fn write_file(img: &Image) -> std::io::Result<()> {
    let file = File::create("./test.ppm")?;
    let mut buff = BufWriter::new(&file);
    write!(buff, "P6\n{} {}\n255\n", &img.get_width(), &img.get_height())?;
    for x in 0..img.get_height(){
        for y in 0..img.get_width(){
            buff.write(&Pixel::from(img.value_at(x , y)))?; 
        }
    }
    buff.flush()?;
    Ok(())
}
fn main() {
    let x = Image::new(1920, 1080);
    write_file(&x).expect("File cannot be created");
}
