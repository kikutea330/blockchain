extern crate chrono;
extern crate sha2;
use chrono::prelude::*;
use sha2::{Sha256, Digest};

fn main() {
    let mut block_chain: BlockChainNode = BlockChainNode::new();
    println!("Hello, world!");
    println!("block_chain is {:?}", block_chain);
    block_chain.generate_block();
    for block in block_chain.block_chain {
        println!("blockchain: {:?}", block);
        println!("hash:{}",block.block_header.calculate_hash());
    }
    
}

///ブロックのヘッダ
#[derive(Debug, Clone)]
struct BlockHeader {
    //プロトコルのバージョン
    //version: u32,
    //ブロック高
    index: u64,
    //親ブロックのハッシュ値: 32Byte Hex String
    previous_block_hash: String,
    //ブロック内トランザクションに対するマークルツリーのルートハッシュ
    // : 32Byte Hex String 
    //merkle_root: String,
    // Unix Timestamp (sec): 4Byte unsigned int
    timestamp: u64,
    //
    //difficulty: f64,
    //nonce: u32,
}
impl BlockHeader {
    fn new(index: u64, previous_block_hash: String)
     -> BlockHeader {
        BlockHeader {
            index,
            previous_block_hash,
            timestamp: Utc::now().timestamp() as u64,
        }
    }
    fn calculate_hash(&self) -> String {
        let self_str = format!("{}{}{}", &self.index, &self.previous_block_hash, &self.timestamp);
        println!("self: {}", self_str);
        let h_self_str = BlockHeader::hash(&self_str[..]);
        println!("sha2(self): {}", h_self_str);
        let h_h_self_str = BlockHeader::hash(&h_self_str[..]);
        println!("sha2(sha2(self)): {}", h_h_self_str);
        h_h_self_str
    }
    fn hash(s: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.input(s.as_bytes());
        hasher.result()
            .iter()
            .map( |b| format!("{:02X}", b) )
            .collect()
    }
}

///トランザクション
#[derive(Debug, Clone)]
struct Transaction  {
    //差出人
    sender: String,
    //宛先
    recipient: String,
    //金額
    amount: u64,
}
impl Transaction{
    fn new(sender: String, recipient: String, amount: u64)
     -> Transaction { 
        Transaction { sender, recipient, amount }
    }
}
/*impl Copy for Transaction{}*/
/*impl Clone for Transaction{
    fn clone(&self) -> Transaction {
        *self
    }
}*/

///ブロック
#[derive(Debug, Clone)]
struct Block {
    //block_size: u32,
    block_header: BlockHeader,
    //transaction_counter: u64,
    transactions: Vec<Transaction>,
}
impl Block {
    fn new(block_header: BlockHeader, transactions: Vec<Transaction>)
     -> Block {
        Block { block_header, transactions }
    }
    fn get_index(&self) -> u64 {
        self.block_header.index
    }
    fn get_hash(&self) -> String {
        self.block_header.calculate_hash()
    }
    fn get_previous_hash(&self) -> &str {
        &self.block_header.previous_block_hash[..]
    }
}

/// ブロックチェーンノード
#[derive(Debug)]
struct BlockChainNode {
    //ブロックチェーン
    block_chain: Vec<Block>,
    //次の
    transaction_pool: Vec<Transaction>,
}
impl BlockChainNode {
    fn new() -> BlockChainNode {
        BlockChainNode {
            block_chain: vec![BlockChainNode::get_genesis_block()],
            transaction_pool: Vec::new(),
        }
    }
    fn get_genesis_block() -> Block {
        Block::new(
            BlockHeader::new(0, String::from("0")),
            vec![Transaction::new(String::from(""), String::from("recipient"), 100)]
        )
    }
    fn generate_block(&mut self) -> Block {
        let latest_block = self.get_latest_block();
        let next_block = Block::new(
            BlockHeader::new(latest_block.get_index()+1, latest_block.get_hash()),
            self.transaction_pool.clone(),
        );
        // トランザクションプールの初期化
        self.transaction_pool = Vec::new();

        self.block_chain.push(next_block.clone());
        //broadcast_block()
        next_block.clone()
    }
    fn get_latest_block(&self) -> &Block {
        self.block_chain.last().unwrap()
    }
    ///ブロックの検証
    fn validate_block(&self, block: &Block) -> bool {
        let latest_block = self.get_latest_block();
        if block.get_index() != latest_block.get_index()+1 {
            return false;
        } else if block.get_previous_hash() == latest_block.get_hash() {
            return false;
        }
        true
    }
    ///ブロックをチェーンに追加
    fn append_block(&mut self, block: Block) {
        if self.validate_block(&block) {
            self.block_chain.push(block);
        }
    }
}