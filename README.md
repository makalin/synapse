# SYNAPSE

> **The Runtime-First AI Orchestration Console.**

**SYNAPSE** is not a text editor. It is an **Operator Console**.

Current "AI Editors" (Cursor, VS Code) are stuck in the past: they put the text buffer in the center and the AI in a sidebar. **SYNAPSE flips the paradigm.** In the age of autonomous agents, the human role shifts from *Typist* to *Orchestrator*.

SYNAPSE places the **Runtime (Terminal)** at the center stage. It is a low-level, GPU-accelerated environment designed to run, monitor, and direct multiple AI CLI agents simultaneously in real-time.

---

## ⚡ The Philosophy

* **Terminal-First:** The code is the artifact; the execution is the reality. The screen real estate is dedicated to `stdout`, logs, and agent interactions.
* **Native Speed:** No Electron. No HTML/CSS. Built entirely in **Rust** using **GPUI**, the same rendering engine powering Zed. It renders at the refresh rate of your monitor.
* **The "Lens" Editor:** Editing is surgical. When you need to write code, the editor appears as a high-contrast modal "lens" over the runtime, then vanishes when you return to orchestration.

## 🛠 Tech Stack

Designed for zero latency and maximum throughput.

* **Core Language:** [Rust](https://www.rust-lang.org/) (Safety, Concurrency, Speed).
* **UI Engine:** [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) (GPU-accelerated, Immediate Mode feel, Retained Mode structure).
* *MacOS:* Metal API.
* *Windows:* Vulkan / DX12.


* **Terminal Backend:** [`portable-pty`](https://www.google.com/search?q=%5Bhttps://github.com/wez/wezterm/tree/main/crates/portable-pty%5D(https://github.com/wez/wezterm/tree/main/crates/portable-pty)) (Robust cross-platform PTY management).
* **Async Runtime:** `smol` or `tokio` for handling concurrent AI streams.

## 🖥️ UI Layout: "The Command Deck"

The interface is divided into three functional zones:

1. **Zone A: The Grid (Center - 60%)**
A tiling window manager for CLI processes. Run your `Test Agent` in the top-left, `Refactor Agent` in the top-right, and `Server Logs` in the bottom.
2. **Zone B: The Roster (Sidebar - 15%)**
A live view of active assets: File Tree, Active Agent Processes, and System Telemetry (CPU/RAM usage of local models).
3. **Zone C: The Lens (Overlay)**
A transient, translucent code editor for manual intervention.

## 🚀 Getting Started

### Prerequisites

* Rust toolchain (`cargo`, `rustc`)
* MacOS or Windows (Linux support coming soon via Vulkan)

### Installation

```bash
# Clone the repository
git clone https://github.com/makalin/synapse.git

# Enter directory
cd synapse

# Build in release mode (Recommended for GPU performance)
cargo run --release

```

## 🗺️ Roadmap

* [ ] **Core:** Initialize GPUI window with tiling layouts.
* [ ] **Pty:** Integrate `portable-pty` to spawn real shells inside the grid.
* [ ] **UI:** Implement the "Lens" modal editor.
* [ ] **Agents:** Create a JSON-RPC bridge to connect external CLI AI tools (e.g., Aider, GPT-Pilot).
* [ ] **Remote:** SSH Tunneling support for remote server orchestration.

## 🤝 Contributing

This is a low-level engineering challenge. We are looking for contributors interested in:

* Rust Systems Programming
* GPU Rendering & Shaders
* Terminal Emulators (ANSI/VT100 parsing)
* CRDTs for text buffers

Please check the [Issues](https://www.google.com/search?q=https://github.com/makalin/synapse/issues) page for current tasks.

## 👤 Author

**Mehmet T. AKALIN**

* **GitHub:** [@makalin](https://github.com/makalin)
* **Web:** [dv.com.tr](https://dv.com.tr)
* **X (Twitter):** [@makalin](https://x.com/makalin)

---

*"Don't just edit. Orchestrate."*
