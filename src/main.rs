#![feature(int_to_from_bytes)]
extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;
use std::iter::repeat;

const PATIENCE: usize = 3;

const GAME_SIZE: u8 = 1+2+3+4+5+6+7+8+9+8+7+6+5+4+3+2+1;

fn to_bit(n: &u8) -> u128 {
    2u128.pow(*n as u32)
}
fn test_bit(bits: &u128, n: &u8) -> bool {
    bits & (1u128 << n) != 0
}

// using foldl (\xs x -> xs + 2 ^ x) 0
// [0, 2, 5, 9, 14, 20, 27, 35, 44, 52, 59, 65, 70, 74, 77, 79, 80]
const EDGE_LEFT:  u128 = 1984611988896094432018981;
// [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 53, 60, 66, 71, 75, 78, 80]
const EDGE_RIGHT: u128 = 1551372338562930514625611;

fn row_size(n: &u8) -> i16 {
         if n == &0 { 1 }
    else if n < &3  { 2 }
    else if n < &6  { 3 }
    else if n < &10 { 4 }
    else if n < &15 { 5 }
    else if n < &21 { 6 }
    else if n < &28 { 7 }
    else if n < &36 { 8 }
    else if n < &45 { 9 }
    else if n < &53 { 8 }
    else if n < &60 { 7 }
    else if n < &66 { 6 }
    else if n < &71 { 5 }
    else if n < &75 { 4 }
    else if n < &78 { 3 }
    else if n < &80 { 2 }
    else            { 1 }
}

