<h1 align="center">💲Multithreaded Async Web Server in Rust from Scratch 📈</h1>

---

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.85.1-black.svg?style=for-the-badge&logo=rust">
  <img alt="Tokio" src="https://img.shields.io/badge/Tokio-1.44.0-1c1c1c.svg?style=for-the-badge&logo=tokio">
  <img alt="Askama" src="https://img.shields.io/badge/Askama-0.12.1-orange.svg?style=for-the-badge">
  <img alt="SQLx" src="https://img.shields.io/badge/SQLx-0.8.3-informational.svg?style=for-the-badge">
  <img alt="serde" src="https://img.shields.io/badge/Serde-1.0.219-blue.svg?style=for-the-badge">
  <img alt="Reqwest" src="https://img.shields.io/badge/Reqwest-0.12.12-blueviolet.svg?style=for-the-badge">
  <img alt="chrono" src="https://img.shields.io/badge/Chrono-0.4.40-green.svg?style=for-the-badge">
</p>

---

## 💡 Project Overview

This project demonstrates a fully-featured, production-ready asynchronous web server built entirely from scratch using modern Rust, **without relying on external frameworks like Actix-Web or Rocket**. The server leverages advanced asynchronous programming with **Tokio** for exceptional performance and scalability.

🌐 **Features included:**

- 🔥 **Multithreaded Asynchronous Execution:** Efficiently handles multiple connections simultaneously.
- 🛠 **Custom HTTP Server:** Implements full HTTP request parsing and response generation from scratch.
- 📦 **Router:** Highly customizable request routing system supporting various HTTP methods and dynamic route segments.
- 🗃 **Database Integration:** SQLite support through SQLx with async queries.
- 🔄 **API Integration:** Real-time stock price fetching through Finnhub API with continuous data synchronization.
- 🖥 **Dynamic HTML Rendering:** Template engine integration via Askama for dynamic HTML responses.
- 🔐 **Robust Error Handling:** Custom application error management ensuring server stability.
- 📡 **RESTful and Flexible:** Supports JSON & HTML responses with content negotiation.

---

## 🚀 Quick Start

**Clone the Repository**

```sh
git clone https://github.com/Jventajas/async_webserver_rust.git
cd async_webserver_rust
```

**Environment Setup**

```sh
cp .env.example .env
# Edit .env to include your Finnhub API key
```

**Run with Cargo**

```sh
cargo run
```

Visit: <http://localhost:8080/>

---

## 🔧 Key Components

### 🟢 **Async HTTP Web Server**
- Custom TCP listener built with Tokio, handling asynchronous stream reads and writes.

### 🟢 **Router & Route Handlers**
- Powerful and extensible router capable of matching paths and HTTP methods to specific handlers.

### 🟢 **Stock Data Sync**
- Automatic periodic synchronization of live market data from the Finnhub API.

### 🟢 **Database Layer**
- SQLite integration with async queries using SQLx for robust data storage and retrieval.

### 🟢 **Dynamic Templating**
- HTML pages dynamically rendered with Askama templates for responsive and modern UI.

---

## 📅 Example Use Case

The project currently demonstrates fetching and real-time updating stock data:

- **Home Page** lists available stock symbols.
- **Detail Pages** give detailed yet concise stock metrics such as current price, high/low, price changes, etc.

---

## 🛠 Future Improvements & TODO

- [ ] Websocket support for real-time stock updates
- [ ] User Authentication and session management
- [ ] Containerization and deployment scripts (Docker & Docker Compose)
- [ ] Extended configuration options via environment variables or configuration files
