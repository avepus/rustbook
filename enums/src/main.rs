fn main() {
    coin_example();

    option_t_example();
}

fn option_t_example() {
    let i = Some(3);

    println!("i after plus 1 is: {:?}",plus_one_option(&i));

    //the line below is shorthand for a special type  of match
    //which only performs the code if the value is not null
    //this shoud only be used when appropriate
    if let Some(val) = i {
        println!("The variable i has a value. It's: {val}");
    }
}

fn plus_one_option(x: &Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn coin_example() {
    let pen = Coin::Penny;

    let nick = Coin::Nickel;

    println!("The value of pen is: {:?}", value_in_cents(pen));
    println!("The value of nick is: {:?}", value_in_cents(nick));
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
