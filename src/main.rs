use std::{collections::HashMap, fmt, fs, io, path::PathBuf, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;

use crate::utils::arg::Arguments;

mod subc;
mod utils;

// *brakoll - d: init setup of args parsing and help subcommand, p: 100, t: feature, s: closed

fn count_files(dir: &PathBuf) -> usize {
    // *brakoll - d: add extra spinner to account for scanning of file amount, p: 100, t: feature, s: closed
    let setup_spinner = ProgressBar::new_spinner().with_message("Initializing program...");
    setup_spinner.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["", "", "", "", "", ""]),
    );

    setup_spinner.enable_steady_tick(Duration::from_millis(80));

    let mut counter = 0;
    let count = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .inspect(|_| {
            counter += 1;
            setup_spinner.set_message(format!("{} files found...", counter))
        })
        .count();
    setup_spinner.finish_and_clear();
    count
}

fn main() -> io::Result<()> {
    let args = utils::arg::parse()?;

    if args.help {
        subc::help::run();
        return Ok(());
    }

    // *brakoll - d: make spinner and progress bar prettier, p: 10, t: fix, s: closed
    let mut g = Glida::new(args.clone(), count_files(&args.target_dir));

    g.pb.set_style(
        ProgressStyle::with_template(" {bar:40.orange/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("█░"),
    );

    g.pb.set_message("Scanning directory...");

    g.scan_dir()?;

    g.pb.finish_and_clear();

    let rvec = g.get_results();

    g.print_results(rvec);

    Ok(())
}

struct File {
    blank_lines: u32,
    comment_lines: u32,
    code_lines: u32,
    lang_type: LangType,
}

#[derive(Debug)]
struct ResultPrint {
    name: String,
    files_amt: u32,
    blank_lines: u32,
    comment_lines: u32,
    code_lines: u32,
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
            Self::Html => "HTML",
            Self::Rust => "Rust",
            Self::D => "D",
            Self::Javascript => "Javascript",
            Self::Markdown => "Markdown",
            Self::Text => "Text",
            Self::Toml => "Toml",
            Self::Json => "Json",
            Self::Css => "CSS",
            Self::Svg => "SVG",
            Self::Shell => "Shell",
            Self::Python => "Python",
            Self::Unknown => "",
        };
        write!(f, "{s}")
    }
}

struct Glida {
    files: Vec<File>,
    args: Arguments,
    files_ignored: u32,
    pb: ProgressBar,
}

// *brakoll - d: implement loading bar or some sort of scanning indication through indicatif, p: 80, t: feature, s: closed
impl Glida {
    fn new(args: Arguments, file_amt: usize) -> Self {
        Self {
            files: Vec::new(),
            args,
            files_ignored: 0,
            pb: ProgressBar::new(file_amt as u64),
        }
    }
    // *brakoll - d: the amount of files of each lang would also be nice to see printed in the result (would require changes in get_results function), p: 50, t: feature, s: closed

    // *brakoll - d: sort results from top to bottom by amount of code lines, p: 80, t: feature, s: open
    // *brakoll - d: add colored output? that can be toggled off with flag (true would be def), p: 20, t: feature, s: open

    /// helper: print_results
    fn print_div(&self) {
        let div = "-".repeat(50);
        println!("{}", div);
    }

    fn print_results(&mut self, rvec: Vec<ResultPrint>) {
        println!("\nFiles ignored: {}", self.files_ignored);
        self.print_div();
        println!(
            "{:<10} {:<10} {:<10} {:<10} {:<10}",
            "Language", "Files", "Code", "Comment", "Blank"
        );
        self.print_div();

        for mut r in rvec {
            if r.name == "total" {
                // capitalize first letter of total
                if let Some(first) = r.name.chars().next() {
                    r.name = first.to_uppercase().to_string()
                        + &r.name
                            .chars()
                            .skip(1)
                            .collect::<String>()
                            .to_lowercase();
                }
                self.print_div();
            }
            println!(
                "{:<10} {:<10} {:<10} {:<10} {:<10}",
                r.name, r.files_amt, r.code_lines, r.comment_lines, r.blank_lines
            );
        }
    }

    // *brakoll - d: first rough version, p: 100, t: feature, s: closed
    // *brakoll - d: prettier results print, p: 100, t: feature, s: closed
    fn get_results(&mut self) -> Vec<ResultPrint> {
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

        let mut rvec: Vec<ResultPrint> = Vec::new();

        // each lang
        let mut total_code = 0;
        let mut total_comm = 0;
        let mut total_blnk = 0;
        let mut total_files = 0;
        for (l, f) in lang_groups {
            let mut code = 0;
            let mut comments = 0;
            let mut blanks = 0;
            let mut files = 0;
            for c in f {
                code += c.code_lines;
                total_code += c.code_lines;
                comments += c.comment_lines;
                total_comm += c.comment_lines;
                blanks += c.blank_lines;
                total_blnk += c.blank_lines;
                files += 1;
                total_files += 1;
            }
            rvec.push(ResultPrint {
                name: l.to_string(),
                files_amt: files,
                blank_lines: blanks,
                comment_lines: comments,
                code_lines: code,
            });
        }
        rvec.push(ResultPrint {
            name: "total".to_string(),
            files_amt: total_files,
            blank_lines: total_blnk,
            comment_lines: total_comm,
            code_lines: total_code,
        });
        rvec
    }

    // *brakoll - d: add files ignored print at result, p: 15, t: feature, s: closed
    fn scan_dir(&mut self) -> io::Result<()> {
        for entry in WalkDir::new(&self.args.target_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            // *brakoll - d: skip git folders, p: , t: feature, s: closed
            if entry.path().to_str().unwrap().contains("git") {
                self.files_ignored += 1;
                self.pb.inc(1);
                continue;
            }

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
                self.files_ignored += 1;
                self.pb.inc(1);
                continue;
            }

            // *brakoll - d: add dir check in scan function to prevent undefined behaviour, p: 100, t: fix, s: closed
            if entry.clone().into_path().is_dir() {
                self.files_ignored += 1;
                self.pb.inc(1);
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
                blank_lines,
                comment_lines,
                code_lines,
                lang_type: l_type,
            });
            self.pb.inc(1);
        }

        Ok(())
    }
}
