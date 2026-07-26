ident_str::ident_str_def! {
    #name = "hello"
    => const _: &str = stringify!(#foo); // just ignore the #foo
}

fn main() {}
