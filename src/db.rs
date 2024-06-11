use rocksdb::DB;
use std::path::Path;

pub struct Databases {
    pub transactions: DB,
    pub sequencer_tx_index: DB,
    pub pending_bundles: DB,
    pub confirmed_bundles: DB,
    pub bundle_hash: DB,
    pub bundles: DB,
    pub to_be_settled: DB,
    pub balances: DB,
    pub deposits_indexed: DB,
}

impl Databases {
    pub fn new(parent_path: &str) -> Self {
        let base_path = Path::new(parent_path);

        Databases {
            transactions: DB::open_default(base_path.join("transactions")).unwrap(),
            sequencer_tx_index: DB::open_default(base_path.join("sequencerTxIndex")).unwrap(),
            pending_bundles: DB::open_default(base_path.join("pendingBundles")).unwrap(),
            confirmed_bundles: DB::open_default(base_path.join("confirmedBundles")).unwrap(),
            bundle_hash: DB::open_default(base_path.join("bundleHash")).unwrap(),
            bundles: DB::open_default(base_path.join("bundles")).unwrap(),
            to_be_settled: DB::open_default(base_path.join("toBeSettled")).unwrap(),
            balances: DB::open_default(base_path.join("balances")).unwrap(),
            deposits_indexed: DB::open_default(base_path.join("depositsIndexed")).unwrap(),
        }
    }
}
