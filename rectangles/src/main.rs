fn main() {
    single_variable_example();

    tuple_example();

    struct_example();

    method_example();
}

fn method_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect4 = Rectangle::square(10);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));

}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

//the directive below allows us to do the debug print on line 27
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_example() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        struct_area(&rect1)
    );

    println!("rect1 is {:?}", rect1);
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn tuple_example() {
    let rec1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        tuple_area(rec1)
    );

}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn single_variable_area (width: u32, height: u32) -> u32 {
    width * height
}

fn single_variable_example() {

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        single_variable_area(width1, height1)
    );
}
