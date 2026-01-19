# ArenaX Smart Contracts

This directory contains the Stellar smart contracts for the ArenaX gaming platform.

## Contracts

### Example Contract

A basic smart contract demonstrating core Soroban SDK functionality:

- Contract initialization
- Persistent storage
- Event emission
- Authentication
- Unit testing

#### Features

- **Greeting System**: Store and retrieve personalized greeting messages
- **Counter**: Simple incrementing counter with persistence
- **Admin Management**: Contract administration functions
- **Events**: Proper event emission for all state changes

#### Development

```bash
# Build the contract
cd contracts
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test
```

#### Deployment

The contract can be deployed to Stellar testnet using the Soroban CLI:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/example_contract.wasm \
  --source <your-secret-key> \
  --network testnet
```

## Architecture

All contracts follow these principles:

- **Security First**: All functions include proper authorization checks
- **Event Driven**: State changes emit events for off-chain monitoring
- **Storage Efficient**: Optimized storage usage with proper data structures
- **Testable**: Comprehensive unit tests for all functionality