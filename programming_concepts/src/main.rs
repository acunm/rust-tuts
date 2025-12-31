fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // this will throw error because x is immutable
    // rustc --explain E0384
    // x = 6;
    // println!("The value of x is: {x}");

    // This is fine because it is mutable
    let mut y = 5;
    y = 6;
    println!("The value of y is: {y}");

    // const is immutable also
    // mut keyword can not be used with const
    // constants must be declared in expression
    // they cant be set that computed in runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in inner scope is: {z}");
    }

    println!("The value of z is: {z}");

    // called shadowing. first one is string
    // second one is number
    // it works
    let spaces = "   ";
    let spaces = spaces.len();

    // this one will throww error because
    // it expects string
    // mutable but not the type. only value
    let mut sp = "   ";
    sp = sp.len();
}
