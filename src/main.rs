mod builder;

use builder::{Global, Job, PipelineBuilder};

fn main() {
    let mut builder = PipelineBuilder::<Global>::new();

    let result = builder
        .trigger("main")
        .pool("rust_meetup", |p| p.image("ubuntu:latest"))
        .stage("Create Artifact".into(), |s| {
            s.add_job(Job::new("Compile"));
            s.add_job(Job::new("Test"));
            s.add_job(Job::new("Apply DB Migrations"));
        })
        .compile();

    match result {
        Some(pipeline) => println!("{}", pipeline),
        None => println!("Failed to build pipeline"),
    }
}
