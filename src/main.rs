use std::*;
use structopt::StructOpt;
mod replacer;
use tempfile::NamedTempFile;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(required(true))]
    pub replace: String,

    #[structopt(conflicts_with("delete"))]
    pub with: Option<String>,

    #[structopt(short, long)]
    pub inplace: bool,

    #[structopt(short, long)]
    pub delete: bool,

    #[structopt(short, long)]
    pub stringliteral: bool,

    #[structopt(short, long, parse(from_os_str))]
    pub files: Vec<path::PathBuf>,
}

fn main() {
    let args = Opt::from_args();
    for file in args.files.iter() {
        process_file(&args, &file)
    }
}

fn process_file(args: &Opt, file: &path::PathBuf) {
    if !file.is_file() {
        return;
    }
    let tmp = NamedTempFile::new_in("").unwrap();

    if args.inplace {
        match args.delete {
            true => replacer::delete_matches(args, file, &tmp),
            false => replacer::replace_matches(args, file, &tmp),
        }
    } else {
        match args.delete {
            true => replacer::delete_matches(args, file, io::stdout()),
            false => replacer::replace_matches(args, file, io::stdout()),
        }
    }

    fs::rename(&tmp.path(), &file).unwrap();
}
