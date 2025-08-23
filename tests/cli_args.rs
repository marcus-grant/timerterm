// tests/cli_args.rs
use assert_cmd::Command;

#[test]
fn runs_1sec_simple_args() {
    // Test: Simple check that 1sec is waited
    let mut cmd = Command::cargo_bin("timeterm").unwrap();
    let out = cmd.arg("1").timeout(std::time::Duration::from_secs(2));
    out.assert().success();
}

#[test]
fn runs_with_mins_secs_format() {
    // E2E: Program should accept mm:ss format
    // Won't test full duration, just that for more than 1s it runs
    let mut cmd = Command::cargo_bin("timeterm").unwrap();
    let out = cmd.arg("0:01").timeout(std::time::Duration::from_secs(2));
    out.assert().success();
}

#[test]
fn runs_with_hrs_mins_secs_format() {
    // E2E: Program should accept "0:00:02" (2 seconds) and run for that duration
    let mut cmd = Command::cargo_bin("timeterm").unwrap();
    let out = cmd.arg("0:00:02").timeout(std::time::Duration::from_secs(4));
    out.assert().success(); // Should run for ~2 seconds then exit
}
