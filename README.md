# medisimp

**Privacy-first medical text redaction and plain-language glossary CLI for the Agentic OS.**

`medisimp` is a fast, zero-dependency command-line tool written in Rust. It prepares raw clinical notes and lab reports for AI agents or patients by automatically redacting personal identifiable information (PII) and appending plain-language explanations for medical abbreviations.

## Why medisimp?
Raw medical notes contain sensitive patient identifiers and dense clinical jargon (`HbA1c`, `WBC`, `HTN`, `ECG`). Pasting them directly into AI agents leaks privacy and wastes tokens. `medisimp` sanitizes and structures medical text locally in milliseconds before any data leaves your machine.

## Installation
Download the pre-compiled static binary from the [Releases](https://github.com/Newbie27-ms/medisimp/releases) page:
- Linux ARM64 (`aarch64-unknown-linux-musl`)
- Linux x86_64 (`x86_64-unknown-linux-musl`)

Or build from source:

    cargo build --release

## Usage

    # Redact sensitive info and generate plain-language glossary
    medisimp sample_report.txt

    # Keep patient identifiers (skip redaction)
    medisimp sample_report.txt --no-redact

## Business & Pricing Model (Open-Core)
- **Community Edition (Free / Open-Source):** Core CLI binary, local PII redaction, and standard medical abbreviation glossary for students, patients, and developers.
- **Pro / Clinical Team License (€19/month per seat):** Extended ICD-10 / LOINC dictionaries, custom regex redaction rules for hospital EHR exports, and HIPAA/GDPR audit logging.
