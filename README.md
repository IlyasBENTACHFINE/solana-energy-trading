# Solana Energy Trading Program

Created by Ilyas Bentachfine

## Overview

This project implements an energy trading system on the Solana blockchain. It allows participants to register as energy producers or consumers, report energy production, post energy demands, and execute trades automatically.

## Features

- Participant registration (Producer, Consumer, Prosumer)
- Energy production reporting
- Energy demand posting
- Automated transaction matching
- Wallet management (deposits and withdrawals)

## Project Structure

```
solana-energy-trading/
├── src/
│   └── lib.rs         # Solana program implementation
├── client/
│   └── index.js       # JavaScript client for interacting with the program
├── README.md          # This file
├── Cargo.toml         # Rust dependencies
├── package.json       # Node.js dependencies
└── .gitignore
```

## Prerequisites

- Rust and Cargo
- Node.js and npm
- Solana CLI tools

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/solana-energy-trading.git
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

## Usage

### Deploying the Solana Program

1. Build the program:
   ```
   cargo build-bpf
   ```

2. Deploy the program:
   ```
   solana program