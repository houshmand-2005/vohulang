use regex::Regex;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let input_string: String = open_file();
    let mut number_list: Vec<(String, i32)> = Vec::new();
    let mut string_list: Vec<(String, String)> = Vec::new();
    for line in input_string.lines() {
        let n_values: Regex = Regex::new(r"N!(.*?)\] > \((.*?)\)").unwrap();
        if let Some(_captures) = n_values.captures(line) {
            n_val(line, n_values, &mut number_list);
        }
        let s_values: Regex = Regex::new(r"S!(.*?)\] > \((.*?)\)").unwrap();
        if let Some(_captures) = s_values.captures(line) {
            s_val(line, s_values, &mut string_list);
        }
        let s_inp_values: Regex = Regex::new(r"INP!(.*?)\] > \(S!(.*?)\)").unwrap();
        if let Some(_captures) = s_inp_values.captures(line) {
            s_inp(line, s_inp_values, &mut string_list);
        }
        let ps_values: Regex = Regex::new(r"\[PS!(.*?)\]").unwrap();
        if let Some(_captures) = ps_values.captures(line) {
            s_ps(line, ps_values, &mut string_list, &mut number_list);
        }
        let plus_statement: Regex = Regex::new(r"\+\!(.*?)\] > \((.*?)\)").unwrap();
        if let Some(_captures) = plus_statement.captures(line) {
            plus_fn(line, plus_statement, &mut string_list, &mut number_list);
        }
        let minus_statement: Regex = Regex::new(r"\-\!(.*?)\] > \((.*?)\)").unwrap();
        if let Some(_captures) = minus_statement.captures(line) {
            minus_fn(line, minus_statement, &mut string_list, &mut number_list);
        }
        let multiply_statement: Regex = Regex::new(r"\*\!(.*?)\] > \((.*?)\)").unwrap();
        if let Some(_captures) = multiply_statement.captures(line) {
            multiply_fn(line, multiply_statement, &mut string_list, &mut number_list);
        }
        let divide_statement: Regex = Regex::new(r"\/\!(.*?)\] > \((.*?)\)").unwrap();
        if let Some(_captures) = divide_statement.captures(line) {
            divide_fn(line, divide_statement, &mut string_list, &mut number_list);
        }
        let for_statement: Regex = Regex::new(r"FOR\!(.*?)\] > \(PS!(.*?)\)").unwrap();
        if let Some(_captures) = for_statement.captures(line) {
            for_statement_fn(line, for_statement, &mut string_list, &mut number_list);
        }
        let end_statement: Regex = Regex::new(r"\[END\]").unwrap();
        if let Some(_captures) = end_statement.captures(line) {
            std::process::exit(1);
        }
    }
}

