use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::internal::entity::color::Color;

pub struct Framebuffer {
    pub width : usize, 
    pub height: usize,
    pub buffer : Vec<u32>,
    background_color : Color,
    current_color : Color
}

pub trait RenderableToFile {
    fn render_buffer(&self, filename: &str) -> io::Result<()>;
    fn write_bmp_header(&self, file: &mut File) -> io::Result<()>;
    fn write_pixel_data(&self, file: &mut File) -> io::Result<()>;
}

impl Framebuffer {
    // Constructor to create a new Framebuffer
    pub fn new(width: usize, height: usize, background_color: Color ) -> Self {
        let buffer_size = width * height;
        let buffer = vec![background_color.to_hex(); buffer_size]; // Initialize buffer with background color
        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color: Color::new(0, 0, 0), // Default current color to black
        }
    }

    // Constructor to create a new Framebuffer
    pub fn new_default(width: usize, height: usize) -> Self {
        let white_color = Color::new(255, 255, 255);
        Self::new(width, height, white_color)
    }

    // Function to clear the framebuffer with the background color
    pub fn clear(&mut self) {
        let background_hex = self.background_color.to_hex();
        for pixel in self.buffer.iter_mut() {
            *pixel = background_hex;
        }
    }

    // Function to draw a point at (x, y) using the current color
    pub fn draw_point(&mut self, x: usize, y: usize) {
        if  0 < x  
            && x < self.width 
            && 0 < y 
            && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = self.current_color.to_hex();
        }
    }

    // Function to draw a point at (x, y) using the current color
    /// owo
    pub fn get_point_color(&mut self, x: usize, y: usize) -> Color{
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            let color = Color::from_hex(self.buffer[index]);
            return  color
        }
        return Color::new(0, 0, 0);
    }

    // Function to set the background color
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.clear(); // Clear buffer with the new background color
    }
    
    pub fn set_background_color_hex(&mut self, hex: u32){
        self.background_color = Color::from_hex(hex);
    }

    // Function to set the current drawing color
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
    pub fn set_current_color_hex(&mut self, hex: u32) {
        self.current_color = Color::from_hex(hex);
    }

}

impl RenderableToFile for Framebuffer {
   fn render_buffer(&self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;

        // Write the BMP header
        self.write_bmp_header(&mut file)?;

        // Write the pixel data
        self.write_pixel_data(&mut file)?;

        Ok(())
   } 

   fn write_bmp_header(&self, file: &mut File) -> io::Result<()> {
    // BMP Header
    let file_size = 14 + 40 + (self.width * self.height * 4) as u32;
    let reserved: u32 = 0;
    let data_offset: u32 = 14 + 40;

    // BITMAPINFOHEADER
    let header_size: u32 = 40;
    let planes: u16 = 1;
    let bits_per_pixel: u16 = 32;
    let compression: u32 = 0;
    let image_size = (self.width * self.height * 4) as u32;
    let x_pixels_per_meter: u32 = 0;
    let y_pixels_per_meter: u32 = 0;
    let total_colors: u32 = 0;
    let important_colors: u32 = 0;

    file.write_all(b"BM")?; // Signature
    file.write_all(&file_size.to_le_bytes())?; // File size
    file.write_all(&reserved.to_le_bytes())?; // Reserved
    file.write_all(&data_offset.to_le_bytes())?; // Data offset

    file.write_all(&header_size.to_le_bytes())?; // Header size
    file.write_all(&(self.width as u32).to_le_bytes())?; // Width
    file.write_all(&(self.height as u32).to_le_bytes())?; // Height
    file.write_all(&planes.to_le_bytes())?; // Planes
    file.write_all(&bits_per_pixel.to_le_bytes())?; // Bits per pixel
    file.write_all(&compression.to_le_bytes())?; // Compression
    file.write_all(&image_size.to_le_bytes())?; // Image size
    file.write_all(&x_pixels_per_meter.to_le_bytes())?; // X pixels per meter
    file.write_all(&y_pixels_per_meter.to_le_bytes())?; // Y pixels per meter
    file.write_all(&total_colors.to_le_bytes())?; // Total colors
    file.write_all(&important_colors.to_le_bytes())?; // Important colors

    Ok(())
}

fn write_pixel_data(&self, file: &mut File) -> io::Result<()> {
    for y in 0..self.height { // BMP files are bottom to top
        for x in 0..self.width {
            let index = y * self.width + x;
            let color_hex = self.buffer[index];
            let r = (color_hex >> 16) & 0xFF;
            let g = (color_hex >> 8) & 0xFF;
            let b = color_hex & 0xFF;
            let a = 0xFF; // BMP does not support alpha channel in this format

            file.write_all(&[b as u8, g as u8, r as u8, a as u8])?;
        }
    }
    Ok(())
}
}