use std::{collections::HashMap, fmt, fs, io};

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

    // println!("{:?}", args.target_dir);
    Ok(())
}

struct File {
    name: String,
    blank_lines: u32,
    comment_lines: u32,
    code_lines: u32,
    lang_type: LangType,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
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
    // *brakoll - d: prettier results print, p: 100, t: feature, s: closed
    fn print_results(&mut self) {
        let mut lang_groups: HashMap<LangType, Vec<&File>> = HashMap::new();

        macro_rules! push_l {
            ($map:expr, $key:expr, $value:expr) => {
                $map.entry($key).or_insert_with(Vec::new).push($value);
            };
        }

        for f in &self.files {
            match f.lang_type {
                LangType::Html => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Rust => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::D => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Javascript => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Markdown => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Text => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Toml => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Json => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Css => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Svg => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Shell => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Python => {
                    push_l!(lang_groups, f.lang_type, f);
                }
                LangType::Unknown => {
                    push_l!(lang_groups, f.lang_type, f);
                }
            };
        }

        // each lang
        let mut total_code = 0;
        let mut total_comm = 0;
        let mut total_blnk = 0;
        for (l, f) in lang_groups {
            let mut code = 0;
            let mut comments = 0;
            let mut blanks = 0;
            for c in f {
                code += c.code_lines;
                total_code += c.code_lines;
                comments += c.comment_lines;
                total_comm += c.comment_lines;
                blanks += c.blank_lines;
                total_blnk += c.blank_lines;
            }
            println!("lang: {}", l);
            println!("code: {}", code);
            println!("comm: {}", comments);
            println!("blnk: {}", blanks);
            println!("----------");
        }
        println!("== TOTAL ==");
        println!("code: {}", total_code);
        println!("comm: {}", total_comm);
        println!("blnk: {}", total_blnk);
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
