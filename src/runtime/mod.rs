use parser::ast::Expression;

#[derive(Debug)]
enum ValueType {
    Number(usize)
}

#[derive(Debug)]
pub struct RuntimeValue {
    r#type: ValueType
}

pub fn evaluate(ast_node: Expression) -> RuntimeValue {
    match ast_node {
        // Expression::Comment(_) => todo!(),
        // Expression::Identifier(_) => todo!(),
        Expression::NumericLiteral(val) => RuntimeValue { r#type: ValueType::Number(val.value) },
        // Expression::Assign(_) => todo!(),
        // Expression::ArithmeticExpression(_) => todo!(),
        // Expression::CallExpression(_) => todo!(),
        _ => todo!("need to implement AST node type: {}", ast_node)
    }
}
