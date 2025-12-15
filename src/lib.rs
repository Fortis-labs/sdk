use crate::pda::get_ephemeral_signer_pda;
pub use solana_client;
pub mod client;
pub mod pda;
pub mod vault_transaction;

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ClientError {
        #[error(transparent)]
        Client(#[from] solana_client::client_error::ClientError),
        #[error("Failed to deserialize account data")]
        DeserializationError,
        #[error("Invalid AddressLookupTableAccount")]
        InvalidAddressLookupTableAccount,
        #[error("Invalid TransactionMessage")]
        InvalidTransactionMessage,
    }
}

pub type ClientResult<T> = Result<T, error::ClientError>;

pub mod state {
    use crate::vault_transaction::compiled_keys::CompiledKeys;
    pub use solana_client;

    use solana_message::AddressLookupTableAccount;
    use solana_message::{AccountKeys, CompileError};
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::pubkey::Pubkey;

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Invalid AddressLookupTableAccount")]
        InvalidAddressLookupTableAccount,
        #[error("Invalid TransactionMessage")]
        InvalidTransactionMessage,
    }
    pub struct MultisigCreateAccounts {
        pub treasury: Pubkey,
        pub multisig: Pubkey,
        pub create_key: Pubkey,
        pub creator: Pubkey,
        pub system_program: Pubkey,
    }
    impl MultisigCreateAccounts {
        pub fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta::new(self.treasury, false),
                AccountMeta::new(self.multisig, false),
                AccountMeta::new_readonly(self.create_key, true),
                AccountMeta::new(self.creator, true),
                AccountMeta::new_readonly(self.system_program, false),
            ]
        }
    }
    //   #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct MultisigCreateArgs {
        /// The number of signatures required to execute a transaction.
        pub threshold: u16,
        ///rent collector
        pub rent_collector: Option<Pubkey>,
        /// The members of the multisig.
        pub members: Vec<Pubkey>,
    }
    impl MultisigCreateArgs {
        pub fn to_vec(&self) -> Vec<u8> {
            let mut data = Vec::new();

            // 1️⃣ threshold
            data.extend_from_slice(&self.threshold.to_le_bytes());

            // 2️⃣ rent_collector Option
            match &self.rent_collector {
                Some(pubkey) => {
                    data.push(1); // Some tag
                    data.extend_from_slice(pubkey.as_ref());
                }
                None => {
                    data.push(0); // None tag
                }
            }

            // 3️⃣ members length as u32 (little endian)
            let members_len = self.members.len() as u32;
            data.extend_from_slice(&members_len.to_le_bytes());

            // 4️⃣ members bytes
            for member in &self.members {
                data.extend_from_slice(member.as_ref());
            }

            data
        }
    }
    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct Multisig {
        /// Key that is used to seed the multisig PDA.
        pub create_key: Pubkey,
        /// The address where the rent for the accounts related to executed, rejected, or cancelled
        /// transactions can be reclaimed. If set to `None`, the rent reclamation feature is turned off.
        pub rent_collector: Pubkey,
        /// Last transaction index. 0 means no transactions have been created.
        pub transaction_index: u64,
        /// Threshold for signatures.
        pub threshold: u16,
        /// Bump for the multisig PDA seed.
        pub bump: u8,
        ///members
        pub members: Vec<Pubkey>,
    }
    pub struct ProposalCreateAccounts {
        pub multisig: Pubkey,
        pub trasaction: Pubkey,
        pub creator: Pubkey,
        pub proposal: Pubkey,
        pub system_program: Pubkey,
    }
    impl ProposalCreateAccounts {
        pub fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta::new(self.multisig, false),
                AccountMeta::new(self.trasaction, false),
                AccountMeta::new(self.creator, true),
                AccountMeta::new(self.proposal, false),
                AccountMeta::new_readonly(self.system_program, false),
            ]
        }
    }
    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct ProposalCreateArgs {
        pub ephemeral_signers: u8,
        pub voting_deadline: i64, //deadline to vote ,else will be conidersed rejected
        pub transaction_message: Vec<u8>,
    }
    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct Proposal {
        /// The multisig this belongs to.
        pub multisig: Pubkey,
        /// Index of the multisig transaction this proposal is associated with.
        pub transaction_index: u64,
        //last updated timestamp
        pub timestamp: i64,
        //deadline for voting
        pub deadline: i64,
        /// The status of the transaction.
        pub status: u8,
        /// PDA bump.
        pub bump: u8,
        /// Keys that have approved/signed.
        pub approved: Vec<Pubkey>,
    }

    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct ProposalApproveArgs {}
    pub struct ProposalApproveAccounts {
        pub multisig: Pubkey,
        pub proposal: Pubkey,
        pub member: Pubkey,
    }
    impl ProposalApproveAccounts {
        pub fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta::new_readonly(self.multisig, false),
                AccountMeta::new(self.proposal, false),
                AccountMeta::new(self.member, true),
            ]
        }
    }
    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct ProposallExecuteArgs {}
    pub struct ProposalExecuteAccounts {
        pub multisig: Pubkey,
        pub proposal: Pubkey,
        pub transaction: Pubkey,
        pub member: Pubkey,
    }
    impl ProposalExecuteAccounts {
        pub fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta::new_readonly(self.multisig, false),
                AccountMeta::new(self.proposal, false),
                AccountMeta::new(self.transaction, false),
                AccountMeta::new(self.member, true),
            ]
        }
    }
    pub struct ProposalAccountsCloseAccounts {
        pub multisig: Pubkey,
        pub proposal: Pubkey,
        pub transaction: Pubkey,
        pub rent_collector: Pubkey,
        pub system_program: Pubkey,
    }
    impl ProposalAccountsCloseAccounts {
        pub fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta::new_readonly(self.multisig, false),
                AccountMeta::new(self.proposal, false),
                AccountMeta::new(self.transaction, false),
                AccountMeta::new(self.rent_collector, true),
                AccountMeta::new_readonly(self.system_program, false),
            ]
        }
    }

    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct VaultTransactionMessage {
        /// The number of signer pubkeys in the account_keys vec.
        pub num_signers: u8,
        /// The number of writable signer pubkeys in the account_keys vec.
        pub num_writable_signers: u8,
        /// The number of writable non-signer pubkeys in the account_keys vec.
        pub num_writable_non_signers: u8,
        /// Unique account pubkeys (including program IDs) required for execution of the tx.
        /// The signer pubkeys appear at the beginning of the vec, with writable pubkeys first, and read-only pubkeys following.
        /// The non-signer pubkeys follow with writable pubkeys first and read-only ones following.
        /// Program IDs are also stored at the end of the vec along with other non-signer non-writable pubkeys:
        ///
        /// ```plaintext
        /// [pubkey1, pubkey2, pubkey3, pubkey4, pubkey5, pubkey6, pubkey7, pubkey8]
        ///  |---writable---|  |---readonly---|  |---writable---|  |---readonly---|
        ///  |------------signers-------------|  |----------non-singers-----------|
        /// ```
        pub account_keys: Vec<Pubkey>,
        /// List of address table lookups used to load additional accounts
        /// for this transaction.
        pub address_table_lookups: Vec<MessageAddressTableLookup>,
        /// List of instructions making up the tx.
        pub instructions: Vec<CompiledInstruction>,
    }
    impl VaultTransactionMessage {
        /// Returns true if the account at the specified index is a part of static `account_keys` and was requested to be writable.
        pub fn is_static_writable_index(&self, key_index: usize) -> bool {
            let num_account_keys = self.account_keys.len();
            let num_signers = usize::from(self.num_signers);
            let num_writable_signers = usize::from(self.num_writable_signers);
            let num_writable_non_signers = usize::from(self.num_writable_non_signers);

            if key_index >= num_account_keys {
                // `index` is not a part of static `account_keys`.
                return false;
            }

            if key_index < num_writable_signers {
                // `index` is within the range of writable signer keys.
                return true;
            }

            if key_index >= num_signers {
                // `index` is within the range of non-signer keys.
                let index_into_non_signers = key_index.saturating_sub(num_signers);
                // Whether `index` is within the range of writable non-signer keys.
                return index_into_non_signers < num_writable_non_signers;
            }

            false
        }

        /// Returns true if the account at the specified index was requested to be a signer.
        pub fn is_signer_index(&self, key_index: usize) -> bool {
            key_index < usize::from(self.num_signers)
        }
        /// This implementation is mostly a copy-paste from `solana_program::message::v0::Message::try_compile()`,
        /// but it constructs a `TransactionMessage` meant to be passed to `vault_transaction_create`.
        pub fn try_compile(
            vault_key: &Pubkey,
            instructions: &[Instruction],
            address_lookup_table_accounts: &[AddressLookupTableAccount],
        ) -> Result<VaultTransactionMessage, CompileError> {
            let mut compiled_keys = CompiledKeys::compile(instructions, Some(*vault_key));

            let mut address_table_lookups = Vec::with_capacity(address_lookup_table_accounts.len());
            let mut loaded_addresses_list = Vec::with_capacity(address_lookup_table_accounts.len());
            for lookup_table_account in address_lookup_table_accounts {
                if let Some((lookup, loaded_addresses)) =
                    compiled_keys.try_extract_table_lookup(lookup_table_account)?
                {
                    address_table_lookups.push(lookup);
                    loaded_addresses_list.push(loaded_addresses);
                }
            }

            let (header, static_keys) = compiled_keys.try_into_message_components()?;
            let dynamic_keys = loaded_addresses_list.into_iter().collect();
            let account_keys = AccountKeys::new(&static_keys, Some(&dynamic_keys));
            let instructions = account_keys.try_compile_instructions(instructions)?;

            let num_static_keys: u8 = static_keys
                .len()
                .try_into()
                .map_err(|_| CompileError::AccountIndexOverflow)?;

            Ok(VaultTransactionMessage {
                num_signers: header.num_required_signatures,
                num_writable_signers: header.num_required_signatures
                    - header.num_readonly_signed_accounts,
                num_writable_non_signers: num_static_keys
                    - header.num_required_signatures
                    - header.num_readonly_unsigned_accounts,
                account_keys: static_keys.into(),
                instructions: instructions
                    .into_iter()
                    .map(|ix| CompiledInstruction {
                        program_id_index: ix.program_id_index,
                        accounts: ix.accounts.into(),
                        data: ix.data.into(),
                    })
                    .collect::<Vec<CompiledInstruction>>()
                    .into(),
                address_table_lookups: address_table_lookups
                    .into_iter()
                    .map(|lookup| MessageAddressTableLookup {
                        account_key: lookup.account_key,
                        writable_indexes: lookup.writable_indexes.into(),
                        readonly_indexes: lookup.readonly_indexes.into(),
                    })
                    .collect::<Vec<MessageAddressTableLookup>>()
                    .into(),
            })
        }

        pub fn get_accounts_for_execute(
            &self,
            vault_pda: &Pubkey,
            transaction_pda: &Pubkey,
            address_lookup_table_accounts: &[solana_message::AddressLookupTableAccount],
            num_ephemeral_signers: u8,
            program_id: &Pubkey,
        ) -> Result<Vec<AccountMeta>, Error> {
            let ephemeral_signer_pdas: Vec<Pubkey> = (0..num_ephemeral_signers)
                .into_iter()
                .map(|ephemeral_signer_index| {
                    crate::get_ephemeral_signer_pda(
                        transaction_pda,
                        ephemeral_signer_index,
                        Some(program_id),
                    )
                    .0
                })
                .collect();

            // region: -- address_lookup_tables map --

            let address_lookup_tables = address_lookup_table_accounts
                .into_iter()
                .map(|alt| (alt.key, alt))
                .collect::<std::collections::HashMap<_, _>>();

            // endregion: -- address_lookup_tables map --

            // region: -- Account Metas --

            // First go the lookup table accounts used by the transaction. They are needed for on-chain validation.
            let lookup_table_account_metas = address_lookup_table_accounts
                .iter()
                .map(|alt| AccountMeta {
                    pubkey: alt.key,
                    is_writable: false,
                    is_signer: false,
                })
                .collect::<Vec<_>>();

            // Then come static account keys included into the message.
            let static_account_metas = self
                .account_keys
                .iter()
                .enumerate()
                .map(|(index, &pubkey)| {
                    AccountMeta {
                        pubkey,
                        is_writable: self.is_static_writable_index(index),
                        // NOTE: vaultPda and ephemeralSignerPdas cannot be marked as signers,
                        // because they are PDAs and hence won't have their signatures on the transaction.
                        is_signer: self.is_signer_index(index)
                            && &pubkey != vault_pda
                            && !ephemeral_signer_pdas.contains(&pubkey),
                    }
                })
                .collect::<Vec<_>>();

            // And the last go the accounts that will be loaded with address lookup tables.
            let loaded_account_metas = self
                .address_table_lookups
                .iter()
                .map(|lookup| {
                    let lookup_table_account = address_lookup_tables
                        .get(&lookup.account_key)
                        .ok_or(Error::InvalidAddressLookupTableAccount)?;

                    // For each lookup, fist list writable, then readonly account metas.
                    lookup
                        .writable_indexes
                        .iter()
                        .map(|&index| {
                            let pubkey = lookup_table_account
                                .addresses
                                .get(index as usize)
                                .ok_or(Error::InvalidAddressLookupTableAccount)?
                                .to_owned();

                            Ok(AccountMeta {
                                pubkey,
                                is_writable: true,
                                is_signer: false,
                            })
                        })
                        .chain(lookup.readonly_indexes.iter().map(|&index| {
                            let pubkey = lookup_table_account
                                .addresses
                                .get(index as usize)
                                .ok_or(Error::InvalidAddressLookupTableAccount)?
                                .to_owned();

                            Ok(AccountMeta {
                                pubkey,
                                is_writable: false,
                                is_signer: false,
                            })
                        }))
                        .collect::<Result<Vec<_>, Error>>()
                })
                .collect::<Result<Vec<_>, Error>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            // endregion: -- Account Metas --

            Ok([
                lookup_table_account_metas,
                static_account_metas,
                loaded_account_metas,
            ]
            .concat())
        }
    }
    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct MessageAddressTableLookup {
        pub account_key: solana_message::Address,
        pub writable_indexes: Vec<u8>,
        pub readonly_indexes: Vec<u8>,
    }
    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct CompiledInstruction {
        pub program_id_index: u8,
        pub accounts: Vec<u8>,
        pub data: Vec<u8>,
    }
    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub struct VaultTransaction {
        /// The multisig this belongs to.
        pub multisig: Pubkey,
        /// Member of the Multisig who submitted the transaction.
        pub creator: Pubkey,
        /// Index of this transaction within the multisig.
        pub index: u64,
        /// bump for the transaction seeds.
        pub bump: u8,
        /// Derivation bump of the vault PDA this transaction belongs to.
        pub vault_bump: u8,
        pub ephemeral_signer_bumps: Vec<u8>,
        /// data required for executing the transaction.
        pub message: VaultTransactionMessage,
    }
}
