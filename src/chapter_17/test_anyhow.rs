use anyhow::{anyhow, Context, Error};

fn parse_then_send(input: &[u8]) -> Result<(), Error> {
    let some_str = std::str::from_utf8(input)
        .with_context(|| "Couldn't parse into a str")?;
    let number = some_str
        .parse::<i32>()
        .with_context(|| format!("Got a weird str to parse: {some_str}"))?;
    send_number(number)?;
    Ok(())
}

fn send_number(number: i32) -> Result<(), Error> {
    if number < 1_000_000 {
        println!("Number sent!");
        Ok(())
    } else {
        println!("Too large!");
        Err(anyhow!("Number is too large"))
    }
}

pub fn test_anyhow() {
    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"10"));
}
