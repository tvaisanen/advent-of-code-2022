use std::fs;

fn day01() {
    let contents = fs::read_to_string("./day01.txt").expect("file");

    let elves = contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let calories = elves
        .iter()
        .map(|ns| {
            ns.iter()
                .filter(|&s| s != &"")
                .map(|s| s.parse::<i64>().expect("int"))
                .reduce(|a, b| a + b)
                .expect("int")
        }).collect::<Vec<i64>>();

    let mut calories_sorted = calories.to_vec();

    calories_sorted.sort_by(|a, b| b.cmp(a));

    let top1 = calories_sorted[0];

    let top3 = calories_sorted
        .iter()
        .take(3)
        .fold(0, |res, a| res + a);

    println!("{:?} {:?}", top1, top3);
}
