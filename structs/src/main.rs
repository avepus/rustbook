fn main() {
    basic_user_example();

    mut_user_example();

    let long = build_user_long(String::from("long@user.com"), String::from("long"));
    print_username_and_email(long);

    let short = build_user_short(String::from("short@user.com"), String::from("short"));
    print_username_and_email(short);

    struct_update_example();
}

fn struct_update_example() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };
 

    //let user2 = User {
    //    active: user1.active,
    //    username: user1.username,
    //    email: String::from("another@example.com"),
    //    sign_in_count: user1.sign_in_count,
    //};
    //The comment code bove is achieved shorthand by the line below
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    print_username_and_email(user2);
}


fn build_user_short(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn build_user_long(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn mut_user_example() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };
    user1.active = false;
    user1.sign_in_count = 0;
    user1.email = String::from("different@email.com");
    print_username_and_email(user1);
}

fn basic_user_example() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };
    print_username_and_email(user1);
}

fn print_username_and_email(user: User) {
    println!("user's email is: {}", user.email);
    println!("user's username is: {}", user.username);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

