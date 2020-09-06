pub fn main() {
    let s = "नमस्ते";
    for c in s.chars() {
        println!("{}", c);
    }
    println!("the len of {} is {}", s, s.len());
    let hello = "中文测试";
    println!("{} the first 3 character are: {}", hello, &hello[0..3]);
    for b in hello.bytes() {
        print!("{},", b);
    }
}
