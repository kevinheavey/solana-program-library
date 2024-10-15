use {
    solana_zk_sdk::{
        zk_elgamal_proof_program::proof_data::{
            BatchedGroupedCiphertext2HandlesValidityProofData,
            BatchedGroupedCiphertext3HandlesValidityProofData, BatchedRangeProofU256Data,
            CiphertextCommitmentEqualityProofData, PercentageWithCapProofData,
        },
    },
};

/// The proof data required for a confidential transfer instruction when the
/// mint is extended for fees
pub struct TransferWithFeeProofData {
    pub equality_proof_data: CiphertextCommitmentEqualityProofData,
    pub transfer_amount_ciphertext_validity_proof_data:
        BatchedGroupedCiphertext3HandlesValidityProofData,
    pub percentage_with_cap_proof_data: PercentageWithCapProofData,
    pub fee_ciphertext_validity_proof_data: BatchedGroupedCiphertext2HandlesValidityProofData,
    pub range_proof_data: BatchedRangeProofU256Data,
}
