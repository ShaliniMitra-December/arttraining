# 🎨 Art Tracking Smart Contract (Soroban)

## 📌 Project Description

The **Art Tracking Smart Contract** is a decentralized application built on the Stellar Soroban platform that enables secure registration, tracking, and transfer of artwork ownership.

This project aims to bring transparency and trust to the art ecosystem by recording artwork metadata and ownership history on-chain, ensuring authenticity and reducing the risk of forgery or disputes.

<img width="1911" height="1077" alt="Screenshot 2026-04-13 135716" src="https://github.com/user-attachments/assets/6670ba5b-cd9d-46b8-8a8b-690b2780b665" />

---

## ⚙️ What it does

This smart contract allows users to:

* Register new artworks with essential metadata (title, creator, owner)
* Retrieve stored artwork information from the blockchain
* Transfer ownership of artwork securely between users
* Maintain an immutable record of provenance and ownership history

---

## ✨ Features

* 🎨 Artwork registration system
* 🔍 Public and transparent ownership tracking
* 🔄 Secure ownership transfer mechanism
* 🧾 Immutable provenance records on blockchain
* ⚡ Fast and low-cost transactions via Stellar Soroban
* 🛠️ Built using Rust for safety and performance
* 🧩 Extensible for NFTs, royalties, and IPFS integration

---

## 🔗 Deployed Smart Contract Link

**Contract Address:**
`CACOKYMVIKEDDIUVRKISS5RJXMALSDJUKADS7D6YFCYDIUUYTY5B4RQI`

You can interact with this contract using the Soroban CLI or integrate it into a frontend dApp.

---

## 🚀 Getting Started

### Prerequisites

* Rust installed
* Soroban CLI installed
* Stellar testnet/mainnet account

---

### Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

### Deploy Contract

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/art_tracking.wasm \
  --source your-account
```

---

## 🧪 Example Usage

### Register Artwork

* ID: 1
* Title: "Digital Sunrise"
* Creator: "Artist A"
* Owner: "Collector X"

### Transfer Ownership

* From: "Collector X"
* To: "Collector Y"

---

## 📚 Tech Stack

* Rust
* Soroban SDK
* Stellar Blockchain

---

## 🔮 Future Improvements

* 🌐 IPFS integration for storing artwork media
* 🖼️ NFT minting support
* 💸 Royalty distribution for creators
* 🔐 Access control (only owner can transfer)
* 🖥️ Web UI for artists and collectors

---

## 📄 License

This project is licensed under the MIT License.
