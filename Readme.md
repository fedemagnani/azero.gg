# Aleph-Zero GG

The Aleph-Zero GG is a multi-part application that allows users to join a Discord server based on their balance of a certain Aleph-Zero token. 

## Table of Contents
- [Use cases](#use-cases)
- [Project Components](#project-components)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
  - [Running the Discord Bot](#running-the-discord-bot)
  - [Running the Web Server](#running-the-web-server)
  - [Running the React Frontend](#running-the-react-frontend)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)


## Use cases
This application is intended to support different use-case scenarios:
* **DAO governance servers**: Servers whose access is restricted to DAO participants holding a certain amount of governance tokens.
* **NFT communities**: NFT holders can join private community private chat rooms.
* **LP token holders**: Liquidity miners and other kind of investors can join private channels that are deeply focused on their interests.

## Solution in a nutshell
The project consists of three main components: a Discord bot written in Rust, a Rust web integration layer and a React frontend enabling users to connect their [Aleph Zero Signer](https://alephzero.org/signer) wallet and authenticate against the application.

## Overview

This project aims to provide a seamless way for users to join a Discord server based on their Aleph-Zero token balance. The three main components work together to achieve this:

1. **Discord Bot (Rust)**: The Discord bot is responsible for monitoring user interactions within the Discord server. When a user requests to join the server, the bot verifies their Aleph-Zero token balance and grants access if the criteria are met.

2. **Web Server (Rust)**: The web server serves as an integration layer between the Discord bot and the React frontend. It handles authentication, authorization, and communication with the Aleph-Zero blockchain to check token balances.

3. **React Frontend**: The React frontend provides a user-friendly interface for connecting the "Aleph Zero Signer" wallet, initiating the join server process, and displaying relevant information about the user's balance and server access status.

## Project Components

The project is organized into three main components, each residing in its own directory:

- **discord-bot**: Contains the Discord bot written in Rust using the Serenity library.
- **web-server**: Contains the Rust web server responsible for handling API requests and interactions with the Aleph-Zero blockchain.
- **react-frontend**: Contains the React frontend application that enables users to interact with the application and join the Discord server.

## Getting Started

### Prerequisites

Before you can run the project, ensure you have the following prerequisites installed:

- Rust and Cargo (for the Discord bot and web server)
- Node.js and npm (for the React frontend)
- Discord bot token (create a bot on the Discord Developer Portal)
- Aleph Zero Signer wallet (ensure users have this wallet installed and configured)

### Installation

Follow these steps to set up and run the project:

1. **Clone the Repository**: Clone this repository to your local machine.

   ```bash
   git clone https://github.com/your-username/aleph-zero-discord-joiner.git
   cd aleph-zero-discord-joiner