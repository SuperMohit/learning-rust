fn main() {
   // mu_string()
   // deep_copy()
   //string_ownership()
   mut_ref()
}

// fn deep_copy() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);
    
// }


fn mut_ref() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}",s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}



// fn string_ownership() {
//     let s = String::from("hello");  // s comes into scope

//     let str  =  takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here
//     println!("{}", str);

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//     println!("{}",x);                                // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) -> String { // some_string comes into scope
//     println!("{}", some_string);
//     let mut str = some_string;
//     str.push_str("Mohit");
//     str
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.



// fn mu_string() {
//     let mut s = String::from("hello world");
//     s.push_str(". Today is great!!");
//     println!("{}", s);
// }
