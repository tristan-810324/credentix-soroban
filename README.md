# CredentiX Protocol | Enterprise Web3 On-Chain Certification Engine

CredentiX is an elite, decentralized credential verification and Soulbound Token (SBT) minting engine built on the **Stellar Soroban** network using Rust. It empowers learning institutions to issue tamper-proof, non-transferable academic and professional credentials directly on the ledger.

## 🚀 Unique Architecture & Features
- **Soulbound Token (SBT) Mechanics:** Credentials are permanently bound to the student's wallet address (`is_soulbound: true`), preventing unauthorized transfers or secondary market selling.
- **Dynamic Grading Engine:** Automatically categorizes performance from `A_EXCELLENT` down to `C_PASS` based on verified on-chain exam parameters.
- **Real-Time Event Indexing:** Broadcasts smart contract events (`cred_minted`) for instant frontend state synchronization.
- **Strict Error Handling:** Comprehensive implementation of protocol-level errors (`Unauthorized`, `InvalidScore`, `CredentialAlreadyExists`).

## 🛠️ Smart Contract Functions & Requirements Met
1. **3 Error Types Handled:** Protocol explicitly handles `Unauthorized`, `InvalidScore`, and `CredentialAlreadyExists`.
2. **Contract Deployed on Testnet:** Successfully deployed and verified.
3. **Wallet Integration:** Built with modular architecture ready for **StellarWalletsKit** and **Freighter API** integration.

## 🔗 Submission Details & Verification Links
- **Live Demo Link:** [View Deployed App (Vercel/Netlify)](https://your-live-demo-link.vercel.app) *(Update this once deployed)*
- **Network:** Stellar Testnet
- **Deployed Contract Address:** `CCRTH6FEEOKFPUPEURQLBD4MUUSRHRCGJXDGB5LBLRIQXUPVYPHYXQ7D`
- **Transaction Hash:** [View on Stellar Expert Explorer](https://stellar.expert/explorer/testnet/tx/1dc6276dab7eae65563426ca6572292604a5a2bc24a8b5ec518b59c2988f3787)

## 📸 Screenshots & Wallet Options
- Integrated with **Freighter Wallet** and multi-wallet UI selection frameworks.
- Check the repository screenshots folder for UI state validation.

## ⚙️ Setup Instructions
1. Clone the repository:
   ```bash
   git clone [https://github.com/tristan-810324/credentix-soroban.git](https://github.com/tristan-810324/credentix-soroban.git)