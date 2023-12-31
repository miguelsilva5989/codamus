use parser;
use runtime::{RuntimeValue, value_types::ValueType};

mod runtime;

fn main() {
    let input = include_str!("sample.c420");

    let program = parser::parse_ast(input);
    let mut env = runtime::environment::Environment::new(None);

    env.declare_var("x".to_owned(), RuntimeValue {r#type: ValueType::Number(100.0)});
    
    match program {
        Ok((rem, program)) => {
            print!("{}\nremaining input: '{}'\n", program, rem);

            let _ = runtime::evaluate_program(program, env);
            // println!("runtime value: {:?}", runtime_val);

            // for expr in program.body {
            //     let runtime_val = runtime::evaluate(expr);
            //     println!("runtime value: {:?}", runtime_val);
            // }
        },
        Err(err) => panic!("error parsing ast: {:?}", err)
    }
}
