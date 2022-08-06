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
