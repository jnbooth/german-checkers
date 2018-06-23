extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;
use std::iter::repeat;

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

#[derive(Debug, PartialEq)]
enum Dir { FlatLeft
         , FlatRight 
         , UpLeft
         , UpRight
         , DownLeft
         , DownRight
         }

#[derive(Debug)]
struct Move { n: u8, dir: Dir }

fn add_move(xs: &mut Vec<Move>, n: i8, dir: Dir) {
    if n > 0 {
        xs.push(Move { n: n as u8, dir: dir })
    }
}

fn neighbors(n: &u8) -> Vec<Move> {
    let mut xs    = Vec::new();
    let     n_    = *n as i8;
    let     row   = row_size(n);
    let     left  = !test_bit(&EDGE_LEFT,  n);
    let     right = !test_bit(&EDGE_RIGHT, n);
    let     up    = n_ > 44;
    let     up_   = !up as i8;
    let     down  = n_ > 35;
    let     down_ = !down as i8;

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
    neighbors(n)
        .into_iter()
        .map(|x| x.n)
        .filter(|n| !test_bit(game, &n))
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
    for n1 in hop(game, n) { 
        let game1 = game0 | to_bit(&n1);
        if hopped.insert(game1 | to_bit(&n1)) {
            add_hops(hopped, &game1, &n1)
        }
    }
}

fn hops(game: &u128) -> HashSet<u128> {
    let mut hopped = HashSet::new();
    hopped.insert(*game);
    for n in (0..GAME_SIZE).filter(|n| test_bit(game, n)) {
        add_hops(&mut hopped, &game, &n)
    }
    hopped.remove(game);
    hopped
}

fn play(games: &mut HashSet<u128>, game: &u128) -> Vec<u128> {
    let mut moves = (0..GAME_SIZE)
        .filter(|n| test_bit(game, n))
        .flat_map(|n| slide(game, &n))
        .collect::<Vec<u128>>();
    moves.extend(hops(game));
    moves.retain(|&x| games.insert(x));
    moves
}

fn show_tiles(i: isize, xs: u128) -> String {
    if i == 0 { String::new() } 
    else {
        let abs_i = i.abs();
        let abs_z = abs_i as usize;
        let pow   = 2u128.pow(abs_i as u32);
        let mut s = format!("{:b}", xs % pow);
        s         = repeat('0').take(abs_z - s.len()).collect::<String>() + &s;
        format!( "{}{}\n{}"
               , repeat(' ').take(9 - abs_z).collect::<String>()
               , s.chars().intersperse(' ').fuse().collect::<String>()
               , &show_tiles(if i == 8 {-9} else {i + 1}, xs / pow)
               )
    }
}

fn row_size(n: &u8) -> i8 {
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

fn solve() -> Vec<u128> {
    let     games   = &mut HashSet::new();
    let     start   = (71..81).map(|x| to_bit(&x)).sum();
    let     end     = (0 ..10).map(|x| to_bit(&x)).sum();
    let mut playing = vec![vec![start]];
    loop {
        println!("{}: {}", playing[0].len(), playing.len());
        let mut playing_: Vec<Vec<u128>> = Vec::new();
        for turns in &playing {
            if let Some(turn) = turns.last() {
                let played = play(games, &turn);
                if played.contains(&end) {
                    return turns.clone()
                }
                playing_.extend(played
                    .into_iter()
                    .map(|game| {
                        let mut turns_ = turns.clone();
                        turns_.push(game);
                        turns_
                    }))
            }
        }
        playing = playing_
    }
}

fn main() {
    println!("{}", solve().into_iter().map(|x| show_tiles(1, x)).join("\n\n"));
    println!("\n{}", show_tiles(1, (0 ..10).map(|x| to_bit(&x)).sum()));
}
