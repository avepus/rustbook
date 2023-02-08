use std::collections::HashMap;

fn main() {
    accessing_values();

    looping();

    hash_ownership();
}

fn hash_ownership() {
    let field_name = String::from

fn looping() {
    let scores = get_team_map();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn get_team_map() -> HashMap<String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores
}

fn accessing_values() {
    let scores = get_team_map();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The value of score is {}", score);
}
