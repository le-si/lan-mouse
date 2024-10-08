use std::process::Command;

fn main() {
    // commit hash
    let git_describe = Command::new("git")
        .arg("describe")
        .arg("--always")
        .arg("--dirty")
        .arg("--tags")
        .output()
        .unwrap();

    let git_describe = String::from_utf8(git_describe.stdout).unwrap();
    println!("cargo::rustc-env=GIT_DESCRIBE={git_describe}");
}
