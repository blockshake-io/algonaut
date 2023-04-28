/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

use algonaut_encoding::Bytes;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StateProofSignature {
    #[serde(rename = "falcon-signature", skip_serializing_if = "Option::is_none")]
    pub falcon_signature: Option<Bytes>,
    #[serde(rename = "merkle-array-index", skip_serializing_if = "Option::is_none")]
    pub merkle_array_index: Option<u64>,
    #[serde(rename = "proof", skip_serializing_if = "Option::is_none")]
    pub proof: Option<Box<crate::models::MerkleArrayProof>>,
    /// \\[vkey\\]
    #[serde(rename = "verifying-key", skip_serializing_if = "Option::is_none")]
    pub verifying_key: Option<Bytes>,
}

impl StateProofSignature {
    pub fn new() -> StateProofSignature {
        StateProofSignature {
            falcon_signature: None,
            merkle_array_index: None,
            proof: None,
            verifying_key: None,
        }
    }
}
