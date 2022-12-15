use std::fs;

fn char_to_input(c: char) -> usize {
    let c_u8 = c as usize;
    if 88 <= c_u8 && c_u8 <= 90 {
        c_u8 - 87
    } else if 65 <= c_u8 && c_u8 <= 67 {
        c_u8 - 64
    } else {
        0
    }
}

fn result(game: Vec<usize>) -> i32 {
    let result_table = vec![
        vec![0, 0, 0, 0],
        vec![0, 3, 6, 0], // rock
        vec![0, 0, 3, 6], // paper
        vec![0, 6, 0, 3], // scissors
    ];

    result_table[game[0]][game[1]] + game[1] as i32
}

fn result2(game: Vec<usize>) -> i32 {
    // game comes directly from
    // the strategy input
    // 1 -> 0, 2 -> 3, 2 -> 6
    let game_score = vec![0, 0, 3, 6];

    let strategy = vec![0, 2, 0, 1];
    let n = strategy[game[0]];

    // create hands for cycling through the options
    let hands = vec![1, 2, 3];
    let hand_score = Vec::from_iter(hands.iter().cycle().skip(n).take(3));

    game_score[game[1]] + hand_score[game[1] - 1]
}

fn main() {
    let input = fs::read_to_string("./day02.txt").expect("file");

    let results: i32 = input
        .lines()
        .map(|s| {
            s.chars()
                .filter(|c| c != &' ')
                .map(char_to_input)
                .collect::<Vec<usize>>()
        })
        .map(result)
        .sum();

    println!("{:?}", results);
}

// added commit into main git worktree
