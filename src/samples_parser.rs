use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::IResult;
use nom::multi::{separated_list1};
use crate::cpu::{Registers, Value};
use crate::Instruction;

pub struct Sample {
    pub before: Registers,
    pub instruction: Instruction,
    pub after: Registers,
}

pub fn parse_samples(input: &str) -> IResult<&str, Vec<Sample>> {
    separated_list1(line_ending, parse_sample)(input)
}

fn parse_sample(input: &str) -> IResult<&str, Sample> {
    let (input, before) = parse_before(input)?;
    let (input, instruction) = parse_instruction(input)?;
    let (input, after) = parse_after(input)?;
    Ok((input, Sample { before, instruction, after }))
}

fn parse_before(input: &str) -> IResult<&str, Registers> {
    let (input, _) = tag("Before: ")(input)?;
    let (input, registers) = parse_registers(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, registers))
}

fn parse_after(input: &str) -> IResult<&str, Registers> {
    let (input, _) = tag("After:  ")(input)?;
    let (input, registers) = parse_registers(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, registers))
}

fn parse_registers(input: &str) -> IResult<&str, Registers> {
    let (input, _) = tag("[")(input)?;
    let (input, registers) = separated_list1(
        tag(", "),
        digit1,
    )(input)?;
    let (input, _) = tag("]")(input)?;
    let registers = registers.iter()
        .map(|register| register.parse::<Value>().unwrap())
        .collect::<Vec<_>>();
    Ok((input, [registers[0], registers[1], registers[2], registers[3]]))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, instruction) = separated_list1(
        tag(" "),
        digit1,
    )(input)?;
    let instruction = instruction.iter()
        .map(|register| register.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (input, _) = line_ending(input)?;
    Ok((input, (instruction[0], instruction[1], instruction[2], instruction[3])))
}