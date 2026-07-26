use std::{
    fs::File,
    io::{self, Write},
};

enum Mode {
    Attribute,
    Macro,
}

fn parse_cli() -> (File, Mode) {
    let mut args = std::env::args();
    let bin = args.next().unwrap();
    let usage = format!("Usage: {} <file> <attr|macro>", bin);
    let file = File::create(args.next().expect(&usage)).unwrap();
    let mode = args.next().expect(&usage);

    let mode = match &*mode {
        "attr" => Mode::Attribute,
        "macro" => Mode::Macro,
        _ => panic!("{}", usage),
    };

    (file, mode)
}

fn main() -> io::Result<()> {
    let (mut file, mode) = parse_cli();

    let n = 10_000;

    match mode {
        Mode::Attribute => {
            writeln!(file, "use ident_str::ident_str;")?;
            for i in 0..n {
                writeln!(file)?;
                writeln!(
                    file,
                    r#"
                    #[ident_str]
                    macro_rules! macro_{} {{
                        ($name: ident) =>
                            let #name = concat!(stringify!($name), "_foo");
                        {{
                            fn #name() {{
                                println!("hello {{}}", stringify!(#name));
                            }}
                        }}
                    }}
                    "#,
                    i
                )?;
            }
            writeln!(file, "fn main() {{")?;
            for i in 0..n {
                writeln!(
                    file,
                    r#"
                    macro_{0}!(expand_{0});
                    expand_{0}_foo();
                    "#,
                    i,
                )?;
            }
            writeln!(file, "}}")?;
        }
        Mode::Macro => {
            writeln!(file, "use ident_str::ident_str_def;")?;
            for i in 0..n {
                writeln!(file)?;
                writeln!(
                    file,
                    r#"
                    macro_rules! macro_{} {{
                        ($name: ident) => {{
                            ident_str_def! {{
                                #name = concat!(stringify!($name), "_foo"),
                                => {{
                                    fn #name() {{
                                        println!("hello {{}}", stringify!(#name));
                                    }}
                                }}
                            }}
                        }}
                    }}
                    "#,
                    i
                )?;
            }
            writeln!(file, "fn main() {{")?;
            for i in 0..n {
                writeln!(
                    file,
                    r#"
                    macro_{0}!(expand_{0});
                    expand_{0}_foo();
                    "#,
                    i,
                )?;
            }
            writeln!(file, "}}")?;
        }
    }

    Ok(())
}
