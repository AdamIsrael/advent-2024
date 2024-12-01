// https://adventofcode.com/2024/day/1
// Within each pair, figure out how far apart the two numbers are; you'll need
// to add up all of those distances. For example, if you pair up a 3 from the
// left list with a 7 from the right list, the distance apart is 4; if you pair
// up a 9 with a 3, the distance apart is 6.


use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let (col_a, col_b) = parse_input(&mut stdin.lock());
    let distance = calculate_distances(&col_a, &col_b);
    let similarity = calculate_similarity(&col_a, &col_b);

    println!("Distance: {}", distance);
    println!("Similarity: {}", similarity);
}

// Parse and get the vectors of numbers
fn parse_input(input: &mut impl io::BufRead) -> (Vec<i32>, Vec<i32>) {
    let mut col_a: Vec<i32> = Vec::new();
    let mut col_b: Vec<i32> = Vec::new();

    // Parse and push the input into two columns
    for line in input.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        col_a.push(numbers[0]);
        col_b.push(numbers[1]);
    }

    // Sort each column numerically
    col_a.sort();
    col_b.sort();

    (col_a, col_b)
}

fn calculate_distances(col_a: &Vec<i32>, col_b: &Vec<i32>) -> i32 {
    let mut distance = 0;

    // Pair up the numbers and calculate the difference
    for (a, b) in col_a.iter().zip(col_b.iter()) {
        distance += calculate_distance(*a, *b);
    }
    distance
}

fn calculate_distance(a: i32, b: i32) -> i32 {
    // println!("{} - {} = {}", a, b, (a - b).abs());
    (a - b).abs()
}

fn calculate_similarity(col_a: &Vec<i32>, col_b: &Vec<i32>) -> i32 {
    let mut similarity = 0;

    // for each number in column a, find out how many times it occurs in column b
    // and multiply the number by the number of occurrences
    for a in col_a.iter() {
        let count = col_b.iter().filter(|&b| *b == *a).count();
        similarity += *a * count as i32;
    }
    similarity
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_distances() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        let (col_a, col_b) = parse_input(&mut io::Cursor::new(input));
        let distance = calculate_distances(&col_a, &col_b);

        assert_eq!(distance, 11);
    }

    #[test]
    fn test_calculate_similarity() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        let (col_a, col_b) = parse_input(&mut io::Cursor::new(input));
        let similarity = calculate_similarity(&col_a, &col_b);

        assert_eq!(similarity, 31);
    }

}