// module for decoding assembly instructions into bytecode
use std::str::FromStr;
use crate::{Opcode, Value, Code};

fn split_lines(src: &str) -> Vec<&str> {
    src.split('\n').collect()
}

fn split_words(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

#[derive(Debug, PartialEq)]
enum Chunk {
    Byte(u8),
    Value(Value),
}

// only needs to parse chunks that are not values
fn parse_chunk(chunk: &str) -> Chunk {
    if let Ok(instr) = Opcode::from_str(chunk) {
        Chunk::Byte(instr as u8)
    }  else if let Ok(byte) = chunk.parse::<u8>() {
        Chunk::Byte(byte)
    } else {
        panic!("Invalid chunk: {}", chunk);
    }
}

type Line = (Opcode, Vec<Chunk>, usize);

fn parse_line(line: &str, ln: usize) -> Line {
    let mut words = split_words(line);
    if words.len() == 0 {
        return (Opcode::HALT, vec![], ln);
    }
    let instr = words.remove(0);
    let opcode = Opcode::from_str(instr).unwrap();
    match opcode {
        Opcode::CONST => {
            let chunk = Chunk::Value(Value::from_str(words[0]).unwrap());
            (opcode, vec![chunk], ln)
        }
        _ => {
            let chunks = words.iter().map(|word| parse_chunk(word)).collect();
            (opcode, chunks, ln)
        }
    }
}

fn parse_lines(lines: Vec<&str>) -> Vec<Line> {
    lines.iter().enumerate().map(|line| parse_line(line.1, line.0)).collect()
}



fn assemble_line(line: Line, code: &mut Code) {
    let (opcode, chunks, ln) = line;
    match opcode {
        Opcode::CONST => {
            match chunks[0] {
                Chunk::Value(v) => code.add_const(v),
                _ => panic!("Invalid chunk for CONST: {:?}", chunks[0]),
            };
        }
        _ => {
            /*if opcode.get_offset() != chunks.len() {
                panic!("Invalid number of chunks for {:?}: {:?}", opcode, chunks);
            }*/
            code.write_code(opcode as u8, ln);
            for chunk in chunks {
                match chunk {
                    Chunk::Byte(b) => code.write_code(b, ln),
                    Chunk::Value(v) => {
                        let byte = code.add_const(v) as u8;
                        code.write_code(byte, ln)
                    }
                }
            }
        }
    };
}

fn assemble_lines(lines: Vec<Line>, code: &mut Code) {
    for line in lines {
        assemble_line(line, code);
    }
}

fn assemble(src: &str) -> Code {
    let mut code = Code::new();
    let lines = split_lines(src);
    let parsed = parse_lines(lines);
    assemble_lines(parsed, &mut code);
    // get rid of terminating HALT
    code.raw.pop();
    code
}

//tests assemble function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_word() {
        let line = "LOAD 0 1";
        let words = split_words(line);
        assert_eq!(words, vec!["LOAD", "0", "1"]);
    }

    #[test]
    fn test_split_line() {
        let line = "LOAD 0 1\nADD 0 1\nHALT\n";
        let words = split_words(line);
        assert_eq!(words, vec!["LOAD", "0", "1", "ADD", "0", "1", "HALT"]);
    }

    #[test]
    fn test_parse_line() {
        let line = "LOAD 0 1";
        let parsed = parse_line(line, 0);
        assert_eq!(parsed.0, Opcode::LOAD);
        assert_eq!(parsed.1.len(), 2);
    }

    #[test]
    fn test_parse_const_line() {
        let line = "CONST 0";
        let parsed = parse_line(line, 0);
        assert_eq!(parsed.0, Opcode::CONST);
        assert_eq!(parsed.1[0], Chunk::Value(Value::I8(0i8)));
    }

    #[test]
    fn test_assemble() {
        let src = "LOAD 0 1";
        let code = assemble(src);
        assert_eq!(code.raw, vec![Opcode::LOAD as u8, 0, 1]);
    }

    #[test]
    fn test_assemble2() {
        let src = "CONST 0\nCONST 1\nLOAD 0 1\nCONST 2\n";
        let code = assemble(src);
        assert_eq!(code.const_pool, vec![Value::I8(0i8), Value::I8(1i8), Value::I8(2i8)]);
    }

    #[test]
    fn test_assemble3() {
        let src = "CONST 0\nLOAD 0 1\nHALT\n";
        let code = assemble(src);
        assert_eq!(code.raw, vec![Opcode::LOAD as u8, 0, 1, 17]);
    }
}