#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        let new_value = match input {
            CalculatorInput::Value(x) => Some(*x),
            CalculatorInput::Add => operate(&mut stack, &|x, y| x + y),
            CalculatorInput::Subtract => operate(&mut stack, &|x, y| x - y),
            CalculatorInput::Multiply => operate(&mut stack, &|x, y| x * y),
            CalculatorInput::Divide => operate(&mut stack, &|x, y| x / y)
        };

        if new_value.is_none() {
            return None;
        }

        stack.push(new_value.unwrap());
    }

    if stack.len() == 1 {
        return stack.pop();
    }

    None
}

/// Performs the operating using the last 2 operands in the stack.
/// Returns None if there are enough enough operands.
fn operate(stack: &mut Vec<i32>, op: &dyn Fn(i32, i32) -> i32) -> Option<i32> {
    if stack.len() < 2 {
        return None;
    }

    let y = stack.pop().unwrap();
    let x = stack.pop().unwrap();
    let result = op(x, y);

    Some(result)
}
