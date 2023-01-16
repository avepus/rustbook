fn main() {
    creating_vectors();

    reading_elements_example();

    iterating_example();

    enum_example();
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_example() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(11.24),
    ];
}

fn iterating_example() {
    let v = vec![100, 32, 57];

    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{}", n_plus_one);
    }

    let mut v = vec![1, 2, 3];

    for n_ref in &mut v {
        //we must deference here so the vector maintains ownership
        *n_ref += 50;
    }

    //all elements had 50 added to them
    println!("{:?}", v);
}

fn reading_elements_example() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //the line below would crash since it's out of bounds
    //let e100 = &v[100];

    let e100: Option<&i32> = v.get(100);

    match e100 {
        Some(third) => println!("The 100th element is {}", third),
        None => println!("There is no 100th element."),
    }
}

fn creating_vectors() {
    let v: Vec<i32> = Vec::new();

    //rust can infer the type if we don't specify it
    let mut v = vec![1, 2, 3];

    println!("The value of v is: {:?}", v);

    v.push(5);
    v.push(6);
    v.push(7);

    println!("The value of v is: {:?}", v);
}
