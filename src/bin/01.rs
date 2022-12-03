fn parse_input(input: &str) -> Option<Vec<u32>> {
    let mut elf_total = 0;
    let mut elves = Vec::<u32>::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(elf_total);
            elf_total = 0;
        } else {
            let calories = line.parse::<u32>().ok()?;
            elf_total += calories
        }
    }
    elves.sort();

    Some(elves)
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = parse_input(input)?;
    let end = elves.len();
    let top_elf = elves.get(end - 1)?;
    Some(*top_elf)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = parse_input(input)?;
    let end = elves.len();
    let (_, top_elves) = elves.split_at(end - 3);
    Some(top_elves.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
