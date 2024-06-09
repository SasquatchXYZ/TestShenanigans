use std::fs;
use std::io::Write;

pub fn creating_files() -> std::io::Result<()> {
    fs::File::create("myfilename.txt")?
        .write_all(b"Let's put this in the file")?;
    fs::write("myfilename.txt", "This will overwrite the file and will be present after the program runs...")?;
    Ok(())
}