use std::process::Command;
use clap::{App, Arg, ArgGroup};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum BranchKind {
    Current(String),
    Other(String),
}

fn main() {
    /*
    let app = App::new("gbranch")
        .version("0.0,0")
        .author("Senk")
        .about("git branch manager")
        .arg(Arg::from_usage("-c --compile <SOURCE> 'source_string'").required(false))
        .arg(Arg::from_usage("<SOURCE_FILE> 'source_file'").required(false))
        .group(ArgGroup::with_name("input").args(&["compile", "SOURCE_FILE"]));
    */

    let output = Command::new("sh")
        .arg("-c")
        .arg("git branch")
        .output()
        .expect("failed to execute process");

    let tmp = String::from_utf8(output.stdout).unwrap();
    let branchs = tmp
        .lines()
        .map(|branch| {
            if is_current_branch(branch) {
                BranchKind::Current(branch.trim_start_matches('*').trim_start().to_owned())
            } else {
                BranchKind::Other(branch.trim_start().to_owned())
            }
        })
        .collect::<Vec<BranchKind>>();

    dbg!(branchs);
}

fn is_current_branch(branch: &str) -> bool {
    branch.starts_with('*')
}
