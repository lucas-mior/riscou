# riscou
File previewer. Riscou executes a program acording to the file's mimetype.

## Installation
```
$ cargo install --path .
```

## Usage
```
riscou %riscou-filename% [%riscou-extra1 ...]
```

Note that you can pass up to 9 arguments to the program after the filename.

## Configuration
Edit `main.rs` and recompile.
By default riscou will only print the mimetype of the file according to [tree_magic](https://docs.rs/tree_magic/latest/tree_magic/).
You have to configure a program and arguments for each regex defined in the `RULES` array.
If a line is not matched but the 1st word is exactly `fpath`,
then the 2nd argument is interpreted as a file path regex.

## Rationale
riscou is intended to be as fast as possible,
so you should probably choose previewing programs that have a short startup time.
