# MultiversX Earners Smart Contract

A smart contract that distributes payments (fees) to multiple addresses based on predefined percentages. Supports both EGLD and ESDT tokens.

## Features

- **Payment Distribution**: Automatically distribute payments to multiple addresses
- **Percentage-Based**: Allocate funds based on configurable percentages
- **Multi-Token Support**: Works with EGLD and any ESDT token
- **Precision Handling**: Returns leftover amounts to the owner
- **Admin Management**: Role-based access control for earner management
- **Pause Mechanism**: Emergency pause functionality

## How It Works

1. **Setup Earners**: Configure addresses and their percentage allocations
2. **Receive Payment**: Contract receives EGLD or ESDT tokens
3. **Distribute Funds**: Automatically splits payment according to percentages
4. **Handle Remainder**: Any leftover amount is sent back to the contract owner

> **Note**: Total percentage allocation must equal exactly 100%

## Structure

```
src/
├── lib.rs          # Main contract trait with documentation
├── admins.rs       # Admin management module
├── earners.rs      # Core earning distribution logic
└── pause.rs        # Pause/unpause functionality

common/
├── constants/      # Shared constants
├── errors/         # Error definitions
└── structs/        # Data structures

interaction/        # Deployment and interaction scripts
meta/              # Contract metadata
output/            # Generated ABI and WASM files
wasm/              # WebAssembly build configuration
```

## Usage Example

1. **Deploy the contract**
2. **Add earners** with their percentages:
   - Address A: 30%
   - Address B: 50%
   - Address C: 20%
3. **Send payment** to the contract
4. **Automatic distribution** occurs based on percentages

## Use Cases

- **Revenue Sharing**: Distribute platform fees among stakeholders
- **Team Payments**: Split earnings among team members
- **Affiliate Programs**: Distribute commissions to partners
- **DAO Treasury**: Allocate funds to different purposes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
