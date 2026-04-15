# ☕ THE SMART CAFE LEDGER ☕

---

## ⚡ PROJECT TITLE: DECENTRALIZED CAFE ORDERING SYSTEM

### 🕵️‍♂️ PROJECT DESCRIPTION

**The Smart Cafe Ledger** is a specialized decentralized point-of-sale and ordering tool built on the Stellar blockchain. It provides a permanent, immutable record for your culinary operations. Whether it's a standard coffee shop or a highly coordinated 11-member cireng kuah business, this ledger ensures every menu item and customer order is archived with cryptographic proof. Stop relying on messy paper tickets; start relying on the blockchain.

---

## 🔭 PROJECT VISION

> "Transparency in every transaction and taste."

We envision a world where every "I ordered that 20 minutes ago" is easily verified by the security of a global blockchain. Our goal is to:

- **Codify Cravings**: Turning menu items and customer orders into structured, verifiable data.
- **Cryptographic Accountability**: Ensuring that every order placed is strictly authenticated by the customer's wallet.
- **Immutable Receipts**: Guaranteeing that once an order is logged, it stands as an unalterable historical fact.

---

## 🛠 KEY FEATURES

- **[MENU MANAGEMENT]** - Add new food/beverage items (`add_menu`) or completely remove them via ID (`delete_menu`) from the on-chain storage.
- **[AUTHENTICATED ORDERS]** - Utilizing strict `require_auth()` to ensure only verified addresses can place orders (`order_menu`).
- **[ACTION CONFIRMATIONS]** - Get direct string feedback (e.g., "Menu berhasil ditambahkan", "Berhasil hapus menu") for every successful write operation.
- **[TRANSPARENT AUDITING]** - Retrieve full lists of active menus (`get_menus`) and pending orders (`get_orders`) instantly for kitchen synchronization.

---

## ⛓ DEPLOYED SMARTCONTRACT DETAILS

```text
+-----------------------------------------------------------------+
| CCQ44T3SOMXO2W5T3NCDT5F2DPFPBZLJXK56SS3D6FLHEHZIRVKW4OVZ               |
+-----------------------------------------------------------------+
