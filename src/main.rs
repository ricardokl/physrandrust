use rand::{Rng, thread_rng};
use rand::seq::IteratorRandom;
use std::{fs, io, result::Result, path::Path};

fn main() {
    mru();
    let dir = Path::new("/home/ricardo/Documents/projetos/physrandrust/src/templates/");
    let ex = template(dir).unwrap();
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

fn template(dir: &Path) -> Result<String, io::Error> {
    let mut rng = thread_rng();
    let files: fs::ReadDir = fs::read_dir(dir).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let text = fs::read_to_string(file.path());
    return text
}
