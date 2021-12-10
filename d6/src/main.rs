use std::fs;

fn part1(nms: Vec<i8>) -> usize{
    let mut nums = nms.clone();
    for i in 0..79 {
        nums = nums.into_iter().map(|x| x - 1).collect::<Vec<i8>>();
        let new_borns = nums
            .clone()
            .into_iter()
            .filter(|x| *x == 0)
            .collect::<Vec<i8>>()
            .len();
        nums = nums
            .clone()
            .into_iter()
            .filter(|x| *x != 0)
            .collect::<Vec<i8>>();
        nums.append(&mut [7].repeat(new_borns));
        nums.append(&mut [9].repeat(new_borns));
    }
    return nums.len()
}

fn main() {
    let filename = "./src/input.txt";
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let mut nums = data
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().expect("Can't parse number'"))
        .collect::<Vec<i8>>();
    println!("{}", part1(nums));
}
