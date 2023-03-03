use {
    safecoin_sdk::{clock::Slot, signature::Signature, transaction::SanitizedTransaction},
    safecoin_transaction_status::TransactionStatusMeta,
    std::sync::{Arc, RwLock},
};

pub trait TransactionNotifier {
    fn notify_transaction(
        &self,
        slot: Slot,
        transaction_slot_index: usize,
        signature: &Signature,
        transaction_status_meta: &TransactionStatusMeta,
        transaction: &SanitizedTransaction,
    );
}

pub type TransactionNotifierLock = Arc<RwLock<dyn TransactionNotifier + Sync + Send>>;
