use super::Opt;
use anyhow::Context;
use regex::Regex;
use std::io::prelude::*;
use std::*;

#[derive(Debug)]
pub enum Changed {
    Yes,
    No,
}

pub fn reader(path: &path::PathBuf) -> io::BufReader<fs::File> {
    let f = fs::File::open(path)
        .with_context(|| format!("Could not read file {:?}", path))
        .unwrap();
    let f = io::BufReader::new(f);
    return f;
}

pub fn replace_matches(args: &Opt, file: &path::PathBuf, mut writer: impl io::Write) -> Changed {
    let f = reader(file);
    let with = args.with.as_ref().unwrap();
    let get_changed = |line: &str, replace_to: &str| {
        if line == replace_to {
            return Changed::No;
        } else {
            return Changed::Yes;
        }
    };
    let mut changed = Changed::No;
    if args.stringliteral {
        for line in f.lines() {
            let line = &line.unwrap();
            let replace_to = line.replace(&args.replace, with);
            changed = match changed {
                Changed::No => get_changed(line, &replace_to),
                _ => Changed::Yes,
            };
            writeln!(writer, "{}", replace_to)
                .with_context(|| format!("cannot write line"))
                .unwrap();
        }
    } else {
        let replace = Regex::new(&args.replace).unwrap();
        for line in f.lines() {
            let line = &line.unwrap();
            let replace_to = replace.replace_all(line, with);
            changed = match changed {
                Changed::No => get_changed(line, &replace_to),
                _ => Changed::Yes,
            };
            writeln!(writer, "{}", replace_to)
                .with_context(|| format!("cannot write line"))
                .unwrap();
        }
    }
    println!("{:?}", changed);
    changed
}

pub fn delete_matches(args: &Opt, file: &path::PathBuf, mut writer: impl io::Write) -> Changed {
    let f = reader(&file);
    let mut changed = Changed::No;
    if args.stringliteral {
        for line in f.lines() {
            let l = line.with_context(|| format!("cannot read line")).unwrap();
            if !l.contains(&args.replace) {
                writeln!(writer, "{}", l)
                    .with_context(|| format!("cannot write lines"))
                    .unwrap();
            } else {
                changed = Changed::Yes;
            }
        }
    } else {
        let todelete = Regex::new(&args.replace).unwrap();
        for line in f.lines() {
            let l = line.with_context(|| format!("cannot read line")).unwrap();
            if !todelete.is_match(&l) {
                writeln!(writer, "{}", l)
                    .with_context(|| format!("cannot write lines"))
                    .unwrap();
            } else {
                changed = Changed::Yes;
            }
        }
    }
    changed
}

// #[test]
// fn find_a_re_match() {
// let mut results = Vec::new();
// let path = path::PathBuf::from("tester.txt");
// replace_matches("[0-9]+", false, "", &path, &mut results);
// assert_eq!(results, b"asd\nsdafsadf\n")
// }

// #[test]
// fn find_a_literal_match() {
// let mut results = Vec::new();
// let path = path::PathBuf::from("tester.txt");
// replace_matches("asd", true, "", &path, &mut results);
// assert_eq!(results, b"123123\nsdafsadf\n")
// }

// #[test]
// fn delete_a_re_match() {
// let mut results = Vec::new();
// let path = path::PathBuf::from("tester.txt");
// delete_matches("[0-9]+", false, &path, &mut results);
// assert_eq!(results, b"sdafsadf\n")
// }

// #[test]
// fn delete_a_literal_match() {
// let mut results = Vec::new();
// let path = path::PathBuf::from("tester.txt");
// delete_matches("sad", true, &path, &mut results);
// assert_eq!(results, b"123123asd\n")
// }
