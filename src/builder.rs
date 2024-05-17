use std::marker::PhantomData;


pub struct Global;
pub struct Job;
pub struct Stage;

trait BuilderState {}

impl BuilderState for Global {}
impl BuilderState for Job {}
impl BuilderState for Stage {}

#[derive(Debug, Clone)]
pub struct YamlBuilder<B: BuilderState> {
    // Zero Sized Marker that exist only at compile time
    _marker: PhantomData<B>,
    yaml : String,
}

impl YamlBuilder<Job> {

    pub(crate) fn step(&mut self, task: &str) -> &mut Self {
        self.indent(1)
            .write("step: ")
            .write(task)
            .new_line();
        self
    }

    pub(crate) fn echo(&mut self, msg: &str) -> &mut Self {
        self.step(&format!("echo \"{}\"", msg))
    }

    pub(crate) fn done(&mut self) -> YamlBuilder<Global> {
        YamlBuilder {
            _marker: PhantomData,
            yaml: self.yaml.clone()
        }
    }
}

impl<B: BuilderState> YamlBuilder<B> {

    const INDENT_STR: &str= "  ";

    pub(crate) fn new() -> YamlBuilder<Global> {
        YamlBuilder::<Global>{
            _marker: PhantomData,
            yaml: String::from("")
        }
    }

    pub(crate) fn write_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, &self.yaml)?;

        Ok(())
    }

    pub(crate) fn job(&mut self) -> YamlBuilder<Job> {
        self.write("job:")
            .new_line();
        YamlBuilder {
            _marker: PhantomData,
            yaml: self.yaml.clone()
        }
    }

    fn new_line(&mut self) -> &mut Self {
        self.yaml.push_str("\n");

        self
    }

    fn write(&mut self, s: &str) -> &mut Self {
        self.yaml.push_str(s);

        self
    }

    fn indent(&mut self, n: i32) -> &mut Self {
        for _ in 0..n {
            self.yaml.push_str(Self::INDENT_STR);
        }

        self
    }

    pub(crate) fn debug(&self) -> &Self {
        println!("{}", self.yaml);

        self
    }
}

