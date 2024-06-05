mod generator;
pub fn print_random_num() {
    let n = generator::gen_random();
    println!("Random u8: {}", n);
}
