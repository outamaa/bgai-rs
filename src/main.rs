use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
use bgai;
use bgai::game::go::{self, Move, Color, GoState};
use bgai::agent::{RandomBot, Agent, MinimaxBot};
use bgai::GameState;

fn main() {
    go_game()
}

fn go_game() {
    let mut game = GoState::new(5);
    let mut bots: HashMap<Color, Box<dyn Agent<GoState>>> = HashMap::new();
    bots.insert(Color::White, Box::new(RandomBot::new()));
    bots.insert(Color::Black, Box::new(MinimaxBot::new(5, go::stone_difference)));

    while !game.is_over() {
        std::thread::sleep(Duration::from_millis(100));
        //print!("{}[2J", 27 as char);
        println!("{:?}", &game.board);
        let mut s=String::new();
        print!("Proceed? (press enter) ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Something went wrong");
        let bot_move = bots.get_mut(&game.next_player.color).unwrap().select_move(&game);
        print_move(game.next_player.color, &bot_move);
        game = game.apply_move(&bot_move);
    }
    // print!("{}[2J", 27 as char);
    println!("{:?}", &game.board);
    println!("Game over!");
}

fn print_move(player: Color, the_move: &Move) {
    let player = match player {
        Color::Black => "Black",
        Color::White => "White"
    };
    let the_move = match the_move {
        Move::Play(point) => format!("plays in ({},{})", point.row, point.col),
        Move::Pass => "passes".to_string(),
        Move::Resign => "resigns".to_string(),
    };

    println!("{} {}", player, the_move);
}