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
```

#### 2. Deploy contract

Copy `.env.example` to `.env`, fill out required values and run:
 - You can use exist private key and rpc url

```shell
cd contract && forge script deploy/Flow.s.sol --ffi --broadcast && cd ../
```

#### 3. Run script
