# 📡 DSP Sandbox — Signal-Oriented Computing with RCA

**Author:** Gavin Walters

**Purpose:** Practical exploration of Digital Signal Processing (DSP) concepts through systems-oriented programming and RCA (Runtime Cell Architecture)

---

## 1. Overview

This repository is a **hands-on laboratory** for exploring:

* Digital Signal Processing (DSP)
* Signal-based thinking in software systems
* Data-oriented and time-oriented computation
* Runtime Cell Architecture (RCA)

The goal is not just to *implement DSP algorithms*, but to:

> Understand computation itself as signal transformation.

---

## 2. Core Idea

Traditional software thinking:

```
Input → Logic → Output
```

Signal-oriented thinking:

```
Signal → Transform → Signal → Transform → ...
```

In this model:

* **Data is a signal**
* **Computation is transformation**
* **Time is fundamental**
* **State is signal history**

---

## 3. Repository Goals

### 3.1 Learn DSP Through Implementation

* Waveforms (sine, square, chirps)
* Sampling & discretization
* FFT / frequency domain analysis
* Filtering (low-pass, high-pass, band-pass)
* Noise & signal reconstruction

### 3.2 Apply DSP to Software Systems

* Treat logs, events, and metrics as signals
* Model systems as pipelines of transformations
* Explore filtering, detection, and classification

### 3.3 Integrate with RCA

This repo serves as a proving ground for:

* Cells as signal processors
* Threads as signal pipelines
* Engine as signal orchestrator

---

## 4. Architecture

### 4.1 Conceptual Model

```
Engine
  ├── Threads
  │     ├── Cells
  │     │     └── Signal Transform
  │     └── Signal Flow
  └── Control + Data Planes
```

### 4.2 Runtime Flow

```
Engine (Data + Control)
    ↓
Threads (Execution Paths)
    ↓
Cells (Transform Units)
    ↓
Signals (Data)
```

---

## 5. Project Structure

```
dsp-sandbox/
├── src/
│   ├── signals/        # Signal generation (sine, chirp, noise)
│   ├── transforms/     # FFT, filtering, convolution
│   ├── analysis/       # Frequency, power, detection
│   ├── rca/            # RCA integration (engine, threads, cells)
│   └── experiments/    # Mini-projects and prototypes
│
├── examples/           # Runnable demos
├── notebooks/          # (Optional) Python or analysis notebooks
├── data/               # Sample signals or datasets
├── docs/               # Notes, diagrams, explanations
└── README.md
```

---

## 6. Experiments (Planned)

### 6.1 Foundations

* Generate sine waves and visualize
* Sample continuous signals
* Aliasing experiments

### 6.2 Frequency Domain

* Implement FFT from scratch
* Compare time vs frequency representations

### 6.3 Filtering

* Moving average filter
* FIR / IIR filters
* Noise reduction

### 6.4 Radar-Inspired Work

* Chirp generation (FMCW-style)
* Mixing signals
* Range estimation concepts

### 6.5 System-Level DSP

* Treat logs as signals
* Detect anomalies via frequency/pattern analysis
* Event stream filtering

---

## 7. RCA Integration

RCA is used as the execution model:

* **Cells** = Stateless or stateful signal processors
* **Threads** = Ordered signal pipelines
* **Engine** = Coordinates data + control flow

Example mental model:

```
[Signal] → [Cell: Filter] → [Cell: FFT] → [Cell: Detect] → [Output]
```

---

## 8. Design Principles

### 8.1 Signal First

Everything is treated as a signal:

* Arrays
* Streams
* Events
* Metrics

### 8.2 Transform Over Logic

Prefer:

```
transform(signal)
```

Over:

```
if/else-heavy logic
```

### 8.3 Time Matters

* Order matters
* Sampling matters
* Latency matters

### 8.4 Small Composable Units

* Build small transforms
* Chain them together

---

## 9. Tech Stack

* **Language:** Rust (primary)
* Optional:

  * Python (for visualization)
  * NumPy / Matplotlib (analysis only)

---

## 10. How to Use This Repo

### 10.1 Run Examples

```
cargo run --example <name>
```

### 10.2 Add New Experiment

1. Create file in `src/experiments/`
2. Define signal
3. Apply transforms
4. Observe output

### 10.3 Extend RCA

* Add new Cell types
* Create new Thread pipelines
* Integrate new DSP transforms

---

## 11. Long-Term Vision

This repo is a stepping stone toward:

* Signal-driven system architecture
* DSP-inspired software design
* Real-time, high-performance systems
* Embedded + radar-aligned processing models

Ultimately:

> Blur the line between software, signal processing, and hardware thinking.

---

## 12. Notes

This is an experimental repo.

Expect:

* Iteration
* Refactoring
* Changing abstractions

The goal is clarity through building.

---

## 13. Dependencies

'''
sudo apt install python3-numpy python3-matplotlib
'''

---

## 14. License

TBD

---

## 15. Closing Thought

> If data is a signal, then software is a waveform in motion.
