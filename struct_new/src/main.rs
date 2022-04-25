
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
    let st = new_student(String::from("srt@gmail.com"), String::from("mobil"));
    let st2 = User{
        ..st
    };

    println!("{}", st2.username);
    println!("{}", st2.active);
    println!("{}", st2.email);
    println!("{}", st2.sign_in_count);

    color_tup()
}

// short hand syntax
fn new_student(email : String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


fn color_tup() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.0);
    println!("{}", origin.1);
}


