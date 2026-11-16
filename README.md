# medisimp

**Privacy-first medical text redaction and plain-language clinical glossary - built from scratch in Rust.**

`medisimp` is a self-contained, zero-dependency software application designed for Health Informatics workflows, AI agent pipelines, and everyday patients. It runs **100% offline** on your device—stripping sensitive personal identifiable information (PII) locally and translating 40+ dense medical abbreviations into clear, plain-English explanations.

- **Live Product Website & Interactive Browser Sandbox:** https://newbie27-ms.github.io/medisimp/
- **Download Latest Releases (macOS & Linux):** https://github.com/Newbie27-ms/medisimp/releases/latest
- **Source Code Repository:** https://github.com/Newbie27-ms/medisimp

---

## Table of Contents
1. [The Problem & Solution](#1-the-problem--solution)
2. [Key Users & Workflows](#2-key-users--workflows)
3. [Key Features](#3-key-features)
4. [Tools & Tech Stack](#4-tools--tech-stack)
5. [System Architecture](#5-system-architecture)
6. [Download & Installation (macOS & Linux)](#6-download--installation-macos--linux)
7. [How to Use (All 3 Modes)](#7-how-to-use-all-3-modes)
8. [Example Input & Output](#8-example-input--output)
9. [Target Groups & Business Model](#9-target-groups--business-model)

---

## 1. The Problem & Solution

### The Problem
Clinical lab results, hospital discharge summaries, and doctor notes are difficult for patients to read because they are filled with specialized abbreviations (`HbA1c`, `eGFR`, `COPD`, `BID`, `PRN`). At the same time, pasting raw medical reports into cloud-based AI tools exposes sensitive personal data (full names, dates of birth, patient IDs, addresses, phone numbers, and email addresses).

### The Solution
`medisimp` acts as a **local privacy firewall and clinical translator**:
- **Before sharing or using AI:** It automatically detects and replaces personal identifiers with `[REDACTED]` locally on your machine.
- **For immediate understanding:** It scans copied or OCR-extracted report text for 40+ common clinical terms and appends a structured, plain-language glossary in clean Markdown.
- **Zero setup overhead:** Instead of heavy Python environments, Docker containers, or external API keys, `medisimp` compiles into a single standalone binary using only the Rust standard library.

---

## 2. Key Users & Workflows

`medisimp` is built as a focused, offline prototype serving two primary user groups who work with copied or OCR-extracted medical text:

1. **Patients & Caregivers (Everyday Offline Use)**
   - **Workflow:** Copy text from a lab report, discharge summary, or image OCR and paste it into the **100% offline local Web UI** (`medisimp --ui` on `http://127.0.0.1:8080`).
   - **Goal:** Decode cryptic lab and prescription abbreviations (`HbA1c`, `eGFR`, `BID`, `PRN`) into plain English immediately without private health data ever leaving their device.

2. **Developers, Health Informatics Students & AI Power Users ("Agentic OS" Workflow)**
   - **Workflow:** Pipe clipboard or file text through `medisimp` in the terminal (`pbpaste | medisimp` or `medisimp report.txt -o clean_report.md`).
   - **Goal:** Strip out sensitive personal identifiers (`[REDACTED]`) and translate clinical jargon into clean Markdown *before* passing medical notes into cloud or local AI agents.

---

## 3. Key Features

- **100% Offline Local Execution:** Zero network requests, zero telemetry, and zero external cloud dependencies. Patient data never leaves your computer.
- **Two-Layer PII Redaction:**
  - **Header Redaction:** Automatically redacts sensitive record lines (`Patient:`, `Name:`, `DOB:`, `Birth:`, `ID:`, `MRN:`, `Address:`, `Phone:`, `Insurance:`).
  - **Inline Pattern Redaction:** Automatically detects and masks inline email addresses (`[REDACTED-EMAIL]`), calendar dates (`[REDACTED-DATE]`), phone numbers, and patient IDs (`[REDACTED-ID]`).
- **40+ Clinical Abbreviation Dictionary:** Covers vital signs (`BP`, `HR`, `SpO2`, `BMI`), diagnoses (`HTN`, `DM`, `T2DM`, `COPD`, `MI`, `CVA`, `SOB`), lab panels (`CBC`, `WBC`, `RBC`, `HGB`, `PLT`, `MCV`, `HbA1c`, `LDL`, `HDL`, `TSH`, `ALT`, `AST`, `eGFR`, `CRP`), diagnostics (`ECG`, `EKG`, `CXR`, `MRI`, `CT`), and prescriptions (`PO`, `IV`, `BID`, `TID`, `QID`, `PRN`).
- **Three Flexible Modes:**
  1. **Built-in Local Web UI (`--ui`)** served offline on `http://127.0.0.1:8080`.
  2. **Unix Command-Line & Piping Tool (`stdin` / file export)** for developers and AI agents.
  3. **Zero-Install Browser Sandbox** hosted on GitHub Pages.

---

## 4. Tools & Tech Stack

- **Core Language:** Rust (100% standard library — `std::fs`, `std::io`, `std::net::TcpListener` — with **zero** third-party crate dependencies in `Cargo.toml`)
- **Cross-Compilation & Static Linking:**
  - Linux (`aarch64-unknown-linux-musl` & `x86_64-unknown-linux-musl`) statically linked via `cargo-zigbuild` and `musl` libc
  - macOS (`aarch64-apple-darwin` & `x86_64-apple-darwin`) compiled natively via `cargo`
- **CI/CD & Release Automation:** GitHub Actions (`.github/workflows/main.yml`) for automated multi-architecture binary builds on every release tag
- **Frontend & Web Hosting:** Embedded zero-dependency HTML/CSS/JS local Web UI (`--ui`) and static GitHub Pages product website & browser sandbox (`docs/index.html`)

---

## 5. System Architecture

`medisimp` is intentionally engineered from scratch with **zero third-party crate dependencies** so every line of logic is transparent, auditable, and easy to maintain:

```text
medisimp/
├── Cargo.toml                  # Package metadata (zero external dependencies)
├── src/
│   ├── lib.rs                  # Core text-processing engine: PII redaction & 40+ term glossary
│   └── main.rs                 # CLI parser, stdin/file I/O, and std::net::TcpListener local web server
├── docs/
│   └── index.html              # GitHub Pages product landing page & live browser sandbox
└── .github/workflows/
    └── main.yml                # Automated CI/CD cross-compilation for macOS & Linux releases
