use std::marker::PhantomData;

pub(crate) struct Global;

#[derive(Debug, Clone)]
pub(crate) struct Pool {
    pool_name: String,
    image_name: Option<String>,
}

impl Pool {
    pub(crate) fn image(&mut self, name: &str) {
        self.image_name = Some(String::from(name))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Job {
    pub name: String,
}

impl Job {
    pub(crate) fn new(name: &str) -> Job {
        Job {
            name: String::from(name),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Stage {
    name: String,
    jobs: Vec<Job>,
}

impl Stage {
    pub(crate) fn add_job(&mut self, job: Job) {
        self.jobs.push(job);
    }
}

pub(crate) trait BuilderState {}
trait Compilable {
    fn compile(&self) -> Option<String>;
}

impl BuilderState for Global {}
impl BuilderState for Job {}
impl BuilderState for Stage {}

impl Compilable for Stage {
    fn compile(&self) -> Option<String> {
        let mut output = String::from("");
        output.push_str(&format!("- stage: {}\n", self.name));
        output.push_str(&format!("  jobs:\n"));
        self.jobs.iter().for_each(|job| {
            output.push_str(&format!("  - job: {}\n", job.name));
        });

        Some(output)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PipelineBuilder<B: BuilderState> {
    // Zero Sized Marker that exist only at compile time
    _marker: PhantomData<B>,
    trigger: Option<String>,
    pool: Option<Pool>,
    stages: Vec<Stage>,
}

impl PipelineBuilder<Global> {
    pub(crate) fn stage<F>(&mut self, name: String, build_stage: F) -> PipelineBuilder<Stage>
    where
        F: Fn(&mut Stage) -> (),
    {
        let mut stage = Stage {
            name,
            jobs: Vec::new(),
        };
        build_stage(&mut stage);

        self.stages.push(stage);

        let Self {
            _marker: _,
            trigger,
            pool,
            stages,
        } = self;
        PipelineBuilder {
            _marker: PhantomData,
            trigger: trigger.take(),
            pool: pool.take(),
            stages: stages.clone(),
        }
    }
}

impl PipelineBuilder<Stage> {
    pub(crate) fn add_job<F>(&mut self, get_job: F) -> &mut Self
    where
        F: Fn() -> Job,
    {
        let new_job = get_job();
        self
    }

    pub(crate) fn compile(self) -> Option<String> {
        let mut output = String::from("");

        if !self.trigger.is_none() {
            output.push_str(&format!("trigger: {}\n\n", self.trigger?));
        }

        if !self.pool.is_none() {
            let pool = self.pool?;
            output.push_str(&format!("pool:\n"));
            output.push_str(&format!("  name: {}\n", pool.pool_name));
            if !pool.image_name.is_none() {
                output.push_str(&format!("  image: {}\n", pool.image_name?));
            }

            output.push_str("\n");
        }

        output.push_str("stages:\n");
        self.stages.iter().for_each(|stage| {
            let res = stage.compile().unwrap();
            output.push_str(&res);
        });

        output.into()
    }
}

impl<B: BuilderState> PipelineBuilder<B> {
    pub(crate) fn new() -> PipelineBuilder<Global> {
        PipelineBuilder::<Global> {
            _marker: PhantomData,
            trigger: None,
            pool: None,
            stages: vec![],
        }
    }

    pub(crate) fn trigger(&mut self, trigger: &str) -> &mut Self {
        self.trigger = Some(String::from(trigger));
        self
    }

    pub(crate) fn pool<F>(&mut self, trigger: &str, build: F) -> &mut Self
    where
        F: Fn(&mut Pool) -> (),
    {
        let mut pool = Pool {
            pool_name: String::from(trigger),
            image_name: None,
        };

        build(&mut pool);

        self.pool = Some(pool);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn debug(&self) -> &Self {
        self
    }
}
