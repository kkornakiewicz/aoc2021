use std::fs;

fn part1(nums: &Vec<i32>) -> i32 {
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();
    let range = (*min..*max).collect::<Vec<i32>>();
    let result: i32 = range
        .into_iter()
        .map(|x| nums.iter().map(|y| (x - y).abs()).sum())
        .min()
        .unwrap();
    return result;
}

fn part2(nums: &Vec<i32>) -> i32 {
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();
    let range = (*min..*max).collect::<Vec<i32>>();
    let result: i32 = range
        .into_iter()
        .map(|x| {
            nums.iter()
                .map(|y| {
                    let distance = (x - y).abs();
                    return distance * (distance + 1) / 2;
                })
                .sum()
        })
        .min()
        .unwrap();
    return result;
}

fn main() {
    let filename = "./src/input.txt";
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let nums = data
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().expect("Can't parse number'"))
        .collect::<Vec<i32>>();
    println!("{:}", part1(&nums));
    println!("{:}", part2(&nums));
}
