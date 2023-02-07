use std::process::exit;
use std::fs;
use std::io::Write;
use std::env;
use regex::Regex;
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
    // let mut extras: Vec<String> = vec![argv.len().to_string()];
    if argv.len() <= 1 {
        usage();
    }

    let path = match fs::canonicalize(&argv[1]) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => {usage(); exit(1)},
    };
    println!("path: {}", path);

    let extras = Vec::from_iter(argv[1..].iter().cloned());
    for ext in extras {
        println!("extras: {}", ext);
    }
    preview();
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
        args: Vec<&'a str>,
    }
);

fn preview() {

    let rules: &[Rule] = &[
    Rule(("fpath v\\S[1,3]::", vec!["vfile.sh", "%riscou-filename%"])),
    Rule(("fpath g\\S[1,3]::", vec!["gdir.sh", "%riscou-filename%"])),
    Rule(("fpath .+\\.fen$", vec!["fen.sh", "%riscou-filename%"])),
    Rule(("text/.*", vec!["bat", "-p", "--pager=never", "--color=always", "%riscou-filename%"])),
    Rule(("inode/directory", vec!["ls", "-1A", "--color", "%riscou-filename%"])),
    Rule(("ms(word|-excel|-power)", vec!["printf", "💩💩💩💩💩💩💩💩💩\n%s\n💩💩💩💩💩💩💩💩💩", "%riscou-filename%"])),
    Rule(("opendoc.+spreadsheet", vec!["ods.sh", "%riscou-filename%"])),
    Rule(("officed.+spreadsheet", vec!["xlsx.sh", "%riscou-filename%"])),
    Rule(("office.+word",         vec!["docx.sh", "%riscou-filename%", "%riscou-extra0%"])),
    Rule(("office.+pres", vec!["ppt.sh", "%riscou-filename%", "%riscou-extra0%"])),
    Rule(("opendocument", vec!["odt2txt", "%riscou-filename%"])),
    Rule(("application/pdf", vec!["pdf.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
    Rule(("application/csv", vec!["column", "-t", "-s", ",", "%riscou-filename%"])),
    Rule(("application/json", vec!["head", "-n", "40", "%riscou-filename%"])),
    Rule(("application/.*execu.+", vec!["execu.sh", "%riscou-filename%", "%riscou-extra0%"])),
    Rule(("application/x-objec.+", vec!["execu.sh", "%riscou-filename%", "%riscou-extra0%"])),
    Rule(("application/zip", vec!["unzip", "-l", "%riscou-filename%"])),
    Rule(("application/gzip", vec!["tar", "tf", "%riscou-filename%"])),
    Rule(("application/x-subrip", vec!["/usr/bin/cat", "%riscou-filename%"])),
    Rule(("fpath .+\\.ff$", vec!["stiv", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
    Rule(("fpath .+\\.[1-9]$", vec!["man", "%riscou-filename%"])),
    Rule(("fpath .+\\.blend$", vec!["blender.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
    Rule(("image/.*dwg", vec!["stat", "%riscou-filename%"])),
    Rule(("image/.*xml", vec!["head", "-n", "40", "%riscou-filename%"])),
    Rule(("image/.*", vec!["stiv", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
    Rule(("audio/.*", vec!["vid.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
    Rule(("video/.*", vec!["vid.sh", "%riscou-filename%", "%riscou-extra0%", "%riscou-extra1%", "%riscou-extra2%", "%riscou-extra3%"])),
    Rule((".+", vec!["file", "--mime-type", "%riscou-filename%"])),
    ];

    for rule in rules {
        let mime = rule.mime();
        if mime.len() >= 4 && &mime[0..5] == "fpath" { 
            println!("FPATH: Rule: {:?}", rule); 
        } else {
            println!("NOT: Rule: {:?}", rule);
        }
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
