mod gridentify;
mod players;

use crate::gridentify::Game;
use crate::players::{greedy::Greedy, most_followup::MostFollowup, random::Random, Player};
use std::collections::HashMap;

use skillratings::{
    glicko2::{glicko2, Glicko2Config, Glicko2Rating},
    Outcomes,
};

use rand::prelude::*;

fn get_players() -> HashMap<&'static str, (Box<dyn Player>)> {
    let mut players: HashMap<&str, (Box<dyn Player>)> = HashMap::new();
    players.insert("random", (Box::new(Random {})));
    players.insert("greedy", (Box::new(Greedy {})));
    players.insert("most_followup", (Box::new(MostFollowup {})));
    players
}

fn make_match<'a>(keys: &'a Vec<&str>) -> Option<(&'a str, &'a str)> {
    let len = keys.len();

    if len >= 2 {
        let mut rng = thread_rng();
        let index1 = rng.gen_range(0..len);

        let mut index2 = rng.gen_range(0..len);
        while index2 == index1 {
            index2 = rng.gen_range(0..len);
        }

        Some((keys[index1], keys[index2]))
    } else {
        None
    }
}

fn play_match(first: &Box<dyn Player>, second: &Box<dyn Player>, seed: u64) -> Outcomes {
    let mut first_game = Game::new(seed);
    while !first_game.done() {
        let m = first.get_move(&first_game.reseed());
        first_game = first_game.child(&m);
    }
    let first_score = first_game.score();
    let mut second_game = Game::new(seed);
    while !second_game.done() {
        let m = second.get_move(&second_game.reseed());
        second_game = second_game.child(&m);
    }
    let second_score = second_game.score();
    match first_score.cmp(&second_score) {
        std::cmp::Ordering::Less => Outcomes::LOSS,
        std::cmp::Ordering::Equal => Outcomes::DRAW,
        std::cmp::Ordering::Greater => Outcomes::WIN,
    }
}

fn main() {
    let config = Glicko2Config::default();
    let players = get_players();
    let keys: Vec<&str> = players.keys().map(|&x| x).collect();
    let mut ratings: HashMap<&str, Glicko2Rating> =
        HashMap::from_iter(keys.iter().map(|&k| (k, Glicko2Rating::new())));

    let mut rng = thread_rng();

    for i in 0..100 {
        println!("Game {i}");
        let (first, second) = make_match(&keys).unwrap();
        let seed: u64 = rng.gen();
        let (player1, player2) = (players.get(first).unwrap(), players.get(second).unwrap());
        println!("{first} - {second} ({seed})");
        let result = play_match(&player1, &player2, seed);
        println!("{result:?}");
        let (new_first, new_second) = glicko2(
            &ratings.get(first).unwrap(),
            &ratings.get(second).unwrap(),
            &result,
            &config,
        );
        ratings.insert(first, new_first);
        ratings.insert(second, new_second);
    }

    let mut keys = keys.clone();
    keys.sort_by_key(|k| -ratings.get(k).unwrap().rating as i64);
    println!();
    for key in keys.iter() {
        println!("{key}\t{}", ratings.get(key).unwrap().rating as i64);
    }
}
