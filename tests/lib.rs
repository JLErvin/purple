use purple::{self, game::Game};


#[test]
fn should_init_default_game() {
    let game = Game::new();
    let moves = game.legal_moves();
    assert_eq!(moves.len(), 20);
}

#[test]
fn should_init_game_from_fen() {
    let game = Game::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    assert!(game.is_ok());
    let game = game.unwrap();
    let moves = game.legal_moves();
    assert_eq!(moves.len(), 48);
}

#[test]
fn should_find_mate_in_one() {
    let game = Game::new();
    let best_move = game.best_move();
}

#[test]
fn should_correctly_run_perft_test() {

}