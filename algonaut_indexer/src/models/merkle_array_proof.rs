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
pub struct MerkleArrayProof {
    #[serde(rename = "hash-factory", skip_serializing_if = "Option::is_none")]
    pub hash_factory: Option<Box<crate::models::HashFactory>>,
    /// \\[pth\\]
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<Bytes>>,
    /// \\[td\\]
    #[serde(rename = "tree-depth", skip_serializing_if = "Option::is_none")]
    pub tree_depth: Option<u64>,
}

impl MerkleArrayProof {
    pub fn new() -> MerkleArrayProof {
        MerkleArrayProof {
            hash_factory: None,
            path: None,
            tree_depth: None,
        }
    }
}
