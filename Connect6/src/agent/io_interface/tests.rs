use super::*;

#[test]
fn test_agent_io() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut agent_io = AgentIO::new(
        &mut stdin as &mut io::Read,
        &mut stdout as &mut io::Write,
    );
//    agent_io.play();

    agent_io.agent.terminate();

    assert!(true);
}