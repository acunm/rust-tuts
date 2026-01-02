fn main() {
    let y = {
        let x = 3;
        x + 1
    }; // this block returns 4

    println!("The value of y is: {y}");

    let mut x = five();

    println!("The value of x is: {x}");

    x = plus_one(5);

    println!("The value of x is: {x}");

    first_function();
    function_with_param(123);
    print_params(4, 'a');
}

fn first_function() {
    println!("first function");
}

fn function_with_param(x: i32) {
    println!("param is {x}");
}

fn print_params(value: i32, label: char) {
    println!("value is {value}, label is {}", label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
