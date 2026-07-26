ident_str::ident_str_def! {
    #name = "hello",
    #foo = None
    => const _: &str = stringify!(#foo); // just ignore the #foo
}

fn main() {}
