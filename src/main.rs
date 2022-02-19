use sha1::{Sha1,Digest};
use std::hash::Hash;
use std::process::exit;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

type Nonce = u32;

fn main() {
    let nonce = rand::random::<Nonce>();

    let mut hasher = Sha1::new();
    hasher.update(nonce.to_ne_bytes());
    let hash_to_match = hasher.finalize().to_vec();


    (Nonce::MIN..=Nonce::MAX).into_par_iter().for_each(|n| {
        let mut hasher = Sha1::new();
        hasher.update(n.to_ne_bytes());
        let hash = hasher.finalize().to_vec();

        if hash == hash_to_match {
            println!("Found nonce: {}", n);
            exit(0);
        }
    })
}
