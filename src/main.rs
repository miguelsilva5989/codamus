
use parser;

fn main() {
    let input = include_str!("sample.c420");

    let tt = parser::parse_dsl(input);
    println!("{:?}", tt);
}