fn n_val(line: &str, n_values: regex::Regex, number_list: &mut Vec<(String, i32)>) {
    for item in n_values.captures_iter(line) {
        let name: String = item.get(1).unwrap().as_str().trim().to_string();
        let value: &str = item.get(2).unwrap().as_str();
        match value.parse::<i32>() {
            Ok(parsed_value) => number_list.push((name, parsed_value)),
            Err(_) => {
                println!("'{}' must be an int, not a str", name);
                std::process::exit(1);
            }
        }
    }
}
fn s_val(line: &str, s_values: regex::Regex, string_list: &mut Vec<(String, String)>) {
    for item in s_values.captures_iter(line) {
        let name: String = item
            .get(1)
            .unwrap()
            .as_str()
            .trim_matches('"')
            .trim()
            .to_string();
        let value: &str = item.get(2).unwrap().as_str().trim_matches('"');
        string_list.push((name, value.to_string()))
    }
}
fn s_inp(line: &str, s_inp_values: regex::Regex, string_list: &mut Vec<(String, String)>) {
    for item in s_inp_values.captures_iter(line) {
        let name: String = item.get(1).unwrap().as_str().trim().to_string();
        let message: String = item.get(2).unwrap().as_str().trim().to_string();
        let mut value: String = String::new();
        println!("{}", message);
        match io::stdin().read_line(&mut value) {
            Ok(_) => {
                let value: &str = value.trim();
                string_list.push((name, value.to_string()))
            }
            Err(error) => {
                println!("Error: {}", error);
                std::process::exit(1)
            }
        }
    }
}
fn s_ps(
    line: &str,
    ps_values: regex::Regex,
    string_list: &mut Vec<(String, String)>,
    number_list: &mut Vec<(String, i32)>,
) {
    for item in ps_values.captures_iter(line) {
        let name: String = item.get(1).unwrap().as_str().trim().to_string();
        let _rt: Option<String> = get_variables(name, string_list, number_list);
        if let Some(val) = _rt {
            println!("{}", val)
        } else {
            let name: String = item.get(1).unwrap().as_str().trim().to_string();
            println!("{} variable didn't found [ERROR V0]", name)
        }
    }
}
fn plus_fn(
    line: &str,
    plus_statement: regex::Regex,
    string_list: &mut Vec<(String, String)>,
    number_list: &mut Vec<(String, i32)>,
) {
    for item in plus_statement.captures_iter(line) {
        let name: String = item.get(1).unwrap().as_str().trim().to_string();
        let numbers = item.get(2).unwrap().as_str().split('+');
        let mut sum_n: i32 = 0;
        for number in numbers {
            match number.trim().parse::<i32>() {
                Ok(parsed_value) => {
                    sum_n += parsed_value;
                }
                Err(_) => {
                    let number: Option<String> =
                        get_variables(number.trim().to_string(), string_list, number_list);
                    if let Some(number) = number {
                        let number: i32 = number.parse().unwrap();
                        sum_n += number;
                    } else {
                        println!("{} variable didn't found [ERROR V0]", name);
                        std::process::exit(1);
                    }
                }
            }
        }
        string_list.push((name, sum_n.to_string()))
    }
}
fn minus_fn(
    line: &str,
    minus_statement: regex::Regex,
    string_list: &mut Vec<(String, String)>,
    number_list: &mut Vec<(String, i32)>,
) {
    for item in minus_statement.captures_iter(line) {
        let name: String = item.get(1).unwrap().as_str().trim().to_string();
        let numbers = item.get(2).unwrap().as_str().split('-');
        let mut sum_n: i32 = 0;
        for number in numbers {
            match number.trim().parse::<i32>() {
                Ok(parsed_value) => {
                    if sum_n == 0 {
                        sum_n = parsed_value
                    } else {
                        sum_n -= parsed_value;
                    }
                }
                Err(_) => {
                    let number: Option<String> =
                        get_variables(number.trim().to_string(), string_list, number_list);
                    if let Some(number) = number {
                        let number: i32 = number.parse().unwrap();
                        if sum_n == 0 {
                            sum_n = number
                        } else {
                            sum_n -= number;
                        }
                    } else {
                        println!("{} variable didn't found [ERROR V0]", name);
                        std::process::exit(1);
                    }
                }
            }
        }
        string_list.push((name, sum_n.to_string()))
    }
}
fn multiply_fn(
    line: &str,
    multiply_statement: regex::Regex,
    string_list: &mut Vec<(String, String)>,
    number_list: &mut Vec<(String, i32)>,
) {
    for item in multiply_statement.captures_iter(line) {
        let name: String = item.get(1).unwrap().as_str().trim().to_string();
        let numbers = item.get(2).unwrap().as_str().split('*');
        let mut sum_n: i32 = 0;
        for number in numbers {
            match number.trim().parse::<i32>() {
                Ok(parsed_value) => {
                    if sum_n == 0 {
                        sum_n = parsed_value
                    } else {
                        sum_n = parsed_value * sum_n;
                    }
                }
                Err(_) => {
                    let number: Option<String> =
                        get_variables(number.trim().to_string(), string_list, number_list);
                    if let Some(number) = number {
                        let number: i32 = number.parse().unwrap();
                        if sum_n == 0 {
                            sum_n = number
                        } else {
                            sum_n = number * sum_n;
                        }
                    } else {
                        println!("{} variable didn't found [ERROR V0]", name);
                        std::process::exit(1);
                    }
                }
            }
        }
        string_list.push((name, sum_n.to_string()))
    }
}
fn divide_fn(
    line: &str,
    divide_statement: regex::Regex,
    string_list: &mut Vec<(String, String)>,
    number_list: &mut Vec<(String, i32)>,
) {
    for item in divide_statement.captures_iter(line) {
        let name: String = item.get(1).unwrap().as_str().trim().to_string();
        let numbers = item.get(2).unwrap().as_str().split('/');
        let mut sum_n: f32 = 0.0;
        for number in numbers {
            match number.trim().parse::<f32>() {
                Ok(parsed_value) => {
                    if sum_n == 0.0 {
                        sum_n = parsed_value
                    } else {
                        sum_n = sum_n / parsed_value;
                    }
                }
                Err(_) => {
                    let number: Option<String> =
                        get_variables(number.trim().to_string(), string_list, number_list);
                    if let Some(number) = number {
                        let number: f32 = number.parse().unwrap();
                        if sum_n == 0.0 {
                            sum_n = number
                        } else {
                            sum_n = sum_n / number;
                        }
                    } else {
                        println!("{} variable didn't found [ERROR V0]", name);
                        std::process::exit(1);
                    }
                }
            }
        }
        string_list.push((name, sum_n.to_string()))
    }
}
fn for_statement_fn(
    line: &str,
    for_statement: regex::Regex,
    string_list: &mut Vec<(String, String)>,
    number_list: &mut Vec<(String, i32)>,
) {
    for item in for_statement.captures_iter(line) {
        let time: String = item.get(1).unwrap().as_str().trim().to_string();
        let name: String = item.get(2).unwrap().as_str().trim().to_string();
        match time.parse::<i32>() {
            Ok(mut parsed_value) => {
                if parsed_value < 1 {
                    println!("The minimum loop iteration count must be 1 or more");
                    std::process::exit(1);
                }
                while parsed_value > 0 {
                    let name: Option<String> =
                        get_variables(name.clone(), string_list, number_list);
                    if let Some(name) = name {
                        println!("{}", name)
                    } else {
                        println!("{:?} variable didn't found [ERROR V0]", name);
                        std::process::exit(1);
                    }
                    parsed_value -= 1
                }
            }
            Err(_) => {
                println!(
                    "{} you must enter an integer for time like:\n [FOR! 5] > (PS! name) ",
                    name
                );
                std::process::exit(1);
            }
        }
    }
}
fn get_variables(
    name: String,
    string_list: &mut Vec<(String, String)>,
    number_list: &mut Vec<(String, i32)>,
) -> Option<String> {
    for (var_name, var_value) in string_list.iter().rev() {
        if name == var_name.to_string() {
            return Some(var_value.clone());
        }
    }
    for (var_name, var_value) in number_list.iter().rev() {
        if name == var_name.to_string() {
            return Some(var_value.to_string());
        }
    }
    None
}
fn open_file() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide the program file as a command-line argument");
        println!("Like 'cargo run your_program.vo' or");
        println!("For compiled veriosn 'vohulang your_program.vo'");
        std::process::exit(1);
    }
    let program_file: &String = &args[1];
    let mut file: File = File::open(program_file).expect("File not found");
    let mut data: String = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let start_regex: Regex = Regex::new(r"\[START\]").unwrap();
    let end_regex: Regex = Regex::new(r"\[END\]").unwrap();
    if start_regex.is_match(&data) {
    } else {
        println!("you never start the app");
        std::process::exit(1);
    }
    if end_regex.is_match(&data) {
    } else {
        println!("you never close the app");
        std::process::exit(1);
    }
    return data;
}
