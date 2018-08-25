use super::*;
use super::super::super::BOARD_SIZE;
use super::super::super::policy::DefaultPolicy;

use std::sync::mpsc;
use std::thread;

struct TestPolicy {
    receiver: mpsc::Receiver<(usize, usize)>,
}

impl Policy for TestPolicy {
    fn next(&mut self, _: &Game) -> Option<(usize, usize)> {
        let pos = self.receiver.recv().unwrap();
        Some(pos)
    }
}

macro_rules! create_test_agent {
    ($sender:ident, $id:ident) => {
        let ($sender,receiver) = mpsc::channel();
        let $id = thread::spawn(move || {
            let mut policy = TestPolicy { receiver };
            Agent::new(&mut policy).play()
        });
    }
}

#[test]
fn test_new() {
    let mut policy = DefaultPolicy::new();
    let _agent = Agent::new(&mut policy);
    assert!(true);
}

#[test]
fn test_play_invalid_position() {
    create_test_agent!(sender, created);
    sender.send((BOARD_SIZE, BOARD_SIZE)).unwrap();

    let result = created.join();
    assert!(result.is_ok());

    match result.unwrap() {
        Ok(_) => assert!(false),
        Err(err) => assert_eq!(err, String::from("agent::play : game::play invalid position")),
    }
}

#[test]
fn test_play_already_set_position() {
    create_test_agent!(sender, created);
    sender.send((0, 0)).unwrap();
    sender.send((0, 0)).unwrap();

    let result = created.join();
    assert!(result.is_ok());

    match result.unwrap() {
        Ok(_) => assert!(false),
        Err(err) => assert_eq!(err, String::from("agent::play : game::play already set position")),
    }
}

#[test]
fn test_play() {
    create_test_agent!(sender, created);

    sender.send((0, 0)).unwrap(); // black
    let record = [
        ((0, 1), (0, 2)), // white
        ((1, 0), (2, 0)), // black
        ((0, 3), (0, 4)), // white
        ((3, 0), (4, 0)), // black
        ((0, 5), (0, 6)), // white
    ];

    // playing game
    for (turn1, turn2) in record.iter() {
        sender.send(*turn1).unwrap();
        sender.send(*turn2).unwrap();
    }

    let result = created.join();
    assert!(result.is_ok());

    let run_result = result.unwrap()
        .map_err(|_| assert!(false))
        .unwrap();
    assert_eq!(run_result.winner, Player::White);
    assert_eq!(run_result.path.len(), 11);

    let mut turn = Player::Black;
    let mut num_remain = 1;
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];

    let mut paths = run_result.path.iter();
    let path = paths.next();
    assert!(path.is_some());
    assert_eq!(*path.unwrap(), Path { turn, board, pos: (0, 0)});

    let mut prev = (0, 0);
    let mut test = |pos: (usize, usize)| {
        let path = paths.next();
        assert!(path.is_some());

        let path = path.unwrap();
        let (row, col) = prev;
        board[row][col] = turn;

        prev = pos;
        num_remain -= 1;
        if num_remain <= 0 {
            num_remain = 2;
            turn.mut_switch();
        }
        assert_eq!(*path, Path { turn, board, pos });
    };

    // expect history equal to record
    for (turn1, turn2) in record.iter() {
        test(*turn1);
        test(*turn2);
    }
}