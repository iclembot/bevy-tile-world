use rand::distributions::{Alphanumeric, DistString};
pub fn rand_seed(len: usize) -> String{
let string = Alphanumeric.sample_string(&mut rand::thread_rng(), len);
string
}
