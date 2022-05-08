use crate::ExMru;
use std::path::Path;

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

pub fn rand_values_only(x: u8, y: u8, dir: &Path) {
    let mut ex_set: Vec<ExMru> = vec!();
    for _ in 0..x {
        let ex = ExMru::new(dir, 100, 100, 100);
        ex_set.push(ex);
    }
    for _ in 0..y {
        println!("{}", "======== Lista ========");
        for ex in ex_set.iter_mut() {
            ex.recalc(70,70,70);
            let new = ex.subs();
            println!("{}", new)
        }
    }
}
