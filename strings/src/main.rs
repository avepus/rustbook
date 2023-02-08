fn main() {
    basic_string_example();

    updating_strings();

    updating_with_plus_and_format();

    string_slicing();
}

fn string_slicing() {
    println!("here's the chars in Зд");
    for c in "Зд".chars() {
        println!("{}", c);
    }

    println!("And heres the bytes. Note: only 2 chars but 4 bytes!");
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}

fn updating_with_plus_and_format() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 is moved here so it can't be used again
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //things get complicated with multiple + concatenations like below
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("value of s is: {s}");

    //the format! macro can be more clear and doesn't lose ownership
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("value of s is: {s}");
    println!("We can still use s1! The value of s1 is: {s1}");
}

fn basic_string_example() {
    let data = "initial contents";

    let _s = data.to_string();

    //the method also works on string literals
    let _s = "initial contents".to_string();

    //using String::from is identical
    let _s = String::from("initial contents");
}

fn updating_strings() {
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);

    println!("value of s is: {s}");
    //push_str doesn't take ownership so the line below is ok
    println!("value of s2 is: {s2}");

    //push is used for a single char
    s.push('|');
    println!("value of s after push is: {s}");
}
