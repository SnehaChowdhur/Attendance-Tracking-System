## ⚙️ What It Does

* Records user attendance on-chain for specific dates
* Prevents duplicate entries for the same user and date
* Allows anyone to verify attendance records
* Ensures only authorized users can mark their own attendance

---

## ✨ Features

* 🔐 **User Authentication**
  Users must authorize transactions before marking attendance

* 🧾 **Immutable Storage**
  Once recorded, attendance data cannot be modified

* 🚫 **Duplicate Protection**
  Prevents multiple attendance entries for the same date

* 🔍 **Public Verification**
  Attendance can be checked by anyone at any time

* ⚡ **Efficient Smart Contract**
  Built using Soroban for low-cost and fast execution

---

## 🏗️ Smart Contract Functions

### `mark_attendance(user, date)`

Marks attendance for a given user on a specific date.

* Requires user authorization
* Fails if attendance is already recorded

### `check_attendance(user, date)`

Checks whether a user has marked attendance on a given date.

* Returns: `true` or `false`

### `get_attendance(user)`

Returns attendance records for a user.

* Note: In this basic version, full history tracking is not implemented

---

## 🛠️ Tech Stack

* Blockchain: Stellar
* Smart Contracts: Soroban (Rust)
* Language: Rust
* Deployment: Stellar CLI

---

## 🚀 Deployment Instructions

1. Install Stellar CLI and Soroban tools

2. Build the smart contract:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

3. Deploy the contract:

   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/attendance_contract.wasm \
     --source <your-wallet-key>
   ```

4. Save the generated Contract ID

---

## 🔗 Deployed Smart Contract Link

https://stellar.expert/explorer/testnet/contract/YOUR_CONTRACT_ID](https://stellar.expert/explorer/testnet/contract/CDO3PWGMM3F66KTRI6RC4U7DMKMTORUUNO4BEQ76RBIYAWWQGG4F4EIS)

---

## 🧠 Future Enhancements

* 📊 Full attendance history storage per user
* 👨‍🏫 Admin-controlled attendance (teacher/employer marking)
* 🧑‍🤝‍🧑 Role-based access control
* 🌐 Frontend integration (React / Next.js)
* 📥 Exportable attendance reports (CSV/PDF)
* ⏰ Time-based attendance validation

---
<img width="1905" height="912" alt="Screenshot 2026-03-24 141308" src="https://github.com/user-attachments/assets/c190f7cd-82fd-41c5-9d25-c449a579c5fd" />

## 🎯 Use Cases

* 🏫 Schools and Colleges
* 🏢 Corporate Offices
* 🎓 Online Learning Platforms
* 💻 Developer Bootcamps
* 🧑‍💼 Remote Work Tracking

---

## 📄 License

This project is licensed under the MIT License.