fn scored(game: &u128) -> i16 {
    (0..GAME_SIZE)
        .filter(|&n| test_bit(&game, &n))
        .map(|n| if n < 45 { row_size(&n) } else { 18 - row_size(&n) })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Dir { 
    FlatLeft, 
    FlatRight, 
    UpLeft, 
    UpRight, 
    DownLeft, 
    DownRight,
}

#[derive(Debug)]
struct Move { n: u8, dir: Dir }

fn add_move(xs: &mut Vec<Move>, n: i16, dir: Dir) {
    if n >= 0 {
        xs.push(Move { n: n as u8, dir: dir })
    }
}

fn neighbors(n: &u8) -> Vec<Move> {
    let mut xs    = Vec::new();
    let     n_    = *n as i16;
    let     row   = row_size(&n);
    let     left  = !test_bit(&EDGE_LEFT,  n);
    let     right = !test_bit(&EDGE_RIGHT, n);
    let     up    = n_ > 44;
    let     up_   = !up as i16;
    let     down  = n_ > 35;
    let     down_ = !down as i16;

    if left {
        add_move(&mut xs, n_ + 1, Dir::FlatLeft);
    }
    if right {
        add_move(&mut xs, n_ - 1, Dir::FlatRight);
    }
    if right || up {
        add_move(&mut xs, n_ - row + up_ - 1, Dir::UpRight);
    }
    if right || !up {
        add_move(&mut xs, n_ + row + down_ - 1, Dir::DownRight);
    }
    if left || up {
        add_move(&mut xs, n_ - row + up_, Dir::UpLeft);
    }
    if left || !up {
        add_move(&mut xs, n_ + row + down_, Dir::DownLeft);
    }
    xs.retain(|x| x.n < GAME_SIZE);
    xs
}

fn slide(game: &u128, n: &u8) -> Vec<u128> {
    let game_ = game & !to_bit(&n);
    neighbors(&n)
        .into_iter()
        .map(|x| x.n)
        .filter(|&n| !test_bit(&game, &n))
        .map(|n| game_ | to_bit(&n))
        .collect()
}

fn hop(game: &u128, n: &u8) -> Vec<u8> {
    neighbors(n)
        .into_iter()
        .filter(|n1| test_bit(&game, &n1.n))
        .flat_map(|n1| neighbors(&n1.n)
            .into_iter()
            .filter(|n2| (n2.dir == n1.dir) && !test_bit(&game, &n2.n))
            .map(|n2| n2.n)
            .collect::<Vec<u8>>())
        .collect()
}

fn add_hops(hopped: &mut HashSet<u128>, game: &u128, n: &u8) {
    let game0 = game & !to_bit(n);
    for n1 in hop(&game, &n) { 
        let game1 = game0 | to_bit(&n1);
        if hopped.insert(&game1 | to_bit(&n1)) {
            add_hops(hopped, &game1, &n1)
        }
    }
}

fn hops(game: &u128) -> HashSet<u128> {
    let mut hopped = HashSet::new();
    hopped.insert(*game);
    for n in (0..GAME_SIZE).filter(|&n| test_bit(&game, &n)) {
        add_hops(&mut hopped, &game, &n)
    }
    hopped.remove(&game);
    hopped
}

fn play(game: &u128) -> HashSet<u128> {
    let mut plays: HashSet<u128> = (0..GAME_SIZE)
        .filter(|&n| test_bit(&game, &n))
        .flat_map(|n| slide(&game, &n))
        .collect();
    plays.extend(hops(&game));
    plays.remove(&game);
    plays
}

fn main() {
    let mut games: Vec<Vec<u128>> = Vec::new();

    games.push(vec![(71..81).map(|x| to_bit(&x)).sum()]);

    let end = (0..10).map(|x| to_bit(&x)).sum();

    let mut lvl = 0u8;

    let mut scores = [std::i16::MAX; PATIENCE];
    let mut example: Vec<u128> = Vec::new();

    loop {
        let score_i = lvl as usize % PATIENCE;
        let prev_score = scores[(lvl + 1) as usize % PATIENCE];
        let mut new_games: Vec<Vec<u128>> = Vec::new();
        let mut tagged: HashSet<u128> = HashSet::new();

        println!("{}", show_tiles(&example));
        println!("Loaded {} posts at level {} and score {}", games.len(), lvl, prev_score);

        for game_chain in games {
            if let Some(game) = game_chain.last() {
                for played in play(&game) {
                    if tagged.insert(played) {
                        let played_score = scored(&played);
                        if played_score <= prev_score {
                            let mut new_chain = game_chain.clone();
                            new_chain.push(played);
                            if played == end {
                                return println!("{}", show_tiles(&new_chain))
                            }
                            if played_score < scores[score_i] {
                                scores[score_i] = played_score;
                                example = new_chain.clone();
                            }
                            new_games.push(new_chain);
                        }
                    }
                }
            }
        }
        if scores[score_i] == prev_score { return }
        games = new_games;
        lvl += 1;
    }
}

fn show_tiles(chain_: &Vec<u128>) -> String {
    let mut output = String::new();
    let mut chain  = chain_.clone();
    if let Some(mut first) = chain.pop() {
        while let Some(next) = chain.pop() {
            output = format!("{}\n\n{}", show_tiles_(&1, &next, &first), output);
            first = next;
        }
        output = format!("{}\n\n{}", show_tiles_(&1, &first, &first), output);
    }
    output.replace("0","·").replace("1","●")
}

fn show_tiles_(depth: &isize, board: &u128, parent: &u128) -> String {
    if depth == &0 { String::new() } 
    else {
        let abs_depth = depth.abs() as usize;
        let pow       = 2u128.pow(abs_depth as u32);
        let remainder = (board % pow) as u32;
        if remainder == 0 {

        }
        format!( 
            "{}{}\n{}", 
            repeat(' ').take(9 - abs_depth).collect::<String>(), 
            format!("{:09b}", remainder)
                .chars().enumerate().map(|(i, item)| {
                    let i_ = 8 - i as u8; 
                    let on_board  = test_bit(&board, &i_);
                    let on_parent = test_bit(&parent, &i_);
                    if on_board && !on_parent {
                        '-'
                    }
                    else if on_parent && !on_board {
                        '◎'
                    }
                    else { 
                        item 
                    }
                }).skip(9 - abs_depth).intersperse(' ').collect::<String>(),
            &show_tiles_(&(if depth == &8 {-9} else {depth + 1}), &(board / pow), &(parent / pow))
        )
    }
}
