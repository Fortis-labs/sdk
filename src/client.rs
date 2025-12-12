use std::vec;

use solana_client::nonblocking::rpc_client::RpcClient;

use crate::state::{
    Error, Multisig, MultisigCreateAccounts, MultisigCreateArgs, ProposalAccountsCloseAccounts,
    ProposalApproveAccounts, ProposalApproveArgs, ProposalCreateAccounts, ProposalCreateArgs,
    ProposalExecuteAccounts, VaultTransaction, VaultTransactionMessage,
};
use solana_message::AddressLookupTableAccount;

use crate::error::ClientError;
use crate::pda::{get_vault_pda, FORTIS_PROGRAM_ID};
use crate::{vault_transaction, ClientResult};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

/// Gets a `Multisig` account from the chain.
pub async fn get_multisig(rpc_client: &RpcClient, multisig_key: &Pubkey) -> ClientResult<Multisig> {
    let multisig_account = rpc_client.get_account(multisig_key).await?;

    let multisig: Multisig = borsh::from_slice(&mut multisig_account.data.as_slice())
        .map_err(|_| ClientError::DeserializationError)?;

    Ok(multisig)
}

pub fn multisig_create(
    accounts: MultisigCreateAccounts,
    args: MultisigCreateArgs,
    program_id: Option<Pubkey>,
) -> Instruction {
    let mut data = vec![0];
    data.extend_from_slice(&args.to_vec());
    Instruction {
        accounts: accounts.to_account_metas(),
        data: data,
        program_id: program_id.unwrap_or(FORTIS_PROGRAM_ID),
    }
}

pub fn proposal_create(
    accounts: ProposalCreateAccounts,
    num_ephemeral_signers: u8,
    message: &VaultTransactionMessage,
    voting_deadline: i64,
    program_id: Option<Pubkey>,
) -> Instruction {
    let args = ProposalCreateArgs {
        ephemeral_signers: num_ephemeral_signers,
        voting_deadline,
        transaction_message: borsh::to_vec(message).unwrap(),
    };
    let mut data = vec![1];
    data.extend_from_slice(&borsh::to_vec(&args).unwrap());
    Instruction {
        accounts: accounts.to_account_metas(),
        data,
        program_id: program_id.unwrap_or(FORTIS_PROGRAM_ID),
    }
}
pub fn proposal_approve(
    accounts: ProposalApproveAccounts,
    args: ProposalApproveArgs,
    program_id: Option<Pubkey>,
) -> Instruction {
    let mut data = vec![2];
    data.extend_from_slice(&borsh::to_vec(&args).unwrap());
    Instruction {
        accounts: accounts.to_account_metas(),
        data: data,
        program_id: program_id.unwrap_or(FORTIS_PROGRAM_ID),
    }
}

pub async fn proposal_execute(
    transaction_account_data: &[u8],
    accounts: ProposalExecuteAccounts,
    address_lookup_table_accounts: &[AddressLookupTableAccount],
    program_id: Option<Pubkey>,
) -> ClientResult<Instruction> {
    let vault_transaction: VaultTransaction = borsh::from_slice(transaction_account_data).unwrap();
    let program_id = program_id.unwrap_or(FORTIS_PROGRAM_ID);

    let vault_pda = get_vault_pda(&accounts.multisig, Some(&program_id)).0;

    let accounts_for_execute = vault_transaction
        .message
        .get_accounts_for_execute(
            &vault_pda,
            &accounts.transaction,
            &address_lookup_table_accounts,
            vault_transaction.ephemeral_signer_bumps.len() as u8,
            &program_id,
        )
        .map_err(|err| match err {
            Error::InvalidAddressLookupTableAccount => {
                ClientError::InvalidAddressLookupTableAccount
            }
            Error::InvalidTransactionMessage => ClientError::InvalidTransactionMessage,
        })?;

    let mut accounts = accounts.to_account_metas();
    // Append the accounts required for executing the inner instructions.
    accounts.extend(accounts_for_execute.into_iter());

    Ok(Instruction {
        accounts,
        data: vec![3],
        program_id,
    })
}

pub fn proposal_accounts_close(
    accounts: ProposalAccountsCloseAccounts,
    program_id: Option<Pubkey>,
) -> Instruction {
    Instruction {
        accounts: accounts.to_account_metas(),
        data: vec![4],
        program_id: program_id.unwrap_or(FORTIS_PROGRAM_ID),
    }
}

pub mod utils {

    use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
    pub trait IntoAccountMetas {
        fn into_account_metas(self, program_id: Pubkey) -> Vec<AccountMeta>;
    }
}
