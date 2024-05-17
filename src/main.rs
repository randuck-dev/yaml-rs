mod builder;

use builder::{Global, YamlBuilder};

fn main() {

    let mut builder = YamlBuilder::<Global>::new();

    let res = builder
        .stage("Build")
        .job("Hello Job")
            .echo("Job 1")
        .job("Another Hello")
            .echo("Job 2")
        .write_to_file("test.yml");

    match res {
        Ok(_) => println!("File written successfully"),
        Err(e) => println!("Error: {}", e)
    }
}

