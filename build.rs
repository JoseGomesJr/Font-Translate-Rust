extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/font_arry.c")
        .compile("libfont.a");
}
