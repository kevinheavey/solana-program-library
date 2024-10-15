use {
    crate::{
        error::TokenError,
        extension::confidential_transfer::{
            ConfidentialTransferAccount, DecryptableBalance, EncryptedBalance,
        },
    },
    bytemuck::{Pod, Zeroable},
    solana_zk_sdk::{
        encryption::{
            auth_encryption::{AeCiphertext, AeKey},
            elgamal::ElGamalKeypair,
        },
        zk_elgamal_proof_program::proof_data::ZeroCiphertextProofData,
    },
    spl_pod::primitives::PodU64,
    spl_token_confidential_transfer_proof_generation::{
        withdraw::{withdraw_proof_data, WithdrawProofData},
    },
};

/// Confidential transfer extension information needed to construct an
/// `EmptyAccount` instruction.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct EmptyAccountAccountInfo {
    /// The available balance
    pub(crate) available_balance: EncryptedBalance,
}
impl EmptyAccountAccountInfo {
    /// Create the `EmptyAccount` instruction account information from
    /// `ConfidentialTransferAccount`.
    pub fn new(account: &ConfidentialTransferAccount) -> Self {
        Self {
            available_balance: account.available_balance,
        }
    }

    /// Create an empty account proof data.
    pub fn generate_proof_data(
        &self,
        elgamal_keypair: &ElGamalKeypair,
    ) -> Result<ZeroCiphertextProofData, TokenError> {
        let available_balance = self
            .available_balance
            .try_into()
            .map_err(|_| TokenError::MalformedCiphertext)?;

        ZeroCiphertextProofData::new(elgamal_keypair, &available_balance)
            .map_err(|_| TokenError::ProofGeneration)
    }
}

/// Confidential Transfer extension information needed to construct an
/// `ApplyPendingBalance` instruction.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct ApplyPendingBalanceAccountInfo {
    /// The total number of `Deposit` and `Transfer` instructions that have
    /// credited `pending_balance`
    pub(crate) pending_balance_credit_counter: PodU64,
    /// The low 16 bits of the pending balance (encrypted by `elgamal_pubkey`)
    pub(crate) pending_balance_lo: EncryptedBalance,
    /// The high 48 bits of the pending balance (encrypted by `elgamal_pubkey`)
    pub(crate) pending_balance_hi: EncryptedBalance,
    /// The decryptable available balance
    pub(crate) decryptable_available_balance: DecryptableBalance,
}
impl ApplyPendingBalanceAccountInfo {
    /// Create the `ApplyPendingBalance` instruction account information from
    /// `ConfidentialTransferAccount`.
    pub fn new(account: &ConfidentialTransferAccount) -> Self {
        Self {
            pending_balance_credit_counter: account.pending_balance_credit_counter,
            pending_balance_lo: account.pending_balance_lo,
            pending_balance_hi: account.pending_balance_hi,
            decryptable_available_balance: account.decryptable_available_balance,
        }
    }

    /// Return the pending balance credit counter of the account.
    pub fn pending_balance_credit_counter(&self) -> u64 {
        self.pending_balance_credit_counter.into()
    }
}

/// Confidential Transfer extension information needed to construct a `Withdraw`
/// instruction.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct WithdrawAccountInfo {
    /// The available balance (encrypted by `encrypiton_pubkey`)
    pub available_balance: EncryptedBalance,
    /// The decryptable available balance
    pub decryptable_available_balance: DecryptableBalance,
}
impl WithdrawAccountInfo {
    /// Create the `ApplyPendingBalance` instruction account information from
    /// `ConfidentialTransferAccount`.
    pub fn new(account: &ConfidentialTransferAccount) -> Self {
        Self {
            available_balance: account.available_balance,
            decryptable_available_balance: account.decryptable_available_balance,
        }
    }

    fn decrypted_available_balance(&self, aes_key: &AeKey) -> Result<u64, TokenError> {
        let decryptable_available_balance = self
            .decryptable_available_balance
            .try_into()
            .map_err(|_| TokenError::MalformedCiphertext)?;
        aes_key
            .decrypt(&decryptable_available_balance)
            .ok_or(TokenError::AccountDecryption)
    }

    /// Create a withdraw proof data.
    pub fn generate_proof_data(
        &self,
        withdraw_amount: u64,
        elgamal_keypair: &ElGamalKeypair,
        aes_key: &AeKey,
    ) -> Result<WithdrawProofData, TokenError> {
        let current_available_balance = self
            .available_balance
            .try_into()
            .map_err(|_| TokenError::MalformedCiphertext)?;
        let current_decrypted_available_balance = self.decrypted_available_balance(aes_key)?;

        withdraw_proof_data(
            &current_available_balance,
            current_decrypted_available_balance,
            withdraw_amount,
            elgamal_keypair,
        )
        .map_err(|e| -> TokenError { e.into() })
    }

    /// Update the decryptable available balance.
    pub fn new_decryptable_available_balance(
        &self,
        withdraw_amount: u64,
        aes_key: &AeKey,
    ) -> Result<AeCiphertext, TokenError> {
        let current_decrypted_available_balance = self.decrypted_available_balance(aes_key)?;
        let new_decrypted_available_balance = current_decrypted_available_balance
            .checked_sub(withdraw_amount)
            .ok_or(TokenError::InsufficientFunds)?;

        Ok(aes_key.encrypt(new_decrypted_available_balance))
    }
}

/// Confidential Transfer extension information needed to construct a `Transfer`
/// instruction.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct TransferAccountInfo {
    /// The available balance (encrypted by `encrypiton_pubkey`)
    pub available_balance: EncryptedBalance,
    /// The decryptable available balance
    pub decryptable_available_balance: DecryptableBalance,
}
impl TransferAccountInfo {
    /// Create the `Transfer` instruction account information from
    /// `ConfidentialTransferAccount`.
    pub fn new(account: &ConfidentialTransferAccount) -> Self {
        Self {
            available_balance: account.available_balance,
            decryptable_available_balance: account.decryptable_available_balance,
        }
    }
}
