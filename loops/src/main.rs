fn main() {
    first_loop();

    loop_with_labels();

    loop_with_while();

    loop_with_for();

    countdown_with_for(4);
}

fn countdown_with_for (countdown_start: i32) {
    for number in (1..countdown_start + 1).rev() {
        println!("{number}");
    }
    println!("LIFTOFF USING FOR!!!");
}

fn loop_with_for () {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFETOFF!!!");
}

fn loop_with_labels() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn first_loop() {

    let mut counter = 0;

    let result = loop {
        println!("again!");
        counter += 1;

        if counter == 10 {
            break counter + 2;
        }
    };

    println!("The result is {result}");
}
