use std::io::{self, Write, BufWriter, Stdout};
use std::path::Path;

use clap::Parser;
use rust_ls::{self, DirectoryContent, list};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long)]
    all: bool,
}

fn print_output(entry: &DirectoryContent, buf_writer: &mut BufWriter<Stdout>) {
    let _ = writeln!(buf_writer, "{}", entry);
}

fn get_filter(show_all: bool) -> Box<dyn list::Filter> {
    if show_all {
        Box::new(list::IncludeDotFiles{})
    } else {
        Box::new(list::ExcludeDotFiles{})
    }
}

fn main() {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let args = Args::parse();
    let path = Path::new(&args.path);

    let filter = get_filter(args.all);

    if !&args.recursive {
        let entry = rust_ls::visit_dir(path, &filter);
        print_output(&entry, &mut handle);
    } else {
        let entries = rust_ls::visit_recursive(path, &filter);

        entries.into_iter().for_each(|f| {
            print_output(&f, &mut handle)
        })
    }

}
