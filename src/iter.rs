pub fn main() {
    let names = vec!["Jane", "Jill", "Jack", "John"];

    let total_bytes = names
        .iter()
        .map(|name| name.len())
        .fold(0, |acc, len| acc + len);

    println!("the total bytes is: {}", total_bytes);

    let player_scores = [("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)];

    let players = player_scores
        .iter()
        .map(|(player, _score)| player)
        .collect::<Vec<_>>();

    println!("the player are {:?}", players);

    let mut teams = [
        [("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)],
        [("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17)],
    ];

    let teams_in_score_order = teams
        .iter_mut()
        .map(|team| {
            team.sort_by(|&a, &b| a.1.cmp(&b.1).reverse());
            team
        })
        .collect::<Vec<_>>();

    println!("Teams: {:?}", teams_in_score_order);

    let x = vec!["Jill", "Jack", "Jane", "John"];

    // let new_x = x.clone().into_iter().take(2).collect::<Vec<_>>(); //克隆了所有元素
    // let new_x = x.iter().map(|i| i.clone()).take(2).collect::<Vec<_>>(); //只克隆前两个元素
    let new_x = x.iter().cloned().take(2).collect::<Vec<_>>(); // Rust 1.1之后推出了新的Cloned方法，解决了这个问题。

    println!("the new x is: {:?}", new_x);

    let _ = (0..5)
        // .inspect(|x| println!("before flat_map: {}", x))
        .flat_map(|x| x * 100..x * 110)
        // .inspect(|x| println!("after flat_map: {}", x))
        .enumerate()
        // .inspect(|(i, v)| println!("index:{}, value:{}", i, v))
        .filter(|&(i, x)| (i + x) % 3 == 0)
        // .collect::<Vec<(_, _)>>();
        .for_each(|(i, x)| println!("{}:{}", i, x));

    let (even, odd): (Vec<i32>, Vec<i32>) = (0..10).partition(|&x| x % 2 == 0);
    println!("even: {:?}", even);
    println!("odd: {:?}", odd);

    let a = [-3_i32, 0, 1, 5, -10];
    println!("Max number: {}", a.iter().max_by_key(|x| x.abs()).unwrap());
    println!("Max number: {}", a.iter().max_by(|x, y| x.cmp(y)).unwrap());
    println!("Min number: {}", a.iter().min_by_key(|x| x.abs()).unwrap());

    println!("Sum of 0..101: {}", (0..101).sum::<u32>());

    fn factorial(n: u32) -> u32 {
        (1..=n).product() //所有元素相乘，`(1..=n)`是包含n，`(1..n)`是不包含n
    }

    println!("5! = {}", factorial(5));

    println!("{:?}", (0..=10).take(5).collect::<Vec<_>>());
    println!("{:?}", (0..=10).skip(3).collect::<Vec<_>>());
    println!("first: {}", (0..=10).collect::<Vec<_>>().first().unwrap());
    println!("last: {}", (0..=10).collect::<Vec<_>>().last().unwrap());
    println!("{:?}", (0..=10).collect::<Vec<_>>().get(0..2).unwrap());

    let mut a = (0..=10).collect::<Vec<_>>();
    a.swap(1, 3);
    println!("swap(1,3): {:?}", a);
    a.reverse();
    println!("reverse: {:?}", a);

    let mut arr = [0u8; 10];
    use rand::distributions::{Standard, Uniform};
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();

    rng.fill(&mut arr[..]);
    arr.sort();
    println!("sorted array: {:?}", arr);

    let v: Vec<u8> = rng.sample_iter(Standard).take(20).collect();
    println!("Vector: {:?}", v);

    let rang = Uniform::new_inclusive(0, 100); //包含起始值的均匀分布
    let mut v: Vec<u8> = rng.sample_iter(rang).take(20).collect();
    println!("Vector: {:?}", v);

    v.sort_by(|a, b| a.cmp(b));
    println!("Sorted Vec: {:?}", v);
    v.sort_by(|a, b| b.cmp(a));
    println!("Reverse Sorting: {:?}", v);

    let rang = Uniform::new_inclusive(-127.0, 128.0);
    let mut v: Vec<f32> = rng.sample_iter(rang).take(20).collect();
    println!("Vector: {:?}", v);

    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Sorted Vector: {:?}", v);

    let rang = Uniform::new_inclusive(-128, 127);
    let mut v: Vec<i8> = rng.sample_iter(rang).take(20).collect();
    println!("Vector: {:?}", v);
    v.sort_by_cached_key(|k| k.abs());
    println!("Sorted by ABS(): {:?}", v);
}
