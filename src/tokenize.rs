
pub fn tokenize(s: &str) -> Vec<String>
{
    let mut tokens = Vec::new();

    // Position of the first char of a token
    let mut start_tok = 0;
    // Position of the first char after a token
    let mut end_tok = 0;

    while end_tok < s.len()
    {
        // When next_token reaches the end of `s`, it will return None and
        // we break
        end_tok += match next_token(&s[start_tok..])
        {
            Some(pos) => pos,
            None => break
        };
        
        println!("{}, {}", start_tok, end_tok);
        tokens.push(s[start_tok..end_tok].to_owned());
        
        start_tok = end_tok - 1;
    }

    tokens
}

/// Returns the position of the first character after
/// the next token
fn next_token(s: &str) -> Option<usize>
{
    let mut iter = s.chars().enumerate();

    if let Some((_, c)) = iter.next()
    {
        if is_alpha(c)
        {
            return Some(ident(s));
        }
        if is_numeric(c)
        {
            return Some(number(s));
        }
        if c == '"'
        {
            return Some(string(s));
        }

        // If we're here, it's because `c` is not alphanumeric and is not '"'.
        // This is an error condition.
        // TODO: Error handling
        panic!("Character does not belong to any token");
    }

    None
}

/// Returns the position of the first character after the identifier
fn ident(s: &str) -> usize
{

    let mut iter = s.chars().enumerate().peekable();

    if let Some((_, c)) = iter.peek()
    {
        if !is_alpha(*c)
        {
            // TODO: Error handling
            panic!("Not an ident");
        }
    }
    else
    {
        // TODO: Error handling
        panic!("End of stream at start of token");
    }

    // Because we peeked the first time, the following loop will always execute
    // at least once

    while let Some((i, c)) = iter.next()
    {
        if !is_alpha_numeric(c)
        {
            return i;
        }
        if !iter.peek().is_some()
        {
            // This is the end of the string
            // Return the position after the last char to indicate this is the
            // last token
            return i + 1;
        }
    }
    
    unreachable!();
}

/// Returns the position of the first position after the number
fn number(s: &str) -> usize
{
    let mut iter = s.chars().enumerate().peekable();
    
    // First, read the integer part

    while let Some((i, c)) = iter.next()
    {
        if !is_numeric(c)
        {
            if i == 0
            {
                // TODO: Error handling
                panic!("token is not a number");
            }
            else
            {
                break;
            }
        }
        // If we've reached the end of `s`, this is the last token in `s`.
        // Return the position past the end of `s`, to indicate the final token
        if !iter.peek().is_some()
        {
            return i + 1;
        }
    }

    // At this point, we've reachaed a non-numeric character.
    // If the char is a '.', then we may have a floating point value.
    // We know iter.next() will give `Some()`

    if let Some((i, c)) = iter.next()
    {
        // If `c` is some character other than '.', this is the end of the token
        if c != '.'
        {
            return i;
        }
    }

    // At this point, we know the last char was a '.', so we expect more
    // numeric chars now.
    // We need to check that the next char is numeric
    if let Some((_, c)) = iter.next()
    {
        if !is_numeric(c)
        {
            // TODO: Error handling
            panic!("number ending in decimal point and no fractional value");
        }
    }
    else
    {
        // If we're here, it's because `s` ended in a '.'
        // This is an error condition.
        // TODO: Error handling
        panic!("number ending in decimal point and no factional value");
    }

    while let Some((i, c)) = iter.next()
    {
        if !is_numeric(c)
        {
            // We've reached the end of the token
            return i;
        }
        // If we've reached the end of `s`, this is the last token in `s`.
        // Return the position past the end of `s`, to indicate the final token
        if !iter.peek().is_some()
        {
            return i + 1;
        }
    }

    unreachable!();
}

/// Returns the first position after the string
fn string(s: &str) -> usize
{
    let mut iter = s.chars().enumerate().peekable();

    if let Some((_, c)) = iter.next()
    {
        if c != '"'
        {
            panic!("Not a string");
        }
    }
    else
    {
        panic!("Empty token");
    }
    
    while let Some((i, c)) = iter.next()
    {
        if c == '"'
        {
            // `c` is the last char of the token.
            // Return the index of the next token.
            return i + 1;
        }
    }

    // If we're here, it's because we encountered the end of `s` directly after
    // the opening quote. This is an error condition.
    // TODO: Error handling
    panic!("String ended before closing quote");
}

/// Returns true if `c` is in the range '0'..'9'
fn is_numeric(c: char) -> bool
{
    c >= '0' && c <= '9'
}

/// Returns true if `c` is in either the range 'a'..'z' or 'A'..'Z'
fn is_alpha(c: char) -> bool
{
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
}

/// Returns true if `c` is alphanumeric
/// Equivalent to `is_alpha(c) && is_numeric(c)`
fn is_alpha_numeric(c: char) -> bool
{
    is_alpha(c) || c >= '0' && c <= '9'
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_is_numeric()
    {
        assert!(is_numeric('0'));
        assert!(is_numeric('9'));

        // '/' is directly below '0' in ascii encoding
        assert!(!is_numeric('/'));

        // ':' is directly above '9' in ascii encoding
        assert!(!is_numeric(':'));
    }

    #[test]
    fn test_is_alpha()
    {
        assert!(is_alpha('a'));
        assert!(is_alpha('z'));
        assert!(is_alpha('Z'));

        // '@' comes before 'A'
        assert!(!is_alpha('@'));
        // '[' comes after 'Z'
        assert!(!is_alpha('['));
        // '`' comes before 'a'
        assert!(!is_alpha('`'));
        // '{' comes after 'z'
        assert!(!is_alpha('{'));
    }
}
