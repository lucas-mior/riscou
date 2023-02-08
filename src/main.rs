extern crate tree_magic;

use std::process::exit;
use std::fs;
use std::env;
use std::path::Path;
use regex::Regex;
use named_tuple::named_tuple;
use std::process::Command;
use std::os::unix::process::CommandExt;

fn usage() {
    println!(
"usage: riscou <filename> [extras...]
ARGUMENTS:
filename: file to preview
extras: extra arguments passed to command");
    exit(1);
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() <= 1 {
        usage();
    }

    let path = match fs::canonicalize(&argv[1]) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => {usage(); exit(1)},
    };

    let filename = path;
    let extras = Vec::from_iter(argv[2..].iter().cloned());
    for rule in RULES {
        let mime = rule.mime();
        let comp_conf;
        let comp_file: String;
        // let mut comp_file;
        if mime.len() >= 4 && &mime[0..5] == "fpath" { 
            comp_conf = &mime[5..];
            comp_file = filename.to_string();
        } else {
            comp_conf = &mime;
            let path = Path::new(&filename);
            comp_file = tree_magic::from_filepath(path);
        }
        let r_conf = Regex::new(comp_conf).unwrap();
        if r_conf.is_match(&comp_file) {
            let r_extras = Regex::new(r"%riscou-extra([0-9])%").unwrap();
            let args: &[&str] = rule.args().clone();
            let mut cargs: Vec<&str> = vec![];
            for arg in args.iter() {
                if arg == &("%riscou-filename%") {
                    cargs.push(&filename);
                } else if r_extras.is_match(arg) {
                    let caps = r_extras.captures(arg).unwrap();
                    let x = caps.get(1).map_or("", |m| m.as_str());
                    let i = x.parse::<usize>().unwrap();
                    if i < extras.len() {
                        cargs.push(&extras[i]);
                    }
                } else {
                    cargs.push(arg);
                }
            }
            Command::new(cargs[0]).args(cargs[1..].iter()).exec();
            break;
        } else {
            continue;
        }
    }
}

named_tuple!(
    #[derive(Clone, Debug)]
    struct Rule<'a> {
        mime: &'a str,
        args: &'static [&'static str],

    }
);

static RULES: &[Rule<'_>] = &[
Rule((".+",                     &["file", "--mime-type", "%riscou-filename%"])),
];
