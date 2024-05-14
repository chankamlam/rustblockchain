
// pub mod block {
    use sha2::{Digest, Sha256};

    // create a block class
    pub struct Block {
        // pub index: u64,
        // pub timestamp: u64,
        pub data: String,
        pub prev_hash: Vec<u8>,
        pub hash:Vec<u8>,
    }

    // create a block constructor
    impl Block {
        pub fn create_genesis() -> Block {
            let mut block = Block {
                data: "Genesis Block".to_string(),
                prev_hash: vec![],
                hash: vec![],
            };
            block.hash = block.derive_hash().to_vec();
            block
        }
        pub fn create_block(data: String, prev_hash: Vec<u8>) -> Block {
            let mut block = Block {
                data: data,
                prev_hash: prev_hash,
                hash: vec![0; 32],
            };
            block.hash = block.derive_hash().to_vec();
            block
        }
        pub fn print(&self) {
            println!("data: {}", self.data);

            // println prev_hash convert to hex string
            let prev_hash_str = self.prev_hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("prev_hash: {}", prev_hash_str);

            // println hash convert to hex string
            let hash_str = self.hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            println!("hash: {}\n", hash_str);
        }
        pub fn derive_hash(&self) -> Vec<u8> {
            let mut hasher = Sha256::new();

            hasher.update(vec![self.data.as_bytes().to_vec(), self.prev_hash.clone()].concat());
            // hasher.update(&self.prev_hash);
            hasher.finalize().to_vec()

        }
    }

// }
