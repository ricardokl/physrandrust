use rand::{Rng, thread_rng};
use rand::seq::IteratorRandom;
use std::{fs, io, option::Option, result::Result};

fn main() {
    mru();
    let ex = template(".");
    println!("{:?}", ex)
}

fn mru() -> [f32; 6] {
    let mut rng = thread_rng();
    let vm = rng.gen_range(-100..100) as f32;
    let xi = rng.gen_range(-5000..5000) as f32;
    let ti = (rng.gen_range(0..96) as f32)/4.0;
    let dt = (rng.gen_range(4..24) as f32)/4.0;
    let tf = &ti + &dt;
    let dx = &vm*&dt;
    return [vm, xi, ti, dt, tf, dx]
}

fn template(dir: &str) -> Result<Option<fs::DirEntry>, io::Error> {
    let mut rng = thread_rng();
    let files: fs::ReadDir = fs::read_dir(dir)?;
    let file = files.choose(&mut rng);
    return file.transpose()
}
