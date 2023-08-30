/*
 * @FilePath: \rustlings\exercises\lifetimes\lifetimes3.rs
 * @Author: error: git config user.name & please set dead value or install git
 * @文件版本: V1.0.0
 * @Date: 2023-07-04 18:51:33
 * @Description: 
 * 
 * 版权信息         : 2023 by ${git_name}, All Rights Reserved.
 */
// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.


struct Book <'a>{
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
