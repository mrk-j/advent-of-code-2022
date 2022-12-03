use std::fs;

fn score_for_shape(shape: &String) -> u32 {
    match shape.as_str() {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Shape [{}] not found", shape),
    }
}

fn score_for_game_result(result: &str) -> u32 {
    match result {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("Result [{}] not found", result),
    }
}

fn shape_to_play(shape_opponent: &str, result: &str) -> String {
    match shape_opponent {
        "A" => match result { // Opponent plays rock
            "X" => String::from("Z"), // Lose so play scissors
            "Y" => String::from("X"), // Tie so play rock
            "Z" => String::from("Y"), // Win so play paper
            _ => panic!("Result [{}] not found", result),
        },
        "B" => match result { // Opponent plays paper
            "X" => String::from("X"), // Lose so play rock
            "Y" => String::from("Y"), // Tie so play paper
            "Z" => String::from("Z"), // Win so play scissors
            _ => panic!("Result [{}] not found", result),
        },
        "C" => match result { // Opponent plays scissors
            "X" => String::from("Y"), // Lose so play paper
            "Y" => String::from("Z"), // Tie so play scissors
            "Z" => String::from("X"), // Win so play rock
            _ => panic!("Result [{}] not found", result),
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
            let result: &str = parts.get(1).unwrap();
            let shape = shape_to_play(shape_opponent, result);

            let score_for_shape = score_for_shape(&shape);
            let score_for_game_result = score_for_game_result(result);

            total_score += score_for_shape + score_for_game_result;
        });

    println!("The total score is {}", total_score);
}