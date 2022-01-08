use std::collections::HashMap;
use std::time::Duration;
use rand::thread_rng;
use bgai;
use bgai::{Move, Player};
use bgai::agent::{RandomBot, Agent};

fn main() {
    let mut game = bgai::GameState::new(9);
    let mut bots: HashMap<Player, Box<dyn Agent>> = HashMap::new();
    bots.insert(Player::White, Box::new(RandomBot::new()));
    bots.insert(Player::Black, Box::new(RandomBot::new()));

    while !game.is_over() {
        std::thread::sleep(Duration::from_millis(100));
        print!("{}[2J", 27 as char);
        println!("{:?}", &game.board);
        let bot_move = bots.get_mut(&game.next_player).unwrap().select_move(&game);
        print_move(game.next_player, &bot_move);
        game = game.apply_move(bot_move);
    }
    print!("{}[2J", 27 as char);
    println!("{:?}", &game.board);
    println!("Game over!");
}

fn print_move(player: Player, the_move: &Move) {
    let player = match player {
        Player::Black => "Black",
        Player::White => "White"
    };
    let the_move = match the_move {
        Move::Play(point) => format!("plays in ({},{})", point.row, point.col),
        Move::Pass => "passes".to_string(),
        Move::Resign => "resigns".to_string(),
    };

    println!("{} {}", player, the_move);
}