# medisimp

**100% offline medical text redaction & plain-language clinical glossary â€” built from scratch in Rust.**

`medisimp` strips sensitive patient identifiers (`[REDACTED]`) and translates 40+ medical abbreviations into plain Englishâ€”available in your browser, as a local offline Web UI, or as a fast CLI tool.

- **Try Live in Browser:** https://newbie27-ms.github.io/medisimp/
- **Download Latest Release:** https://github.com/Newbie27-ms/medisimp/releases/latest

---

## 1. Download & Install

### macOS â€” Apple Silicon (M1 / M2 / M3 / M4)
1. Download **`aarch64-apple-darwin.tar.gz`** from [Releases](https://github.com/Newbie27-ms/medisimp/releases/latest).
2. Extract and clear the macOS Gatekeeper flag once:
   ```bash
   cd ~/Downloads
   tar -xzf aarch64-apple-darwin.tar.gz
   xattr -d com.apple.quarantine ./medisimp
   ```

### macOS â€” Intel
1. Download **`x86_64-apple-darwin.tar.gz`** from [Releases](https://github.com/Newbie27-ms/medisimp/releases/latest).
2. Extract and clear the macOS Gatekeeper flag once:
   ```bash
   cd ~/Downloads
   tar -xzf x86_64-apple-darwin.tar.gz
   xattr -d com.apple.quarantine ./medisimp
   ```

### Linux (`x86_64` & `aarch64`)
1. Download **`x86_64-unknown-linux-musl.tar.gz`** (or `aarch64-unknown-linux-musl.tar.gz`) from [Releases](https://github.com/Newbie27-ms/medisimp/releases/latest).
2. Extract:
   ```bash
   tar -xzf x86_64-unknown-linux-musl.tar.gz
   ```B‚ˆÈÈÈZ[œ›ÛHÛİ\˜ÙH
[HÔÈÚ]\İ
B˜˜\Ú™Ú]ÛÛ™HÎ‹ËÙÚ]X‹˜ÛÛKÓ™]ØšYLË[\ËÛYY\Ú[\™Ú]	‰ˆÙYY\Ú[\˜Ø\™ÛÈ[œİ[K\]‚˜‚‹KKB‚ˆÈÈ‹ˆİÈÈ\ÙH
[[Ù\ÊB‚ˆÈÈÈ[ÙHNˆÙ™›[™HØØ[ÙXˆRH
]™\Y^H\Ù\œÊB“][˜ÚHZ[Z[ˆØØ[Ù\™\ˆ
™\›È[\›™]™\]Z\™Y
H[™Ü[ˆ
Š˜‹ËÌLËŒŒŒN
Šˆ[ˆ[İ\ˆœ›İÜÙ\‚˜˜\Ú‹‹ÛYY\Ú[\K]ZB˜‚ˆÈÈÈ[ÙHˆÛÛ[X[™S[™H	ˆ\[™È
]™[Ü\œÈ	ˆRHYÙ[ÊB˜˜\ÚˆÈ›ØÙ\ÜÈHYYXØ[™\Üš[B‹‹ÛYY\Ú[\™\Ü‚ˆÈ^ÜØ[š]^™YX\šÙİÛˆÈHš[B‹‹ÛYY\Ú[\™\Ü[ÈÛX[—Ü™\Ü›Y‚ˆÈ\H\™XİHœ›ÛHXXÓÔÈÛ\›Ø\™
ÛÜH^Ú]ÛY
ĞÈš\œİ
Bœœ\İH‹ÛYY\Ú[\‚ˆÈÙY\\œÛÛ˜[]Z[Èš\ÚX›H›Üˆš]˜]HÙ™›[™H™XY[™Â‹‹ÛYY\Ú[\™\ÜK[›Ë\™YXİ˜‚ˆÈÈÈ[ÙHÎˆ™\›ËR[œİ[œ›İÜÙ\ˆ[[Â“Ü[ˆ
ŠšÎ‹ËÛ™]ØšYLË[\Ë™Ú]X‹š[ËÛYY\Ú[\ÊŠˆÈ\İYY\Ú[\\™XİH[ˆ[İ\ˆœ›İÜÙ\ˆÚ]İ]İÛ›ØY[™È[][™