# Solana Staking (Anchor)

Lightweight Solana staking project built with [Anchor](https://www.anchor-lang.com/) and a TypeScript client/test setup.

## What this project includes

- Anchor program in `programs/constants`
- Core instructions:
  - create/update stake pool
  - deposit reward tokens
  - stake tokens
  - unstake tokens (with reward calculation)
- TypeScript test/client scaffolding in `tests` and `client`

## Tech stack

- Rust + Anchor (`anchor-lang`, `anchor-spl`)
- Solana Web3 + SPL Token 2022
- TypeScript + Mocha + ts-mocha

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- Node.js + Yarn

## Quick start

```bash
yarn install
anchor build
anchor test
```

Run the TypeScript client script:

```bash
anchor run client
```

## Project structure

- `programs/constants`: on-chain staking program (Rust)
- `tests`: Anchor/TypeScript tests
- `client`: local client scripts
- `migrations`: Anchor migration scripts

## Notes

- `Anchor.toml` is set to `localnet` by default.
- Program ID and wallet path are configured in `Anchor.toml`.
- Some test/client code appears to be work-in-progress; use as scaffolding for your own flows.
