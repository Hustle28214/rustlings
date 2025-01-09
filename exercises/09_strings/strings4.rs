fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string("blue".to_string()); // "blue" 是 &str，需要转换为 String

    string("red".to_string()); // "red".to_string() 已经是 String

    string(String::from("hi")); // String::from("hi") 已经是 String

    string_slice("rust is fun!"); // "rust is fun!" 是 &str

    string("nice weather".into()); // "nice weather".into() 转换为 String

    string(format!("Interpolation {}", "Station")); // format! 返回 String

    string_slice(&String::from("abc")[0..1]); // 字符串切片是 &str

    string_slice("  hello there ".trim()); // trim() 返回 &str

    string("Happy Monday!".replace("Mon", "Tues")); // replace() 返回 String

    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // to_lowercase() 返回 String
}