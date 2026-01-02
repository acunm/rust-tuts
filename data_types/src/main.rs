fn main() {
    // integer types
    // i8 (-128, 127)
    // i16 (-32768, 32767)
    // i32, i64, i128 (...)
    // isize (-9223372036854775808, 9223372036854775807)
    // isize is platform dependent
    // u8 (0, 255)
    // u16 (0, 65535)
    // u32, u64, u128 (...)
    // usize (0, 18446744073709551615)
    // usize also platform dependent

    let _x: i8 = 100;
    let _y: u8 = 200;

    // Decimal 98222
    // Hex 0xff
    // Octal 0o87
    // binary 0b1111_0000
    // byte(u8) b'A'

    // Floating numbers
    //
    // f32 and f64, only 2 types.
    // All signed.

    let _a = 2.0; // f64

    let _b: f32 = 2.0; //f32

    // Compund Types
    //
    // Tuple
    // They are fixed size and can not change. Cant shrink or grow.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_xx, _yy, _zz) = tup;
    println!("The value of _yy is {_yy}");
    let tup0 = tup.0;
    let tup1 = tup.1;
    let tup2 = tup.2;
    println!("Value of tup1 is {tup1}, value of tup2 is {tup2}, value of tup0 is {tup0}");
    
    
    // Array
    // arrays are fixed size.
    let arr = [1,2,3,4,5];
    // better to use if u know you will have same size data always like;
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    
    // array type definition
    let iArr: [i32; 5] = [1,2,3,4,5];
    
    let sArr = [3; 5];
    // [3, 3, 3, 3, 3];
    
    // Array element access
    let first = iArr[0];
    let second = iArr[1];
    
    
}
