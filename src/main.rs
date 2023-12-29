use parser;

fn main() {
    let input = include_str!("sample.c420");

    let program = parser::parse_ast(input);
    println!("{:?}", program);

    match program {
        Ok((rem, program)) => {
            program.body.iter().for_each(|st| {

            });
        },
        Err(err) => panic!("oh no: {:?}", err)
    }
}
