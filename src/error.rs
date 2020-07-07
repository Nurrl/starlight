use derive_more::Display;
use std::error::Error;

#[derive(Debug, Display, PartialEq)]
pub enum InputError {
    #[display(fmt = "An error occured while reading from standard input")]
    InError,

    #[display(fmt = "Length constraint not respected (1 >= N >= 25): {}", _0)]
    LengthDisrespect(usize),

    #[display(
        fmt = "Format not respected (only '0' and '1' are fine, and both lines must have the same length)"
    )]
    FormatDisrespect,
}

impl Error for InputError {}
