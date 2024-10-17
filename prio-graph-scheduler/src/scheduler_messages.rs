use {
    solana_sdk::{clock::Slot, transaction::SanitizedTransaction},
    std::fmt::Display,
};

/// A unique identifier for a transaction batch.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TransactionBatchId(u64);

impl TransactionBatchId {
    pub fn new(index: u64) -> Self {
        Self(index)
    }
}

impl Display for TransactionBatchId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for TransactionBatchId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

/// A unique identifier for a transaction.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionId(u64);

impl TransactionId {
    pub fn new(index: u64) -> Self {
        Self(index)
    }
}

impl Display for TransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for TransactionId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MaxAge {
    pub epoch_invalidation_slot: Slot,
    pub alt_invalidation_slot: Slot,
}

/// Message: [Scheduler -> Worker]
/// Transactions to be consumed (i.e. executed, recorded, and committed)
pub struct ConsumeWork {
    pub batch_id: TransactionBatchId,
    pub ids: Vec<TransactionId>,
    pub transactions: Vec<SanitizedTransaction>,
    pub max_ages: Vec<MaxAge>,
}

/// Message: [Worker -> Scheduler]
/// Processed transactions.
pub struct FinishedConsumeWork {
    pub work: ConsumeWork,
    pub retryable_indexes: Vec<usize>,
}