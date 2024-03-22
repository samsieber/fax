
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

#[derive(Copy, Clone)]
pub enum BW {
    B, W
}

pub struct PPM {
    pub height: u32,
    pub width: u32,
    pub data: Vec<BW>,
}

impl PPM {
    pub fn new(height: u32, width: u32) -> PPM {
        let size = height * width;
        let data = vec![BW::W; size as usize];
        PPM { height, width, data }
    }
    
    pub fn from_data(height: u32, width: u32, data: Vec<BW>) -> PPM {
        let size = height * width;
        assert_eq!(size as usize, data.len());
        PPM { height, width, data }
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        if x > self.width {
            None
        } else if y > self.height {
            None
        } else {
            Some((y * self.width + x) as usize)
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<BW> {
        self.get_offset(x, y).map(|o| self.data[o])
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: BW) -> bool {
        self.get_offset(x, y).map(|o| {
            self.data[o] = color;
            true
        }).unwrap_or(false)
    }

    pub fn to_file_bytes(&self) -> std::io::Result<Vec<u8>> {
        let mut file = vec!();
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        let data: Vec<u8> = self.data.iter().flat_map(|v| {
            match v {
                BW::B => [0u8,0,0],
                BW::W => [255u8, 255, 255],
            }
        }).collect();
        file.write(header.as_bytes())?;
        file.write(&data)?;
        Ok(file)
    }

    pub fn write_file(&self, filename: impl AsRef<str>) -> std::io::Result<()> {
        let file_path = Path::new(filename.as_ref());
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("Failed to open file");
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        let data: Vec<u8> = self.data.iter().flat_map(|v| {
            match v {
                BW::B => [0u8,0,0],
                BW::W => [255u8, 255, 255],
            }
        }).collect();
        file.write(header.as_bytes())?;
        file.write(&data)?;
        Ok(())
    }
}
