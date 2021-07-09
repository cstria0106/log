use std::{env, path::Path};

use syntect::{dumps::dump_to_file, parsing::SyntaxSetBuilder};

fn main() {
    let out = env::var("OUT_DIR").unwrap();

    let mut builder = SyntaxSetBuilder::new();
    builder
        .add_from_folder("../../assets/syntax/toml-syntax", false)
        .unwrap();

    let syntax_set = builder.build();
    dump_to_file(&syntax_set, Path::new(&out).join("syntax.dump")).unwrap();
}
