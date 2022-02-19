use sha1::{Sha1,Digest};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Nonce = u16;

fn sha1(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

fn main() {
    for _ in 0..10000 {
        let nonce = rand::random::<Nonce>();

        let hash_to_match = sha1(&nonce.to_be_bytes());
        (Nonce::MIN..=Nonce::MAX).into_par_iter().find_first(|n| {
            hash_to_match == sha1(&n.to_be_bytes())
        });
    }
}
