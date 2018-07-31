use super::*;

fn new_table() -> [[Player; 19]; 19] {
    [[Player::None; 19]; 19]
}

#[test]
fn test_base_search() {
    let mut table = new_table();
    // Right
    for i in 0..5 {
        table[10][5+i] = Player::Black;
    }
    assert_eq!(search(&table), Player::None);

    table[10][10] = Player::Black;
    assert_eq!(search(&table), Player::Black);

    // Down
    for i in 0..5 {
        table[5+i][10] = Player::White;
    }
    assert_eq!(search(&table), Player::Black);

    table[10][10] = Player::White;
    assert_eq!(search(&table), Player::White);

    // RightDown
    for i in 0..5 {
        table[5+i][5+i] = Player::Black;
    }
    assert_eq!(search(&table), Player::White);

    table[10][10] = Player::Black;
    assert_eq!(search(&table), Player::Black);

    // LeftDown
    for i in 0..5 {
        table[5+i][15-i] = Player::White;
    }
    assert_eq!(search(&table), Player::Black);

    table[10][10] = Player::White;
    assert_eq!(search(&table), Player::White);
}

#[test]
fn test_boundary_search() {
    // boundary test
    let mut table = new_table();
    for i in 0..5 {
        table[0][i] = Player::White;
        table[0][i + 6] = Player::White;
        table[0][18 - i] = Player::White;
    }
    assert_eq!(search(&table), Player::None);

    table[0][13] = Player::White;
    assert_eq!(search(&table), Player::White);

    let mut table = new_table();
    for i in 0..5 {
        table[18][i] = Player::White;
        table[18][i + 6] = Player::White;
        table[18][18 - i] = Player::White;
    }
    assert_eq!(search(&table), Player::None);

    table[18][5] = Player::White;
    assert_eq!(search(&table), Player::White);

    let mut table = new_table();
    for i in 0..5 {
        table[i][0] = Player::Black;
        table[i + 6][0] = Player::Black;
        table[18 - i][0] = Player::Black;
    }
    assert_eq!(search(&table), Player::None);

    table[13][0] = Player::Black;
    assert_eq!(search(&table), Player::Black);

    let mut table = new_table();
    for i in 0..5 {
        table[i][18] = Player::Black;
        table[i + 6][18] = Player::Black;
        table[18 - i][18] = Player::Black;
    }
    assert_eq!(search(&table), Player::None);

    table[5][18] = Player::Black;
    assert_eq!(search(&table), Player::Black);
}

#[test]
fn test_cross_search() {
    let mut table = new_table();
    for i in 0..5 {
        table[i][i] = Player::Black;
        table[i + 6][i + 6] = Player::Black;
        table[18 - i][18 - i] = Player::Black;
    }
    assert_eq!(search(&table), Player::None);

    table[11][11] = Player::Black;
    assert_eq!(search(&table), Player::Black);

    for i in 0..5 {
        table[i][18 - i] = Player::White;
        table[i + 6][12 - i] = Player::White;
        table[18 - i][i] = Player::White;
    }
    assert_eq!(search(&table), Player::None);

    table[11][7] = Player::White;
    assert_eq!(search(&table), Player::White);
}