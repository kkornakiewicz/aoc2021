use std::fs;

fn sum_vecs(vec_1: Vec<i32>, vec_2: Vec<i32>) -> Vec<i32> {
    return vec_1.iter().zip(vec_2.iter()).map(|(a, b)| a + b).collect();
}

fn part1(lines: Vec<Vec<i32>>) -> isize {
    let half_count: i32 = (lines.len() / 2).try_into().unwrap();
    let sum = lines
        .into_iter()
        .reduce(sum_vecs)
        .expect("Failed merging lists");

    let gamma_binary = sum
        .iter()
        .map(|x| if x > &half_count { "1" } else { "0" })
        .collect::<String>();
    let epsilon_binary = sum
        .iter()
        .map(|x| if x < &half_count { "1" } else { "0" })
        .collect::<String>();

    let gamma = isize::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_binary, 2).unwrap();
    return gamma * epsilon;
}

fn main() {
    let filename = "./src/input.txt";
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let lines: Vec<Vec<i32>> = data
        .lines()
        .map(|x| -> Vec<i32> {
            x.split("")
                .filter(|y| !y.is_empty())
                .map(|y| -> i32 { y.parse().expect("Unable to parse digit") })
                .collect()
        })
        .collect();

    println!("{}", part1(lines))
}
