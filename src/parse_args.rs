#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
    TooSmallValue(u32),
    TooLargeValue(u32),
}

pub fn parse_args() -> Result<Frame, ParseError> {
    let mut args = ParseArgs::new();

    // skip the command name
    args.require_arg()?;

    let width_str = args.require_arg()?;
    let height_str = args.require_arg()?;

    args.require_no_args()?;

    let width = parse_u32(width_str)?;
    let height = parse_u32(height_str)?;

    if width < 10 {
        return Err(ParseError::TooSmallValue(width));
    }
    if height < 10 {
        return Err(ParseError::TooSmallValue(height));
    }

    Ok(Frame { width, height })
}

struct ParseArgs(std::env::Args);

impl ParseArgs {
    fn new() -> Self {
        ParseArgs(std::env::args())
    }

    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            Some(s) => Ok(s),
            None => Err(ParseError::TooFewArgs),
        }
    }

    fn require_no_args(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            None => Ok(()),
            Some(_) => Err(ParseError::TooManyArgs), 
        }
    }
}

fn parse_u32(s: String) -> Result<u32, ParseError> {
    match s.parse() {
        Ok(v) => Ok(v),
        Err(_) => Err(ParseError::InvalidInteger(s))
    }
}
