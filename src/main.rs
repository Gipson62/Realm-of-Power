use realm_of_power::Game;

fn main() {
    let game = Game::new().expect("Failed to create game");

    game.run().expect("Failed to run game");
}
