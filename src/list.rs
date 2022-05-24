use crate::ExMru;
use std::path::Path;
use std::fs::File;
use std::fs::write;
use std::io::{Write, Error};
use std::fs::OpenOptions;

fn file_generator(ex: String, out: &Path) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(out)?;
    file.write_all(ex.as_bytes())?;
    Ok(())
}

pub fn gen_list(x: u8, dir: &Path) {
    for _ in 0..x {
        let ex = ExMru::new(dir, 100, 100, 100);
        let texto = ex.subs();
        println!("{}", texto);
    }
}

pub fn completely_randomized_set(x: u8, y:u8, dir: &Path) {
    for _ in 0..x {
        println!("{}", "=========== Lista ===========");
        gen_list(y, dir);
    }
}

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
