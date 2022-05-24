#![allow(unused_imports)]
#![allow(dead_code)]
mod exercicios;
mod list;

use list::rand_values_only;
// use list::gen_list;
// use list::completely_randomized_set;
use exercicios::ExMru;
use std::path::Path;

fn main() {
    let dir = Path::new("/home/ricardo/Documents/projetos/physrandrust/src/templates/");
    let out = Path::new("/home/ricardo/Documents/projetos/physrandrust/teste.txt");
    // let dir2 = Path::new("/home/invalid");
    rand_values_only(2,3,dir, out);
}
