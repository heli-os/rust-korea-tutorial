fn main() {
    let mut a = 10;
    println!("a = {:?}", a);

    a = 20;
    println!("a = {:?}", a);

    const A_CONST: i32 = 1000;
    let a_val = some_function();

    println!("A_CONST = {:?}", A_CONST);
    println!("a_val = {:?}", a_val);

    let c = 10;
    println!("c = {:?}", c);
    {
        let c = 20;
        println!("c = {:?}", c);
    }
    println!("c = {:?}", c);
}

fn some_function() -> i32 {
    55
}