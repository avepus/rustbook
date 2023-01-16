fn main() {
    string_ownership();

    known_size_ownership(); 

    losing_ownership();

    borrowing();

    mutable_borrow();
}

fn mutable_borrow() {
    let mut s = String::from("hello");
    
    println!("value of s is: {s}");

    change(&mut s);

    println!("value of s is: {s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", there!!");
}

fn borrowing() {
    let s1 = String::from("hello");

    let len = calculate_lengths(&s1);

    //no error below because calculate_lengths refers to s1. it doesn't own it
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_lengths(s: &String) -> usize {
    s.len()
}

fn losing_ownership() {
    let x = 3;
    let s = String::from("my string");

    takes_ownership(s);

    //the line below gives us an error because the function took ownershihp of the str
    //println!("value of s is: {}", s);

    makes_copy(x);

    //the line below doesn't give us an error because int type is stored on the stack
    println!("value of x is: {x}")
}

fn takes_ownership(some_string: String) {
    println!("some_string is: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer is: {}", some_integer);
}

fn known_size_ownership() {
    let x = 5;
    let y = x;

    //the code below is avlid for types like integers because they're stored on the stack
    println!("x = {}, y = {}", x, y);
}

fn string_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    //the commend code below would create an error becuase
    //strings are stored on the heap and the reference to s1 is not valid
    //println!("{}, world!", s1); 

    //this code works becuase we use clone to make a copy
    let s3 = s2.clone();

    println!("s2 value: {s2}, s3 value: {s3}");
}
