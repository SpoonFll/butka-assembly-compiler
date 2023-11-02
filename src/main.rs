use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::prelude::*;
use std::ops::Deref;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("argument expected please give argument.");
        process::exit(0x0100);
    }
    let mut outPath: String = String::new();
    /**
     * if no argument specified output as a.out
     */
    match args.get(2) {
        None => outPath = "a.out".to_string(),
        _ => outPath = args.get(2).unwrap().deref().to_string(),
    }
    let contents = getContents(args);
    let processedContents = processFile(contents);
    let binary = compile(processedContents);
    writeBinary(binary, outPath);
}
/**
 * write the bin array from compile()
 * to the specified or default path
 */
fn writeBinary(bin: [u16; 32], path: String) {
    let mut file = fs::File::create(path);
    let mut binArray: [u8; 64] = [0; 64];
    let mut i = 0;
    for byt in bin.iter() {
        binArray[i] = byt.to_be_bytes()[0];
        binArray[i + 1] = byt.to_be_bytes()[1];
        i += 2;
    }
    file.unwrap().write_all(&binArray.as_slice());
}
/**
 * give a string parsed from file to here to translate to a
 * register value thankyou lex for pointing this out
 */
fn loadRegister(element: &String) -> u16 {
    let mut reg: u16 = 0;
    match element.as_str().to_uppercase().deref() {
        "R0" => reg = 0,
        "R1" => reg = 1,
        "R2" => reg = 2,
        "R3" => reg = 3,
        "R4" => reg = 4,
        "R5" => reg = 5,
        "R6" => reg = 6,
        "R7" => reg = 7,
        "R8" => reg = 8,
        "R9" => reg = 9,
        "R10" => reg = 10,
        "R11" => reg = 11,
        "R12" => reg = 12,
        "R13" => reg = 13,
        "R14" => reg = 14,
        "R15" => reg = 15,
        _ => {
            println!("Invalid input");
            process::exit(0x02);
        }
    }
    reg
}
/**
 * traverse the now structured data to become a
 * binary array that can be outputed to a file
 */
fn compile(buf: Vec<Vec<String>>) -> [u16; 32] {
    let mut bin: [u16; 32] = [0; 32];
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut add: u16 = 0;
    for line in buf.iter() {
        let instruction = line.get(0).unwrap();

        if instruction.chars().last().unwrap() == ':' {
            let chopped = &instruction[0..instruction.len() - 1].to_string();
            labels.insert(chopped.clone(), j);
            if j > 0 {
                j -= 1;
                add = 1;
            } else {
                add = 0;
            }
        }
        j += add;
    }
    for line in buf.iter() {
        let instruction = line.get(0).unwrap();
        if instruction == "NOP" || instruction == "nop" {
            bin[i as usize] = 0x0000;
        } else if instruction == "ADD" || instruction == "add" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let regB: u16 = loadRegister(line.get(2).unwrap());
            bin[i as usize] = (0x0001 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "LDI" || instruction == "ldi" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let mut immediate: u16 = line.get(2).unwrap().parse().unwrap();
            immediate = immediate & 0x00FF;
            bin[i as usize] = (0x0002 << 12) | (regA << 8) | immediate;
        } else if instruction == "SUB" || instruction == "sub" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let regB: u16 = loadRegister(line.get(2).unwrap());
            bin[i as usize] = (0x0003 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "INV" || instruction == "inv" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            bin[i as usize] = (0x0004 << 12) | (regA << 8);
        } else if instruction == "AND" || instruction == "and" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let regB: u16 = loadRegister(line.get(2).unwrap());
            bin[i as usize] = (0x0005 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "OR" || instruction == "or" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let regB: u16 = loadRegister(line.get(2).unwrap());
            bin[i as usize] = (0x0006 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "XOR" || instruction == "xor" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let regB: u16 = loadRegister(line.get(2).unwrap());
            bin[i as usize] = (0x0007 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "MOV" || instruction == "mov" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let regB: u16 = loadRegister(line.get(2).unwrap());
            bin[i as usize] = (0x0008 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "SR" || instruction == "sr" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let regB: u16 = loadRegister(line.get(2).unwrap());
            bin[i as usize] = (0x0009 << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "SL" || instruction == "sl" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let regB: u16 = loadRegister(line.get(2).unwrap());
            bin[i as usize] = (0x000A << 12) | (regA << 8) | (regB << 4);
        } else if instruction == "IN" || instruction == "in" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let mut immediate: u16 = line.get(2).unwrap().parse().unwrap();
            immediate = immediate & 0x00FF;
            bin[i as usize] = (0x000B << 12) | (regA << 8) | (immediate << 4);
        } else if instruction == "OUT" || instruction == "out" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let mut immediate: u16 = line.get(2).unwrap().parse().unwrap();
            immediate = immediate & 0x00FF;
            bin[i as usize] = (0x000C << 12) | (regA << 8) | immediate;
        } else if instruction == "JZ" || instruction == "jz" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let label = labels.get(line.get(2).unwrap()).unwrap();
            bin[i as usize] = (0x000D << 12) | (regA << 8) | label;
        } else if instruction == "JLT" || instruction == "jlt" {
            let regA: u16 = loadRegister(line.get(1).unwrap());
            let label = labels.get(line.get(2).unwrap()).unwrap();
            bin[i as usize] = (0x000E << 12) | (regA << 8) | label;
        } else if instruction == "J" || instruction == "j" {
            let label = labels.get(line.get(1).unwrap()).unwrap();
            bin[i as usize] = (0x000F << 12) | label;
        } else {
            if instruction.chars().last().unwrap() != ':' {
                println!("Invalid instruction: {:?}", instruction);
                process::exit(0x1111);
            } else {
                if i > 0 {
                    i -= 1;
                    add = 1;
                } else {
                    add = 0;
                }
            }
        }
        i += add;
    }
    bin
}

/**
 * convert string file to a structure of instructions to
 * be sent to compile()
 */
fn processFile(buf: String) -> Vec<Vec<String>> {
    let mut retValue: Vec<Vec<String>> = Vec::new();

    for line in buf.lines() {
        let linetemp: Vec<&str> = line.split(" ").collect();
        let mut tempVec: Vec<String> = Vec::new();
        for element in linetemp.iter() {
            tempVec.push(element.to_string());
        }
        retValue.push(tempVec.clone());
    }
    retValue
}

/**
 * from the args for the file specified to get a string buffer
 */
fn getContents(args: Vec<String>) -> String {
    let contents = fs::read_to_string(args.get(1).unwrap()).expect("could not open file");
    contents
}
