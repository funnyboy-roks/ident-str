ident_str::ident_str! {
    #name = "hello",
    #foo = None
    => const _: &str = stringify!(#foo); // just ignore the #foo
}

fn main() {}
