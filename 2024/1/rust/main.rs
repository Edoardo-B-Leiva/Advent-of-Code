use std::fs::read_to_string;

fn main() {
    let problem_content: Vec<String> = read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut problem: [Vec<i32>; 2] = [Vec::new(), Vec::new()];
    for i in &problem_content {
        let temp_vec: Vec<i32> = i
            .split("   ")
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();

        problem[0].push(temp_vec[0]);
        problem[1].push(temp_vec[1]);
    }

    std::mem::drop(problem_content);

    // sorting, duh...
    problem[0].sort();
    problem[1].sort();

    let mut distances: Vec<i32> = Vec::new();
    for i in 0..problem[0].len() {
        distances.push((problem[1][i] - problem[0][i]).abs());
    }

    let total_distance: i32 = distances.iter().sum();
    println!("{}", total_distance);
}
