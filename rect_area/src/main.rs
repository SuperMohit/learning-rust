#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self, weight : u32) -> u32 {
        self.width * self.height * weight
    }

    fn perimeter(&self) -> u32 {
        self.width + self.height * 2
    } 
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // move the ownership
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("printing total struct {:?}", rect1);
    dbg!(&rect1);

    println!("{}", rect1.area(2));

    println!("{}", rect1.perimeter());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

