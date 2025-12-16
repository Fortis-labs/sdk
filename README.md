<h1 align="center">
  Fortis sdk
</h1>
<p align="center">
<img width="500" height="394" alt="logo1" src="https://github.com/user-attachments/assets/6e48acf5-e9c7-4435-9ef5-88b1710f848c" />
</p>
<p align="center">
rust sdk for fortis multisig.
</p>
Fortis rust sdk is an extensive toolkit to support multisig proposal workflows on SVM

## Program (Smart contract) Addresses
The program is deployed to:

- Solana Mainnet: ```2Zwsw5UBArhtKwGV3mQpGMaaH4q8hVDwEyzVjMqTyvgi```
- Solana Devnet: ```2Zwsw5UBArhtKwGV3mQpGMaaH4q8hVDwEyzVjMqTyvgi```


## Add fortis 
```console
cargo add fortis_sdk
```

Fortis multisig sdk offers helpers types and methods for various operations 
## Architecture
The high-level architecture of Fortis multisig operations works as follows:

## Multisig Creation
Any member can create a new multisig by choosing a threshold ,members and an optional rent collector.

## Proposal Creation
After the multisig is created, any member can submit a proposal.
A proposal specifies:

the operation (i.e., the on-chain transaction) the member wants to execute,
and a voting deadline.

## Vault Requirements
The vault associated with the multisig must hold the required tokens and accounts needed to perform the proposed operation.

## Voting & Execution
If the proposal reaches the required threshold within the given timeframe, it becomes executable.
Once executed, the proposal is finalized.

## Cleanup
If a proposal is executed or if it fails to reach approval before the deadline, the related accounts can be safely closed.
Any remaining rent is transferred to the rent collector (if one was specified during creation).

## A demonstration of all functionalities can be found [here](https://github.com/Fortis-labs/demo) 

Fortis Multisig is composed of the following Accounts and Instructions:
## Accounts

- Treasury: A Fortis treasury account responsible for handling multisig creation.

- Multisig: An account that stores the DAO’s top-level configuration and operations.

- Proposal: An account that stores proposal details, responses, and status.

- Transaction: An account that stores the transaction to be executed on-chain.

- Vault: A vault account for the multisig, acting as the entity that performs actions on behalf of the multisig.

Below is a minimal example for creating a multisig,transfering sol to vault ,& creating a proposal
```rust
use fortis_sdk::{
    client::{
        get_multisig, multisig_create, proposal_accounts_close, proposal_approve, proposal_create,
        proposal_execute,
    },
    pda::{TREASURY, get_multisig_pda, get_proposal_pda, get_transaction_pda, get_vault_pda},
    state::{
        MultisigCreateAccounts, MultisigCreateArgs, ProposalAccountsCloseAccounts,
        ProposalApproveAccounts, ProposalApproveArgs, ProposalCreateAccounts,
        ProposalExecuteAccounts, VaultTransactionMessage,
    },
};
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::Keypair,
    signer::{EncodableKey, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::transfer as native_transfer;

pub const SYSTEM_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("11111111111111111111111111111111");
use solana_client::nonblocking::rpc_client::RpcClient;
#[tokio::main]
pub async fn main() {
    let kp_path = "PATH_TO_WALLET";
    let cluster = "https://api.devnet.solana.com ".to_string();
    let rpc = RpcClient::new(cluster);

    let bob = Keypair::read_from_file(kp_path).unwrap();
    let alice = Keypair::new();

    let threshold = 1;
    let members = vec![bob.pubkey(), alice.pubkey()];

    let create_key = Keypair::new();
    let multisig_pda = get_multisig_pda(&create_key.pubkey(), None).0;
    let vault_pda = get_vault_pda(&multisig_pda, None).0;
    let multisig_create_ix = multisig_create(
        MultisigCreateAccounts {
            treasury: TREASURY,
            multisig: multisig_pda,
            create_key: create_key.pubkey(),
            creator: bob.pubkey(),
            system_program: SYSTEM_PROGRAM_ID,
        },
        MultisigCreateArgs {
            threshold,
            rent_collector: Some(bob.pubkey()),
            members,
        },
        None,
    );
    let transfer_to_vault_ix = native_transfer(&bob.pubkey(), &vault_pda, 1_000_000);
    println!("Fortis program id: {}", multisig_create_ix.program_id);

    let transaction_index = 1u64;

    let transaction_pda = get_transaction_pda(&multisig_pda, transaction_index, None).0;
    let proposal_pda = get_proposal_pda(&multisig_pda, transaction_index, None).0;

    let proposal_accounts = ProposalCreateAccounts {
        multisig: multisig_pda,
        trasaction: transaction_pda,
        creator: bob.pubkey(),
        proposal: proposal_pda,
        system_program: SYSTEM_PROGRAM_ID,
    };

    let voting_deadline = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        + (86400 * 30); // 30 days

    // Vault action inside the proposal
    let receiver = Keypair::new();
    let vault_transfer_ix = native_transfer(&vault_pda, &receiver.pubkey(), 1_000_000);

    let vault_message =
        VaultTransactionMessage::try_compile(&vault_pda, &[vault_transfer_ix.clone()], &[])
            .expect("Failed to compile vault message");

    let proposal_create_ix = proposal_create(
        proposal_accounts,
        0,              // num_ephemeral_signers
        &vault_message, // transaction message
        voting_deadline,
        None,
    );
    let tx = Transaction::new(
        &[bob.insecure_clone(), create_key.insecure_clone()],
        Message::new(
            &[multisig_create_ix, proposal_create_ix, transfer_to_vault_ix],
            Some(&bob.pubkey()),
        ),
        rpc.get_latest_blockhash().await.unwrap(),
    );
    println!(
        "Transaction 1:\n{:#?}",
        rpc.send_and_confirm_transaction(&tx).await
    );
}
```
## Responsibility
By interacting with this program, users acknowledge and accept full personal responsibility for any consequences, regardless of their nature. This includes both potential risks inherent to the smart contract, also referred to as program, as well as any losses resulting from user errors or misjudgment.

By using a multisig, it is important to acknowledge certain concepts. Here are some that could be misunderstood by users:

- Loss of Private Keys: If a participant loses their private key, the multisig may not be able to execute transactions if a threshold number of signatures is required.
- Single Point of Failure with Keys: If all keys are stored in the same location or device, a single breach can compromise the multisig.
- Forgetting the Threshold: Misremembering the number of signatures required can result in a deadlock, where funds cannot be accessed.
- No Succession Planning: If keyholders become unavailable (e.g., due to accident, death), without a plan for transition, funds may be locked forever.
- Transfer of funds to wrong address: Funds should always be sent to the multisig vault account, and not the multisig account address. Due to the design of the Fortis program, funds deposited to the multisig account may not be recoverable.
- If the config_authority of a multisig is compromised, an attacker can change multisig settings, potentially reducing the required threshold for transaction execution or instantly being able to remove and add new members.
- If the underlying SVM compatible blockchain undergoes a fork and a user had sent funds to the orphaned chain, the state of the blockchain may not interpret the owner of funds to be original one.
- Users might inadvertently set long or permanent time-locks in their multisig, preventing access to their funds for that period of time.
- Multisig participants might not have enough of the native token of the underlying SVM blockchain to pay for transaction and state fees.




