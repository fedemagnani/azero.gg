# Azero.gg

🎉 [Aleph-zero bounty winner project at ETHWarsaw 2023 (3rd place)](https://devpost.com/software/azero-gg) 🎉

An Aleph-Zero based platform for gated access to DAOs and token holder communities, which has the goal of connecting decentralized communities across the ecosystem.




## Table of Contents

- [Overview](#overview)
- [Use cases](#use-cases)
- [Solution in a nutshell](#solution-in-a-nutshell)
- [Usage](#usage)

## Use cases

This application is intended to support different use-case scenarios:

- **DAO governance servers**: Servers whose access is restricted to DAO participants holding a certain amount of governance tokens.
- **NFT communities**: NFT holders can join private community private chat rooms.
- **LP token holders**: Liquidity miners and other kind of investors can join private channels that are deeply focused on their interests.

## Solution in a nutshell

The project consists of three main components: a Discord bot written in Rust, a Rust web integration layer and a React frontend enabling users to connect their [Aleph Zero Signer](https://alephzero.org/signer) wallet and authenticate against the application.

The project is organized into three main components, each residing in its own directory:

1. **Discord Bot (Rust)**: The Discord bot is responsible for monitoring user interactions within the Discord server. When a user requests to join the server, the bot verifies their Aleph-Zero token balance and grants access if the criteria are met.

2. **Web Server (Rust)**: The web server serves as an integration layer between the Discord bot and the React frontend. It handles authentication, authorization, and communication with the Aleph-Zero blockchain to check token balances.

3. **React Frontend**: The React frontend provides a user-friendly interface for connecting the "Aleph Zero Signer" wallet, initiating the join server process, and displaying relevant information about the user's balance and server access status.

```mermaid
sequenceDiagram
    User->>+Discord app: Click authorize button
    Discord app->>+dApp: Navigate to dApp
    dApp->>+dApp: User connects "Aleph-zero" wallet
    dApp->>+dApp: User signs a message with wallet private key
    dApp->>+Bot REST API: Send "address", "signature"
    Bot REST API->>+Bot REST API: Verify signature
    Bot REST API->>+Aleph-zero chain: Check on-chain balance
    Bot REST API->>+Bot REST API: Persist user authorization
    Bot REST API->>+Discord app: Enable user to the server
    User->>+Discord app: Free to chat
```

## Usage

```txt
# Create a .env file in the main directory
DISCORD_TOKEN=<TOKEN> # The discord token associated to your account
AUTH_ROLE_NAME=Authorization
```

Open 2 terminals
```bash
# Frontend
npm install
npm run dev
```

```bash
# Backend
cargo +nightly run
```
