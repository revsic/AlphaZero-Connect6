use super::*;

#[test]
fn test_switch() {
    let none = Player::None;
    assert_eq!(none.switch(), Player::None);

    let black = Player::Black;
    assert_eq!(black.switch(), Player::White);

    let white = Player::White;
    assert_eq!(white.switch(), Player::Black);
}

#[test]
fn test_mut_switch() {
    let mut player = Player::None;

    player.mut_switch();
    assert_eq!(player, Player::None);

    player = Player::Black;
    player.mut_switch();
    assert_eq!(player, Player::White);

    player.mut_switch();
    assert_eq!(player, Player::Black);
}