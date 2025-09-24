mod game_store {
    pub enum Games {
        StrategyGames,
        RpgGames { name: String, platform: String },
    }

    impl Games {
        pub fn print_rpg_game_name(&self) {
            match self {
                Games::RpgGames { name, platform } => {
                    println!("The name of the rpg game is {name} and it runs on {platform}")
                }
                Games::StrategyGames => println!("The game is not an rpg game"),
            }
        }
    }
}

use game_store::Games;

fn main() {
    let game = Games::RpgGames {
        name: String::from("Sekiro"),
        platform: String::from("PS5"),
    };

    if let Games::RpgGames { .. } = &game {
        game.print_rpg_game_name();
    }
}
