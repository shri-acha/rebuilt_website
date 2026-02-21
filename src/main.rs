mod fonts;
mod footer;
mod header;
mod index;
mod section;
mod sections;
mod table;
mod graph_gen;

use index::generate_index;
use std::fs::File;
use std::io::prelude::*;

const BUILD_DIR: &'static str = "public";

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all(format!("{BUILD_DIR}"))?;
    let mut index = File::create(format!("{BUILD_DIR}/index.html"))?;

    index.write_all(generate_index().as_bytes())?;
    Ok(())
}
