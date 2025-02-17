<h1>Anchor Vault Program</h1>
This is an Anchor-based Solana program that implements a simple vault system. Users can initialize a vault, deposit funds, withdraw funds, and close the vault.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/whitecometx/Anchor_Vault
   cd Anchor_Vault
   ```

2. Install the required dependencies:
   ```bash
   anchor build
   ```

3. Ensure your Solana wallet is set up and funded on the devnet.

## Project Structure
The project consists of several key files:

| File                  | Description                                                   |
|-----------------------|---------------------------------------------------------------|
| `lib.rs`              | Contains the main logic of the vault program, including methods for initialization, deposits, withdrawals, and closing accounts. |
| `anchor_vault.ts`    | Contains tests for the vault program using the Anchor testing framework. |

Features:

1. Initialize a personal vault

2. Deposit SOL into the vault

3. Withdraw SOL from the vault

4. Close the vault and retrieve remaining funds.

<h2>Account Structures</h2>
<h3>VaultState</h3>
Stores the bump seeds for the vault and state PDAs:

vault_bump: u8

state_bump: u8

<h3>Instructions</h3>

Initialize:
Creates a new vault for the user.

Deposit:
Allows users to deposit SOL into their vault.

Withdraw:
Enables users to withdraw SOL from their vault.

Close:
Closes the vault, transferring any remaining funds to the user.
