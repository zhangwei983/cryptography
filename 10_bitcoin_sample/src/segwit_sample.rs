use bitcoin::hashes::Hash;
use bitcoin::secp256k1::Message;
use bitcoin::sighash::SighashCache;
use bitcoin::{absolute, transaction, EcdsaSighashType, Sequence, Transaction, TxIn, Witness};
use bitcoin::{
    key::{rand, Secp256k1},
    secp256k1::{SecretKey, Signing},
    Address, Amount, OutPoint, PublicKey, ScriptBuf, TxOut, Txid, WPubkeyHash,
};
use std::str::FromStr;

const DUMMY_UTXO_AMOUNT: Amount = Amount::from_sat(20_000_000);
const SPEND_AMOUNT: Amount = Amount::from_sat(5_000_000);
const CHANGE_AMOUNT: Amount = Amount::from_sat(14_999_000);

// Generate a secret key and a wpkh for a mock up sender.
fn senders_keys<C: Signing>(secp: &Secp256k1<C>) -> (SecretKey, WPubkeyHash) {
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let public_key = PublicKey::new(secret_key.public_key(secp));

    let wpkh = public_key
        .wpubkey_hash()
        .expect("Failed to generate the witness public key hash.");
    assert_eq!(wpkh.to_byte_array().len(), 20);

    (secret_key, wpkh)
}

// Generate a receiver address from a hardcoded string.
fn receivers_address() -> Address {
    Address::from_str("bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf")
        .expect("Failed to parse address")
        .require_network(bitcoin::Network::Bitcoin)
        .expect("Valid address for Bitcoin network")
}

// Generate a dummy `SegWit V0 P2WPKH` UTXO for the sender.
fn dummy_unspent_transaction_output(wpkh: &WPubkeyHash) -> (OutPoint, TxOut) {
    let script_pubkey = ScriptBuf::new_p2wpkh(wpkh);

    // Dummy transaction output.
    let out_point = OutPoint {
        txid: Txid::all_zeros(), // Invalid.
        vout: 0,
    };

    let utxo = TxOut {
        value: DUMMY_UTXO_AMOUNT,
        script_pubkey,
    };

    (out_point, utxo)
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    let secp = Secp256k1::new();

    // Get a secret key and a wpkh for a mock up sender.
    let (sender_secret_key, sender_wpkh) = senders_keys(&secp);
    // Get a receiver address.
    let receiver_address = receivers_address();

    // Create a dummy UTXO for the sender.
    let (dummy_out_point, dummy_utxo) = dummy_unspent_transaction_output(&sender_wpkh);

    // Input of the transaction.
    let input = TxIn {
        previous_output: dummy_out_point,
        script_sig: ScriptBuf::default(),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    // Spend output.
    let spend = TxOut {
        value: SPEND_AMOUNT,
        script_pubkey: receiver_address.script_pubkey(),
    };

    // Change output.
    let change = TxOut {
        value: CHANGE_AMOUNT,
        script_pubkey: ScriptBuf::new_p2wpkh(&sender_wpkh),
    };

    // The unsigned transaction.
    let mut unsigned_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![input],
        output: vec![spend, change],
    };

    let input_index = 0;

    // Get the sighash to sign.
    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut unsigned_tx);
    let sighash = sighasher
        .p2wpkh_signature_hash(
            input_index,
            &dummy_utxo.script_pubkey,
            DUMMY_UTXO_AMOUNT,
            sighash_type,
        )
        .expect("Failed to create sighash");

    // Sign the sighash.
    let msg = Message::from(sighash);
    let sig = secp.sign_ecdsa(&msg, &sender_secret_key);

    // Update the witness.
    let sig = bitcoin::ecdsa::Signature {
        signature: sig,
        sighash_type,
    };
    let public_key = sender_secret_key.public_key(&secp);
    *sighasher.witness_mut(input_index).unwrap() = Witness::p2wpkh(&sig, &public_key);

    // Get the signed transaction.
    let tx = sighasher.into_transaction();

    // Transaction signed and ready to broadcast.
    println!("Transaction: {:#?}", tx);

    println!("--- End module: {}", module_path!());
}
