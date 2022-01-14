# Qwestive Stake Smart Contract
Qwestive Stake Smart Contract

 ### Environment Setup
1. Install Rust from https://rustup.rs/
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. When installed i twill add the following PATH to your shell configurations
$ export PATH="$HOME/.cargo/bin:$PATH"

3. Install Solana v1.8.5 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool
$ sh -c "$(curl -sSfL https://release.solana.com/v1.8.5/install)"

4. Install will add a new PATH to your shell configurations
$ export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH" 

5. Install Anchor v0.19.0 or later
$ cargo install --git https://github.com/project-serum/anchor anchor-cli --locked

6. Install Yarn v1.22.17 or later
# Using npm global dependencies.
$ npm install -g yarn

# Using homebrew on Mac.
$ brew install yarn

# Using apt on Linux
$ apt install yarn

### Build and test for program compiled
```
$ anchor build
$ anchor run test
```
### Deploy program compiled for BPF
```
$ anchor deploy
```
### Build, Deploy, and Run Test
```
$ anchor test
```
### To run Solana locally
```
solana config set --url localhost
```

### To run Solana devnet
```
solana config set --url localhost
```

### To generate new solana test key
```
solana-keygen new
```