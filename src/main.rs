use std::io;

mod subc;
mod utils;

// *brakoll - d: init setup of args parsing and help subcommand, p: 100, t: feature, s: closed
fn main() -> io::Result<()> {
    let args = utils::arg::parse()?;

    if args.help {
        subc::help::run();
        return Ok(());
    }

    println!("{:?}", args.target_dir);
    Ok(())
}