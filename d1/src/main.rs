use std::fs;

fn part1(lines: &Vec<u32>) -> u32 {
    let mut iter = lines.iter();
    let mut last = iter.next().unwrap();
    let mut result = 0;
    for x in iter {
        if last < x {
            result = result + 1;
        }
        last = x;
    }
    return result
}

fn part2(lines: &Vec<u32>) -> u32 {
    let mut iter = lines.iter();
    let mut new_list = Vec::new();
    let mut a = iter.next().unwrap();
    let mut b = iter.next().unwrap();
    let mut c = iter.next().unwrap();
    new_list.push(a+b+c);
    for x in iter {
        a=b;
        b=c;
        c=x;
        new_list.push(a+b+c);
    }
    return part1(&new_list)
}

fn main() {
    let filename = "./src/input.txt";
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let lines: Vec<u32> = data
        .lines()
        .map(|x| -> u32 { x.parse().expect("Cant parse int") })
        .collect();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
