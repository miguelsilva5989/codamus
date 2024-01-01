# Codamus Lang

My first take on creating a simple programming language in Rust.

Is uses [Nom](https://github.com/rust-bakery/nom) for parsing and creating the AST.

Currently this is only an interpreted language and does not support many features.

## References

Based on the youtube series "Programming Language From Scratch" in TypeScript by https://www.youtube.com/@tylerlaceby

## Note

This language is unfinished and not suited for the real world!

## Current parser supported features: 
- literals: `5;`, `false;`, `1 + 2;`
- arithmetic expressions: `45 - 5 * (2 / bar_ze);`
- declarations statements: `let x = 32;`, `const x = 32;`, `const obj = {x: 4, foo, complex: {bar: true}}`
- print statements: `print(x);`

## Current runtime supported features: 
- literals: `5;`, `false;`, `1 + 2;`
- declarations statements: `let x = 32;`, `const x = 32;`, `const obj = {x: 4, foo, complex: {bar: true}}`

## Example

Currently it reads file `sample.c420`, parses it and the interpreter will print out the runtime values. 

Execute `cargo run`

Sample input

```rust
// this is a comment

5;
10 + 5;

let asd = 10 + 5 - 5;

let foo = 1;
let bar_ze = 2;

let sum_test = 10 - foo + bar_ze;
let mul_test = 10 + 5 * 3;
let mod_test = 10 % 2 - 3;

let x = 100;
let y = false;
let z = x + 1;
z;
z = 0;
z;

const a = 0;

const obj = {
    x: 4,
    y: 20,
    foo,
    complex: {
        bar: true
    }
};
```

AST output
```
Program:
        Comment:                comment test
        Numeric Literal:        5
        Arithmetic Expression:  10 + 5
        Declaration:            id: asd, constant: false, expression: Arithmetic Expression:  10 + 5 - 5
        Declaration:            id: foo, constant: false, expression: Arithmetic Expression:  1
        Declaration:            id: bar_ze, constant: false, expression: Arithmetic Expression:  2
        Declaration:            id: sum_test, constant: false, expression: Arithmetic Expression:  10 - foo + bar_ze
        Declaration:            id: mul_test, constant: false, expression: Arithmetic Expression:  10 + 5 * 3
        Declaration:            id: mod_test, constant: false, expression: Arithmetic Expression:  10 % 2 - 3
        Declaration:            id: x, constant: false, expression: Arithmetic Expression:  100
        Declaration:            id: y, constant: false, expression: Boolean Literal:    false
        Declaration:            id: z, constant: false, expression: Arithmetic Expression:  x + 1
        Identifier:     z
        Assign:                 id: z, constant: false, expression: Arithmetic Expression:  0
        Identifier:     z
        Declaration:            id: a, constant: true, expression: Arithmetic Expression:  0
        Declaration:            id: obj, constant: true, expression: Object Literal:    {
        key: x - value: Numeric Literal:        4
        key: y - value: Numeric Literal:        20
        key: foo - no value
        key: complex - value: Object Literal:   {
        key: bar - value: Boolean Literal:      true
}
}

remaining input: ''
```

Interpreter output
```
Statement Comment:              comment test
  - runtime value: RuntimeValue { type: None }
Statement Numeric Literal:      5
  - runtime value: RuntimeValue { type: Number(5.0) }
Statement Arithmetic Expression:  10 + 5
  - runtime value: RuntimeValue { type: Number(15.0) }
Statement Declaration:          id: asd, constant: false, expression: Arithmetic Expression:  10 + 5 - 5
  - runtime value: RuntimeValue { type: Number(10.0) }
Statement Declaration:          id: foo, constant: false, expression: Arithmetic Expression:  1
  - runtime value: RuntimeValue { type: Number(1.0) }
Statement Declaration:          id: bar_ze, constant: false, expression: Arithmetic Expression:  2
  - runtime value: RuntimeValue { type: Number(2.0) }
Statement Declaration:          id: sum_test, constant: false, expression: Arithmetic Expression:  10 - foo + bar_ze
  - runtime value: RuntimeValue { type: Number(11.0) }
Statement Declaration:          id: mul_test, constant: false, expression: Arithmetic Expression:  10 + 5 * 3
  - runtime value: RuntimeValue { type: Number(25.0) }
Statement Declaration:          id: mod_test, constant: false, expression: Arithmetic Expression:  10 % 2 - 3
  - runtime value: RuntimeValue { type: Number(-3.0) }
Statement Declaration:          id: x, constant: false, expression: Arithmetic Expression:  100
  - runtime value: RuntimeValue { type: Number(100.0) }
Statement Declaration:          id: y, constant: false, expression: Boolean Literal:    false
  - runtime value: RuntimeValue { type: Bool(false) }
Statement Declaration:          id: z, constant: false, expression: Arithmetic Expression:  x + 1
  - runtime value: RuntimeValue { type: Number(101.0) }
Statement Identifier:   z
  - runtime value: RuntimeValue { type: Number(101.0) }
Statement Assign:               id: z, constant: false, expression: Arithmetic Expression:  0
  - runtime value: RuntimeValue { type: Number(0.0) }
Statement Identifier:   z
  - runtime value: RuntimeValue { type: Number(0.0) }
Statement Declaration:          id: a, constant: true, expression: Arithmetic Expression:  0
  - runtime value: RuntimeValue { type: Number(0.0) }
Statement Declaration:          id: obj, constant: true, expression: Object Literal:    {
        key: x - value: Numeric Literal:        4
        key: y - value: Numeric Literal:        20
        key: foo - no value
        key: complex - value: Object Literal:   {
        key: bar - value: Boolean Literal:      true
}
}
  - runtime value: RuntimeValue { type: Object({"complex": RuntimeValue { type: Object({"bar": RuntimeValue { type: Bool(true) }}) }, "foo": RuntimeValue { type: Number(1.0) }, "x": RuntimeValue { type: Number(4.0) }, "y": RuntimeValue { type: Number(20.0) }}) }
```