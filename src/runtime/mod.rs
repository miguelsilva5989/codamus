pub mod environment;
pub mod value_types;

use parser::{
    ast::{ArithmeticExpression, Assign, Identifier, Oper, Statement, Object},
    Program,
};

use value_types::ValueType;

use self::environment::Environment;

#[derive(Debug, Clone)]
pub struct RuntimeValue {
    pub r#type: ValueType,
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

fn eval_left_right(env: &mut Environment, left: ArithmeticExpression, right: ArithmeticExpression, operator: Oper) -> RuntimeValue {
    let left = evaluate(env, Statement::ArithmeticExpression(left));
    let right = evaluate(env, Statement::ArithmeticExpression(right));

    if matches!(left.r#type, ValueType::Number(_)) && matches!(left.r#type, ValueType::Number(_)) {
        return RuntimeValue {
            r#type: evaluate_numeric_arithmetic_expression(left.r#type.into(), right.r#type.into(), operator),
        };
    }

    return RuntimeValue { r#type: ValueType::None };
}

fn evaluate_arithmetic_expression(env: &mut Environment, expr: ArithmeticExpression) -> RuntimeValue {
    match expr {
        ArithmeticExpression::Value(val) => RuntimeValue {
            r#type: ValueType::Number(val),
        },
        ArithmeticExpression::Identifier(id) => evaluate_identifier(env, id),
        ArithmeticExpression::Add(left, right) => eval_left_right(env, *left, *right, Oper::Add),
        ArithmeticExpression::Sub(left, right) => eval_left_right(env, *left, *right, Oper::Sub),
        ArithmeticExpression::Mul(left, right) => eval_left_right(env, *left, *right, Oper::Mul),
        ArithmeticExpression::Div(left, right) => eval_left_right(env, *left, *right, Oper::Div),
        ArithmeticExpression::Mod(left, right) => eval_left_right(env, *left, *right, Oper::Mod),
        ArithmeticExpression::Paren(_) => todo!(),
    }
}

fn evaluate_identifier(env: &mut Environment, id: Identifier) -> RuntimeValue {
    return env.lookup_var(id.id);
}

fn evaluate_object_literal(env: &mut Environment, id: Object) -> RuntimeValue {
    todo!()
}

fn evaluate_declaration(env: &mut Environment, assign: Assign) -> RuntimeValue {
    let expr = evaluate(env, *assign.expression);
    return env.declare_var(assign.id.to_owned(), expr, assign.constant);
}

fn evaluate_assign(env: &mut Environment, assign: Assign) -> RuntimeValue {
    let expr = evaluate(env, *assign.expression);
    return env.assign_var(assign.id.to_owned(), expr);
}

fn evaluate(env: &mut Environment, ast_node: Statement) -> RuntimeValue {
    match ast_node {
        Statement::Comment(_) => RuntimeValue { r#type: ValueType::None },
        Statement::BooleanLiteral(val) => RuntimeValue {
            r#type: ValueType::Bool(val),
        },
        Statement::Identifier(id) => evaluate_identifier(env, id),
        Statement::NumericLiteral(val) => RuntimeValue {
            r#type: ValueType::Number(val.value),
        },
        Statement::ObjectLiteral(val) => evaluate_object_literal(env, val),
        Statement::Declaration(assign) => evaluate_declaration(env, assign),
        Statement::Assign(assign) => evaluate_assign(env, assign),
        Statement::ArithmeticExpression(expr) => evaluate_arithmetic_expression(env, expr),
        // Statement::CallExpression(_) => todo!(),
        _ => todo!("Need to implement AST node type evaluation: {}", ast_node),
    }
}

pub fn evaluate_program(program: Program) -> RuntimeValue {
    let mut last_evaluated = RuntimeValue { r#type: ValueType::None };

    let mut env = Environment::new(None);

    for statement in program.body {
        println!("Statement {}", statement);
        last_evaluated = evaluate(&mut env, statement);
        println!("  - runtime value: {:?}", last_evaluated);
    }

    return last_evaluated;
}
