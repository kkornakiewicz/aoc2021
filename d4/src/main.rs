use std::fs;

#[derive(Clone, Debug)]
struct Cell {
    marked: bool,
    num: i32,
}

type Board = Vec<Cell>;

fn mark_board(board: Board, num: i32) -> Board {
    return board
        .into_iter()
        .map(|x| {
            if x.num == num {
                Cell {
                    marked: true,
                    num: num,
                }
            } else {
                x
            }
        })
        .collect();
}

fn is_winning_board(b: &Board) -> bool {
    if (b[0].marked && b[1].marked && b[2].marked && b[3].marked && b[4].marked)
        || (b[5].marked && b[6].marked && b[7].marked && b[8].marked && b[9].marked)
        || (b[10].marked && b[11].marked && b[12].marked && b[13].marked && b[14].marked)
        || (b[15].marked && b[16].marked && b[17].marked && b[18].marked && b[19].marked)
        || (b[20].marked && b[21].marked && b[22].marked && b[23].marked && b[24].marked)
        || (b[0].marked && b[5].marked && b[10].marked && b[15].marked && b[20].marked)
        || (b[1].marked && b[6].marked && b[11].marked && b[16].marked && b[21].marked)
        || (b[2].marked && b[7].marked && b[12].marked && b[17].marked && b[22].marked)
        || (b[3].marked && b[8].marked && b[13].marked && b[18].marked && b[23].marked)
        || (b[4].marked && b[9].marked && b[14].marked && b[19].marked && b[24].marked)
    {
        return true;
    }
    return false;
}

fn part1(draws: &Vec<i32>, bds: Vec<Board>) -> Option<i32> {
    let mut boards = bds.clone();
    for draw in draws {
        boards = boards.into_iter().map(|b| mark_board(b, *draw)).collect();
        let new_boards = boards
            .clone()
            .into_iter()
            .filter(is_winning_board)
            .collect::<Vec<Board>>();
        if new_boards.len() == 1 {
            let winning_board = &new_boards[0];
            let result = winning_board
                .into_iter()
                .filter(|x| x.marked == false)
                .map(|x| x.num)
                .sum::<i32>()
                * draw;
            return Some(result);
        }
    }
    return None;
}

fn part2(draws: &Vec<i32>, bds: Vec<Board>) -> Option<i32> {
    let mut boards = bds.clone();
    for draw in draws {
        let new_boards = boards
            .clone()
            .into_iter()
            .map(|b| mark_board(b, *draw))
            .filter(|x| !is_winning_board(&x))
            .collect::<Vec<Board>>();
        if new_boards.len() == 0 {
            let winning_board = mark_board(boards[0].clone(), *draw);
            let result = winning_board
                .into_iter()
                .filter(|x| x.marked == false)
                .map(|x| x.num)
                .sum::<i32>()
                * draw;
            return Some(result);
        }
        boards = new_boards;
    }
    return None;
}

fn main() {
    let filename = "./src/input.txt";
    let data = fs::read_to_string(filename).expect("Unable to read file");
    let mut lines = data.lines();
    let draws: Vec<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().expect("Can't parse digit'"))
        .collect();

    let raw_boards: Vec<Vec<i32>> = lines
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().expect("Can't parse digit'"))
                .collect::<Vec<i32>>()
        })
        .collect();
    let boards: Vec<Board> = raw_boards
        .chunks(5)
        .map(|b| {
            b.iter()
                .map(|x| {
                    x.iter().map(|y| Cell {
                        marked: false,
                        num: *y,
                    })
                })
                .flatten()
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Board>>();

    println!("{}", part1(&draws, boards.clone()).unwrap());
    println!("{}", part2(&draws, boards.clone()).unwrap());
}
