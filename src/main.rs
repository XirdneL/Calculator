mod parser;
mod solver;

fn main() {
    let equation: Vec<String> = match parser::get_equation() {
        None => return,
        Some(val) => val
    };
    println!("{} = {}", parser::get_user_input(), solver::solve(equation));
}
