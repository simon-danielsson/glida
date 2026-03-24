use std::{env, io, path::PathBuf};

#[derive(PartialEq, Clone, Debug)]
pub struct Arguments {
    pub help: bool,
    pub target_dir: PathBuf,
}
impl Arguments {
    fn new() -> Self {
        Self {
            help: false,
            target_dir: PathBuf::new(),
        }
    }
}

pub fn parse() -> io::Result<Arguments> {
    let mut a = Arguments::new();
    let mut it = std::env::args().skip(1); // skip prog name

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "help" => {
                a.help = true;
            }

            other => {
                a.target_dir = PathBuf::from(other.trim());
                break;
            }
        }
    }
    if !a.target_dir.exists() {
        a.target_dir = env::current_dir()?;
    }
    Ok(a)
}
