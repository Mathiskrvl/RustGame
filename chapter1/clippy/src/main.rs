#![warn(clippy::all,clippy::pedantic)]
fn main() {
    let mylist: [&str;3] = ["One", "Two", "Three"];
    for (i, item) in mylist.iter().enumerate() {
        print!("{i} : ");
        println!("{item}");
    }
    println!("{mylist:?}");
}
