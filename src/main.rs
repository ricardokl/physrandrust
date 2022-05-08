#![allow(dead_code)]
#![allow(unused_imports)]
mod exercicios;
mod list;

use list::rand_values_only;
use list::gen_list;
use list::completely_randomized_set;
use exercicios::ExMru;
use std::path::{Path, PathBuf};

fn main() {
    let dir = Path::new("/home/ricardo/Documents/projetos/physrandrust/src/templates/");
    // let dir2 = Path::new("/home/invalid");
    rand_values_only(2,3,dir);
}