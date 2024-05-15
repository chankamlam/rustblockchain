use num_bigint::BigUint;
use sha2::{Digest, Sha256};

use crate::block::Block;
pub struct ProofOfWork<'a> {
    pub block: &'a Block,
    pub target: BigUint,
}
const DIFFICULTY: u64 = 18;
impl<'a> ProofOfWork<'a> {
    pub fn create_proof(block: &'a Block) -> ProofOfWork<'a> {
        let target = BigUint::from(1u64);
        ProofOfWork {
            block: block,
            target: target<<(256-DIFFICULTY),
        }
    }
    pub fn prepare_data(&self, nonce: u64) -> Vec<u8> {
        let mut data = vec![];
        data.extend(self.block.prev_hash.iter());
        data.extend(self.block.data.as_bytes());
        data.extend(DIFFICULTY.to_be_bytes());
        data.extend(&nonce.to_be_bytes());
        data
    }
    pub fn run(&self) -> (u64, Vec<u8>) {
        let mut nonce = 0;
        let mut hash = vec![];
        loop {
            let data = self.prepare_data(nonce);
            let mut hasher = Sha256::new();
            hasher.update(&data);
            hash = hasher.finalize().to_vec();
            let hash_int = BigUint::from_bytes_be(&hash);

            print!("\r{:x}", hash_int);
            if hash_int < self.target {
                break;
            } else {
                nonce += 1;
            }
        }
        println!();
        (nonce, hash)
    }
}