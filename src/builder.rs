use std::marker::PhantomData;


pub(crate) struct Global;
pub(crate) struct Job;
pub(crate) struct Stage;

pub(crate) trait BuilderState {}

impl BuilderState for Global {}
impl BuilderState for Job {}
impl BuilderState for Stage {}

#[derive(Debug, Clone)]
pub(crate) struct YamlBuilder<B: BuilderState> {
    // Zero Sized Marker that exist only at compile time
    _marker: PhantomData<B>,
    yaml : String,
}

impl YamlBuilder<Global> {
    pub(crate) fn stage(&mut self, name: &str) -> &mut Self {
        self.write("stages:")
            .new_line()
            .indent(1)
            .write("- ")
            .write(name)
            .new_line();
        self
    }
}

impl YamlBuilder<Job> {
    pub(crate) fn script(&mut self, task: &str) -> &mut Self {
        self.indent(1)
            .write("script: ")
            .write(task)
            .new_line();
        self
    }

    pub(crate) fn echo(&mut self, msg: &str) -> &mut Self {
        self.script(&format!("echo \"{}\"", msg))
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

    pub(crate) fn job(&mut self, name: &str) -> YamlBuilder<Job> {
        self.write("job:")
            .new_line()
            .indent(1)
            .write("name: ")
            .write(name)
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

    #[allow(dead_code)]
    pub(crate) fn debug(&self) -> &Self {
        println!("{}", self.yaml);

        self
    }
}

