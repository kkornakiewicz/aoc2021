use std::fs;

fn part1(lines: &Vec<(&str, i32)>) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    for x in lines {
        match x.0 {
            "forward" => position = (position.0 + x.1, position.1),
            "up" => position = (position.0, position.1 - x.1),
            "down" => position = (position.0, position.1 + x.1),
            _ => panic!("Should not happend, command: {}",x.0),
        }
    }
    return position.0 * position.1;
}

fn part2(lines: &Vec<(&str, i32)>) -> i32 {
    let mut position: (i32, i32, i32) = (0, 0, 0);
    for x in lines {
        match x.0 {
            "forward" => position = (position.0 + x.1, position.1 + x.1*position.2, position.2),
            "up" => position = (position.0, position.1, position.2 - x.1 ),
            "down" => position = (position.0, position.1, position.2 + x.1),
            _ => panic!("Should not happend, command: {}",x.0),
        }
    }
    return position.0 * position.1;
}

fn main() {
    let filename = "./src/input.txt";
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let lines: Vec<(&str, i32)> = data
        .lines()
        .map(|x| -> (&str, i32) {
            let mut splitted = x.split_whitespace();
            return (
                splitted.next().expect("Unable to parse input"),
                splitted
                    .next()
                    .expect("Unable to parse input")
                    .parse()
                    .expect("Unable to parse number"),
            );
        })
        .collect();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
