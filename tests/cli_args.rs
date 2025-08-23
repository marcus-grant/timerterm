// tests/cli_args.rs
use assert_cmd::Command;

#[test]
fn duration_parsing() {
    // Test: Program should accept '30' as a 30-second duration
    let mut cmd = Command::cargo_bin("timeterm").unwrap();
    let out = cmd.arg("30").timeout(std::time::Duration::from_secs(2));
    out.assert().success();
}
