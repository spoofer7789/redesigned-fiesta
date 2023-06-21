use std::any::Any;

use orchard::bundle::Flags;
use orchard::tree::{MerkleHashOrchard,Anchor};
use orchard::value::NoteValue;
use orchard::Address;
use orchard::keys::{SpendingKey, FullViewingKey, Scope, OutgoingViewingKey};
use orchard::builder::{Builder, OutputError};
use rand::Rng;
use zcash_primitives::merkle_tree::{MerklePath, self};
use zcash_primitives::transaction::components::amount;
use zcash_primitives::transaction::components::amount::Amount;
use orchard::{Note};
use zcash_primitives::sapling::Node;
pub fn generate_zcash_wallet() -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 32];
    rng.fill(&mut random_bytes);

    let sk: Option<SpendingKey> = SpendingKey::from_bytes(random_bytes).into();
    let sk = sk.expect("Failed to create spending key from bytes");
    let full_viewing_key = FullViewingKey::from(&sk);
    let address = full_viewing_key.address_at(0u32, Scope::External);

    (format!("{:?}", sk), format!("{:?}", address))
}
// pub fn sign_transaction(sk: SpendingKey, recipient: Address, value: Amount,memo: Option<String>) -> Result<(), OutputError> {
//     // Convert the SpendingKey to a FullViewingKey
//     let fvk = FullViewingKey::from(&sk);
//     // Get the OutgoingViewingKey from the FullViewingKey
//     let ovk = Some(fvk.to_ovk(Scope::External));
 
//     let spends_enabled = true;
//     let outputs_enabled = true;
//     // Compute the anchor
//     let anchor = Anchor::from.clone().MerkleHashOrchard;
    
//     let mut builder =
//         orchard::builder::Builder::new(Flags::from_parts(spends_enabled, outputs_enabled,), anchor);
//     // Convert the Amount to a NoteValue
//     let note_value = NoteValue::default;
//     // Convert the memo to a [u8; 512]
//     let memo_bytes = memo.map(|s| {
//         let mut bytes = [0u8; 512];
//         bytes.copy_from_slice(s.as_bytes());
//         bytes
//     });
//     // Add the recipient to the transaction
//     builder.add_recipient(ovk, recipient, note_value(), memo_bytes)
// }