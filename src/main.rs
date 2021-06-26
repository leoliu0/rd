use std::*;
use structopt::StructOpt;
mod replacer;
use replacer::Changed;
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
    let replace_file = |changed| match changed {
        Changed::Yes => {
            fs::rename(&tmp.path(), &file).unwrap();
        }
        _ => (),
    };

    if args.inplace {
        match args.delete {
            true => {
                let changed = replacer::delete_matches(args, file, &tmp);
                replace_file(changed);
            }
            false => {
                let changed = replacer::replace_matches(args, file, &tmp);
                replace_file(changed);
            }
        }
    } else {
        match args.delete {
            true => {
                let _ = replacer::delete_matches(args, file, io::stdout());
            }
            false => {
                let _ = replacer::replace_matches(args, file, io::stdout());
            }
        }
    }
}
