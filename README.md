## 🦀 Lab Rust

**A personal collection of Rust code snippets and mini projects from weekend practice.**

---

### 🔧 SETUP

| Step | Description             | Command                          |
|------|-------------------------|----------------------------------|
| 1️⃣  | Launch Docker container | `docker compose up -d`          |
| 2️⃣  | Enter dev shell         | `docker exec -it lab-rust bash` |
| 3️⃣  | Create Cargo project    | `cargo new project-name`        |

📁 This creates a folder with `Cargo.toml` and starter code in `src/main.rs`

Example:

```bash
cargo new hello-world
cd hello-world
cargo run
```

---

### 🔨 BUILDING

#### 🗃 Compile a standalone file with `rustc`

```bash
rustc calculator.rs
./calculator
```

📍 Output: `calculator` binary in the current folder

---

#### 📦 Build and run a Cargo project

**From a folder with `Cargo.toml`:**

| Action      | Command                 | Output Location     |
|-------------|-------------------------|---------------------|
| Build       | `cargo build --release` | `target/release/`   |
| Run (debug) | `cargo run`             | `target/debug/`     |
```


