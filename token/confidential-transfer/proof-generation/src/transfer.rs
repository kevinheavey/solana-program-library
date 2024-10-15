use {
    solana_zk_sdk::{
        zk_elgamal_proof_program::proof_data::{
            BatchedGroupedCiphertext3HandlesValidityProofData, BatchedRangeProofU128Data,
            CiphertextCommitmentEqualityProofData,
        },
    },
};

/// The proof data required for a confidential transfer instruction when the
/// mint is not extended for fees
pub struct TransferProofData {
    pub equality_proof_data: CiphertextCommitmentEqualityProofData,
    pub ciphertext_validity_proof_data: BatchedGroupedCiphertext3HandlesValidityProofData,
    pub range_proof_data: BatchedRangeProofU128Data,
}
