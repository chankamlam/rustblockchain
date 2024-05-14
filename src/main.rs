use crate::blockchain::Blockchain;

mod block;
mod blockchain; // Add this line

fn main() {
    let mut blockchain = Blockchain::create_blockchain(); // Update this line
    blockchain.add_block("First Block".to_string());
    blockchain.add_block("Second Block".to_string());
    blockchain.add_block("Third Block".to_string());
    blockchain.print();
}
