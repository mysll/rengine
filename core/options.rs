pub struct Options {
    pub port: i32,
}

pub fn load_option(opts: &[impl Fn(&mut Options)]) -> Options {
    let mut options = Options { port: 0 };
    for opt in opts {
        opt(&mut options)
    }
    options
}

pub fn with_port(port: i32) -> impl Fn(&mut Options) {
    move |options: &mut Options| options.port = port
}
