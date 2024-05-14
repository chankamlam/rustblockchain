
// pub mod blockchain{
    use crate::block::Block;
    pub struct Blockchain {
        pub blocks: Vec<Block>,
    }
    impl Blockchain {
        pub fn create_blockchain() -> Blockchain {
            let blocks = vec![Block::create_genesis()];
            Blockchain { blocks }
        }
        pub fn add_block(&mut self, data: String) {
            let prev_hash = self.blocks.last().unwrap().hash.clone();
            let block = Block::create_block(data, prev_hash);
            self.blocks.push(block);
        }
        pub fn print(&self) {
            for block in &self.blocks {
                block.print();
            }
        }
    }
// }