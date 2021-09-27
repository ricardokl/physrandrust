use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let vm = rng.gen_range(-100..100) as f32;
    let _xi = rng.gen_range(-5000..5000) as f32;
    let ti = (rng.gen_range(0..96) as f32)/4.0;
    let dt = (rng.gen_range(4..24) as f32)/4.0;
    let _tf = ti + dt;
    let dx = vm*dt;
    println!("vm = {} = {} / {}",vm,dx,dt)
}
