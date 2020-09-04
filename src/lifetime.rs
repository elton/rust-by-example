// 函数签名表明对于某些生命周期 'a，函数会获取两个参数，他们都是与生命周期 'a 存在的一样长的字符串 slice。
// 函数会返回一个同样也与生命周期 'a 存在的一样长的字符串 slice。
// 它的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。
// 这就是我们告诉 Rust 需要其保证的约束条件。
// 记住通过在函数签名中指定生命周期参数时，我们并没有改变任何传入值或返回值的生命周期，而是指出任何不满足这个约束条件的值都将被借用检查器拒绝。

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
