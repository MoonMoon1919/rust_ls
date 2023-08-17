use std::fs::DirEntry;
use std::io::{self, Write, BufWriter, Stdout};
use std::path::Path;

use clap::Parser;
use rust_ls;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, long)]
    recursive: bool,
}

fn print_output(entries: &Vec<DirEntry>, buf_writer: &mut BufWriter<Stdout>) {
    for ent in entries {
        let fname = ent.file_name().to_string_lossy().into_owned();
        let _ = writeln!(buf_writer, "{}", fname);
    }
}

fn main() {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let args = Args::parse();
    let path = Path::new(&args.path);

    if !&args.recursive {
        let entries = rust_ls::visit_dir(path);
        print_output(&entries, &mut handle);
    } else {
        let entries = rust_ls::visit_recursive(path);
        for ent in entries {
            println!("---");
            println!("{:?}", ent);
        }
    }

}
