use solana_sdk::pubkey;
pub const SEED_PREFIX: &[u8] = b"multisig";
pub const SEED_MULTISIG: &[u8] = b"multisig";
pub const SEED_PROPOSAL: &[u8] = b"proposal";
pub const SEED_TRANSACTION: &[u8] = b"transaction";
pub const SEED_VAULT: &[u8] = b"vault";
pub const SEED_EPHEMERAL_SIGNER: &[u8] = b"ephemeral_signer";

pub const FORTIS_PROGRAM_ID: Pubkey = pubkey!("2vVxSvZcQZTZJWr6jopCBmUZVhGVMg2kX5B6wHnPJpcj");
use solana_sdk::pubkey::Pubkey;

pub fn get_multisig_pda(create_key: &Pubkey, program_id: Option<&Pubkey>) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[SEED_PREFIX, SEED_MULTISIG, create_key.to_bytes().as_ref()],
        program_id.unwrap_or(&FORTIS_PROGRAM_ID),
    )
}

pub fn get_vault_pda(multisig_pda: &Pubkey, program_id: Option<&Pubkey>) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[SEED_PREFIX, multisig_pda.to_bytes().as_ref(), SEED_VAULT],
        program_id.unwrap_or(&FORTIS_PROGRAM_ID),
    )
}

pub fn get_transaction_pda(
    multisig_pda: &Pubkey,
    transaction_index: u64,
    program_id: Option<&Pubkey>,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            SEED_PREFIX,
            multisig_pda.to_bytes().as_ref(),
            SEED_TRANSACTION,
            transaction_index.to_le_bytes().as_ref(),
        ],
        program_id.unwrap_or(&FORTIS_PROGRAM_ID),
    )
}

pub fn get_proposal_pda(
    multisig_pda: &Pubkey,
    transaction_index: u64,
    program_id: Option<&Pubkey>,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            SEED_PREFIX,
            multisig_pda.to_bytes().as_ref(),
            SEED_TRANSACTION,
            &transaction_index.to_le_bytes(),
            SEED_PROPOSAL,
        ],
        program_id.unwrap_or(&FORTIS_PROGRAM_ID),
    )
}

pub fn get_ephemeral_signer_pda(
    transaction_pda: &Pubkey,
    ephemeral_signer_index: u8,
    program_id: Option<&Pubkey>,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            SEED_PREFIX,
            &transaction_pda.to_bytes(),
            SEED_EPHEMERAL_SIGNER,
            &ephemeral_signer_index.to_le_bytes(),
        ],
        program_id.unwrap_or(&FORTIS_PROGRAM_ID),
    )
}
