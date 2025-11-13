ident_str::ident_str! {
    #name = "hello"
    => const _: &str = stringify!(#foo); // just ignore the #foo
}

fn main() {}
