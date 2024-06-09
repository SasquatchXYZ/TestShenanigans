use std::fs::{read_to_string, write, File};
use std::io::Write;

pub fn test_using_open_options() -> std::io::Result<()> {
    write("calvin_with_dad.txt",
          "Calvin: Dad, how come old photographs are always black and white?  Didn't they have color film back then?\nDad: Sure they did.  In fact, those photographs are in color.  It's just the world was black and white then.")?;

    let mut calvin_file = File::options()
        .append(true)
        .read(true)
        .open("calvin_with_dad.txt")?;
    calvin_file.write_all(b"Calvin: Really?\n")?;
    write!(&mut calvin_file, "Dad: Yep.  The world didn't turn color until sometime in the 1930s...\n")?;
    println!("{}", read_to_string("calvin_with_dad.txt")?);
    Ok(())
}