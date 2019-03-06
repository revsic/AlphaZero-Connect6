//! C++ Bindings for libconnect6

extern crate connect6;
extern crate rand;

pub mod cppbind;
pub use cppbind::ffi_test::*;

/// Return Connect6 playing results with given cpp callback.
///
/// # Arguments
///
/// * `callback` - callback for cpp_policy, void(float* boards, int* result).
/// * `cpp_alloc_path` - cppbind::RawPath allocator for obtaining memory from cpp ffi.
/// * `cpp_alloc_result` - cppbind::RawPlayResult allocator for obtaining memory from cpp ffi.
/// * `debug` - bool, enable debug mode. if enable, selection and board status will be printed
/// * `num_game_thread` - i32, number of threads asynchronously self-playing connect6
///
#[no_mangle]
pub extern "C" fn cpp_play(
    callback: cppbind::PolicyCallback,
    cpp_alloc_path: cppbind::AllocatorType<cppbind::RawPath>,
    cpp_alloc_result: cppbind::AllocatorType<cppbind::RawPlayResult>,
    debug: bool,
    num_game_thread: i32,
) -> cppbind::RawVec<cppbind::RawPlayResult> {
    use connect6::agent;

    let alloc_path = cppbind::Allocator::new(cpp_alloc_path);
    let alloc_result = cppbind::Allocator::new(cpp_alloc_result);

    let raw_result = if num_game_thread == 1 {
        let mut cpp_policy = cppbind::CppPolicy::new(callback);
        let mut agent = if debug {
            agent::Agent::debug(&mut cpp_policy)
        } else {
            agent::Agent::new(&mut cpp_policy)
        };

        let result = agent.play().unwrap();
        vec![cppbind::RawPlayResult::with_result(&result, &alloc_path)]
    } else {
        let policy_gen = || cppbind::CppPolicy::new(callback);
        let agent = if debug {
            agent::AsyncAgent::debug(policy_gen)
        } else {
            agent::AsyncAgent::new(policy_gen)
        };

        agent
            .run(num_game_thread)
            .iter()
            .map(|x| cppbind::RawPlayResult::with_result(x, &alloc_path))
            .collect::<Vec<_>>()
    };

    cppbind::RawVec::with_vec(raw_result, &alloc_result)
}

/// Return Connect6 self-playing results with given cpp callback and hyperparameters
///
/// # Arguments
///
/// * `callback` - callback for cppbind, void(int player, float* values, float* boards, int length).
/// * `cpp_alloc_path` - cppbind::RawPath allocator for obtaining memory from cpp ffi.
/// * `cpp_alloc_result` - cppbind::RawPlayResult allocator for obtaining memory from cpp ffi.
/// * `num_simulation` - i32, number of simulations for each turn.
/// * `epsilon` - f32, ratio for applying exploit, exploration. lower epsilon, more exploit
/// * `dirichlet_alpha` - f64, hyperparameter for dirichlet distribution
/// * `c_puct` - f32, ratio of q-value and puct, hyperparameter of AlphaZero MCTS
/// * `debug` - bool, enable debug mode. if enable, selection and board status will be printed
/// * `num_game_thread` - i32, number of threads asynchronously self-playing connect6
///
#[no_mangle]
pub extern "C" fn cpp_self_play(
    callback: cppbind::Callback,
    cpp_alloc_path: cppbind::AllocatorType<cppbind::RawPath>,
    cpp_alloc_result: cppbind::AllocatorType<cppbind::RawPlayResult>,
    num_simulation: i32,
    epsilon: f32,
    dirichlet_alpha: f64,
    c_puct: f32,
    debug: bool,
    num_game_thread: i32,
) -> cppbind::RawVec<cppbind::RawPlayResult> {
    use connect6::{agent, policy};

    let param = policy::HyperParameter {
        num_simulation,
        epsilon,
        dirichlet_alpha,
        c_puct,
    };

    let alloc_path = cppbind::Allocator::new(cpp_alloc_path);
    let alloc_result = cppbind::Allocator::new(cpp_alloc_result);

    let raw_result = if num_game_thread == 1 {
        let cppeval = Box::new(cppbind::CppEval::new(callback));
        let mut alphazero = policy::AlphaZero::with_param(cppeval, param);
        let mut agent = if debug {
            agent::Agent::debug(&mut alphazero)
        } else {
            agent::Agent::new(&mut alphazero)
        };

        let result = agent.play().unwrap();
        vec![cppbind::RawPlayResult::with_result(&result, &alloc_path)]
    } else {
        let policy_gen =
            || policy::AlphaZero::with_param(Box::new(cppbind::CppEval::new(callback)), param);
        let async_agent = if debug {
            agent::AsyncAgent::debug(policy_gen)
        } else {
            agent::AsyncAgent::new(policy_gen)
        };

        async_agent
            .run(num_game_thread)
            .iter()
            .map(|x| cppbind::RawPlayResult::with_result(x, &alloc_path))
            .collect::<Vec<_>>()
    };

    cppbind::RawVec::with_vec(raw_result, &alloc_result)
}

/// Returns Connect6 results with given cpp policy and user selection as io_policy
///
/// # Arguments
///
/// * `callback` - callback for cppbind, void(int player, float* values, float* boards, int length).
/// * `cpp_alloc_path` - cppbind::RawPath allocator for obtaining memory from cpp ffi.
/// * `num_simulation` - i32, number of simulations for each turn.
/// * `epsilon` - f32, ratio for applying exploit, exploration. lower epsilon, more exploit.
/// * `dirichlet_alpha` - f64, hyperparameter for dirichlet distribution.
/// * `c_puct` - f32, ratio of q-value and puct, hyperparameter of AlphaZero MCTS.
///
#[no_mangle]
pub extern "C" fn cpp_play_with(
    callback: cppbind::Callback,
    cpp_alloc_path: cppbind::AllocatorType<cppbind::RawPath>,
    num_simulation: i32,
    epsilon: f32,
    dirichlet_alpha: f64,
    c_puct: f32,
) -> cppbind::RawPlayResult {
    use connect6::{agent, policy};

    let param = policy::HyperParameter {
        num_simulation,
        epsilon,
        dirichlet_alpha,
        c_puct,
    };

    let cppeval = Box::new(cppbind::CppEval::new(callback));
    let mut cpp_policy = policy::AlphaZero::with_param(cppeval, param);

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut io_policy = policy::IoPolicy::new(&mut stdin, &mut stdout);

    let mut multi_policy = policy::MultiPolicy::new(&mut cpp_policy, &mut io_policy);
    let result = agent::Agent::debug(&mut multi_policy).play();

    let alloc_path = cppbind::Allocator::new(cpp_alloc_path);
    cppbind::RawPlayResult::with_result(&result.unwrap(), &alloc_path)
}
