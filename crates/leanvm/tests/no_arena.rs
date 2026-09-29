//! The no-arena path, in its own binary: `enable_arena` is a process-wide
//! one-way opt-in, so a test that must not have it cannot share a process with
//! `api.rs`.

use leanvm::*;

const LEAF_INDEX: xmss::LeafIndex = 5;

#[test]
fn aggregate_without_the_arena() {
    setup_prover_without_arena();
    assert!(!zk_alloc::is_enabled(), "this path must leave the arena disengaged");
    let rng = &mut rand::rng();

    let message = [3; xmss::MESSAGE_LEN];
    let signers = (0..2)
        .map(|_| {
            let (secret_key, pub_key) = xmss::key_gen(rng, LEAF_INDEX, LEAF_INDEX).unwrap();
            let signature = xmss::sign(&secret_key, &message, LEAF_INDEX).unwrap();
            (pub_key, LEAF_INDEX, message, signature)
        })
        .collect();

    let aggregated = aggregate(&[], signers, vec![], &[], None, 2).unwrap();
    aggregated.verify().unwrap();
}
