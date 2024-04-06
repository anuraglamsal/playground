// Short-circuit evaluation: https://en.wikipedia.org/wiki/Short-circuit_evaluation

fn main(){
    // prints true without triggering "func."
    println!("{}", short_circuit_test(true, false)); 

    // prints true after triggering "func." Therefore, 
    // "The function got triggered!" message is also
    // printed.
    println!("{}", short_circuit_test(false, true));

    // Overall, the terminal output is:

    // true
    // The function got triggered!
    // true
}

fn short_circuit_test(val1: bool, val2: bool) -> bool {
    return val1 || func(val2);
}

fn func(val: bool) -> bool {
    println!("The function got triggered!");
    return val;
}
