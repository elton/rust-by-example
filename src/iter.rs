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
}
