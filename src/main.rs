use std::collections::{HashMap, HashSet};
use crate::cpu::{CPU, Opcode, Registers, Value};

mod cpu;
mod samples_parser;

type Instruction = (usize, usize, usize, usize);

fn main() {
    let opcodes = calculate_opcodes();
    println!("Opcodes: {:?}", opcodes);

    let program_str = include_str!("../data/input.txt");
    let program = program_str.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let instruction = line.split(" ")
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (instruction[0], instruction[1], instruction[2], instruction[3])
        })
        .collect::<Vec<_>>();
    let mut cpu = CPU::new();
    for instruction in program {
        cpu.do_opcode(opcodes[&instruction.0], instruction.1, instruction.2, instruction.3);
    }
    println!("CPU registers after program: {:?}", cpu.registers);
}

fn calculate_opcodes() -> HashMap<Value, Opcode> {
    let samples_str = include_str!("../data/samples.txt");
    let (left_str, samples) = samples_parser::parse_samples(samples_str).unwrap();
    assert!(left_str.is_empty());

    let mut opcodes_intersections: HashMap<Value, HashSet<Opcode>> = HashMap::new();
    for sample in samples {
        let possible_opcodes = find_possible_opcodes(&sample.before, sample.instruction, &sample.after);
        let opcode_intersections = opcodes_intersections
            .entry(sample.instruction.0)
            .or_insert(possible_opcodes.clone().into_iter().collect());
        *opcode_intersections = opcode_intersections
            .intersection(&possible_opcodes.into_iter().collect())
            .map(|opcode| *opcode)
            .collect();
    }
    println!("Possible opcodes for each value: {:?}", opcodes_intersections);

    let mut opcodes: HashMap<Value, Opcode> = HashMap::new();
    while !opcodes_intersections.is_empty() {
        let (&opcode_value, &ref opcode_set) = opcodes_intersections
            .iter()
            .find(|(_, opcodes)| opcodes.len() == 1)
            .unwrap();
        let opcode = *opcode_set.iter().next().unwrap();
        opcodes.insert(opcode_value, opcode);
        opcodes_intersections.remove(&opcode_value);
        for (_, opcode_set) in opcodes_intersections.iter_mut() {
            opcode_set.remove(&opcode);
        }
    }

    opcodes
}

fn find_possible_opcodes(before: &Registers, instruction: Instruction, after: &Registers) -> Vec<Opcode> {
    let possible_opcodes = vec![
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Setr,
        Opcode::Seti,
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
    ];
    possible_opcodes.into_iter()
        .filter(|opcode| {
            let mut cpu = CPU::new();
            cpu.registers = before.clone();
            cpu.do_opcode(*opcode, instruction.1, instruction.2, instruction.3);
            cpu.registers == *after
        })
        .collect()
}