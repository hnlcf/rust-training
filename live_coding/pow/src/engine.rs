use crate::proto::{Block, BlockHash};
use blake3::Hasher;
use rayon::prelude::*;

const PREFI_ZERO_NUM: usize = 3;

/// Proof of work
///
/// Search a nonce, calculate the hash of data of [`Block`] and nonce.
///
/// If the hash matches requirements, return [`BlockHash`], otherwise `None`.
pub fn pow(block: Block) -> Option<BlockHash> {
    let hasher = blake3_base_hash(&block.data);
    let nonce = (0..u32::MAX).into_par_iter().find_any(|&n| {
        let hash = blake3_hash(hasher.clone(), n);
        hash[0..PREFI_ZERO_NUM] == [0; PREFI_ZERO_NUM]
    });

    nonce.map_or(None, |n| {
        let id = get_block_id(&block);
        let hash = blake3_hash(hasher, n);
        Some(BlockHash {
            id,
            hash,
            nonce: nonce.unwrap_or(0),
        })
    })
}

fn get_block_id(block: &Block) -> Vec<u8> {
    blake3::hash(&block.data).as_bytes().to_vec()
}

/// Calcute the final hash based on nonce and hasher of [`Block`] data.
///
/// data + nonce(as BE bytes slice) => hash
fn blake3_hash(mut hasher: Hasher, nonce: u32) -> Vec<u8> {
    hasher.update(&nonce.to_be_bytes());

    hasher.finalize().as_bytes().to_vec()
}

/// Only calculate the hash of [`Block`] data.
fn blake3_base_hash(data: &[u8]) -> Hasher {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher
}
