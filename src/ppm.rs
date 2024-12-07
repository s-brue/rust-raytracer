use std::fs::*;
use std::io::*;
use std::path::Path;

pub struct PPMFile {
    file: File,
    width: u32,
    height: u32,
}

impl PPMFile {
    pub fn create(p: &str, width: u32, height: u32) -> PPMFile {
        let path = Path::new(p);
        let file = File::create(path).expect("Unable to create file");
        let mut ppm = PPMFile {
            file,
            width,
            height,
        };
        ppm.write_header();
        ppm
    }

    fn write_header(&mut self) {
        let mut writer = BufWriter::new(&self.file);
        writeln!(writer, "P3\n{} {}\n255", self.width, self.height)
            .expect("Can't write to the PPM file");
    }

    pub fn write_next_pixel(&mut self, r: u8, g: u8, b: u8) {
        let mut writer = BufWriter::new(&self.file);
        writeln!(writer, "{} {} {} ", r, g, b).expect("Can't write to the PPM file");
    }
}
