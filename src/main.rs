use std::env;
use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::net::TcpListener;
use std::process;

const UI_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>medisimp — Local Offline Medical Simplifier</title>
  <style>
    :root { --bg: #0f172a; --card: #1e293b; --border: #334155; --text: #f8fafc; --muted: #94a3b8; --accent: #3b82f6; --green: #10b981; }
    * { box-sizing: border-box; }
    body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; margin: 0; padding: 28px 20px; background: var(--bg); color: var(--text); }
    .wrap { max-width: 1040px; margin: 0 auto; }
    header { display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 12px; border-bottom: 1px solid var(--border); padding-bottom: 16px; margin-bottom: 24px; }
    h1 { margin: 0; font-size: 1.75rem; }
    .pill { background: rgba(16,185,129,0.15); color: var(--green); border: 1px solid rgba(16,185,129,0.4); padding: 4px 12px; border-radius: 999px; font-size: 0.82rem; font-weight: 600; }
    .split { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; }
    @media (max-width: 768px) { .split { grid-template-columns: 1fr; } }
    .panel { background: var(--card); border: 1px solid var(--border); border-radius: 12px; padding: 18px; }
    label { display: block; font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--muted); margin-bottom: 10px; font-weight: 600; }
    textarea, pre { width: 100%; height: 360px; background: #0b1120; color: #e2e8f0; border: 1px solid var(--border); border-radius: 8px; padding: 14px; font-family: ui-monospace, SFMono-Regular, Menlo, monospace; font-size: 0.9rem; line-height: 1.5; margin: 0; }
    pre { overflow-y: auto; white-space: pre-wrap; }
    .bar { display: flex; justify-content: space-between; align-items: center; margin-top: 14px; flex-wrap: wrap; gap: 10px; }
    button { background: var(--accent); color: #fff; border: none; padding: 10px 20px; border-radius: 8px; font-weight: 600; cursor: pointer; font-size: 0.95rem; }
    button.sec { background: #334155; }
  </style>
</head>
<body>
  <div class="wrap">
    <header>
      <div>
        <h1>medisimp — Local Medical Report Simplifier</h1>
        <small style="color: var(--muted);">Powered by your local Rust engine (100% offline, zero data leaves your device)</small>
      </div>
      <span class="pill">Offline Local Server • localhost:8080</span>
    </header>
    <div class="split">
      <div class="panel">
        <label for="input">Raw Medical Report / Lab Note</label>
        <textarea id="input" placeholder="Paste any lab report or discharge summary here...">Patient: Lukas Weber
DOB: 19.04.1968
ID: PAT-449102
Email: lukas.weber@example.de
Hx: T2DM, HTN, and mild COPD.
Labs: HbA1c 7.9%, LDL 148 mg/dL, eGFR 74, WBC 11.4.
Rx: Metformin 1000 mg PO BID; Salbutamol PRN for SOB.</textarea>
        <div class="bar">
          <label style="margin:0; text-transform:none; color:var(--text); cursor:pointer;">
            <input type="checkbox" id="redact" checked> Redact Personal Info (PII)
          </label>
          <button onclick="processText()">Simplify Report</button>
        </div>
      </div>
      <div class="panel">
        <label>Sanitized Report & Plain-Language Glossary</label>
        <pre id="output">Click "Simplify Report" to process via Rust backend...</pre>
        <div class="bar">
          <span style="font-size:0.82rem; color:var(--muted);">Processed locally by medisimp Rust binary</span>
          <button class="sec" onclick="navigator.clipboard.writeText(document.getElementById('output').textContent)">Copy Markdown</button>
        </div>
      </div>
    </div>
  </div>
  <script>
    async function processText() {
      const text = document.getElementById('input').value;
      const redact = document.getElementById('redact').checked ? '1' : '0';
      const res = await fetch('/api/simplify?redact=' + redact, { method: 'POST', body: text });
      document.getElementById('output').textContent = await res.text();
    }
    window.onload = processText;
  </script>
</body>
</html>"#;

fn run_local_ui() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap_or_else(|err| {
        eprintln!("Could not start local UI on {}: {}", addr, err);
        process::exit(1);
    });

    println!("medisimp Local Web UI running at: http://{}", addr);
    println!("100% offline — open that link in your browser. Press Ctrl+C to stop.");

    for stream in listener.incoming().flatten() {
        let mut stream = stream;
        let mut buffer = [0u8; 65536];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(n) if n > 0 => n,
            _ => continue,
        };

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let first_line = request.lines().next().unwrap_or("");

        if first_line.starts_with("POST /api/simplify") {
            let redact = !first_line.contains("redact=0");
            let body = request.split("\r\n\r\n").nth(1).unwrap_or("");
            let result = medisimp::simplify_report(body, redact);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                result.len(),
                result
            );
            let _ = stream.write_all(response.as_bytes());
        } else {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                UI_HTML.len(),
                UI_HTML
            );
            let _ = stream.write_all(response.as_bytes());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("medisimp - Privacy-first medical text redaction and plain-language glossary CLI\n");
        println!("USAGE:");
        println!("  medisimp --ui                                  Launch local offline web UI on http://127.0.0.1:8080");
        println!("  medisimp <report.txt> [--no-redact] [-o out.md]");
        println!("  cat report.txt | medisimp [--no-redact]");
        process::exit(0);
    }

    if args.iter().any(|a| a == "--ui") {
        run_local_ui();
        return;
    }

    let redact_pii = !args.iter().any(|a| a == "--no-redact");

    let output_path = args
        .iter()
        .position(|a| a == "-o" || a == "--output")
        .and_then(|idx| args.get(idx + 1));

    let mut input_file: Option<&String> = None;
    let mut skip_next = false;
    for arg in args.iter().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "-o" || arg == "--output" {
            skip_next = true;
            continue;
        }
        if !arg.starts_with('-') {
            input_file = Some(arg);
            break;
        }
    }

    let content = if let Some(path) = input_file {
        fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("Error reading '{}': {}", path, err);
            process::exit(1);
        })
    } else if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap_or_else(|err| {
            eprintln!("Error reading from stdin: {}", err);
            process::exit(1);
        });
        buffer
    } else {
        eprintln!("Error: Provide a file path, pipe via stdin, or run `medisimp --ui`.");
        process::exit(1);
    };

    let simplified = medisimp::simplify_report(&content, redact_pii);

    if let Some(out_path) = output_path {
        fs::write(out_path, &simplified).unwrap_or_else(|err| {
            eprintln!("Error writing to '{}': {}", out_path, err);
            process::exit(1);
        });
        println!("Saved sanitized Markdown report to '{}'", out_path);
    } else {
        println!("{}", simplified);
    }
}
