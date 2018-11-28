use connect6::{agent, game::Player, policy::Evaluator, BOARD_CAPACITY, BOARD_SIZE};
use cppbind::*;

#[no_mangle]
pub extern "C" fn test_new_raw_path() -> RawPath {
    RawPath::new()
}

#[no_mangle]
pub extern "C" fn test_with_raw_path() -> RawPath {
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let id = ((i * BOARD_SIZE + j) % 3) as i32 - 1;
            board[i][j] = Player::from(id);
        }
    }

    let path = agent::Path {
        turn: Player::White,
        board,
        pos: (0, BOARD_SIZE % 5 + 1),
    };
    RawPath::with_path(&path)
}

#[no_mangle]
pub extern "C" fn test_echo_raw_path(
    turn: CInt,
    board_ptr: *mut CInt,
    row: CInt,
    col: CInt,
) -> RawPath {
    let board_slice = unsafe { ::std::slice::from_raw_parts(board_ptr, BOARD_CAPACITY) };
    let mut board = [[0; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = board_slice[i * BOARD_SIZE + j];
        }
    }
    RawPath {
        turn,
        board,
        row,
        col,
    }
}

#[no_mangle]
pub extern "C" fn test_with_raw_play_result(allocator: AllocatorType<RawPath>) -> RawPlayResult {
    let mut vec = Vec::new();
    let mut player = Player::Black;

    for i in 0..10 {
        let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
        for j in 0..i + 1 {
            board[j][j] = Player::from(((i + j) % 3) as i32 - 1);
        }
        vec.push(agent::Path {
            turn: player,
            board,
            pos: (i, i + 1),
        });

        player.mut_switch();
    }

    let result = agent::PlayResult {
        winner: Player::Black,
        path: vec,
    };

    let alloc = Allocator::new(allocator);
    RawPlayResult::with_result(&result, &alloc)
}

#[no_mangle]
pub extern "C" fn test_echo_raw_play_result(
    winner: CInt,
    path: *mut RawPath,
    len: CInt,
    allocator: AllocatorType<RawPath>,
) -> RawPlayResult {
    let path_s = unsafe { ::std::slice::from_raw_parts(path, len as usize) };

    let mut vec = Vec::new();
    for i in 0..len as usize {
        let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
        for j in 0..BOARD_SIZE {
            for k in 0..BOARD_SIZE {
                board[j][k] = Player::from(path_s[i].board[j][k]);
            }
        }

        vec.push(agent::Path {
            turn: Player::from(path_s[i].turn),
            board,
            pos: (path_s[i].row as usize, path_s[i].col as usize),
        });
    }

    let result = agent::PlayResult {
        winner: Player::from(winner),
        path: vec,
    };

    let alloc = Allocator::new(allocator);
    RawPlayResult::with_result(&result, &alloc)
}

#[no_mangle]
pub extern "C" fn test_with_raw_vec(allocator: AllocatorType<CInt>) -> RawVec<CInt> {
    let vec = vec![0, 1, 2, 3, 4, 5];
    let alloc = Allocator::new(allocator);
    RawVec::with_vec(vec, &alloc)
}

#[no_mangle]
pub extern "C" fn test_echo_raw_vec(
    ptr: *const CInt,
    len: CInt,
    allocator: AllocatorType<CInt>,
) -> RawVec<CInt> {
    let raw_slice = unsafe { ::std::slice::from_raw_parts(ptr, len as usize) };
    let mut vec = Vec::new();
    for i in 0..len {
        vec.push(raw_slice[i as usize]);
    }
    let alloc = Allocator::new(allocator);
    RawVec::with_vec(vec, &alloc)
}

#[no_mangle]
pub extern "C" fn test_echo_cppeval(
    turn: CInt,
    boards: *const CInt,
    len: CInt,
    callback: Callback,
    allocator: AllocatorType<CFloat>,
) -> RawVec<CFloat> {
    let turn = Player::from(turn);
    let len = len as usize;

    let boards = unsafe { ::std::slice::from_raw_parts(boards, len * BOARD_CAPACITY) };

    let mut cnt = 0;
    let mut vec = Vec::new();
    for _ in 0..len {
        let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                board[r][c] = Player::from(boards[cnt]);
                cnt += 1;
            }
        }
        vec.push(board);
    }

    let cppeval = CppEval::new(callback);
    let res = cppeval.eval(turn, &vec);

    assert!(res.is_some());
    let (vals, policies) = res.unwrap();

    assert_eq!(vals.len(), len);
    assert_eq!(policies.len(), len);

    let mut ret = Vec::new();
    for val in vals {
        ret.push(val);
    }

    for policy in policies {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                ret.push(policy[i][j]);
            }
        }
    }

    let alloc = Allocator::new(allocator);
    RawVec::with_vec(ret, &alloc)
}
