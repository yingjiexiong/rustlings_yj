// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut res: i32 = 42;
    let option: Option<i32> = Some(12);
    if let Some(x) = option{
      res+=x;
    }
    println!("{}", res);
}
