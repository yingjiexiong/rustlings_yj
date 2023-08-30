/*
 * @FilePath: \rustlings\exercises\move_semantics\move_semantics5.rs
 * @Author: error: git config user.name & please set dead value or install git
 * @文件版本: V1.0.0
 * @Date: 2023-07-04 18:51:33
 * @Description: 
 * 
 * 版权信息         : 2023 by ${git_name}, All Rights Reserved.
 */
// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut x = 100;
    {
    let y = &mut x;
    *y += 100;
    }
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
