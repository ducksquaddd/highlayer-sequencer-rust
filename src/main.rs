mod db;
mod events;

use db::Databases;
use events::block::proccess_new_block;
use lazy_static::lazy_static;
// use std::time::Duration;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use tokio::{
    task,
    time::{sleep, Duration},
};

static PARENT_PATH: &str = "./db/";

lazy_static! {
    pub static ref DATABASE: Databases = Databases::new(PARENT_PATH);
    pub static ref NODE: Client = Client::new("http://127.0.0.1:18332", Auth::None).unwrap();
}

#[tokio::main]
async fn main() {
    let mut current_block: u64 = NODE.get_block_count().unwrap();

    // Using block polling because I couldnt figure out how to make a compatible bsock web clicnet
    let polling_task = task::spawn(async move {
        loop {
            new_block_polling(&mut current_block);
            sleep(Duration::from_secs(10)).await;
        }
    });

    let _ = tokio::join!(polling_task);
}

fn new_block_polling(current_block: &mut u64) {
    let block_height = NODE.get_block_count().unwrap();

    if *current_block < block_height {
        *current_block = block_height;
        proccess_new_block()
    }

    println!("Block height: {}", block_height);
}

// pub struct global_state {
//     node: Client,
//     last_block: u64,
//     pending_transaction_length: u64, // Transactions that are not yet in a confirmed bundle
//     sequencer_tx_index: u64,         // The total transactions length
//     bundle_length: u64,              // The total amount of bundles
//     recent_bundle_hash: String,      // The most recent parent bundle hash
//     /*
//       Bundles are made every 5 blocks however when the node is syncing it will spam blocks
//       (AFAIK bcoin doesnt have a fully synced event that i could get working)

//       so this is kinda a hacky way to not make bundles when syncing
//     */
//     last_block_time: u64,
//     block_counter: u16,
//     block_reset_threshold: u16,
// }

// r#"421["watch chain"]"#
// r#"420["auth",""]"#
// Database operations example
// DATABASE.transactions.put(b"my key", b"my value").unwrap();
// match DATABASE.transactions.get(b"my d key") {
//     Ok(Some(value)) => println!("retrieved value: {}", String::from_utf8(value).unwrap()),
//     Ok(None) => println!("value not found"),
//     Err(e) => println!("operational problem encountered: {}", e),
// }

// Bitcoin RPC example
// let rpc = Client::new("http://127.0.0.1:18332", Auth::None).unwrap();
// let best_block_hash = rpc.get_block_count().unwrap();
// println!("Best block hash: {}", best_block_hash);

// RocksDB operations example
// let db = DB::open_default(path).unwrap();
// db.put(b"my key", b"my value").unwrap();
// match db.get(b"my key") {
//     Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
//     Ok(None) => println!("value not found"),
//     Err(e) => println!("operational problem encountered: {}", e),
// }
// db.delete(b"my key").unwrap();
// let _ = DB::destroy(&Options::default(), path);

struct Block {
    hash: String,
    version: String,
    prev_block: String,
    merkle_root: String,
    time: u32,
    bits: u32,
    nonce: u32,
    height: u32,
    chainwork: String,
}

//  {
//   hash: '00000000af0b7ef335f244551d16fea91475366d0642d65e009fa5ce4edd1bba',
//   version: '20000000',
//   prevBlock: '00000000bd7af467bc6274514e604f943ded9cb01023d9480c43efabdc38a43d',
//   merkleRoot: '126e1871a178846fcf6bb4105e973562378cc170f2311e166e9927b882c6aabd',
//   time: 1717899248,
//   bits: 486604799,
//   nonce: 3333928836,
//   height: 2820563,
//   chainwork: '000000000000000000000000000000000000000000000e29418ce1523add8c19'
// }

// Our helper method which will read data from stdin and send it along the
// sender provided.
