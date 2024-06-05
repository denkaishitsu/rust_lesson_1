
use rand::prelude::*;
pub fn gen_random() -> u8 {
  let mut rng = rand::thread_rng();
  let n: u8 = rng.gen();
  n
}
