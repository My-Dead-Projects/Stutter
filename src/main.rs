fn main()
{
    let message = match recognize("4")
    {
        Ok(_) => "Recognized.".to_owned(),
        Err(m) => format!("Error: {}", m)
    };

    println!("{}", message);
}

/// Recognizes a string as belonging to the stutter language
fn recognize(string: &'static str) -> Result<(), &'static str>
{
    Ok(())
}
