fn main() {
    less_than_5_test();

    non_zero_test();

    else_if_test();

    print_5_if_true_else_0(false);
}

fn print_5_if_true_else_0(print_five: bool) {
    let number = if print_five { 5 } else { 0 };

    println!("The value of number is: {number}");
}
fn else_if_test() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisble by 3");
    } else if number % 2 == 0 {
        println!("Number is divisble by 2");
    } else {
        println!("number is not divisble by 4, 3, or 2");
    }
}

fn non_zero_test() {
    let number = 3;
    
    if number != 0 {
        println!("Number was something other than zero");
    }
}
fn less_than_5_test() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

