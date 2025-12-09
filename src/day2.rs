use std::error::Error;
use std::{fmt, cmp};

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError {
    source: String
}
impl fmt::Display for ParseRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseRangeError: {}", self.source)
    }
}
impl Error for ParseRangeError {}

fn parse_range(input: &str) -> Result<Vec<(u64, u64)>, Box<dyn Error>> {
    input.split(",").map(|x| -> Result<(u64, u64), Box<dyn Error>> {
        let pair = x.split_once("-");
        match pair {
            Some((a, b)) => Ok((a.parse::<u64>()?,
                                b.parse::<u64>()?)),
            None => Err(Box::new(ParseRangeError {source: x.to_string()}))
        }
    }).collect()
}

fn to_even_digits((low, hi): (u64, u64)) -> (u64, u64) {
    let low_dig = low.to_string().len();
    let hi_dig = hi.to_string().len();
    let new_low = if low_dig % 2 == 1 {
        10_u64.pow(low_dig as u32)
    } else {
        low
    };
    let new_hi = if hi_dig % 2 == 1 {
        10_u64.pow((hi_dig - 1) as u32) - 1
    } else {
        hi
    };
    (new_low, new_hi)
}

fn split_halves((low, hi): (u64, u64)) -> ((u64, u64), (u64, u64)) {
    let low_str = low.to_string();
    let hi_str = hi.to_string();
    let low_cutoff = low_str.len() / 2;
    let hi_cutoff = hi_str.len() / 2;
    let lo_msd = low_str[..low_cutoff]
                         .parse::<u64>()
                         .unwrap();
    let lo_lsd = low_str[low_cutoff..]
                         .parse::<u64>()
                         .unwrap();
    let hi_msd = hi_str[..hi_cutoff]
                        .parse::<u64>()
                        .unwrap();
    let hi_lsd = hi_str[hi_cutoff..]
                        .parse::<u64>()
                        .unwrap();
    ((lo_msd, lo_lsd), (hi_msd, hi_lsd))
}

fn half_to_invalid(i: u64) -> u64 {
    format!("{}{}", i, i).parse::<u64>().unwrap()
}

fn generate_invalids(((lo_msd, _lo_lsd), (hi_msd, _hi_lsd)): ((u64, u64), (u64, u64))) -> Vec<u64> {
    (lo_msd..=hi_msd).map(|x| half_to_invalid(x)).collect()
}

pub fn d2p1(input: &str) -> Result<u64, Box<dyn Error>> {
    let ranges: Vec<(u64, u64)> = parse_range(input)?;
    Ok(ranges.into_iter().map(|(low, hi)| {
        let (new_low, new_hi) = to_even_digits((low, hi));
        if new_hi < new_low {
            return 0_u64; // single digit hi
        }
        let (split_low, split_hi) = split_halves((new_low, new_hi));
        let invalids = generate_invalids((split_low, split_hi));
        invalids.into_iter()
                .filter(|&x| x <= hi && x >= low)
                .sum::<u64>()
    }).sum())
}

#[derive(Debug)]
struct Series {
    repetitions: u64,
    digits_per_rep: u64,
    begin: u64,
    end: u64
}

fn num_digits(d: u64) -> u64 {
    d.to_string().len() as u64
}

fn get_divisors(d: u64) -> Vec<u64> {
    let mut divs = vec![];
    for i in 1..d {
        if d % i == 0 {
            divs.push(i);
        }
    }
    divs
}

fn create_series(digits: u64, low: u64, hi: u64) -> Vec<Series> {
    if digits == 1 {
        return vec![];
    }
    let unpadded_low = low.to_string();
    let pad_zeros = &"0000000000000000000"[..digits as usize - unpadded_low.len()];
    let low_str = format!("{}{}", pad_zeros, unpadded_low);
    let hi_str = hi.to_string();
    get_divisors(digits).into_iter().map(|divisor| { 
        let low_bound = low_str[..divisor as usize].parse::<u64>().unwrap();
        let hi_bound = if unpadded_low.len() != hi_str.len() { 
            "99999999999999999"[..digits as usize].parse::<u64>().unwrap()
        } else {
            hi_str[..divisor as usize].parse::<u64>().unwrap()
        };
        Series {
            repetitions: digits / divisor,
            digits_per_rep: divisor,
            begin: low_bound,
            end: cmp::max(hi_bound, low_bound)
    }}).collect()
}

fn invalids(s: &Series) -> Vec<u64> {
    let mut invalids = Vec::new();
    for i in cmp::max(s.begin, 1)..=s.end {
        if num_digits(i) == s.digits_per_rep {
            invalids.push(vec![i.to_string();s.repetitions as usize].join("").parse::<u64>().unwrap());
        }
    }
    invalids
}

pub fn d2p2(input: &str) -> Result<u64, Box<dyn Error>> {
    // Imagine a range with low = x digits and high = y digits
    // There will be a series of repetitions in all divisors of all numbers of digits from x to y
    // e.g. if there are 10 digits there will be a series of repetitions of 1, repetitions of 2, and repetitions of 5
    // So 1212121212 is possible, and 1234512345 is also possible as well as 1111111111
    // For series s,
    // we take the top s digits of the high and low, turn it into an iterator and generate all
    // possible invalids in that iterator
    // Then we filter
    let ranges: Vec<(u64, u64)> = parse_range(input)?;
    Ok(ranges.into_iter().map(|(low, hi)| {
        let (num_digits_low, num_digits_hi) = (num_digits(low), num_digits(hi));
        let mut possible_invalids: Vec<u64> = (num_digits_low..=num_digits_hi).flat_map(|d| {
            let possible_series = create_series(d, low, hi);
            let mut invalid_accs = vec![];
            for i in possible_series.iter() {
                invalid_accs.append(&mut invalids(i));
            }
            invalid_accs.into_iter()
            // possible_series.iter().flat_map(|x| invalids(x).into_iter())
        }).collect();

        possible_invalids.sort();
        possible_invalids.dedup();
        possible_invalids.into_iter()
                         .filter(|&x| x <= hi && x >= low)
                         .sum::<u64>()
    }).sum())
}