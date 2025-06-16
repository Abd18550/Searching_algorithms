# Maze Solvers: Normal Search vs. Quantum Search

This project contains two different Rust-based implementations of maze solving using SDL2 for visualization:

- `normal_search`: A classic recursive backtracking maze solver.
- `quantum_search`: A quantum-inspired search using multiple branching agents exploring in parallel.

---

## 📁 Project Structure

.
├── normal_search
│ ├── Cargo.lock
│ ├── Cargo.toml
│ └── src
│ └── main.rs
└── quantum_search
├── Cargo.lock
├── Cargo.toml
└── src
└── main.rs


---

## 🧠 Algorithms Overview

### 🔎 normal_search (Classic Backtracking)

- Uses recursive depth-first search (DFS) with backtracking.
- Explores the maze step-by-step, one path at a time.
- Backtracks when a dead end is hit.
- Visually shows:
  - 🟩 Green: Start cell
  - 🟥 Red: End cell
  - 🔵 Blue: Current path being explored
  - 🔴 Light Red: Backtracked (wrong) path

### ⚛️ quantum_search (Quantum-Inspired Multi-Agent Search)

- Simulates a quantum search with branching agents.
- Each agent explores in parallel and splits when multiple options exist.
- The first agent to reach the goal is considered the winner.
- Visually shows:
  - 🟩 Green: All current agent positions
  - 🟥 Red: End cell
  - 🔵 Blue: Visited cells
  - 🔴 Light Red: Dead ends
  - 🌊 Light Blue: Final path after the goal is reached

---

## ▶️ How to Run

### Prerequisites

Make sure you have:

- Rust installed → [https://rustup.rs](https://rustup.rs)
- SDL2 installed on your system:

#### On Ubuntu:

```bash
sudo apt update
sudo apt install libsdl2-dev

On macOS (with Homebrew):

brew install sdl2

Run normal_search

cd normal_search
cargo run

    Press Enter to start solving the maze.

    Press ESC to exit.

Run quantum_search

cd quantum_search
cargo run

    Runs automatically with agents branching in parallel.

    Press ESC to exit.


Built with Rust and SDL2 for educational and visualization purposes.

Developed by: Abd Abu Dawood