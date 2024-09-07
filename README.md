# Solana Energy Trading Program

## Created and Owned by Ilyas BENTACHFINE

Copyright © 2024 Ilyas BENTACHFINE. All rights reserved.

## Project Overview

This innovative Solana Energy Trading Program, conceptualized and developed by Ilyas BENTACHFINE, implements a cutting-edge energy trading system on the Solana blockchain. It revolutionizes the way participants interact in energy markets by allowing seamless registration as energy producers or consumers, efficient energy production reporting, dynamic energy demand posting, and automated trade execution.

## Key Features

- Secure participant registration (Producer, Consumer, Prosumer)
- Real-time energy production reporting
- Dynamic energy demand posting
- Smart automated transaction matching
- Integrated wallet management (deposits and withdrawals)

## Project Structure

```
solana-energy-trading/
├── src/
│   └── lib.rs         # Solana program implementation by Ilyas BENTACHFINE
├── client/
│   └── index.js       # JavaScript client for program interaction
├── README.md          # Project documentation
├── Cargo.toml         # Rust dependencies
├── package.json       # Node.js dependencies
└── .gitignore
```

## Prerequisites

- Rust and Cargo
- Node.js and npm
- Solana CLI tools

## Setup Instructions

1. Clone the repository:
   ```
   git clone https://github.com/IlyasBentachfine/solana-energy-trading.git
   cd solana-energy-trading
   ```

2. Install Rust dependencies:
   ```
   cargo build
   ```

3. Install Node.js dependencies:
   ```
   cd client
   npm install
   ```

## Usage Guide

### Deploying the Solana Program

1. Build the program:
   ```
   cargo build-bpf
   ```

2. Deploy the program:
   ```
   solana program deploy target/deploy/solana_energy_trading.so
   ```

### Running the Client

1. Navigate to the client directory:
   ```
   cd client
   ```

2. Run the client:
   ```
   node index.js
   ```

## Contributing

While this project is owned and maintained by Ilyas BENTACHFINE, contributions and suggestions are welcome. Please contact Ilyas BENTACHFINE directly for collaboration opportunities.

## License

This project is proprietary software owned by Ilyas BENTACHFINE. All rights reserved. Unauthorized copying, modification, distribution, or use of this software, via any medium, is strictly prohibited without the express permission of Ilyas BENTACHFINE.

## Contact

For any inquiries or further information about this project, please contact Ilyas BENTACHFINE directly at:

Email: ilyas.bentachfine@usms.ma

---

© 2024 Ilyas BENTACHFINE. All rights reserved.
