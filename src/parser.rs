use std::cmp::Ordering;

pub fn get_user_input() -> String {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    return args.join("").split_whitespace().collect();
}

pub fn is_trivial_elem(curr_elem: &String, prev_elem: &String) -> bool {
    let curr_elem_is_operand: bool = is_operand(curr_elem);
    let prev_elem_is_operand: bool = is_operand(prev_elem);

    return curr_elem_is_operand
        || curr_elem == "+"
        || curr_elem == "-"
        || curr_elem == ")"
        || (curr_elem == "(" && !prev_elem_is_operand && prev_elem != ")");
}

pub fn is_operand(string: &String) -> bool {
    if string.chars().all(char::is_numeric) {
        return true;
    }

    return match string.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false
    };
}

pub fn add_open_bracket(mut equation: Vec<String>, operator_index: usize) -> Vec<String> {
    if is_operand(&equation[operator_index - 1]) {
        equation.insert(operator_index - 1, String::from("("));
    }
    else if &equation[operator_index - 1] == ")" {
        let mut num_brackets_to_match: u32 = 1;
        let mut iter: usize = operator_index - 2;
        loop {
            match equation[iter].as_str() {
                "(" => num_brackets_to_match -= 1,
                ")" => num_brackets_to_match += 1,
                _ => ()
            }
            match num_brackets_to_match {
                0 => break,
                _ => {
                    if iter == 0 {
                        equation.insert(iter, String::from("("));
                        break;
                    }
                    iter -= 1;
                }
            }
        }
        equation.insert(iter, String::from("("));
    }
    return equation;
}

pub fn add_close_bracket(mut equation: Vec<String>, operator_index: usize) -> Vec<String> {
    if is_operand(&equation[operator_index + 1]) {
        equation.insert(operator_index + 2, String::from(")"));
    }
    else if equation[operator_index + 1] == "(" {
        let mut num_brackets_to_match: u32 = 1;
        let mut iter: usize = operator_index + 2;
        while iter < equation.len() {
            match equation[iter].as_str() {
                ")" => num_brackets_to_match -= 1,
                "(" => num_brackets_to_match += 1,
                _ => ()
            }
            match num_brackets_to_match {
                0 => break,
                _ => iter += 1
            }
        }
        equation.insert(iter, String::from(")"));
    }
    return equation;
}

pub fn convert_equation_string_to_vec(equation_string: String) -> Vec<String> {
    let valid_operators: Vec<char> = vec!['+', '-', '*', '/', '%', '(', ')'];
    let mut equation: Vec<String> = Vec::new();
    let mut operand: String = String::from("");
    for i in equation_string.chars() {
        match i.is_numeric() {
            true => operand += &i.to_string(),
            false => {
                if i == '.' {
                    if operand.is_empty() {
                        equation.push(i.to_string());
                        return equation; // Early return with invalid decimal
                    }
                    else {
                        operand += &i.to_string();
                        continue;
                    }
                }

                if !valid_operators.contains(&i) {
                    equation.push(i.to_string());
                    return equation; // Early return with invalid operator
                }

                if !operand.is_empty() {
                    equation.push(operand);
                    operand = String::from("");
                }

                equation.push(i.to_string());
            }
        }
    }

    if !operand.is_empty() {
        equation.push(operand);
    }

    return equation;
}

pub fn get_equation() -> Option<Vec<String>> {
    let mut equation: Vec<String> = convert_equation_string_to_vec(get_user_input());
    if !is_equation_valid(&equation) {
        return None;
    }

    // Format equation with brackets and explicit multiplication sign
    let mut i: usize = 1;
    while i < equation.len() {
        let curr_elem: &String = &equation[i];
        let prev_elem: &String = &equation[i - 1];

        if is_trivial_elem(curr_elem, prev_elem) {
            i += 1;
            continue;
        }

        // Check for implicit multiplication format. e.g. "x(y + z)". Need to add explicit '*' operator
        if curr_elem == "(" {
            equation.insert(i, String::from("*"));
        }

        // Add brackets for current operator
        equation = add_open_bracket(equation, i);
        equation = add_close_bracket(equation, i + 1);
        i += 3;
    }

    return Some(equation);
}

pub fn is_equation_valid(equation: &Vec<String>) -> bool {
    if equation.is_empty() {
        eprintln!("Error: No equation found, please enter an equation. Example usage: ./calculator \"1+3*5\"");
        return false;
    }

    let valid_operators: Vec<char> = vec!['+', '-', '*', '/', '%', '(', ')'];
    let last_elem: &String = &equation[equation.len() - 1];
    let last_elem_is_operand: bool = is_operand(last_elem);

    if !last_elem_is_operand {
        for elem in last_elem.chars() {
            if elem == '.' {
                eprintln!("Error: Invalid floating point number found. Please enter in format x.y. e.g. 10.58");
                return false;
            }

            if !valid_operators.contains(&elem) {
                eprintln!("Error: Invalid operator {} found. Calculator only supports '+', '-', '*', '/', '%'", elem);
                return false;
            }
        }
    }

    let first_elem: &String = &equation[0];
    let first_elem_is_operand: bool = is_operand(first_elem);
    if !first_elem_is_operand && first_elem != "(" {
        eprintln!("Error: Operation cannot start with operator '{}'", first_elem);
        return false;
    }

    if !last_elem_is_operand && last_elem != ")" {
        eprintln!("Error: Equation cannot end with operator '{}'", last_elem);
        return false;
    }

    let num_opening_brackets: usize = equation.iter().filter(|n| *n == "(").count();
    let num_closing_brackets: usize = equation.iter().filter(|n| *n == ")").count();

    // Minimum number of operands/operators excluding brackets should be 3
    if equation.len() - num_opening_brackets - num_closing_brackets < 3 {
        eprintln!("Error: Invalid equation");
        return false;
    }

    return match num_opening_brackets.cmp(&num_closing_brackets) {
        Ordering::Less => {
            eprintln!("Error: {} extra closing bracket(s) found", num_closing_brackets - num_opening_brackets);
            false
        }
        Ordering::Greater => {
            eprintln!("Error: {} extra opening bracket(s) found", num_opening_brackets - num_closing_brackets);
            false
        }
        Ordering::Equal => true
    };
}