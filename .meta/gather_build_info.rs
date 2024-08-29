use std::process::{Command};

fn gather_build_info() {
    println!("cargo:rerun-if-changed=.git");
    gather_git_info();
}

fn gather_git_info() {
    let git_status = Command::new("git")
        .arg("status")
        .status();
    let Ok(status) = git_status else {
        println!("cargo:warning=failed to execute git command");
        return;
    };
    let Some(statuscode) = status.code() else {
        println!("cargo:warning=git status returned with error");
        return;
    };
    if statuscode != 0 {
        // we are not in a git repo
        return;
    }

    // get last commit id
    let git_commit = Command::new("git")
        .arg("log")
        .arg("--format=\"%H\"")
        .arg("-n")
        .arg("1").output();
    if let Ok(git_commit) = git_commit {
        if let Some(statuscode) = git_commit.status.code() {
            if statuscode == 0 {
                let stdout = git_commit.stdout;
                let stdout_line = String::from_utf8(stdout).unwrap();
                let stdout_line = stdout_line.trim();
                println!("cargo:rustc-env=BUILD_GIT_HASH={}", stdout_line);
            }
        }
    }

    let git_branch = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output();
    if let Ok(git_branch) = git_branch {
        if let Some(statuscode) = git_branch.status.code() {
            if statuscode == 0 {
                let stdout = git_branch.stdout;
                let stdout_line = String::from_utf8(stdout).unwrap();
                let stdout_line = stdout_line.trim();
                println!("cargo:rustc-env=BUILD_GIT_BRANCH={}", stdout_line);
            }
        }
    }

    let date = Command::new("date")
        .arg("+\"%Y-%m-%d %T\"")
        .env("TZ", "UTC")
        .output();

    if let Ok(date) = date {
        if let Some(statuscode) = date.status.code() {
            if statuscode == 0 {
                let stdout = date.stdout;
                let stdout_line = String::from_utf8(stdout).unwrap();
                let stdout_line = stdout_line.trim();
                println!("cargo:rustc-env=BUILD_DATETIME={}", stdout_line);
            }
        }
    }
}