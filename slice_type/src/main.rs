fn main() {
    no_slice_example(); 

    slice_example();

    better_slice_example();
}

fn better_slice_example() {
    let my_string = String::from("hello world");

    // `better_first_word` works on slices of `String`s, whether partial or whole
    let _word = better_first_word(&my_string[0..6]);
    let _word = better_first_word(&my_string[..]);
    // `better_first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = better_first_word(&my_string);

    let my_string_literal = "hello world";

    // `better_first_word` works on slices of string literals, whether partial or whole
    let _word = better_first_word(&my_string_literal[0..6]);
    let _word = better_first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = better_first_word(my_string_literal);
}

fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn slice_example() {
    let mut my_string = String::from("this string");

    let word = first_word_using_slice(&my_string);

    println!("the value of my_string is: {my_string}");
    println!("the value of word is: {word}");

    my_string.clear();

    //the line below would error because word is no longer valid after the clear
    //println!("The value of word is {word}.");
}   

fn first_word_using_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn no_slice_example() {
    let mut my_string = String::from("this string");

    let word = first_word_no_slice(&my_string);

    my_string.clear();

    //word is still 4 even though that's not valid for string anymore
    println!("The value of word is {word}.");
}

fn first_word_no_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
