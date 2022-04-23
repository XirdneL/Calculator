pub fn resolve_elem(elem: &String) -> f64 {
    return match elem.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Could not convert {}", elem);
            0.0
        }
    };
}

pub fn perform_operation(first_operand: &String, operator: &String, second_operand: &String) -> f64 {
    let first_operand: f64 = resolve_elem(first_operand);
    let second_operand: f64 = resolve_elem(second_operand);

    match operator.as_str() {
        "+" => return first_operand + second_operand,
        "-" => return first_operand - second_operand,
        "*" => return first_operand * second_operand,
        "/" => {
            if second_operand == 0.0 {
                eprintln!("Error: Dividing by 0");
                return 0.0;
            }
            return first_operand / second_operand
        },
        "%" => return first_operand % second_operand,
        other => {
            eprintln!("Error: Invalid operator {} found", other);
            return 0.0;
        }
    }
}

pub fn solve(mut equation: Vec<String>) -> f64 {
    match equation.len() {
        1 => return resolve_elem(&equation[0]),
        3 => return perform_operation(&equation[0], &equation[1], &equation[2]),
        _ => ()
    }

    // Resolve operations within brackets first
    while equation.contains(&String::from("(")) {
        let mut open_bracket_pos: usize = equation.iter().position(|x| x == "(").unwrap();
        let mut close_bracket_pos: usize = open_bracket_pos + 1;
        while equation[close_bracket_pos] != ")" {
            if equation[close_bracket_pos] == "(" {
                open_bracket_pos = close_bracket_pos;
                close_bracket_pos += 1;
            }
            else {
                close_bracket_pos += 1
            }
        }
        let equation_start_pos: usize = open_bracket_pos + 1;
        let equation_end_pos: usize = close_bracket_pos - 1;
        let sub_equation: Vec<String> = Vec::from_iter(equation[equation_start_pos..=equation_end_pos].iter().cloned()).to_vec();
        equation.drain(open_bracket_pos..=close_bracket_pos);
        equation.insert(open_bracket_pos, solve(sub_equation).to_string());
    }

    while equation.len() > 3 {
        let sub_equation_start_index: usize = 0;
        let sub_equation_end_index: usize = 2;
        let sub_equation: Vec<String> = Vec::from_iter(equation[sub_equation_start_index..=sub_equation_end_index].iter().cloned()).to_vec();
        equation.drain(sub_equation_start_index..=sub_equation_end_index);
        equation.insert(sub_equation_start_index, solve(sub_equation).to_string());
    }

    return solve(equation);
}