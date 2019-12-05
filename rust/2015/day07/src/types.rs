use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum ValueSource {
    Constant(u16),
    Variable(String),
}

impl fmt::Display for ValueSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueSource::Constant(v) => write!(f, "{}", v),
            ValueSource::Variable(v) => write!(f, "{}", v),
        }
    }
}

impl FromStr for ValueSource {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if 's' is a number then we return Constant, otherwise
        // we return a Variable; this cannot fail
        match s.parse::<u16>() {
            Ok(v) => Ok(ValueSource::Constant(v)),
            Err(_) => Ok(ValueSource::Variable(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Source {
    Value(ValueSource),
    And(ValueSource, ValueSource),
    Or(ValueSource, ValueSource),
    LeftShift(ValueSource, ValueSource),
    RightShift(ValueSource, ValueSource),
    Not(ValueSource),
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::Value(v) => write!(f, "{}", v),
            Source::And(v1, v2) => write!(f, "{} AND {}", v1, v2),
            Source::Or(v1, v2) => write!(f, "{} OR {}", v1, v2),
            Source::LeftShift(v1, v2) => write!(f, "{} LSHIFT {}", v1, v2),
            Source::RightShift(v1, v2) => write!(f, "{} RSHIFT {}", v1, v2),
            Source::Not(v) => write!(f, "NOT {}", v),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub src: Source,
    pub dest: String,
}

impl Instruction {
    pub fn new(src: Source, dest: String) -> Instruction {
        Instruction { src, dest }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.src, self.dest)
    }
}
