## Getting Started

### Requirements

The following will need to be installed in order to use this. Please follow the links and instructions.

-   [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)  
    -   You'll know you've done it right if you can run `git --version`
-   [NodeJS](https://nodejs.org/en/download/package-manager)
    -   Check that Node is installed by running: `node --version` and get an output like: `v18.16.1`
-   [Foundry](https://book.getfoundry.sh/getting-started/installation.html)
    -   Install Foundry by running the following command:
        ```sh
        curl -L https://foundry.paradigm.xyz | bash
        ```
    -   Initialize Foundry by running:
        ```sh
        foundryup
        ```
    -   Verify installation by running: `forge --version`
-   [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
    -   Install Cargo by running the following command:
        ```sh
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        ```
    -   Follow the on-screen instructions to complete the installation.
    -   Verify installation by running: `cargo --version`
### Quickstart

#### 1. Clone this repo

```shell
git clone https://github.com/0sttap/EtherFlow.git
cd EtherFlow
npm install
```

#### 2. Deploy contract
* You can skip this part, for test contract already deployed, but run `npm run build-contract` instead

Copy `contract/.env.example` to `contract/.env`, fill out required values and run:
 - For test you can use exist private key and rpc url

```shell
npm run deploy
```

After deploy you see in terminal logs like that:

```
##### mainnet
✅  [Success]Hash: 0x2f6679b1ff1dcbb8c23441610dfc75ea93b7aaa59d7fa283580af53e75730ea4
Contract Address: 0x29e95B6bbCd23fbF66e9d8ee2029564A734560De
Block: 20670268
Paid: 0.00257579449014608 ETH (637744 gas * 4.03891607 gwei)

✅ Sequence #1 on mainnet | Total Paid: 0.00257579449014608 ETH (637744 gas * avg 4.03891607 gwei)
```

You should copy `Contract Address` and add to main .env file:
```
CONTRACT_ADDRESS=0x29e95B6bbCd23fbF66e9d8ee2029564A734560De
```

#### 3. Run script

Copy `.env.example` to `.env`, fill out required values and run:
 - For test you can use exist private keys, rpc url and contract address (*only if you don't deploy new contract)

To start the script, use:

```shell
cargo run --release
```

#### 4. Test API

In folder test you have two files `collect.js` and `disperse.js`

You can use exist data for test api, just run:

```shell
node test/collect.js 1
```
or

```shell
node test/disperse.js 1
```

The number is:
* 1 - disperse/collect eth by fixed amount
* 2 - disperse/collect eth by percentage amount
* 3 - disperse/collect erc20 by fixed amount
* 4 - disperse/collect erc20 by percentage amount

After run a script, you see a response like that:
```
{
  transactionHash: '0xd886539abde60b3e00363bc48219bef0d409fb6c266d57b3c5db66a4e958b990'
}
```
