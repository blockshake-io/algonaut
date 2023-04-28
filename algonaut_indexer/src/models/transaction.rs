/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

use algonaut_crypto::{deserialize_opt_hash, HashDigest};
use algonaut_encoding::Bytes;

/// Transaction : Contains all fields common to all transactions and serves as an envelope to all transactions type. Represents both regular and inner transactions.  Definition: data/transactions/signedtxn.go : SignedTxn data/transactions/transaction.go : Transaction

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(
        rename = "application-transaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_transaction: Option<Box<crate::models::TransactionApplication>>,
    #[serde(
        rename = "asset-config-transaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_config_transaction: Option<Box<crate::models::TransactionAssetConfig>>,
    #[serde(
        rename = "asset-freeze-transaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_freeze_transaction: Option<Box<crate::models::TransactionAssetFreeze>>,
    #[serde(
        rename = "asset-transfer-transaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_transfer_transaction: Option<Box<crate::models::TransactionAssetTransfer>>,
    /// \\[sgnr\\] this is included with signed transactions when the signing address does not equal the sender. The backend can use this to ensure that auth addr is equal to the accounts auth addr.
    #[serde(rename = "auth-addr", skip_serializing_if = "Option::is_none")]
    pub auth_addr: Option<String>,
    /// \\[rc\\] rewards applied to close-remainder-to account.
    #[serde(rename = "close-rewards", skip_serializing_if = "Option::is_none")]
    pub close_rewards: Option<u64>,
    /// \\[ca\\] closing amount for transaction.
    #[serde(rename = "closing-amount", skip_serializing_if = "Option::is_none")]
    pub closing_amount: Option<u64>,
    /// Round when the transaction was confirmed.
    #[serde(rename = "confirmed-round", skip_serializing_if = "Option::is_none")]
    pub confirmed_round: Option<u64>,
    /// Specifies an application index (ID) if an application was created with this transaction.
    #[serde(
        rename = "created-application-index",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_application_index: Option<u64>,
    /// Specifies an asset index (ID) if an asset was created with this transaction.
    #[serde(
        rename = "created-asset-index",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_asset_index: Option<u64>,
    /// \\[fee\\] Transaction fee.
    #[serde(rename = "fee")]
    pub fee: u64,
    /// \\[fv\\] First valid round for this transaction.
    #[serde(rename = "first-valid")]
    pub first_valid: u64,
    /// \\[gh\\] Hash of genesis block.
    #[serde(
        rename = "genesis-hash",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_opt_hash"
    )]
    pub genesis_hash: Option<HashDigest>,
    /// \\[gen\\] genesis block ID.
    #[serde(rename = "genesis-id", skip_serializing_if = "Option::is_none")]
    pub genesis_id: Option<String>,
    /// Application state delta.
    #[serde(rename = "global-state-delta", skip_serializing_if = "Option::is_none")]
    pub global_state_delta: Option<Vec<crate::models::EvalDeltaKeyValue>>,
    /// \\[grp\\] Base64 encoded byte array of a sha512/256 digest. When present indicates that this transaction is part of a transaction group and the value is the sha512/256 hash of the transactions in that group.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Bytes>,
    /// Transaction ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Inner transactions produced by application execution.
    #[serde(rename = "inner-txns", skip_serializing_if = "Option::is_none")]
    pub inner_txns: Option<Vec<crate::models::Transaction>>,
    /// Offset into the round where this transaction was confirmed.
    #[serde(rename = "intra-round-offset", skip_serializing_if = "Option::is_none")]
    pub intra_round_offset: Option<u64>,
    #[serde(rename = "keyreg-transaction", skip_serializing_if = "Option::is_none")]
    pub keyreg_transaction: Option<Box<crate::models::TransactionKeyreg>>,
    /// \\[lv\\] Last valid round for this transaction.
    #[serde(rename = "last-valid")]
    pub last_valid: u64,
    /// \\[lx\\] Base64 encoded 32-byte array. Lease enforces mutual exclusion of transactions.  If this field is nonzero, then once the transaction is confirmed, it acquires the lease identified by the (Sender, Lease) pair of the transaction until the LastValid round passes.  While this transaction possesses the lease, no other transaction specifying this lease can be confirmed.
    #[serde(rename = "lease", skip_serializing_if = "Option::is_none")]
    pub lease: Option<Bytes>,
    /// \\[ld\\] Local state key/value changes for the application being executed by this transaction.
    #[serde(rename = "local-state-delta", skip_serializing_if = "Option::is_none")]
    pub local_state_delta: Option<Vec<crate::models::AccountStateDelta>>,
    /// \\[lg\\] Logs for the application being executed by this transaction.
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<Bytes>>,
    /// \\[note\\] Free form data.
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Bytes>,
    #[serde(
        rename = "payment-transaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_transaction: Option<Box<crate::models::TransactionPayment>>,
    /// \\[rr\\] rewards applied to receiver account.
    #[serde(rename = "receiver-rewards", skip_serializing_if = "Option::is_none")]
    pub receiver_rewards: Option<u64>,
    /// \\[rekey\\] when included in a valid transaction, the accounts auth addr will be updated with this value and future signatures must be signed with the key represented by this address.
    #[serde(rename = "rekey-to", skip_serializing_if = "Option::is_none")]
    pub rekey_to: Option<String>,
    /// Time when the block this transaction is in was confirmed.
    #[serde(rename = "round-time", skip_serializing_if = "Option::is_none")]
    pub round_time: Option<u64>,
    /// \\[snd\\] Sender's address.
    #[serde(rename = "sender")]
    pub sender: String,
    /// \\[rs\\] rewards applied to sender account.
    #[serde(rename = "sender-rewards", skip_serializing_if = "Option::is_none")]
    pub sender_rewards: Option<u64>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Box<crate::models::TransactionSignature>>,
    #[serde(
        rename = "state-proof-transaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub state_proof_transaction: Option<Box<crate::models::TransactionStateProof>>,
    /// \\[type\\] Indicates what type of transaction this is. Different types have different fields.  Valid types, and where their fields are stored: * \\[pay\\] payment-transaction * \\[keyreg\\] keyreg-transaction * \\[acfg\\] asset-config-transaction * \\[axfer\\] asset-transfer-transaction * \\[afrz\\] asset-freeze-transaction * \\[appl\\] application-transaction * \\[stpf\\] state-proof-transaction
    #[serde(rename = "tx-type")]
    pub tx_type: TxType,
}

