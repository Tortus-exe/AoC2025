use std::str::FromStr;
use std::error::Error;
use std::fmt;

const DAY1DIALSIZE: i32 = 100;
const DAY1DIALSTART: i32 = 50;

#[derive(Debug, Clone, Copy)]
enum Move {
    R(u16),
    L(u16)
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMoveError {
    source: String
}

impl fmt::Display for ParseMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseMoveError: {}", self.source)
    }
}
impl Error for ParseMoveError { }

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iterator = s.chars();
        let prefix: Option<char> = iterator.next();
        let inner: u16 = iterator.collect::<String>().parse::<u16>().map_err(|_| ParseMoveError{ source: s.to_string() })?;
        if prefix == Some('L') {
            Ok(Move::L(inner))
        } else if prefix == Some('R') {
            Ok(Move::R(inner))
        } else {
            Err(ParseMoveError{ source: s.to_string() })
        }
    }
}

fn d1p1_parseinput(input: &str) -> Result<Vec<Move>, Box<dyn Error>> {
    input.split("\n").map(|x| x.parse::<Move>().map_err(|e| Box::new(e) as Box<dyn Error>)).collect()
}

pub fn d1p1(input: String) -> Result<u32, Box<dyn Error>> {
    let parsed_input = d1p1_parseinput(&input)?;
    let (_, count): (i32, u32) = parsed_input.iter().fold((DAY1DIALSTART, 0u32), |(sum, count), val| {
        let newsum = match val {
            Move::L(s) => (sum + DAY1DIALSIZE - *s as i32) % DAY1DIALSIZE,
            Move::R(s) => (sum + *s as i32) % DAY1DIALSIZE,
        };
        let newcount = if newsum == 0 { count + 1 } else { count };
        (newsum, newcount)
    });
    // println!("{:?}", parsedInput);
    Ok(count)
}
