// tests/cli_args.rs
use assert_cmd::Command;

#[test]
fn runs_1sec_simple_args() {
    // Test: Simple check that 1sec is waited
    let mut cmd = Command::cargo_bin("timeterm").unwrap();
    let out = cmd.arg("1").timeout(std::time::Duration::from_secs(2));
    out.assert().success();
}
