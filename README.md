# exchange
Development of hybrid market place technology for carbon credit tokenization

We take a look at the industry standard open source [Toucan Base Carbon Tonne](https://toucan.earth/home/) contract from 2021 Solidity code and discuss its elements and chain-agnostic implementation as well as Python or Typescript deployment like [Opshin](https://github.com/OpShin/opshin), [MeshJS](https://meshjs.dev/).

We examined the Solidity smart contract of the BCT carbon credit token. This review looks into its structure and functionality to create a similar template in chain-agnostic Rust-based smart contract language.

Here is the content of the BCT Solidity smart contract file with comments, imports and boilerplate code removed:

~~~
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract BasicCarbonOffsets is ERC20 {
    address public admin;
    mapping(address => uint256) public balances;

    constructor(uint256 initialSupply) ERC20("BasicCarbonOffsets", "BCO") {
        _mint(msg.sender, initialSupply);
        admin = msg.sender;
    }

    function mint(address to, uint256 amount) external {
        require(msg.sender == admin, "only admin can mint");
        _mint(to, amount);
    }

    function burn(uint256 amount) external {
        _burn(msg.sender, amount);
    }
}
~~~

This Solidity contract uses the OpenZeppelin library for ERC20 token implementation. It defines a simple ERC20 token with minting and burning functionalities controlled by an admin.

### Universal Smart Contract

Below is the equivalent template for a smart contract that allows first class abstractions and static typing. Please note that Cardano and other non-EVM languages and its smart contract models are different from Solidity.

~~~
module BasicCarbonOffsets::BCO {
    use 0x1::Signer;
    use 0x1::Account;
    use 0x1::Coin;
    use 0x1::Event;
    use 0x1::Vector;
    
    struct MintEvent has copy, drop, store {
        amount: u64,
        to: address,
    }

    struct BurnEvent has copy, drop, store {
        amount: u64,
        from: address,
    }

    struct BCO has key {
        balance: Coin.T<BCO>,
        admin: address,
    }

    public fun initialize(account: &signer, initial_supply: u64) {
        let admin = Signer.address_of(account);
        let coin = Coin.mint<BCO>(initial_supply);
        let bco = BCO {
            balance: coin,
            admin: admin,
        };
        move_to(account, bco);
    }

    public fun mint(account: &signer, to: address, amount: u64) {
        let bco = borrow_global_mut<BCO>(Signer.address_of(account));
        assert!(Signer.address_of(account) == bco.admin, 1);
        let coin = Coin.mint<BCO>(amount);
        Coin.deposit(to, coin);
        Event::emit_event<MintEvent>(&bco.mint_events, MintEvent { amount, to });
    }

    public fun burn(account: &signer, amount: u64) {
        let bco = borrow_global_mut<BCO>(Signer.address_of(account));
        let from = Signer.address_of(account);
        let coin = Coin.withdraw<BCO>(&bco.balance, amount);
        Coin.burn(coin);
        Event::emit_event<BurnEvent>(&bco.burn_events, BurnEvent { amount, from });
    }
}
~~~

### Python Deploy Script
This basic Python script *deploy.py* can be used for deploying either the Solidity or universal (chain-agnostic, Cardano etc.) contract based on user selection:

~~~
import subprocess
import sys
from web3 import Web3

def deploy_solidity_contract():
    # Connect to the blockchain
    w3 = Web3(Web3.HTTPProvider('http://127.0.0.1:8545'))
    w3.eth.default_account = w3.eth.accounts[0]

    # Read the Solidity contract
    with open('BasicCarbonOffsets.sol', 'r') as file:
        contract_source_code = file.read()

    # Compile the contract
    compiled_sol = subprocess.run(['solc', '--combined-json', 'abi,bin', 'BasicCarbonOffsets.sol'], capture_output=True)
    compiled_sol = json.loads(compiled_sol.stdout)
    
    contract_id, contract_interface = compiled_sol.popitem()
    bytecode = contract_interface['bin']
    abi = contract_interface['abi']

    # Deploy the contract
    BasicCarbonOffsets = w3.eth.contract(abi=abi, bytecode=bytecode)
    tx_hash = BasicCarbonOffsets.constructor(1000000).transact()
    tx_receipt = w3.eth.wait_for_transaction_receipt(tx_hash)

    print(f"Contract deployed at address: {tx_receipt.contractAddress}")

def deploy_move_contract():
    # Example command to compile and deploy Move contract using a CLI tool
    move_cli_command = "move-cli publish --address <your_address>"
    subprocess.run(move_cli_command, shell=True)

if __name__ == "__main__":
    if len(sys.argv) != 2 or sys.argv[1] not in ["solidity", "move"]:
        print("Usage: python deploy.py [solidity|move]")
        sys.exit(1)

    if sys.argv[1] == "solidity":
        deploy_solidity_contract()
    elif sys.argv[1] == "move":
        deploy_move_contract()
~~~

This script provides a simple way to deploy a carbon offset smart contract using either Solidity or another non-EVM language by choice. It requires the user to adjust the CLI command based on the specific environment and setup. 

[List of Community-built Developer Tools](https://www.essentialcardano.io/article/a-list-of-community-built-developer-tools-on-cardano)

[Calling Endpoint via CLI](https://forum.cardano.org/t/how-can-i-call-endpoint-via-cardano-cli-transaction-build-command/111154)

### Getting started with Mesh
To get started with Mesh, you need to install the latest version of Mesh with npm:

~~~
npm install @meshsdk/core @meshsdk/react
~~~

Below is an alternative deploy.js script written in TypeScript-compatible JavaScript. This script uses ethers.js for deploying the Solidity contract and an example command to deploy the universal smart contract.
Install the necessary dependencies:

`npm install ethers @types/node`

Use the deploy.ts script:

~~~
import { ethers } from 'ethers';
import { exec } from 'child_process';
import { promisify } from 'util';
import fs from 'fs';
import path from 'path';

const execAsync = promisify(exec);

async function deploySolidityContract() {
    // Connect to the blockchain
    const provider = new ethers.providers.JsonRpcProvider('http://127.0.0.1:8545');
    const signer = provider.getSigner();

    // Read the Solidity contract
    const contractPath = path.resolve(__dirname, 'BasicCarbonOffsets.sol');
    const contractSource = fs.readFileSync(contractPath, 'utf8');

    // Compile the contract
    const solc = require('solc');
    const input = {
        language: 'Solidity',
        sources: {
            'BasicCarbonOffsets.sol': {
                content: contractSource,
            },
        },
        settings: {
            outputSelection: {
                '*': {
                    '*': ['abi', 'evm.bytecode'],
                },
            },
        },
    };
    const output = JSON.parse(solc.compile(JSON.stringify(input)));
    const abi = output.contracts['BasicCarbonOffsets.sol']['BasicCarbonOffsets'].abi;
    const bytecode = output.contracts['BasicCarbonOffsets.sol']['BasicCarbonOffsets'].evm.bytecode.object;

    // Deploy the contract
    const factory = new ethers.ContractFactory(abi, bytecode, signer);
    const contract = await factory.deploy(1000000);

    console.log(`Contract deployed at address: ${contract.address}`);
}

async function deployMoveContract() {
    // Example command to compile and deploy Move contract using a CLI tool
    const moveCliCommand = 'move-cli publish --address <your_address>';
    const { stdout, stderr } = await execAsync(moveCliCommand);
    if (stderr) {
        console.error(`Error deploying Move contract: ${stderr}`);
    } else {
        console.log(`Move contract deployed: ${stdout}`);
    }
}

(async () => {
    const args = process.argv.slice(2);
    if (args.length !== 1 || (args[0] !== 'solidity' && args[0] !== 'move')) {
        console.error('Usage: ts-node deploy.ts [solidity|move]');
        process.exit(1);
    }

    if (args[0] === 'solidity') {
        await deploySolidityContract();
    } else if (args[0] === 'move') {
        await deployMoveContract();
    }
})();
~~~

Usage
To run this script, use ts-node, which allows you to execute TypeScript files directly. First, install ts-node if you haven't already:


`npm install -g ts-node`


`Run the script with:`


`ts-node deploy.ts [solidity|move]`


Replace [solidity|move] with either solidity or move depending on which contract you want to deploy.

This script reads, compiles, and deploys the Solidity contract using ethers.js and executes a command to deploy the Move contract using the CLI tool. Adjust the Move deployment command as necessary for your specific environment and setup.

[MeshJS](https://docs.meshjs.dev/classes/MeshWallet)

