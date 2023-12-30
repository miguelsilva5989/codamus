use parser;

fn main() {
    let input = include_str!("sample.c420");

    let program = parser::parse_ast(input);
    
    match program {
        Ok((rem, program)) => {
            print!("program: {:?} - rem: '{}'", program, rem)
        },
        Err(err) => panic!("error parsing ast: {:?}", err)
    }
}
