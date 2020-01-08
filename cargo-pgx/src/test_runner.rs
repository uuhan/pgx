use std::process::{Command, Stdio};

pub(crate) fn test_extension(version: &str) -> Result<(), std::io::Error>{
    let versions = if version == "all" {
        vec!["pg10", "pg11", "pg12"]
    } else {
        vec![version]
    };

    for version in versions {
        let result = Command::new("cargo")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("test")
            .arg("--all")
            .arg("--features")
            .arg(version)
            .arg("--no-default-features")
            .env("RUST_BACKTRACE", "1")
            .status();

        if !result.is_ok() {
            return Err(result.err().unwrap());
        }
    };

    Ok(())
}