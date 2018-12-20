fn foo() { } //does nothing

fn print_number(n: i64) {
    println!("A number: {}", n);
}

fn print_sum(a: i32, b: i32) {
    println!("A sum: {}", a + b);
}

fn square(n: i32) -> i32 { //returns i32
    n * n  //no "return" keyword?!
}

//fn is_prime(n: u64) -> bool {
//    if x <= 1 {
//        return false;
//    }
//    //lots of code calculation `prime: bool`
//    prime
//}

fn double_triple(n: i32) -> (i32, i32) {
    (2 * n, 3 * n) 
}

fn main() {
    println!("Hello, world!");
    foo();
    print_number(20);
    print_sum(20, 22);
    println!("3 mal 3 = {}", square(3));
    let (double, triple) = double_triple(7);
}
