use std::{fmt, fs, io};

use walkdir::WalkDir;

use crate::utils::arg::Arguments;

mod subc;
mod utils;

// *brakoll - d: init setup of args parsing and help subcommand, p: 100, t: feature, s: closed
fn main() -> io::Result<()> {
    let args = utils::arg::parse()?;

    if args.help {
        subc::help::run();
        return Ok(());
    }

    let mut g = Glida::new(args.clone());

    g.scan_dir()?;

    g.print_results();

    println!("{:?}", args.target_dir);
    Ok(())
}

struct File {
    name: String,
    blank_lines: u32,
    comment_lines: u32,
    code_lines: u32,
    lang_type: LangType,
}

#[derive(Debug, PartialEq)]
enum LangType {
    Html,
    Rust,
    D,
    Javascript,
    Markdown,
    Text,
    Toml,
    Json,
    Css,
    Svg,
    Shell,
    Python,
    Unknown,
}

impl fmt::Display for LangType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Html => "html",
            Self::Rust => "rs",
            Self::D => "d",
            Self::Javascript => "js",
            Self::Markdown => "md",
            Self::Text => "txt",
            Self::Toml => "toml",
            Self::Json => "json",
            Self::Css => "css",
            Self::Svg => "svg",
            Self::Shell => "sh",
            Self::Python => "py",
            Self::Unknown => "",
        };
        write!(f, "{s}")
    }
}

struct Glida {
    files: Vec<File>,
    args: Arguments,
}

impl Glida {
    fn new(args: Arguments) -> Self {
        Self {
            files: Vec::new(),
            args,
        }
    }

    // *brakoll - d: first rough version, p: 100, t: feature, s: closed
    fn print_results(&mut self) {
        for f in &self.files {
            println!("Name: {}", f.name);
            println!("Lang: {}", f.lang_type);
            println!("Blnk: {}", f.blank_lines);
            println!("Comm: {}", f.comment_lines);
            println!("Code: {}", f.code_lines);
            println!("-------------")
            // println!("Path: {}", f.fpath.display());
        }
    }

    fn scan_dir(&mut self) -> io::Result<()> {
        for entry in WalkDir::new(&self.args.target_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            // get name
            let f_name = entry.file_name().to_str().unwrap();

            // get lang_type based on ext
            let l_type: LangType = match f_name.split('.').last().unwrap() {
                "html" => LangType::Html,
                "rs" => LangType::Rust,
                "d" => LangType::D,
                "js" => LangType::Javascript,
                "md" => LangType::Markdown,
                "txt" => LangType::Text,
                "toml" => LangType::Toml,
                "json" => LangType::Json,
                "css" => LangType::Css,
                "svg" => LangType::Svg,
                "sh" => LangType::Shell,
                "py" => LangType::Python,
                _ => LangType::Unknown,
            };

            // skip unknown files
            if l_type == LangType::Unknown {
                continue;
            }

            let fpath = entry.clone().into_path();

            // scan lines
            let contents = fs::read_to_string(&fpath)?;

            let mut comment_lines: u32 = 0;
            let mut blank_lines: u32 = 0;
            let mut code_lines: u32 = 0;

            for line in contents.lines() {
                if line.starts_with("//")
                || line.starts_with("#") || line.starts_with("<!--")
                || line.starts_with("--!>")
                {
                    comment_lines += 1;
                } else if line.trim().is_empty() {
                    blank_lines += 1;
                } else {
                    code_lines += 1;
                }
            }

            self.files.push(File {
                name: f_name.to_string(),
                blank_lines,
                comment_lines,
                code_lines,
                lang_type: l_type,
            });
        }

        Ok(())
    }
}