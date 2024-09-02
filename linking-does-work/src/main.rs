#[allow(non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::env;
use std::process::{ Stdio, Command };
use fork::{daemon, Fork};
use std::fs::File;

fn main() {
    println!("[Rust] Hello, world!");

    let cwd = env::current_dir().unwrap();
    if let Ok(Fork::Child) = daemon(false, true) { // see [StackOverflow](https://stackoverflow.com/a/62978274), set 2nd arg to `true` to display `stdout,stderr`                                               //
        let file = File::create("/tmp/output.txt").expect("Failed to create file");

        assert!(env::set_current_dir(&cwd).is_ok());
        let _output = Command::new("cargo")
            .args(["run","--bin","other-bin"])
            .stdout(Stdio::from(file))
            .spawn()
            .expect("Failed to execute ioverbs-fuse");

        // println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    }
    println!("Forked");
}
