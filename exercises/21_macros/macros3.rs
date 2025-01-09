// TODO: Fix the compiler error without taking the macro definition out of this
// module.
// 定义宏模块
// 定义宏模块
mod macros {
    // 使用 #[macro_export] 将宏导出到 crate 的根作用域
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}



fn main() {
    // 使用宏
    my_macro!();
}