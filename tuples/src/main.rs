use std::io;
use std::cmp::Ordering;

fn main() {
    let tup = ("x", 1, 3.4);
    println!("{:?}", tup);

    let mut a: [i32; 6] = [1, 2, 3, 4, 5, 4];

    println!("{:?}", a);

    a[2] = -3;

    println!("{:?}", a);

    let a = [3; 5];

    println!("{:?}", a);
//------------------------------------------------------------------

    // let a  = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: integer = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let max_index = 4;    
    // match index.compare(&max_index){
    //     Ordering::Greater => {
    //         println!("Index out of bound")
    //         exit;
    //     }
    // }


    // let element = a[index];

    // println!(
    //     "The value of the element at index {} is: {}",
    //     index, element
    // );
}
