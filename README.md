# Learning Rust for Fun

Learning Rust with a clear goal — **backend development**.
Step by step through the basics, then build a real production-grade backend server.

> Why backend Rust? It's replacing Node.js/Go in performance-critical services.
> Companies like Discord, Cloudflare, Figma, and 1Password have moved core backend services to Rust.
> Same REST/GraphQL/DB concepts you already know from Node.js — just faster and safer.

## Install Rust

Go to https://rustup.rs and download + run `rustup-init.exe`

After install, verify with:
```bash
rustc --version
cargo --version
```

## How This Works

Each stage = build a program that teaches specific concepts.
No reading theory in isolation — we write code first, understand as we go.
Every stage builds toward the final goal: a production backend API in Rust.

---

## Timeline Overview

| Stage | Focus | Duration |
|---|---|---|
| Stage 1 | Rust basics — 8 small programs | ~2–3 weeks |
| Stage 2 | Backend building blocks — 6 programs | ~3 weeks |
| Stage 3 | Real backend project | ~3–4 weeks |
| **Total** | **Zero to production backend** | **~8–10 weeks** |

> Assuming 1–2 hours per day. Ownership/lifetimes (Program 8) will need extra time — hardest concept in Rust, but critical for backend work.

---

## Stage 1 — Rust Basics (Build small programs)
**Goal:** Get comfortable with the language. Duration: ~2–3 weeks

| # | Program to Build | Concepts Covered | Est. Time |
|---|---|---|---|
| 1 | Hello World | `println!`, macros, running with cargo | 1 day |
| 2 | Calculator (CLI) | Variables, mutability, data types, functions | 2 days |
| 3 | Guess the Number | User input, conditionals, loops, `Result` | 2 days |
| 4 | Temperature Converter | Functions, type casting, modules | 2 days |
| 5 | Simple To-Do List | Structs, `Vec`, enums, pattern matching | 3 days |
| 6 | Word Counter | Strings, `HashMap`, iterators, closures | 2 days |
| 7 | File Reader | File I/O, error handling, `Option` / `Result` | 2 days |
| 8 | Ownership Demo | Ownership, borrowing, lifetimes (the hard part) | 3–4 days |

---

## Stage 2 — Backend Building Blocks
**Goal:** Learn exactly what you need for backend APIs. Duration: ~2–3 weeks

Each program here is a direct building block of a real backend server.

| # | Program to Build | Concepts Covered | Est. Time |
|---|---|---|---|
| 9  | Async Task Runner | `async/await`, `tokio`, concurrency | 3 days |
| 10 | HTTP Client | Calling external APIs with `reqwest`, JSON parsing with `serde` | 3 days |
| 11 | Basic REST API | `axum` router, handlers, JSON responses | 4 days |
| 12 | API + Database | `sqlx` + PostgreSQL, connection pooling, CRUD | 4 days |
| 13 | Auth Middleware | JWT validation, middleware layers, error handling | 4 days |
| 14 | WebSocket Server | Real-time connections, `tokio-tungstenite`, broadcast channels | 4 days |

> After Program 14 you'll have all the pieces to build a real backend.

---

## Stage 3 — Real Backend Project
**Goal:** Ship a production-grade Rust backend. Duration: ~3–4 weeks

### DeFi Portfolio Tracker API

A high-performance REST API that replaces your current Node.js/Express backend pattern — built in Rust with `axum`.

**Why this project:**
- You already know DeFi data (vault balances, yields, positions, cross-chain state)
- You've built this kind of backend in Node.js — so the domain is familiar, focus is purely on Rust
- Real use case: serve your vault contract data without The Graph limitations
- Directly showcases Rust backend skills to employers

**What you'll build:**

```
defi-portfolio-api/
├── src/
│   ├── main.rs              # Server entry, router setup
│   ├── routes/
│   │   ├── portfolio.rs     # GET /portfolio/:address
│   │   ├── vaults.rs        # GET /vaults, GET /vaults/:id
│   │   └── health.rs        # GET /health
│   ├── handlers/            # Business logic
│   ├── db/                  # sqlx queries, models
│   ├── onchain/             # Ethereum RPC calls (ethers-rs)
│   └── middleware/          # Auth, rate limiting, logging
├── migrations/              # SQL migration files
└── Cargo.toml
```

**Features:**
- Fetch live vault data from Ethereum/Base/Arbitrum via JSON-RPC
- Cache on-chain data in PostgreSQL
- REST endpoints: portfolio by wallet, vault stats, yield history
- JWT auth middleware
- Rate limiting
- Docker-ready

**Crates used:**

| Crate | Purpose | Node.js Equivalent |
|---|---|---|
| `axum` | Web framework | Express.js |
| `tokio` | Async runtime | Node.js event loop |
| `sqlx` | Database (PostgreSQL) | pg / Prisma |
| `serde` | JSON serialization | JSON.parse / JSON.stringify |
| `ethers` | Ethereum RPC client | ethers.js |
| `tower` | Middleware layers | Express middleware |
| `tracing` | Structured logging | Winston / Pino |
| `jsonwebtoken` | JWT auth | jsonwebtoken (npm) |

---

## Key Difference: Node.js vs Rust Backend

| | Node.js (what you know) | Rust (what you're learning) |
|---|---|---|
| Concurrency | Event loop, single thread | True multi-threading, async |
| Performance | Good | 10–20x faster |
| Memory | Garbage collected | Manual, no GC, no crashes |
| Errors | Runtime crashes | Caught at compile time |
| Docker image | ~200MB | ~5MB (static binary) |

---

## Project Structure

```
learning rust for fun/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs       # active program we're working on
```

As we progress, each program may get its own file or module under `src/`.

---

## Resources (use when needed)

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) — exercises
- [Zero to Production in Rust](https://www.zero2prod.com/) — best book for backend Rust specifically
- [axum docs](https://docs.rs/axum/latest/axum/) — the framework we'll use
