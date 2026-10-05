# 📦 reatler

**reatler** is a CLI tool that scans a project directory, detects its type, and bundles matching files into a single output file (`output.json`).  
It supports **automatic project type detection** (Rust, JS, Python, etc.) or **manual selection** of file types, with optional smart directory search.

---

## ✨ Features

- 🔍 **Smart search** for directories by name (using [`fd`](https://github.com/sharkdp/fd) if available, otherwise recursive search).
- 🧠 **Automatic project type detection** based on common project files (`Cargo.toml`, `package.json`, etc.).
- 🛠 **Manual mode** for custom file type and ignore patterns.
- 📂 **.gitignore support** — automatically excludes ignored files.
- 📜 **Single-file bundling** — bundles all matched files into `output.json`.
- ⚡ **Fast scanning** with optional `fd` integration.

---

## 📥 Installation

### From source (with `cargo`)

```bash
git clone https://github.com/yourusername/reatler.git
cd reatler
cargo install --path .
```

---

## 🚀 Usage

```bash
reatler [OPTIONS] [DIRECTORY]
```

### Options

| Flag / Option     | Description                                                              |
| ----------------- | ------------------------------------------------------------------------ |
| `--smart <query>` | Search for a directory whose name contains `<query>` (case-insensitive). |
| `--manual` / `-m` | Manual mode — prompts for file types and ignore patterns.                |
| _(no flags)_      | Auto mode — detects project type and includes relevant files.            |

---

### Examples

#### 1️⃣ Auto-detect project type and bundle files

```bash
reatler .
```

- Detects project type (e.g., Rust, JS, Python).
- Includes relevant files automatically.
- Writes bundled content to `output.json`.

#### 2️⃣ Manual mode

```bash
reatler --manual
```

- Prompts:
  - File formats to include (`rs toml json`)
  - Files/directories to ignore (`target dist .d.ts`)

#### 3️⃣ Smart search for a subproject

```bash
reatler --smart api
```

- Finds directories containing `"api"` in their name.
- Lets you choose from matches.
- Scans and bundles files from the chosen directory.

---

## 📂 Output format

`output.json` contains a JSON object where each key is the matched file path and each value is the full contents of that file:

```json
{
  "src/main.rs": "fn main() { ... }",
  "Cargo.toml": "[package]\\nname = ..."
}
```
