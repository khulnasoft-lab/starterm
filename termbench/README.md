# 🚀 termbench

**`termbench`** is a fast, scriptable benchmarking tool focused on measuring **PTY read performance** in terminal emulators — perfect for comparing how different terminals handle massive streams of output.

---

## ⚠️ Disclaimer

This benchmark **does not** provide a complete overview of terminal performance. It only evaluates **how quickly a terminal reads from the PTY**, and does **not** consider:

- Frame rate
- Input latency
- Rendering performance

📌 **Do not draw broad conclusions** from these results unless you're familiar with how PTY internals work.

---

## 🧪 Getting Started

To run all default benchmarks (after setting up a [Rust toolchain](https://rustup.rs)):

```bash
cargo run --release
```

Benchmarks are located in the `./benchmarks/` directory and are defined as folders containing:

- `benchmark` – the main executable (required)
- `setup` – optional executable for one-time setup

`termbench` will read the **stdout** of each benchmark and repeat its output to ensure a meaningful sample size.

---

## 📊 Plotting Results

You can visualize your benchmark results using `gnuplot`.

### 1. Generate a `.dat` file:
```bash
cargo run --release -- --dat results.dat
```

### 2. Generate a summary SVG plot:
```bash
./gnuplot/summary.sh results.dat output.svg
```

### 3. Combine multiple results:
```bash
./gnuplot/summary.sh *.dat output.svg
```

### 4. Detailed per-benchmark plots:
```bash
./gnuplot/detailed.sh *.dat output/
```

---

## 🧩 Adding Your Own Benchmarks

Want to contribute or test your own scenarios?

1. Create a new folder inside `./benchmarks/`
2. Add a `benchmark` executable (and optionally `setup`)
3. Ensure your benchmark writes to **stdout**
4. Keep setup-only logic in the `setup` executable

📬 Feel free to [submit a pull request](https://github.com/khulnasoft-lab) with useful or interesting benchmarks — especially ones that highlight differences between terminal versions or emulators.

---

## 📎 License

MIT or Apache 2.0 – choose whichever suits you.

---

## 🙌 Credits

Originally inspired by efforts to stress-test terminal performance in real-world workloads.

Built with 💙 by the [StarTerm](https://github.com/khulnasoft-lab) team.