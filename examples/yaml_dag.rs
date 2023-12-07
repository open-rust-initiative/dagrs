//! Read the task information configured in the yaml file.

extern crate dagrs;

use dagrs::Dag;
use dagrs::utils::file::load_file;
use std::collections::HashMap;

fn main() {
    env_logger::init();
    let mut job = Dag::with_yaml("tests/config/correct.yaml", HashMap::new()).unwrap();
    assert!(job.start().unwrap());

    let content = load_file("tests/config/correct.yaml").unwrap();
    let mut job = Dag::with_yaml_str(&content, HashMap::new()).unwrap();
    assert!(job.start().unwrap());

}
