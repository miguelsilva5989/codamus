mod value_types;

use parser::{
    ast::{ArithmeticExpression, Oper, Statement},
    Program,
};

use value_types::ValueType;

#[derive(Debug)]
pub struct RuntimeValue {
    r#type: ValueType,
}

fn evaluate_numeric_arithmetic_expression(left: ValueType, right: ValueType, operator: Oper) -> ValueType {
    match operator {
        Oper::Add => left + right,
        Oper::Sub => left - right,
        Oper::Mul => left * right,
        Oper::Div => left / right,
        Oper::Mod => left % right,
    }
}

fn eval_left_right(left: ArithmeticExpression, right: ArithmeticExpression, operator: Oper) -> RuntimeValue {
    let left = evaluate(Statement::ArithmeticExpression(left));
    let right = evaluate(Statement::ArithmeticExpression(right));


    if matches!(left.r#type, ValueType::Number(_)) && matches!(left.r#type, ValueType::Number(_)) {
        return RuntimeValue {
            r#type: evaluate_numeric_arithmetic_expression(left.r#type.into(), right.r#type.into(), operator),
        };
    }

    return RuntimeValue { r#type: ValueType::None };
}

fn evaluate_arithmetic_expression(expr: ArithmeticExpression) -> RuntimeValue {
    match expr {
        ArithmeticExpression::Value(val) => RuntimeValue { r#type: ValueType::Number(val) },
        ArithmeticExpression::Identifier(_) => todo!(),
        ArithmeticExpression::Add(left, right) => eval_left_right(*left, *right, Oper::Add),
        ArithmeticExpression::Sub(left, right) => eval_left_right(*left, *right, Oper::Sub),
        ArithmeticExpression::Mul(left, right) => eval_left_right(*left, *right, Oper::Mul),
        ArithmeticExpression::Div(left, right) => eval_left_right(*left, *right, Oper::Div),
        ArithmeticExpression::Mod(left, right) => eval_left_right(*left, *right, Oper::Mod),
        ArithmeticExpression::Paren(_) => todo!(),
    }
}

fn evaluate(ast_node: Statement) -> RuntimeValue {
    match ast_node {
        Statement::Comment(_) => RuntimeValue { r#type: ValueType::None },
        // Statement::Identifier(_) => todo!(),
        Statement::NumericLiteral(val) => RuntimeValue {
            r#type: ValueType::Number(val.value),
        },
        // Statement::Assign(_) => todo!(),
        Statement::ArithmeticExpression(expr) => evaluate_arithmetic_expression(expr),
        // Statement::CallExpression(_) => todo!(),
        _ => todo!("need to implement AST node type: {}", ast_node),
    }
}

pub fn evaluate_program(program: Program) -> RuntimeValue {
    let mut last_evaluated = RuntimeValue { r#type: ValueType::None };

    for statement in program.body {
        println!("statement {}", statement);
        last_evaluated = evaluate(statement);
        println!("- runtime value: {:?}", last_evaluated);
    }

    return last_evaluated;
}
