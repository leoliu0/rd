use super::Opt;
use anyhow::Context;
use regex::Regex;
use std::io::prelude::*;
use std::*;

pub fn reader(path: &path::PathBuf) -> io::BufReader<fs::File> {
    let f = fs::File::open(path)
        .with_context(|| format!("Could not read file {:?}", path))
        .unwrap();
    let f = io::BufReader::new(f);
    return f;
}

pub fn replace_matches(args: &Opt, file: &path::PathBuf, mut writer: impl io::Write) {
    let f = reader(file);
    let with = args.with.as_ref().unwrap();
    if args.stringliteral {
        for line in f.lines() {
            writeln!(writer, "{}", &line.unwrap().replace(&args.replace, with))
                .with_context(|| format!("cannot write line"))
                .unwrap();
        }
    } else {
        let replace = Regex::new(&args.replace).unwrap();
        for line in f.lines() {
            writeln!(writer, "{}", replace.replace_all(&line.unwrap(), with))
                .with_context(|| format!("cannot write line"))
                .unwrap();
        }
    }
}

pub fn delete_matches(args: &Opt, file: &path::PathBuf, mut writer: impl io::Write) {
    let f = reader(&file);
    if args.stringliteral {
        for line in f.lines() {
            let l = line.with_context(|| format!("cannot read line")).unwrap();
            if !l.contains(&args.replace) {
                writeln!(writer, "{}", l)
                    .with_context(|| format!("cannot write lines"))
                    .unwrap();
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
            }
        }
    }
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