impl Transaction {
    /// Contains all fields common to all transactions and serves as an envelope to all transactions type. Represents both regular and inner transactions.  Definition: data/transactions/signedtxn.go : SignedTxn data/transactions/transaction.go : Transaction
    pub fn new(
        fee: u64,
        first_valid: u64,
        last_valid: u64,
        sender: String,
        tx_type: TxType,
    ) -> Transaction {
        Transaction {
            application_transaction: None,
            asset_config_transaction: None,
            asset_freeze_transaction: None,
            asset_transfer_transaction: None,
            auth_addr: None,
            close_rewards: None,
            closing_amount: None,
            confirmed_round: None,
            created_application_index: None,
            created_asset_index: None,
            fee,
            first_valid,
            genesis_hash: None,
            genesis_id: None,
            global_state_delta: None,
            group: None,
            id: None,
            inner_txns: None,
            intra_round_offset: None,
            keyreg_transaction: None,
            last_valid,
            lease: None,
            local_state_delta: None,
            logs: None,
            note: None,
            payment_transaction: None,
            receiver_rewards: None,
            rekey_to: None,
            round_time: None,
            sender,
            sender_rewards: None,
            signature: None,
            state_proof_transaction: None,
            tx_type,
        }
    }
}

/// \\[type\\] Indicates what type of transaction this is. Different types have different fields.  Valid types, and where their fields are stored: * \\[pay\\] payment-transaction * \\[keyreg\\] keyreg-transaction * \\[acfg\\] asset-config-transaction * \\[axfer\\] asset-transfer-transaction * \\[afrz\\] asset-freeze-transaction * \\[appl\\] application-transaction * \\[stpf\\] state-proof-transaction
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TxType {
    #[serde(rename = "pay")]
    Pay,
    #[serde(rename = "keyreg")]
    Keyreg,
    #[serde(rename = "acfg")]
    Acfg,
    #[serde(rename = "axfer")]
    Axfer,
    #[serde(rename = "afrz")]
    Afrz,
    #[serde(rename = "appl")]
    Appl,
    #[serde(rename = "stpf")]
    Stpf,
}

impl Default for TxType {
    fn default() -> TxType {
        Self::Pay
    }
}
