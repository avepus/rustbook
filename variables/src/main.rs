fn main() {
    tups();
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }

    println!("The value of x is: {x}");

}

fn tups() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let tup1 = tup.1;
    println!("The value of tup.1 is: {tup1}");
    
}
