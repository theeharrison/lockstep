# ⚖️ Lockstep: Milestone Escrow on Solana

Lockstep is a trustless, milestone-based escrow smart contract built on the Solana blockchain. It solves the trust problem between clients and freelancers (or DAOs and contributors) by holding funds securely in an on-chain vault until work is approved.

## 🚨 The Problem
In Web3 and traditional freelancing, clients are afraid to pay upfront, and freelancers are afraid to do the work without a guarantee of payment. Current solutions rely on centralized platforms (like Upwork or Fiverr) that take high fees and can freeze accounts.

## 💡 The Solution
Lockstep acts as a neutral, decentralized vault.
1. A **Client** creates an escrow for a **Freelancer** and sets the milestone amount.
2. The Client deposits the funds (SOL, with USDC support coming soon) into the Lockstep vault.
3. Once the work is done, the Client approves the milestone.
4. The smart contract automatically releases the funds to the Freelancer.

## 🛠️ Current Status (Proof of Work)
- [x] Smart Contract written in Rust using the Anchor Framework.
- [x] Custom PDA (Program Derived Address) generation for secure vaults.
- [x] `initialize_escrow` instruction to create a deal.
- [x] `deposit` instruction to lock funds into the vault.
- [x] Fully tested locally with passing TypeScript integration tests.

## 🚀 Roadmap (What the Grant will Fund)
- [ ] Add SPL Token support (USDC escrow).
- [ ] Add a `release_funds` instruction for client approval.
- [ ] Add an arbitration system for disputes.
- [ ] Deploy to Solana Devnet and Mainnet.
- [ ] Build a clean, user-friendly Web3 Frontend using Next.js.

## 💻 Tech Stack
- **Blockchain:** Solana
- **Smart Contract:** Rust, Anchor Framework
- **Testing:** TypeScript, Mocha, Chai

## ⚙️ How to Run Locally
1. Clone the repository.
2. Install dependencies: `yarn install`
3. Build the program: `anchor build`
4. Run the test suite: `anchor test`


