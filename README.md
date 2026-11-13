# medisimp

**Privacy-first medical text redaction and plain-language clinical glossary - built from scratch in Rust.**

`medisimp` is a self-contained, zero-dependency software application designed for Health Informatics workflows, AI agent pipelines, and everyday patients. It runs **100% offline** on your device—stripping sensitive personal identifiable information (PII) locally and translating 40+ dense medical abbreviations into clear, plain-English explanations.

- **Live Product Website & Interactive Browser Sandbox:** https://newbie27-ms.github.io/medisimp/
- **Download Latest Releases (macOS & Linux):** https://github.com/Newbie27-ms/medisimp/releases/latest
- **Source Code Repository:** https://github.com/Newbie27-ms/medisimp

---

## Table of Contents
1. [The Problem & Solution](#1-the-problem--solution)
2. [Key Features](#2-key-features)
3. [System Architecture](#3-system-architecture)
4. [Download & Installation (macOS & Linux)](#4-download--installation-macos--linux)
5. [How to Use (All 3 Modes)](#5-how-to-use-all-3-modes)
6. [Example Input & Output](#6-example-input--output)
7. [Target Groups & Business Model](#7-target-groups--business-model)

---

## 1. The Problem & Solution

### The Problem
Clinical lab results, hospital discharge summaries, and doctor notes are difficult for patients to read because they are filled with specialized abbreviations (`HbA1c`, `eGFR`, `COPD`, `BID`, `PRN`). At the same time, pasting raw medical reports into cloud-based AI tools exposes sensitive personal data (full names, dates of birth, patient IDs, addresses, phone numbers, and email addresses).

### The Solution
`medisimp` acts as a **local privacy firewall and clinical translator**:
- **Before sharing or using AI:** It automatically detects and replaces personal identifiers with `[REDACTED]` locally on your machine.
- **For immediate understanding:** It scans the report for 40+ common clinical terms and appends a structured, plain-language glossary in clean Markdown.
- **Zero setup overhead:** Instead of heavy Python environments, Docker containers, or external API keys, `medisimp` compiles into a single standalone binary using only the Rust standard library.

---

## 2. Key Features

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

## 3. System Architecture

`medisimp` is intentionally engineered from scratch with **zero third-party crate dependencies** (`Cargo.toml` contains no external packages) so every line of logic is transparent, auditable, and easy to maintain:

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
