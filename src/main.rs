use parser;

mod runtime;

fn main() {
    let input = include_str!("sample.c420");

    let program = parser::parse_ast(input);
    
    match program {
        Ok((rem, program)) => {
            print!("{}\nremaining input: '{}'\n", program, rem);

            for expr in program.body {
                let runtime_val = runtime::evaluate(expr);
                println!("runtime value: {:?}", runtime_val);
            }
        },
        Err(err) => panic!("error parsing ast: {:?}", err)
    }
}
