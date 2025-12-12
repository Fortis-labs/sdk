<h1 align="center">
  Fortis sdk
</h1>
<p align="center">
<img width="500" height="394" alt="logo1" src="https://github.com/user-attachments/assets/6e48acf5-e9c7-4435-9ef5-88b1710f848c" />
</p>
<p align="center">
rust sdk for fortis multisig.
</p>
Fortis rust sdk is an extesive toolkit to support multisig proposal workflows on SVM

## Program (Smart contract) Addresses
The Squads Protocol v4 program is deployed to:

- Solana Mainnet-beta: 2vVxSvZcQZTZJWr6jopCBmUZVhGVMg2kX5B6wHnPJpcj
- Solana Devnet: 2vVxSvZcQZTZJWr6jopCBmUZVhGVMg2kX5B6wHnPJpcj

## Responsibility
By interacting with this program, users acknowledge and accept full personal responsibility for any consequences, regardless of their nature. This includes both potential risks inherent to the smart contract, also referred to as program, as well as any losses resulting from user errors or misjudgment.

By using a multisig, it is important to acknowledge certain concepts. Here are some that could be misunderstood by users:

- Loss of Private Keys: If a participant loses their private key, the multisig may not be able to execute transactions if a threshold number of signatures is required.
- Single Point of Failure with Keys: If all keys are stored in the same location or device, a single breach can compromise the multisig.
- Forgetting the Threshold: Misremembering the number of signatures required can result in a deadlock, where funds cannot be accessed.
- No Succession Planning: If keyholders become unavailable (e.g., due to accident, death), without a plan for transition, funds may be locked forever.
- Transfer of funds to wrong address: Funds should always be sent to the multisig vault account, and not the multisig account address. Due to the design of the Squads Protocol program, funds deposited to the multisig account may not be recoverable.
- If the config_authority of a multisig is compromised, an attacker can change multisig settings, potentially reducing the required threshold for transaction execution or instantly being able to remove and add new members.
- If the underlying SVM compatible blockchain undergoes a fork and a user had sent funds to the orphaned chain, the state of the blockchain may not interpret the owner of funds to be original one.
- Users might inadvertently set long or permanent time-locks in their multisig, preventing access to their funds for that period of time.
- Multisig participants might not have enough of the native token of the underlying SVM blockchain to pay for transaction and state fees.


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

Fortis Multisig is composed of the following Accounts and Instructions:
## Accounts

- Treasury: A Fortis treasury account responsible for handling multisig creation.

- Multisig: An account that stores the DAO’s top-level configuration and operations.

- Proposal: An account that stores proposal details, responses, and status.

- Transaction: An account that stores the transaction to be executed on-chain.

- Vault: A vault account for the multisig, acting as the entity that performs actions on behalf of the multisig.
