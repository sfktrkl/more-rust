use stack::stack::*;

fn is_valid_prenthesis(expression: String) -> bool {
    let mut stack = Stack::<char>::new();
    for c in expression.chars() {
        if c == '{' || c == '[' || c == '(' {
            stack.push(c);
        } else if c == '}' || c == ']' || c == ')' {
            if c == '}' && (stack.is_empty() || stack.top() != Some('{')) {
                return false;
            } else if c == ']' && (stack.is_empty() || stack.top() != Some('[')) {
                return false;
            } else if c == ')' && (stack.is_empty() || stack.top() != Some('(')) {
                return false;
            } else {
                stack.pop();
            }
        }
    }
    if stack.is_empty() {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut stack = Stack::<i32>::new();
    stack.push(32);
    stack.push(47);
    stack.push(18);
    stack.push(59);
    stack.push(88);
    stack.push(91);
    stack.print();

    while !stack.is_empty() {
        stack.pop();
    }
    stack.print();

    println!(
        "{{()[{{}}]}} Valid: {}",
        is_valid_prenthesis(String::from("{()[{}]}"))
    );
    println!(
        "{{()[{{}}] Valid: {}",
        is_valid_prenthesis(String::from("{()[{}]"))
    );
    println!(
        "{{()[{{]}} Valid: {}",
        is_valid_prenthesis(String::from("{()[{]}"))
    );
}
