use std::ops::Add;

use crate::colour::{BLACK, Colour, colour};

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Colour>>,
}

pub fn canvas(width: u32, height: u32) -> Canvas {
    Canvas {
        width,
        height,
        pixels: vec![vec![BLACK; width as usize]; height as usize],
    }
}

impl Canvas {
    pub fn write_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        self.pixels[y as usize][x as usize] = colour
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm = ppm.add("P3\n");
        ppm = ppm.add(&self.width.to_string());
        ppm = ppm.add(" ");
        ppm = ppm.add(&self.height.to_string());
        ppm = ppm.add("\n");
        ppm = ppm.add("255\n");

        let mut line_contents = String::new();

        for height in 0..self.height as usize {
            line_contents = String::new();
            for width in 0..self.width as usize {
                line_contents = line_contents.add(&self.pixels[height][width].to_string() as &str)
            }
            if line_contents.len() > 70 {
                let mut line = String::new();
                let nums = line_contents.split(" ");
                for num in nums {
                    // add current line if current line plus next number is too long
                    if line.len() + num.len() > 70 {
                        ppm = ppm.add(line.trim_end());
                        ppm = ppm.add("\n");
                        // reset line
                        line = String::from(num);
                        line = line.add(" ");
                    } else {
                        line = line.add(num);
                        line = line.add(" ")
                    }
                }
                ppm = ppm.add(line.trim_end());
                ppm = ppm.add("\n");
            } else {
                // remove last space
                if line_contents.ends_with(" ") {
                    line_contents = String::from(line_contents.trim_end());
                }
                ppm = ppm.add(&line_contents);
                ppm = ppm.add("\n");
            }
        }
        return ppm;
    }
}
