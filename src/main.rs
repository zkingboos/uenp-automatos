use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{stdin, BufRead, BufReader, Write}, time::Instant,
};

#[derive(Deserialize, Serialize, Debug)]
struct InputAutomaton {
    pub initial: u16,
    #[serde(rename = "final")]
    pub final_steps: Vec<u16>,
    pub transitions: Vec<Transition>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Transition {
    pub from: u16,
    pub to: u16,
    pub read: char,
}

#[derive(Debug)]
struct Automaton {
    initial_state: u16,
    steps: HashMap<(u16, char), u16>,
    results: Vec<u16>,
}

impl Automaton {
    fn new(initial_state: u16, results: Vec<u16>) -> Self {
        Self {
            initial_state,
            steps: HashMap::new(),
            results,
        }
    }

    fn add_transition(&mut self, from: u16, read: char, to: u16) {
        self.steps.insert((from, read), to);
    }

    fn get_next_step(&self, from: u16, read: char) -> Option<u16> {
        self.steps.get(&(from, read)).copied()
    }

    fn step_transition(&self, state: u16, input: Option<&str>) -> Option<u16> {
        let input = input?;
        if input.is_empty() {
            return Some(state);
        }

        let mut input_chars = input.chars();
        let current_symbol = input_chars.next()?;
        let remaining_symbols = input_chars.as_str();

        self.get_next_step(state, current_symbol)
            .and_then(|next_step| self.step_transition(next_step, Some(remaining_symbols)))
    }

    fn execute(&self, input: Option<&str>) -> bool {
        self.step_transition(self.initial_state, input)
            .map_or(false, |final_state| self.results.contains(&final_state))
    }
}

fn read_input_default(default_name: &str) -> String {
    let mut file_buffer = String::new();
    stdin()
        .read_line(&mut file_buffer)
        .expect("Error reading line");

    match file_buffer.trim_end() {
        "" => default_name.to_string(),
        file => file.to_string(),
    }
}


fn main() -> Result<()> {
    println!("Enter the path to the instruction file (instruction.json): ");
    let file_instruction_name = read_input_default("instruction.json");
    let file_instruction =
        File::open(file_instruction_name).expect("Error opening the provided file");

    let instruction_reader = BufReader::new(file_instruction);
    let input: InputAutomaton = serde_json::from_reader(instruction_reader)?;

    let mut automaton = Automaton::new(input.initial, input.final_steps);
    for transition in input.transitions {
        automaton.add_transition(transition.from, transition.read, transition.to);
    }

    println!("{:?}", automaton);

    println!("Enter the path to the input file (input.csv): ");
    let file_input_name = read_input_default("input.csv");

    let file_input = File::open(file_input_name).expect("Error opening the provided file");
    let input_reader = BufReader::new(file_input);

    let mut results: Vec<String> = Vec::new();
    let mut lines = input_reader.lines();
    while let Some(line) = lines.next() {
        let line = line.expect("Error reading line");
        let mut parts = line.trim().split(";");
        let input = parts.next();
        let expected_output = parts.next().expect("Error reading expected output");
        let current_time = Instant::now();

        let result = automaton.execute(input);
        let evaluation_time = current_time.elapsed().as_secs_f64() * 1000.0;
        let result_output = match result {
            true => "1",
            false => "0",
        };

        results.push(format!(
            "{};{};{};{}",
            input.unwrap_or_default(),
            expected_output,
            result_output,
            evaluation_time
        ));
    }

    println!("Enter the path of the output file (output.csv): ");
    let file_output_name = read_input_default("output.csv");
    let mut file_output = File::create(file_output_name).expect("Error creating the output file");
    file_output
        .write(results.join("\n").as_bytes())
        .expect("Error writing to the output file");
    Ok(())
}