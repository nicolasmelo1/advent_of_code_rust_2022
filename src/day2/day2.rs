use std::{fs, str::Split};

fn get_shape_from_play(play: &str) -> &'static str {
    let is_rock = play == "A" || play == "X";
    let is_paper = play == "B" || play == "Y";
    if is_rock {
        return "rock";
    } else if is_paper {
        return "paper";
    } else {
        return "scissors";
    }
}

fn score_for_shape(shape: &str) -> i32 {
    return match shape {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => 0,
    };    
}

fn get_winner_for_round(player1_shape: &str, player2_shape: &str) -> i32 {
    let players_shapes = [player1_shape, player2_shape];    

    return match players_shapes {
        ["rock", "scissors"] => 1,
        ["scissors", "paper"] => 1,
        ["paper", "rock"] => 1,
        ["scissors", "rock"] => 2,
        ["paper", "scissors"] => 2,
        ["rock", "paper"] => 2,
        _ => 0,
    };
}

fn score_for_outcome_of_round(outcome: &str) -> i32 {
    return match outcome {
        "win" => 6,
        "lose" => 0,
        _ => 3,
    };
}

fn your_play_for_expected_outcome(opponents_shape: &str, outcome: &str) -> &'static str {
    let owned_opponents_shape = opponents_shape.to_owned();
    let owned_outcome = outcome.to_owned();
    let mut shapes = vec!["rock", "paper", "scissors"];
    return match outcome {
        "Y" => {
            let index = shapes.iter().position(|&r| r == opponents_shape).unwrap();
            return shapes[index];
        },
        "Z" => {
            let remove_index = shapes.iter().position(|x| *x == opponents_shape).unwrap();
            shapes.remove(remove_index);

            for expected_shape in shapes.iter() {
                let expected_outcome = get_winner_for_round(opponents_shape, expected_shape);
                if expected_outcome == 2 {
                    return expected_shape;
                }
            }
            return "";
        },
        _ => {
            let remove_index = shapes.iter().position(|x| *x == opponents_shape).unwrap();
            shapes.remove(remove_index);

            for expected_shape in shapes.iter() {
                let expected_outcome = get_winner_for_round(opponents_shape, expected_shape);
                if expected_outcome == 1 {
                    return expected_shape;
                }
            }
            return "";
        }
    };
}


pub fn part1(splitted_contents: Split<char>) {
    let mut player2_total_score = 0;

    for game in splitted_contents {
        let players: Vec<&str> = game.split_whitespace().collect();
        let player1 = players.get(0).unwrap();
        let player2 = players.get(1).unwrap();
        let player1_shape = get_shape_from_play(player1);
        let player2_shape = get_shape_from_play(player2);

        let who_won = get_winner_for_round(player1_shape, player2_shape);
        
        let player2_score_for_shape = score_for_shape(player2_shape);
        let player2_score_for_outcome = score_for_outcome_of_round(if who_won == 2 { "win" } else if who_won == 1 { "lose" } else { "draw" });

        let player2_score = player2_score_for_shape + player2_score_for_outcome;
        
        player2_total_score = player2_total_score + player2_score;
    }
}

pub fn part2(splitted_contents: Split<char>) {
    let mut player2_total_score = 0;

    for game in splitted_contents {
        let players: Vec<&str> = game.split_whitespace().collect();
        let player1 = players.get(0).unwrap();
        let how_game_should_end = players.get(1).unwrap();
        let player1_shape = get_shape_from_play(player1);
        let player2_shape = your_play_for_expected_outcome(player1_shape, how_game_should_end);

        let player2_score_for_shape = score_for_shape(player2_shape);
        let player2_score_for_outcome = score_for_outcome_of_round(if *how_game_should_end == "Z" { "win" } else if *how_game_should_end == "X" { "lose" } else { "draw" });
        println!("{} {} {} {}", player1_shape, player2_shape, how_game_should_end, player2_score_for_outcome);
        let player2_score = player2_score_for_shape + player2_score_for_outcome;
        
        player2_total_score = player2_total_score + player2_score;
    }

    println!("Player 2 total score: {}", player2_total_score);
}

pub fn run() {
    let contents = fs::read_to_string("./src/day2/input.txt")
        .expect("Should have been able to read the file");

    let splitted = contents.split('\n');
    part2(splitted);
}