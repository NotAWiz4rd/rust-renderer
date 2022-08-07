use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

pub fn a_equal_b(a: f64, b: f64) -> bool {
    a.abs() - b.abs() < 1e-5
}

pub fn write_file(filepath: &str, content: &String) -> std::io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(content.as_ref())?;
    Ok(())
}

pub fn radians_i(degrees: i32) -> f64 {
    radians(degrees as f64)
}

pub fn radians(degrees: f64) -> f64 {
    degrees / 180.0 * PI
}

pub fn normalize_coordinate(coordinate: f64, canvas_size: u32) -> u32 {
    let mut value = ((coordinate * (canvas_size as f64 / 2.0)) as i32) + (canvas_size / 2) as i32;
    if value < 0 {
        value = 0;
    }
    if value as u32 >= canvas_size {
        value = canvas_size as i32 - 1;
    }
    value as u32
}