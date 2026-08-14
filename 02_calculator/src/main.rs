use std::{
    collections::{HashMap, hash_map::Entry},
    io::stdin,
};

fn main() {
    let mut memory = Memory::new();
    let mut prev_result: f64 = 0.0;

    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();
        // 메모리 기록
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with('+') {
            memory.add_and_print(tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            memory.add_and_print(tokens[0], -prev_result);
            continue;
        }

        // 수식 계산
        let left: f64 = memory.eval_token(tokens[0]);
        let right: f64 = memory.eval_token(tokens[2]);
        let result = eval_expression(left, tokens[1], right);

        print_output(result);
        prev_result = result
    }
}

fn print_output(value: f64) {
    println!("  => {}", value);
}

struct Memory {
    slots: HashMap<String, f64>,
}
impl Memory {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }

    fn add_and_print(&mut self, token: &str, prev_result: f64) {
        let slot_name = token[3..token.len() - 1].to_string();
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += prev_result;
                print_output(*entry.get());
            }
            Entry::Vacant(entry) => {
                entry.insert(prev_result);
                print_output(prev_result);
            }
        }
    }

    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let slot_name = &token[3..];
            self.slots.get(slot_name).copied().unwrap_or(0.0)
        } else {
            token.parse().unwrap()
        }
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            unreachable!()
        }
    }
}
