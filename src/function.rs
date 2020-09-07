pub fn main() {
    let res: i32 = (0..10)
        .map(|x| x * x)
        .inspect(|x| println!("value {}", x))
        .filter(|x| *x < 3)
        .filter_map(|x| Some(x))
        .fold(0, |x, y| x + y);
    println!("{}", res);

    let _: Vec<i32> = (0..10)
        .chain(10..20)
        .inspect(|a| println!("{:?}", a))
        .collect();
    let _: Vec<(i32, i32)> = (0..10)
        .zip(10..20)
        .inspect(|(a, b)| println!("({:?},{:?})", a, b))
        .collect();
}
