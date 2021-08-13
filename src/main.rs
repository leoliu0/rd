use std::*;
use structopt::StructOpt;
mod replacer;
use log::info;
use replacer::Changed;
use simple_logger::SimpleLogger;
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

    #[structopt(short, long, conflicts_with("delete"), conflicts_with("with"),required(false))]
    pub add: i32,

    #[structopt(short, long)]
    pub stringliteral: bool,

    #[structopt(short, long, parse(from_os_str))]
    pub files: Vec<path::PathBuf>,
}

pub enum Job {
    Replace,
    Delete,
    Add,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let args = Opt::from_args();
    let job = match args.delete {
        true => Job::Delete,
            false => match args.add {
                i if i>=0 => Job::Add,
                _ => Job::Replace,
            }
    }; 
    for file in args.files.iter() {
        process_file(&args, &file, &job)
    }
}

fn process_file(args: &Opt, file: &path::PathBuf, job:&Job) {
    if !file.is_file() {
        return;
    }
    let tmp = NamedTempFile::new_in("").unwrap();
    let replace_file = |changed| match changed {
        Changed::Yes => {
            fs::rename(&tmp.path(), &file).unwrap();
            info!("{:?} has changed!", &file)
        }
        _ => (),
    };

    if args.inplace {
        match job {
            &Job::Delete => {
                let changed = replacer::delete_matches(args, file, &tmp);
                replace_file(changed);
            }
            &Job::Replace => {
                let changed = replacer::replace_matches(args, file, &tmp);
                replace_file(changed);
            }
            &Job::Add => {
                let changed = replacer::add(args, file, &tmp);
                replace_file(changed);
            }
        }
    } else {
        match job {
            &Job::Delete => {
                replacer::delete_matches(args, file, io::stdout());
            }
            &Job::Replace => {
                replacer::replace_matches(args, file, io::stdout());
            }
            &Job::Add => {
                replacer::add(args, file, io::stdout());
            }
    }
    }
}
