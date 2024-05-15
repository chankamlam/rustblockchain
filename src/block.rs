
use crate::proof::ProofOfWork;
use crate::util::vec2hex;

// create a block class
pub struct Block {
    pub data: String,
    pub prev_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u64,
}

// create a block constructor
impl Block {
    pub fn create_genesis() -> Block {
        Block::create_block("Genesis Block".to_string(), vec![])
    }
    pub fn create_block(data: String, prev_hash: Vec<u8>) -> Block {
        let mut block = Block {
            data: data,
            prev_hash: prev_hash,
            hash: vec![],
            nonce: 0,
        };
        // block.hash = block.derive_hash().to_vec();
        let pow = ProofOfWork::create_proof(&block);
        let (nonce,hash) = pow.run();
        block.hash = hash;
        block.nonce = nonce;
        block
    }
    pub fn print(&self) {
        println!("data: {}", self.data);
        println!("prev_hash: {}", vec2hex(&self.prev_hash));
        println!("hash: {}\n", vec2hex(&self.hash));
    }
}
