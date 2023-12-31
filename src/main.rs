use parser;

mod runtime;

fn main() {
    let input = include_str!("sample.c420");

    let program = parser::parse_ast(input);
    
    match program {
        Ok((rem, program)) => {
            print!("{}\n", program);
            print!("remaining input: '{}'\n\n", rem);

            let _ = runtime::evaluate_program(program);
            // println!("runtime value: {:?}", runtime_val);

            // for expr in program.body {
            //     let runtime_val = runtime::evaluate(expr);
            //     println!("runtime value: {:?}", runtime_val);
            // }
        },
        Err(err) => panic!("error parsing ast: {:?}", err)
    }
}
