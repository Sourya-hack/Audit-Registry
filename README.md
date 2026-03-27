# 🛡️ Audit Registry – Soroban Smart Contract

## 📌 Project Description

Audit Registry is a lightweight decentralized smart contract built on Stellar’s Soroban platform. It enables secure and tamper-proof storage of audit records on the blockchain, ensuring transparency and trust in audit processes.

This project is designed for auditors, organizations, and compliance systems that need a reliable way to store and verify audit logs without relying on centralized systems.

---

## ⚙️ What It Does

The Audit Registry smart contract allows users to:

* Add audit records containing:

  * Auditor name
  * Report hash (proof of document integrity)

* Retrieve all stored audit records from the blockchain

Instead of storing full audit reports, the contract stores a hash of the report, making it efficient while still ensuring data integrity and verification.

---

## ✨ Features

### 🔐 Tamper-Proof Storage

All audit records are stored on-chain and cannot be modified once added.

### 📄 Report Integrity Verification

Stores cryptographic hashes of audit reports, allowing verification without exposing sensitive data.

### 📊 Transparent Access

Audit records can be retrieved anytime, ensuring accountability and trust.

### ⚡ Lightweight Design

Minimal storage usage for efficient performance on Soroban.

### 🧩 Simple & Beginner-Friendly

Designed with simplicity in mind, making it easy to understand and extend.

---

## 🏗️ Smart Contract Functions

### `add_audit(auditor, report_hash)`

Adds a new audit record to the blockchain.

### `get_audits()`

Returns all stored audit records.

---

## 🛠️ Tech Stack

* Rust
* Soroban SDK
* Stellar Blockchain

---

## 🚀 How to Use

1. Clone the repository
2. Build the contract:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```
3. Deploy to Soroban testnet
4. Interact with functions:

   * Call `add_audit()` to store records
   * Call `get_audits()` to fetch records

---

## 🔮 Future Improvements

* Add authentication (only authorized auditors)
* Include timestamps for each audit
* Search/filter functionality
* Integration with IPFS for full report storage
* Event logging for real-time tracking

---
## Deploy Smart Contract

Contract Address: CC2P2N6AXH3LELCU2DTGNCNVJVKPTCAW724IWKBXUF7P5X7ZVHHWQ2EY
<img width="1919" height="1079" alt="Screenshot 2026-03-27 133616" src="https://github.com/user-attachments/assets/50a219c9-590c-4544-935d-5d4c21e4c567" />


## 👨‍💻 Author

**Sourya Pratim Das**
B.Tech IT | Full Stack Developer | Blockchain Enthusiast

---

## 📜 License

This project is open-source and available under the MIT License.
