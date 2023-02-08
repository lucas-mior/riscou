extern crate tree_magic;

use std::process::exit;
use std::fs;
use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
// use regex::Regex;
use named_tuple::named_tuple;

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
    // let mut extras: Vec<String> = [argv.len().to_string()];
    if argv.len() <= 1 {
        usage();
    }

    let path = match fs::canonicalize(&argv[1]) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => {usage(); exit(1)},
    };
    println!("path: {}", path);

    let extras = Vec::from_iter(argv[2..].iter().cloned());
    for ext in extras {
        println!("extras: {}", ext);
    }
    preview(&path);
    // return 0;
}

// static void parse_args(char *cargs[]) {

//     r_filename = Regex::new(r".*(%riscou-filename%).*").unwrap();
//     r_extras = Regex::new(r"%riscou-extra([0-9])%").unwrap();

//     for (size_t i = 0; i < 10; i++) {
//         if (cargs[i] == NULL)
//             break;

//         if (!regexec(&r_filename, cargs[i], 10, groups, 0)) {
//             cargs[i] = filename;
//         }
//         if (!regexec(&r_extras, cargs[i], 10, groups, 0)) {
//             int num;
//             char copy[strlen(cargs[i]) + 1];
//             strcpy(copy, cargs[i]);
//             copy[groups[1].rm_eo] = 0;
//             num = atoi(copy + groups[1].rm_so);
//             cargs[i] = extras[num];
//         }
//     }
//     execvp(cargs[0], cargs);
//     return;
// }

named_tuple!(
    #[derive(Clone, Debug)]
    struct Rule<'a> {
        mime: &'a str,
        args: &'static [&'static str],

    }
);

static RULES: &[Rule<'_>] = &[
Rule(("fpath v\\S[1,3]::",      &["vfile.sh", "%riscou-filename%"])),
Rule(("fpath g\\S[1,3]::",      &["gdir.sh", "%riscou-filename%"])),
Rule(("fpath .+\\.fen$",        &["fen.sh", "%riscou-filename%"])),
Rule(("text/.*",                &["bat", "-p", "--pager=never", "--color=always", "%riscou-filename%"])),
Rule(("inode/directory",        &["ls", "-1A", "--color", "%riscou-filename%"])),
Rule(("ms(word|-excel|-power)", &["printf", "💩💩💩💩💩💩💩💩💩\n%s\n💩💩💩💩💩💩💩💩💩", "%riscou-filename%"])),
Rule(("opendoc.+spreadsheet",   &["ods.sh", "%riscou-filename%"])),
Rule(("officed.+spreadsheet",   &["xlsx.sh", "%riscou-filename%"])),
Rule(("office.+word",           &["docx.sh", "%riscou-filename%", "%riscou-extra0%"])),
Rule(("office.+pres",           &["ppt.sh", "%riscou-filename%", "%riscou-extra0%"])),
Rule(("opendocument",           &["odt2txt", "%riscou-filename%"])),
Rule(("application/pdf",        &["pdf.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule(("application/csv",        &["column", "-t", "-s", ",", "%riscou-filename%"])),
Rule(("application/json",       &["head", "-n", "40", "%riscou-filename%"])),
Rule(("application/.*execu.+",  &["execu.sh", "%riscou-filename%", "%riscou-extra0%"])),
Rule(("application/x-objec.+",  &["execu.sh", "%riscou-filename%", "%riscou-extra0%"])),
Rule(("application/zip",        &["unzip", "-l", "%riscou-filename%"])),
Rule(("application/gzip",       &["tar", "tf", "%riscou-filename%"])),
Rule(("application/x-subrip",   &["/usr/bin/cat", "%riscou-filename%"])),
Rule(("fpath .+\\.ff$",         &["stiv", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule(("fpath .+\\.[1-9]$",      &["man", "%riscou-filename%"])),
Rule(("image/.*dwg",            &["stat", "%riscou-filename%"])),
Rule(("image/.*xml",            &["head", "-n", "40", "%riscou-filename%"])),
Rule(("image/.*",               &["stiv", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule(("audio/.*",               &["vid.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule(("video/.*",               &["vid.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
Rule((".+",                     &["file", "--mime-type", "%riscou-filename%"])),
];

fn preview(filename: & String) {
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
            let path = Path::new(filename);
            comp_file = tree_magic::from_filepath(path);
        }
        println!("comp_conf: {}", comp_conf);
        println!("comp_file: {}", comp_file);
    }
}
    //     mime_conf = rules[i].mime;
    //     if (!strncmp(mime_conf, "fpath", 5)) {
    //         comp_conf = mime_conf + 6;
    //         while (*comp_conf == ' ')
    //             comp_conf++;
    //         comp_file = filename;
    //     } else {
    //         magic_t m;
    //         comp_conf = mime_conf;
    //         m = magic_open(MAGIC_MIME_TYPE);
    //         magic_load(m, NULL);
    //         mime_file = (char *) magic_file(m, filename);
    //         comp_file = mime_file;
    //     }

    //     v = regcomp(&r, comp_conf, REG_EXTENDED);
    //     if (v != 0) {
    //         fprintf(stderr, "Error creating regex for mime_conf %s\n", comp_conf);
    //         continue;
    //     }

    //     if (regexec(&r, comp_file, 0, NULL, 0) == REG_NOMATCH) {
    //         continue;
    //     } else {
    //         parse_args(rules[i].args);
    //         break;
    //     }
    // }
