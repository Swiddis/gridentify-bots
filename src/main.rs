use std::collections::HashMap;

use skillratings::{
    glicko2::{glicko2, Glicko2Config, Glicko2Rating},
    Outcomes
};

use rand::prelude::*;
use serde_json;

fn main() {
    let config = Glicko2Config::default();

    let player_list = vec!["p1", "p2", "p3"];
    let mut players: HashMap<&str, Glicko2Rating> = HashMap::new();
    players.insert("p1", Glicko2Rating::new());
    players.insert("p2", Glicko2Rating::new());
    players.insert("p3", Glicko2Rating::new());

    let mut rng = thread_rng();

    for _ in 0..500 {
        let (a, b) = (rng.gen_range(0..3), rng.gen_range(0..3));
        if a == b {
            continue;
        }
        let (new_a, new_b) = glicko2(
            &players.get(player_list[a]).unwrap(),
            &players.get(player_list[b]).unwrap(),
            &Outcomes::WIN,
            &config
        );

        players.insert(player_list[a], new_a);
        players.insert(player_list[b], new_b);
    }

    println!("{}", serde_json::to_string_pretty(&players).unwrap());
}
