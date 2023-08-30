/*
 * @FilePath: \rustlings\exercises\clippy\clippy3.rs
 * @Author: error: git config user.name & please set dead value or install git
 * @文件版本: V1.0.0
 * @Date: 2023-07-04 18:51:33
 * @Description: 
 * 
 * 版权信息         : 2023 by ${git_name}, All Rights Reserved.
 */
// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.


use std::mem::swap;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    //if my_option.is_none() {
        my_option.unwrap();
    //}

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let  my_empty_vec = vec![1, 2, 3, 4, 5];
    // let() = my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
  swap(&mut value_a, &mut value_b);
  //  value_a = value_b;
  //  value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
}
