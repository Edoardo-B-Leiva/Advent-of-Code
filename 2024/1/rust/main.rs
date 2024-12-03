use std::fs::read_to_string;

fn main(){
    let problem_content : Vec<String> = read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut problem : [Vec<u32>; 2] = [Vec::new(), Vec::new()];
    for i in &problem_content{
        let temp_vec : Vec<u32> = i
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
    for i in &mut problem{
        i.sort();
    }

    // calculating distances
    let mut distances: Vec<i32> = Vec::new();
    for i in 0..problem[0].len() {
        distances.push((problem[1][i] as i32 - problem[0][i] as i32).abs());
    }

    // printing sum of distances
    let total_distance: i32 = distances.iter().sum();
    println!("Total distance: {}", total_distance);
}
