use bitcoin::hashes::Hash;
use bitcoin::key::{Keypair, TapTweak, TweakedKeypair, UntweakedPublicKey, Verification};
use bitcoin::secp256k1::Message;
use bitcoin::sighash::{Prevouts, SighashCache};
use bitcoin::{absolute, transaction, Sequence, TapSighashType, Transaction, TxIn, Witness};
use bitcoin::{
    key::{rand, Secp256k1},
    secp256k1::{SecretKey, Signing},
    Address, Amount, OutPoint, ScriptBuf, TxOut, Txid,
};
use std::str::FromStr;

const DUMMY_UTXO_AMOUNT: Amount = Amount::from_sat(20_000_000);
const SPEND_AMOUNT: Amount = Amount::from_sat(5_000_000);
const CHANGE_AMOUNT: Amount = Amount::from_sat(14_999_000);

// Generate a secret key and a wpkh for a mock up sender.
fn senders_keys<C: Signing>(secp: &Secp256k1<C>) -> Keypair {
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    Keypair::from_secret_key(secp, &secret_key)
}

// Generate a receiver address from a hardcoded string.
fn receivers_address() -> Address {
    Address::from_str("bc1q7cyrfmck2ffu2ud3rn5l5a8yv6f0chkp0zpemf")
        .expect("Failed to parse address")
        .require_network(bitcoin::Network::Bitcoin)
        .expect("Valid address for Bitcoin network")
}

// Generate a dummy `P2TR` UTXO for the sender.
fn dummy_unspent_transaction_output<C: Verification>(
    secp: &Secp256k1<C>,
    internal_key: UntweakedPublicKey,
) -> (OutPoint, TxOut) {
    let script_pubkey = ScriptBuf::new_p2tr(secp, internal_key, None);

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
    let keypair = senders_keys(&secp);
    let (internal_key, _) = keypair.x_only_public_key();

    // Get a receiver address.
    let receiver_address = receivers_address();

    // Create a dummy UTXO for the sender.
    let (dummy_out_point, dummy_utxo) = dummy_unspent_transaction_output(&secp, internal_key);

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
        script_pubkey: ScriptBuf::new_p2tr(&secp, internal_key, None),
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
    let sighash_type = TapSighashType::Default;
    let prevouts = vec![dummy_utxo];
    let prevouts = Prevouts::All(&prevouts);

    let mut sighasher = SighashCache::new(&mut unsigned_tx);
    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("Failed to create sighash");

    // Sign the sighash.
    let tweaked: TweakedKeypair = keypair.tap_tweak(&secp, None);
    let msg = Message::from_digest(sighash.to_byte_array());
    let sig = secp.sign_schnorr(&msg, &tweaked.to_inner());

    // Update the witness.
    let sig = bitcoin::taproot::Signature {
        signature: sig,
        sighash_type,
    };
    sighasher
        .witness_mut(input_index)
        .unwrap()
        .push(&sig.to_vec());

    // Get the signed transaction.
    let tx = sighasher.into_transaction();

    // Transaction signed and ready to broadcast.
    println!("Transaction: {:#?}", tx);

    println!("--- End module: {}", module_path!());
}
