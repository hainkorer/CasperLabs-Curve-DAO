# CasperLabs-LiquidNFT

Implementation of `Liquid Base`, `Liquid Helper` and `Liquid Locker` Contract for the CasperLabs platform.

## Steps

There are 3 contracts with 1 crate in this folder

1. Liquid Base Crate
2. Liquid Helper Contract
3. Liquid Locker Contract
4. ERC20 Contract

## Table of contents

- [Interacting with the contract](#interacting-with-the-contract)
  - [Install the prerequisites](#install-the-prerequisites)
  - [Creating Keys](#creating-keys)
  - [Usage](#usage)
    - [Install](#install)
    - [Build Individual Smart Contract](#build-individual-smart-contract)
    - [Build All Smart Contracts](#build-all-smart-contracts)
    - [Individual Test Cases](#individual-test-cases)
    - [All Test Cases](#all-test-cases)
  - [Known contract hashes](#known-contract-hashes)

### Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

```bash
# Update package repositories
sudo apt update
# Install the command-line JSON processor
sudo apt install jq -y
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#Install the nightly version (by default stable toolchain is installed)
rustup install nightly
#Check that nightly toolchain version is installed(this will list stable and nightly versions)
rustup toolchain list
#Set rust nightly as default
rustup default nightly
# Install wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown
#rust Version
rustup --version
#Install Cmake
 sudo apt-get -y install cmake
Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html
#cmake Version
cmake --version
#Installing the Casper Crates
cargo install cargo-casper
# Add Casper repository
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
sudo apt-key add casper-repo-pubkey.ascr
sudo apt update
# Install the Casper client software
Install Casper-client
cargo +nightly install casper-client
# To check Casper Client Version
Casper-client --version
# Commands for help
casper-client --help
casper-client <command> --help
```

### Creating Keys

```bash
# Create keys
casper-client keygen <TARGET DIRECTORY>
```

### Usage

To run the Contracts make sure you are in the root folder.

#### Install

Make sure `wasm32-unknown-unknown` is installed.

```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

#### Build individual Smart Contract

You can run this commands to build individual smart contracts.

```
make build-contract-liquid-helper
make build-contract-liquid-locker
```

#### Test individual Smart Contract

You can run this commands to build individual smart contracts.

```
make test-liquid-helper
make test-liquid-locker
```

#### Run ALL Smart Contracts

Run this command to build & test all smart contract.

```
make all
```

### Deploying Liquid Helper contract manually

If you need to deploy the `Liquid Helper` contract manually you need to pass some parameters. Following is the command to deploy the `Liquidity Helper contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="LiquidHelper-entry-point-methods"></a>

Following are the LiquidityHelper's entry point methods.

- #### get_tokens <a id="LiquidHelper-get-tokens"></a>
  Returns IDs of NFTs being held.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `Vec<U256>`.

- #### ownerless_locker <a id="LiquidHelper-ownerless-locker"></a>
  Returns true if owner is zero address.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `bool`.

- #### floor_not_reached <a id="LiquidHelper-floor-not-reached"></a>
  Returns true if contributions have not reached min asked.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `bool`.

- #### not_single_provider <a id="LiquidHelper-not-single-provider"></a>
  Returns true if the provider address is not the single provider.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| check_address  | Key  |

This method **returns** `bool`.

- #### reached_total <a id="LiquidHelper-reached-total"></a>
  Returns true if the contributor will reach the ceiling asked with the provided token amount.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| contributor    | Key  |
| token_amount   | U256 |

This method **returns** `bool`.

- #### missed_activate <a id="LiquidHelper-missed-activate"></a>
  Returns true if locker has not been enabled within 7 days after contribution phase.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `bool`.

- #### missed_deadline <a id="LiquidHelper-missed-deadline"></a>
  Returns true if owner has not paid back within 7 days of last payment.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `bool`.

- #### payment_time_not_set <a id="LiquidHelper-payment-time-not-set"></a>
  Returns true if nextDueTime is 0, mean it has not been initialized (unix timestamp).

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `bool`.

- #### below_floor_asked <a id="LiquidHelper-below-floor-asked"></a>
  Returns true total collected is below the min asked.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `bool`.

- #### contribution_phase <a id="LiquidHelper-contribution-phase"></a>
  Returns true if contract is in contribution phase time window.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `bool`.

- #### payback_timestamp <a id="LiquidHelper-payback-timestamp"></a>
  Returns final due time of loan.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `U256`.

- #### starting_timestamp <a id="LiquidHelper-starting-timestamp"></a>
  Returns approximate time the loan will/did start.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `U256`.

- #### liquidate_to <a id="LiquidHelper-liquidate-to"></a>
  Returns address to transfer NFT to in event of liquidation.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `Key`.

- #### time_since <a id="LiquidHelper-time-since"></a>
  Returns calc of time since a certain timestamp to block timestamp.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `U256`.

### Deploying Liquid Locker contract manually

If you need to deploy the `Liquid Locker` contract manually you need to pass some parameters. Following is the command to deploy the `Liquidity Locker contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"
    --session-arg="contract_name:string='contract_name'"
    --session-arg="contract_name:string='contract_name'"
    --session-arg="trustee_multisig:Key='trustee-multisig-hash'"
    --session-arg="payment_token:Key='payment-token-hash'"
```

## Entry Point methods <a id="LiquidLocker-entry-point-methods"></a>

Following are the LiquidityLocker's entry point methods.

- #### initialize <a id="LiquidLocker-initialize"></a>
  This is a call made by the constructor to set up variables on a new locker. This is essentially equivalent to a constructor, but for our gas saving cloning operation instead. This may also be used in locker-reuse in version 2.

Following is the table of parameters.

| Parameter Name | Type      |
| -------------- | --------- |
| token_id       | Vec<U256> |
| token_address  | Key       |
| token_owner    | Key       |
| floor_asked    | U256      |
| total_asked    | U256      |
| payment_time   | U256      |
| payment_rate   | U256      |

This method **returns** nothing.

- #### liquidate_locker <a id="LiquidLocker-liquidate-locker"></a>
  If the owner has missed payments by 7 days this call will transfer the NFT to either the singleProvider address or the trusted multisig to be auctioned.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### claim_interest_single <a id="LiquidLocker-claim-interest-single"></a>
  Claim payed back tokens as a single contributor.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### claim_interest_public <a id="LiquidLocker-claim-interest-public"></a>
  Claim payed back tokens as with multiple contributors. We need 2 functions because we cannot wipe all the contributions of users before someone became the sole contributor.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### decrease_payment_time <a id="LiquidLocker-decrease-payment-time"></a>
  During the contribution phase, the owner can decrease the duration of the loan. The owner can only decrease the loan to a shorter duration, he cannot make it longer once the contribution phase has started.

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| new_payment_rate | U256 |

This method **returns** nothing.

- #### increase_payment_rate <a id="LiquidLocker-increase-payment-rate"></a>
  During the contribution phase, the owner can increase the rate they will pay for the loan. The owner can only increase the rate to make the deal better for contributors, he cannot decrease it.

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| new_payment_rate | U256 |

This method **returns** nothing.

- #### enable_locker <a id="LiquidLocker-enable-locker"></a>
  If the floor is reached early. The owner can also prepay an amount to pay off some of the earnings at enable time. The locker owner owes the earnings linearly until the end, then all of the actual loan plus any penalties are due at the end.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| prepay_amount  | U256 |

This method **returns** nothing.

- #### disable_locker <a id="LiquidLocker-disable-locker"></a>
  If the floor asked was not reached during contributions, this function will return the nft to the owner and allow all the contributors to claim their funds back.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### rescue_locker <a id="LiquidLocker-rescue-locker"></a>
  There are a couple edge cases with extreme payment rates that cause enableLocker to revert. These are never callable on our UI and doing so would require a manual transaction. This function will disable a locker in this senario, allow contributors to claim their money and transfer the NFT back to the owner. Only the team multisig has permission to do this.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### refund_due_disabled <a id="LiquidLocker-refund-due-disabled"></a>
  Allow users to claim funds when a locker is disabled.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| refund_address | Key  |

This method **returns** nothing.

- #### refund_due_single <a id="LiquidLocker-refund-due-single"></a>
  Allow users to claim funds when a someone kicks them out to become the single provider.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| refund_address | Key  |

This method **returns** nothing.

- #### donate_funds <a id="LiquidLocker-donate-funds"></a>
  Someone can add funds to the locker and they will be split among the contributors. This does not count as a payment on the loan.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| donation_amount | U256 |

This method **returns** nothing.

- #### pay_back_funds <a id="LiquidLocker-pay-back-funds"></a>
  Locker owner can payback funds. Penalties are given if the owner does not pay the earnings linearally over the loan duration. If the owner pays back the earnings, loan amount, and penalties aka fully pays off the loan they will be transfered their nft back.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| payment_amount | U256 |

This method **returns** nothing.

- #### calculate_epoch <a id="LiquidLocker-calculate-epoch"></a>
  Calculate how many sends should be added before the next payoff is due based on payment amount.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| total_value    | U256 |
| payment_time   | U256 |
| payment_rate   | U256 |

This method **returns** `U256`.

- #### calculate_paybacks <a id="LiquidLocker-calculate-paybacks"></a>
  Calulate how much the usage fee takes off a payments, and how many tokens are due per second of loan (epochPayback is amount of tokens to extend loan by 1 second. Only need to pay off earnings).

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| total_value    | U256 |
| payment_time   | U256 |
| payment_rate   | U256 |

This method **returns** `(U256, U256, U256)`.

- #### get_late_days <a id="LiquidLocker-get-late-days"></a>
  Helper for the days math of calcualte penalties. Returns +1 per day before the 4th day and +2 for each day after the 4th day.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** `U256`.

- #### penalty_amount <a id="LiquidLocker-penalty-amount"></a>
  Public pure accessor for get_penalty_amount.

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| total_collected  | U256 |
| late_days_amount | U256 |

This method **returns** `U256`.

- #### penalty_amount <a id="LiquidLocker-penalty-amount"></a>
  Public users can add tokens to the pool to be used for the loan. The contributions for each user along with the total are recorded for splitting funds later. If a user contributes up to the maximum asked on a loan, they will become the sole provider (See users_increase and reached_total for functionality on becoming the sole provider). The sole provider will receive the token instead of the trusted multisig in the case if a liquidation.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| token_amount   | U256 |
| token_holder   | Key  |

This method **returns** `(U256, U256)`.
