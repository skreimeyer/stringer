use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use syn::spanned::Spanned;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // executable name

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: dump-syntax path/to/filename.rs");
            process::exit(1);
        }
    };

    let mut file = File::open(&filename).expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");
    println!("There area {} items", syntax.items.len());
    for i in syntax.items.iter() {
        match i {
            syn::Item::Fn(func) => {
                println!("Found a function with identity: {:?}", func.sig.ident);
                let sp = i.span();
                let st = sp.start();
                let en = sp.end();
                let st_loc = (st.line, st.column);
                let en_loc = (en.line, en.column);

                println!("Span is: line:{} col:{} to line:{} col:{}", st_loc.0, st_loc.1, en_loc.0, en_loc.1)
            },
            _ => continue
        }
    }
}