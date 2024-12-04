use nom::bytes::complete::{tag, take, take_while_m_n};
use nom::character::complete::char;
use nom::combinator::map_res;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;
use std::error::Error;

pub fn part_1(input: &str) -> Option<i32> {
    let mut result = 0;
    let mut input = input;
    loop {
        match mul_expr(input) {
            Ok((i, o)) => {
                result += o;
                input = i;
            }
            Err(_) => match take::<usize, &str, ()>(1usize)(input) {
                Ok((i, _)) => {
                    input = i;
                }
                Err(_) => {
                    break;
                }
            },
        }
    }
    Some(result)
}

fn from_digits(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}

fn multiply((a, b): (i32, i32)) -> Result<i32, Box<dyn Error>> {
    Ok(a * b)
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn triple_dig(i: &str) -> IResult<&str, i32> {
    map_res(take_while_m_n(1, 3, is_digit), from_digits)(i)
}

fn mul_expr(i: &str) -> IResult<&str, i32> {
    preceded(
        tag("mul"),
        delimited(
            char('('),
            map_res(separated_pair(triple_dig, char(','), triple_dig), multiply),
            char(')'),
        ),
    )(i)
}

pub fn part_2(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_from_digits() {
        let result = mul_expr("mul(2,4)");
        assert_eq!(result.unwrap(), ("", 8));
    }

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, None);
    }
}
