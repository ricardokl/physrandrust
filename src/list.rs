use crate::ExMru;
use std::path::Path;
use std::fs::{File,write};
use std::io::{Write, Error};
use std::fs::OpenOptions;

// File generator to print the lists on files
fn file_generator(ex: String, out: &Path) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(out)?;
    file.write_all(ex.as_bytes())?;
    Ok(())
}

// Geneator for  a random list with a number of problems
// Takes as inout the dir of the templates folder
pub fn gen_list(x: u8, dir: &Path) {
    for _ in 0..x {
        let ex = ExMru::new(dir, 100, 100, 100);
        let texto = ex.subs();
        println!("{}", texto);
    }
}

// Generator for a set os completely randomized lists
// Eacho one will be ompletely diferent from each other
pub fn completely_randomized_set(x: u8, y:u8, dir: &Path) {
    for _ in 0..x {
        println!("{}", "=========== Lista ===========");
        gen_list(y, dir);
    }
}

// Generates a set os lists
// All lists will have the same problems but different values for the variables
pub fn rand_values_only(x: u8, y: u8, dir: &Path, out: &Path) {
    let mut ex_set: Vec<ExMru> = vec!();
    for _ in 0..x {
        let ex = ExMru::new(dir, 100, 100, 100);
        ex_set.push(ex);
    }
    for _ in 0..y {
        for ex in ex_set.iter_mut() {
            ex.recalc(70,70,70);
            let new = ex.subs();
            file_generator(new, out).unwrap();
        }
    }
}
