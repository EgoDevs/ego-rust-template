# ego-rust-template

This Template is created for EGO projects

## ENV setup

- rust 1.65.0+
- dfx 0.12.1+
- didc [download binary](https://github.com/dfinity/candid/releases), export PATH to `didc`

- **!! Important !! Setup Credentials**

  - Under `credentials` folder, you need to add 3 files.
    1.  `seedphrase.txt`: 12 words mnemonic phrases, to create secp256k1 account for local test
    2.  `production.pem`: pem file with secp256k1 curve encoded, use for `mainnet` deployment
    3.  `production_cycles_wallet.txt`: cycles wallet of mainnet, use for `mainnet` deployment
  - You can change file names on `ego-config`.json

- setup project, see `ego-projects.json`,

## Quick Start

1. `pnpm install`
2. `pnpm run ego:run` to create and deploy
3. `pnpm run test ego_example` to run test file in `clients/tests`
