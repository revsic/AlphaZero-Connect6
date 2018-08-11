use super::*;
use super::super::super::agent::Agent;
use super::super::super::game::Player;

use std::time::Instant;

#[test]
#[ignore]
fn test_agent_io() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut policy = IoPolicy::new(&mut stdin, &mut stdout);

    let now = Instant::now();
    let result = Agent::new(&mut policy).play();
    let done = now.elapsed().as_secs();

    println!("{} elapsed", done);
    let result = result.map_err(|_| assert!(false)).unwrap();
    if let Some(last) = result.path.last() {
        if result.winner != Player::None {
            assert_eq!(last.turn, result.winner);
        }
    }
    assert!(true);
}