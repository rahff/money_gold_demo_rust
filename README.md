# Gold Exchange Platform

## Overview

The **Gold Exchange Platform** is a Rust-based application designed to simulate the buying, holding, and exchanging of gold within a secure digital environment. Users can create accounts, purchase gold, and transfer gold to other users seamlessly.

## Features

- **Account Management**: Create and manage user accounts.
- **Gold Purchase**: Buy gold in grams to add to your account balance.
- **Gold Transfer**: Send gold to other users on the platform in exchange for goods or services.
- **Real-Time Balance Updates**: Ensure accurate and instant updates of account balances after transactions.

## How It Works

1. **Account Creation**: New users register on the platform and start with a zero gold balance.
2. **Gold Purchase**: Users can purchase gold, which is added to their account balance.
3. **Gold Transfer**: Users can transfer gold to others on the platform. For example:
    - **Person A** buys 300g of gold. Their account balance is now 300g.
    - **Person B** opens an account with a balance of 0g.
    - If Person B sells an item to Person A for 250g, Person A transfers 250g to Person B.
    - After the transaction:
        - Person A's balance: 50g
        - Person B's balance: 250g

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can download it from [Rust's official website](https://www.rust-lang.org/).

## Technologies Used

- **Rust**: The primary programming language for the platform.
- **Actix-Web**: For building the web server (if applicable).
- **SQLite**: For database storage (or another database technology, depending on your implementation).

## Future Enhancements

- **Real-Time Notifications**: Notify users of transactions and balance changes.
- **Integration with Payment Gateways**: Enable real-world purchases of gold.
- **Audit Logs**: Provide a history of all transactions for better transparency.
- **Security Enhancements**: Strengthen authentication and encryption for user data.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---


