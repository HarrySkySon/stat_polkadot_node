use std::process::{Command};
use std::thread;

fn main () {
    thread::spawn(|| {
        let _output = Command::new("cmd")
            .current_dir("E:/substrate/substrate-node-template/target/release")
            .args(&["/C", "node-template --dev"])
            .output()
            .expect("fail to execute process");
    });

    let _output2  = Command::new("cmd")
        .current_dir("E:/substrate/substrate-front-end-template")
        .args(&["/C", "yarn start"])
        .output()
        .expect("fail to execute process");

}