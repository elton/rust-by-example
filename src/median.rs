// 求一组随机数正整数的平均数、中位数和众数
use rand::{thread_rng, Rng};
use std::collections::HashMap;

// 获取`num`个小于100的自然数
fn rand_vec(num: u32) -> Vec<u32> {
    let mut rng = thread_rng();
    let mut i = 0;
    let mut numbers: Vec<u32> = Vec::new();
    while i < num {
        numbers.push(rng.gen_range(1, 100));
        i += 1;
    }
    // 打印生成好的随机自然数 Vector
    print!("The original Vector is: ");
    numbers.iter().fold((), |_, n| print!("{}, ", n));
    println!();

    numbers
}

// 计算平均数
fn mean(data: &Vec<u32>) -> f64 {
    let sum = data.iter().sum::<u32>();
    let size = data.len();
    sum as f64 / size as f64
}

// 计算中位数
fn median(data: &mut Vec<u32>) -> f64 {
    let size = data.len();
    data.sort();

    match size {
        even if even % 2 == 0 => {
            let fst_med = *data.get(even / 2 - 1).unwrap();
            let snd_med = *data.get(even / 2).unwrap();
            (fst_med + snd_med) as f64 / 2.0
        }
        odd => *data.get(odd / 2).unwrap() as f64,
    }
}

// 计算众数
fn mode(data: &Vec<u32>) -> u32 {
    // 生成 (&value, count)形式的HashMap
    // fold有两个参数，第一个参数是初始值，第二个参数是一个闭包函数，它也有两个参数，第一个是累计值，第二个是循环的元素
    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value)
        .unwrap();

    mode
}

pub fn main() {
    let mut result = rand_vec(11);

    println!("Mean of the data is: {}", mean(&result));
    println!("Median of the data is: {}", median(&mut result));
    println!("Mode of the data is: {}", mode(&result));
}
