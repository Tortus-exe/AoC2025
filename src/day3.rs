use std::fmt;
use std::error::Error;
use std::str::FromStr;
use std::cmp::max;

#[derive(Debug)]
struct BatteryBank {
    batteries: Vec<u32>
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBatteryError {
    source: String
}

impl fmt::Display for ParseBatteryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseMoveError: {}", self.source)
    }
}
impl Error for ParseBatteryError { }

impl FromStr for BatteryBank {
    type Err = ParseBatteryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatteryBank {
            batteries: s.chars()
                        .map(|c| c.to_digit(10)
                                  .ok_or(ParseBatteryError { 
                                      source: s.to_string() 
                                  }))
                        .collect::<Result<Vec<u32>, _>>()?
        })
    }
}

pub fn d3p1(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut parsed_input: Vec<BatteryBank> = input.split("\n").map(|line| line.parse::<BatteryBank>()).collect::<Result<Vec<BatteryBank>, _>>()?;
    
    Ok(parsed_input.iter_mut().map(|bank| {
        let (max_inx, max_fst) = bank.batteries[..bank.batteries.len()-1]
                                   .iter()
                                   .enumerate()
                                   .reduce(|(i1, v1), (i2, v2)| {
                                        if v2 > v1 {
                                            (i2, v2)
                                        } else {
                                            (i1, v1)
                                        }
                                   })
                                   .unwrap();
        let max_snd = bank.batteries[max_inx+1..]
                               .iter()
                               .reduce(max)
                               .unwrap();
        max_fst * 10 + max_snd
    }).sum())
}

fn reconstruct_number(digits: &mut [(usize, u32)]) -> u64 {
    digits.sort_by_key(|&(inx, _)| inx);
    digits.iter()
          .map(|&(_, v)| v.to_string())
          .collect::<Vec<String>>()
          .join("")
          .parse::<u64>()
          .unwrap()
}

pub fn d3p2(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut parsed_input: Vec<BatteryBank> = input.split("\n").map(|line| line.parse::<BatteryBank>()).collect::<Result<Vec<BatteryBank>, _>>()?;

    Ok(parsed_input.iter_mut().map(|bank| {
        let mut buckets: Vec<Vec<(usize, u32)>> = vec![Vec::new();9];
        bank.batteries.iter().enumerate().for_each(|(inx, &num)| {
            buckets[(num-1) as usize].push((inx, num));
        });
        let sorted_refs = &mut buckets.into_iter()
                                      .flatten()
                                      .collect::<Vec<(usize, u32)>>();
        sorted_refs.reverse();
        let a = reconstruct_number(&mut sorted_refs[..2]);
        dbg!(&a)
            a
        // sorted_refs.parse::<u64>().unwrap()
    }).sum())
}