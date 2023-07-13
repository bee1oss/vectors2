#[derive(Debug)]
enum Types {
    Int(i32),
    Float(f64),
    Bool(bool),
    Text(String),
}

fn main() {
    let list = vec![
        Types::Int(7),
        Types::Float(7.7),
        Types::Bool(true),
        Types::Text("Hello".to_string()),
    ];

    match &list[3] {
        Types::Int(i_num) => {
            println!("Int is {}", i_num);
        }
        Types::Float(f_num) => {
            println!("Float is {}", f_num);
        }
        Types::Bool(logic) => {
            println!("Bool is {}", logic);
        }
        Types::Text(str) => {
            println!("Text is {}", str);
        }
    }
}
