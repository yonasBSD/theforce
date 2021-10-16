use std::collections::HashMap;

use crate::ast::{BinaryOperation, Node};

struct Frame {
    variables: HashMap<String, Node>,
    current: Node,
}

impl Frame {
    fn new() -> Frame {
        Frame {
            variables: HashMap::new(),
            current: Node::Noop,
        }
    }
}

struct State {
    functions: HashMap<String, Node>,
    stack: Vec<Frame>,
}

impl State {
    fn new() -> State {
        State {
            functions: HashMap::new(),
            stack: vec![Frame::new()],
        }
    }
}

pub fn evaluate(ast: Vec<Node>) -> Result<(), String> {
    let mut main = Node::Noop;
    let state = &mut State::new();

    for node in ast {
        match &node {
            Node::Main(_) => {
                main = node;
            }
            Node::DeclareFunction(name, _, _) => {
                state.functions.insert(name.to_string(), node);
            }
            _ => unreachable!(), // TODO: Get actual error message
        }
    }

    evaluate_node(main, state)
}

fn evaluate_node(ast: Node, state: &mut State) -> Result<(), String> {
    match ast {
        Node::AssignVariable(variable_name, first_value, operations) => {
            // TODO: Check if value is actually a value?
            // Place value at top of stack
            let _ = evaluate_node(*first_value, state); 
            for operation in operations {
                // TOOD: Actual error message
                let _ = match operation {
                    Node::Binary(operation, value) => evaluate_binary(operation, *value, state),
                    Node::Unary(operation) => todo!(),
                    _ => Err("Invalid operation".to_string()),
                };
            }
            let new_current = state.stack.last().unwrap().current.clone();
            state
                .stack
                .last_mut()
                .unwrap()
                .variables
                .insert(variable_name, new_current);
            Ok(())
        }
        Node::Binary(_, _) => todo!(),
        Node::Boolean(_) => {
            state.stack.last_mut().unwrap().current = ast;
            Ok(())
        }
        Node::CallFunction(_, _) => todo!(),
        Node::DeclareBoolean(name, boolean) => {
            if let Node::Boolean(value) = *boolean {
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name, Node::Boolean(value));
                // TODO: Decide if replacing an existing var is an error
                Ok(())
            } else {
                Err("Not boolean".to_string())
            }
        }
        Node::DeclareFloat(name, float) => {
            if let Node::Float(value) = *float {
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name, Node::Float(value));
                Ok(())
            } else {
                Err("Not float".to_string())
            }
        }
        Node::DeclareFunction(_, _, _) => todo!(),
        Node::DeclareString(name, string) => {
            if let Node::String(value) = *string {
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name, Node::String(value));
                Ok(())
            } else {
                Err("Not string".to_string())
            }
        }
        Node::Float(_) => {
            state.stack.last_mut().unwrap().current = ast;
            Ok(())
        }
        Node::For(_, _, _) => todo!(),
        Node::If(_, _, _) => todo!(),
        Node::Main(statments) => {
            for statment in statments {
                evaluate_node(statment, state)?;
            }
            Ok(())
        }
        Node::Print(node) => {
            evaluate_node(*node, state)?;
            let value = state.stack.last().unwrap().current.clone();
            println!("{}", value);
            Ok(())
        }
        Node::Return(_) => todo!(),
        Node::ReadBoolean(_) => todo!(),
        Node::ReadFloat(_) => todo!(),
        Node::ReadString(_) => todo!(),
        Node::String(_) => {
            state.stack.last_mut().unwrap().current = ast;
            Ok(())
        }
        Node::Unary(_) => todo!(),
        Node::Variable(name) => {
            // Error if not found
            let value = state
                .stack
                .last_mut()
                .unwrap()
                .variables
                .get(&name)
                .unwrap()
                .clone();
            state.stack.last_mut().unwrap().current = value;
            Ok(())
        }
        Node::While(_, _) => todo!(),
        Node::Noop => Ok(()),
    }
}

fn evaluate_binary(op: BinaryOperation, value: Node, state: &mut State) -> Result<(), String> {
    match op {
        BinaryOperation::Add => math_operations(|x, y| x + y, value, state),
        BinaryOperation::Subtract => math_operations(|x, y| x - y, value, state),
        BinaryOperation::Multiply => math_operations(|x, y| x * y, value, state),
        BinaryOperation::Divide => math_operations(|x, y| x / y, value, state),
        BinaryOperation::Exponent => math_operations(|x, y| x.powf(y), value, state),
        BinaryOperation::Modulus => math_operations(|x, y| x % y, value, state),
        BinaryOperation::Equal => todo!(),
        BinaryOperation::GreaterThan => todo!(),
        BinaryOperation::LessThan => todo!(),
        BinaryOperation::Or => todo!(),
        BinaryOperation::And => todo!(),
    }
}

fn math_operations<F>(math_operation: F, value: Node, state: &mut State) -> Result<(), String>
where
    F: Fn(f32, f32) -> f32,
{
    match (state.stack.last().unwrap().current.clone(), value) {
        (Node::Float(float_x), Node::Float(float_y)) => {
            let new_current = Node::Float(math_operation(float_x, float_y));
            state.stack.last_mut().unwrap().current = new_current;
            Ok(())
        }
        (Node::Float(float_x), Node::Variable(var_name)) => {
            let var_value = state
                .stack
                .last()
                .unwrap()
                .variables
                .get(&var_name)
                .unwrap();
            if let Node::Float(float_y) = var_value {
                let new_current = Node::Float(math_operation(float_x, *float_y));
                state.stack.last_mut().unwrap().current = new_current;
                Ok(())
            } else {
                Err("Variable is not float".to_string())
            }
        }
        _ => unreachable!(),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn hello_there() {
//         let ast = vec![Node::Main(vec!(Node::Print(Box::new(Node::String(
//             "Hello there".to_string())))))];

//         assert_eq!(
//             evaluate(ast)

//         );
//     }

//     #[test]
//     fn hello_test() {
//         let mut stdout = Vec::new();

//         // pass fake stdout when calling when testing
//         hello(&mut stdout);

//         assert_eq!(stdout, b"Hello world\n");
//     }
// }
