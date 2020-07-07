use std::io;

use crate::error::InputError;

type Result<T> = std::result::Result<T, InputError>;

fn readline() -> Result<(String, usize)> {
    /* Read each line and check for the constraint (1 <= N <= 25) */
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => (),
        Err(_) => return Err(InputError::InError),
    }

    let trimmed = buf.trim().to_string();
    let count = trimmed.chars().count();
    if count < 1 || count > 25 {
        return Err(InputError::LengthDisrespect(count));
    }

    Ok((trimmed, count))
}

fn is01(x: char) -> bool {
    /* Is our input in the range '0' -> '1' ? */
    x == '0' || x == '1'
}

pub fn retrieve() -> Result<(String, String, usize)> {
    /* Get both lines and check for errors in the input provided
     * then return them along with the size.
     */
    let (start, startlen) = readline()?;
    let (target, targetlen) = readline()?;

    if startlen != targetlen {
        return Err(InputError::FormatDisrespect);
    }
    if !start.chars().all(is01) || !target.chars().all(is01) {
        return Err(InputError::FormatDisrespect);
    }

    Ok((start, target, startlen))
}
