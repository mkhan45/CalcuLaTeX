#![feature(bindings_after_at)]

use notify::{self, Watcher};

mod parser;

mod expr;
use expr::val::Val;

mod statement;
use statement::State;

mod latex;

use std::io::Write;

fn main() {
    // Usage: {cmd} [input_file] [output_pdf]
    let args = std::env::args().collect::<Vec<String>>();
    let filename = &args[1];

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::PollWatcher::with_delay_ms(tx, 500).unwrap();

    watcher
        .watch(filename, notify::RecursiveMode::NonRecursive)
        .unwrap();

    let rebuild = || {
        println!("rebuilding pdf");
        let contents = std::fs::read_to_string(filename).unwrap();

        let mut state = State::new(&contents);

        state.exec();

        let mut md_file = tempfile::NamedTempFile::new().unwrap();
        write!(md_file, "{}", state.output).unwrap();

        let mut pandoc = pandoc::new();
        pandoc.set_input_format(pandoc::InputFormat::Latex, Vec::new());
        pandoc.add_input(&md_file.path());

        pandoc.set_output(pandoc::OutputKind::File(args[2].to_string().into()));
        pandoc.execute().unwrap();
        println!("done rebuilding pdf");
    };

    rebuild();

    loop {
        match rx.recv() {
            Ok(_) => rebuild(),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
