use watertower::*;

// TODO let something else handle the command parsing

macro_rules! abort {
    ($code:expr=> $f:expr, $($args:expr),* $(,)?) => {
        abort!($code=> format_args!($f, $($args),*))
    };
    ($code:expr=> $msg:expr) => {{
        eprintln!("{}", $msg);
        std::process::exit($code)
    }};
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match (
        args.get(1).map(String::as_str),
        args.get(2).map(String::as_str),
    ) {
        (Some("dump"), file) => {
            let mut file = match file {
                Some(file) => std::fs::File::open(file).unwrap_or_else(|err| {
                    abort!(1=> "invalid file: {}",err);
                }),
                None => abort!(1=> "try: dump <file.class>"),
            };

            let class = ClassFile::read(&mut file)
                .unwrap_or_else(|err| abort!(2=> "cannot read class: {}",err));
            println!("{:#?}", class);
        }
        (Some(d), ..) => abort!(3=> "unknown command: {}", d),
        (None, ..) => abort!(3=> "try dump <file.class>"),
    }
}
