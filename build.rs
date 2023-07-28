extern crate cc;

fn main() {
    cc::Build::new().file("src/callback.c").compile("callback");
}
