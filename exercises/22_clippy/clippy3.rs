// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables)] // 仍然允许未使用的变量，因为示例可能不完整
fn main() {
    let my_option: Option<()> = None;
    if let Some(value) = my_option {
        println!("{:?}", value);
    } else {
        println!("my_option is None");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear(); // 不需要填充值，因为我们是在清空Vec
    println!("This Vec is empty, see? {:?}", my_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}