extern crate tree_magic;
use bat::PrettyPrinter;

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
        let mut string: String;
        if r_conf.is_match(&comp_file) {
            let r_extras = Regex::new(r"%riscou-extra([0-9])%").unwrap();
            let r_filename = Regex::new(r".*(%riscou-filename%).*").unwrap();
            let args: &[&str] = rule.args().clone();
            let mut cargs: Vec<&str> = vec![];
            for arg in args.iter() {
                if r_filename.is_match(arg) {
                    string = arg.replace("%riscou-filename%", &filename);
                    cargs.push(Box::leak(Box::new(string)));
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
            let text = Regex::new("text/*").unwrap();
            if text.is_match(&comp_file) {
                PrettyPrinter::new()
                    .input_file(&filename)
                    .print()
                    .unwrap();
                break;
            }
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
Rule(("fpath v\\S[1,3]::",     &["vfile.sh", "%riscou-filename%"])),
Rule(("fpath g\\S[1,3]::",     &["gdir.sh", "%riscou-filename%"])),
Rule(("fpath .+\\.fen$",       &["fen.sh", "%riscou-filename%"])),
Rule(("text/.*",               &["bat", "-p", "--pager=never", "--color=always", "%riscou-filename%"])),
Rule(("inode/directory",       &["ls", "-1A", "--color", "%riscou-filename%"])),
Rule(("ms(word|-exce|-powe)",  &["printf", "💩💩💩💩💩💩💩💩💩\n%s\n💩💩💩💩💩💩💩💩💩", "%riscou-filename%"])),
Rule(("opendoc.+spreadsheet",  &["ods.sh", "%riscou-filename%"])),
Rule(("officed.+spreadsheet",  &["xlsx.sh", "%riscou-filename%"])),
Rule(("office.+word",          &["docx.sh", "%riscou-filename%", "%riscou-extra0%"])),
Rule(("office.+pres",          &["ppt.sh", "%riscou-filename%", "%riscou-extra0%"])),
Rule(("opendocument",          &["odt2txt", "%riscou-filename%"])),
Rule(("application/pdf",       &["pdf.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule(("application/csv",       &["column", "-t", "-s", ",", "%riscou-filename%"])),
Rule(("application/json",      &["head", "-n", "40", "%riscou-filename%"])),
Rule(("application/.*execu.+", &["objdump", "-T", "%riscou-filename%", "%riscou-extra0%"])),
Rule(("application/x-objec.+", &["objdump", "-T", "%riscou-filename%", "%riscou-extra0%"])),
Rule(("application/zip",       &["unzip", "-l", "%riscou-filename%"])),
Rule(("application/gzip",      &["tar", "tf", "%riscou-filename%"])),
Rule(("application/x-subrip",  &["/usr/bin/cat", "%riscou-filename%"])),
Rule(("fpath .+\\.ff$",        &["stiv", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule(("fpath .+\\.[1-9]$",     &["man", "%riscou-filename%"])),
Rule(("image/.*dwg",           &["stat", "%riscou-filename%"])),
Rule(("image/.*xml",           &["head", "-n", "40", "%riscou-filename%"])),
Rule(("image/.*",              &["stiv", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule(("application/x-riff",    &["stiv", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule(("audio/.*",              &["vid.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule(("video/.*",              &["vid.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule((".+",                    &["file", "--mime-type", "%riscou-filename%"])),
];
