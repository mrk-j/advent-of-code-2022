use std::fs;

fn score_for_shape(shape: &str) -> u32 {
    match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Shape [{}] not found", shape),
    }
}

fn score_for_game_result(shape_opponent: &str, shape: &str) -> u32 {
    match shape_opponent {
        "A" => match shape { // Opponent plays rock
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => panic!("Shape [{}] not found", shape),
        },
        "B" => match shape { // Opponent plays paper
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Shape [{}] not found", shape),
        },
        "C" => match shape { // Opponent plays scissors
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => panic!("Shape [{}] not found", shape),
        },
        _ => panic!("Shape opponent [{}] not found", shape_opponent),
    }
}

fn main() {
    let mut total_score: u32 = 0;

    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let shape_opponent: &str = parts.get(0).unwrap();
            let shape: &str = parts.get(1).unwrap();

            let score_for_shape = score_for_shape(shape);
            let score_for_game_result = score_for_game_result(shape_opponent, shape);

            total_score += score_for_shape + score_for_game_result;
        });

    println!("The total score is {}", total_score);
}