#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
enum Strategy {
    Lose,
    Draw,
    Win,
}

impl Hand {
    fn value(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn beats(&self, other: Hand) -> bool {
        *self == Hand::Rock && other == Hand::Scissors ||
        *self == Hand::Paper && other == Hand::Rock ||
        *self == Hand::Scissors && other == Hand::Paper
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<(Hand, Hand)> = input
        .lines()
        .filter_map(|x|
            x
                .split_once(' ')
                .map(|split|
                    (split.0.to_string(), split.1.to_string())
                )
        )
        .map(|(a, b)|
            (
                match a.as_str() {
                    "A" => Hand::Rock,
                    "B" => Hand::Paper,
                    "C" => Hand::Scissors,
                    _ => panic!("Invalid Char")
                },
                match b.as_str() {
                    "X" => Hand::Rock,
                    "Y" => Hand::Paper,
                    "Z" => Hand::Scissors,
                    _ => panic!("Invalid Char")
                }
            )
        )
        .collect();

    let mut score: u32 = 0;
    for (opponent_hand, your_hand) in lines {
        score += your_hand.value();

        if your_hand == opponent_hand {
            score += 3;
        } else if your_hand.beats(opponent_hand) {
            score += 6;
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    fn calculate_response(strategy: &Strategy, opponent_hand: &Hand) -> Hand {
        match strategy {
            Strategy::Draw => match opponent_hand {
                Hand::Rock => Hand::Rock,
                Hand::Paper => Hand::Paper,
                Hand::Scissors => Hand::Scissors,
            },
            Strategy::Win => match opponent_hand {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            },
            Strategy::Lose => match opponent_hand {
                Hand::Rock => Hand::Scissors,
                Hand::Paper => Hand::Rock,
                Hand::Scissors => Hand::Paper,
            }
        }
    }

    fn calculate_score(your_hand: Hand, opponent_hand: Hand) -> u32 {
        let mut score = 0; 
        score += your_hand.value();

        if your_hand == opponent_hand {
            score += 3;
        } else if your_hand.beats(opponent_hand) {
            score += 6;
        }
        
        score
    }

    let lines: Vec<(Hand, Strategy)> = input
        .lines()
        .filter_map(|x| {
            x
                .split_once(' ')
                .map(|split| (split.0.to_string(), split.1.to_string()))
        })
        .map(|(a, b)|
            (
                match a.as_str() {
                    "A" => Hand::Rock,
                    "B" => Hand::Paper,
                    "C" => Hand::Scissors,
                    _ => panic!("Invalid Char")
                },
                match b.as_str() {
                    "X" => Strategy::Lose,
                    "Y" => Strategy::Draw,
                    "Z" => Strategy::Win,
                    _ => panic!("Invalid Char")
                }
            )
        )
        .collect();

    let mut score: u32 = 0;
    for (opponent_hand, strategy) in lines {
        let your_hand = calculate_response(&strategy, &opponent_hand);
        score += calculate_score(your_hand, opponent_hand)
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
