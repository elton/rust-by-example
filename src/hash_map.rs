use std::collections::HashMap;
pub fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        println!("{}", word);
        // `or_insert` 方法事实上会返回这个键的值的一个可变引用(&mut V)，
        // `or_insert`传入的参数类型，决定了返回的可引用变量的类型。
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
