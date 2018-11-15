/// Create IoPolicy with stdio
///
/// # Examples
/// ```ignore
/// # #[macro_use] extern crate connect6;
/// # use connect6::agent::Agent;
/// io_policy_stdio!(policy);
/// let result = Agent::new(&mut policy).play();
/// assert!(result.is_ok());
/// ```
#[macro_export]
macro_rules! io_policy_stdio {
    ($policy:ident) => {
        use std;
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let mut $policy = $crate::policy::IoPolicy::new(&mut stdin, &mut stdout);
    };
}
