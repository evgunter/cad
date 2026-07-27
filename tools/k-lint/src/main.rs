//! The large-K lint CLI (M4 PR 8b, D3): `k-lint <fresh.csv>...`
//!
//! ADVISORY (first iteration): flags PRINT and the exit code stays 0 —
//! the CI row never fails on a finding. It fails (exit 1) only on
//! harness breakage: no inputs, unreadable file, malformed CSV.
//! Thresholds + baseline provenance: `lib.rs` module docs.

use k_lint::lint_csv;

fn main() {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("usage: k-lint <k-probe-csv>...");
        std::process::exit(1);
    }
    let mut total_flags = 0usize;
    for path in &paths {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("k-lint: cannot read {path}: {e}");
                std::process::exit(1);
            }
        };
        let (scanned, flags) = match lint_csv(&text) {
            Ok(r) => r,
            Err(e) => {
                eprintln!(
                    "k-lint: {path}:{}: malformed sweep row (harness breakage): {}",
                    e.line, e.text
                );
                std::process::exit(1);
            }
        };
        println!("k-lint: {path}: {scanned} samples, {} flagged", flags.len());
        // Print every flag, but cap the per-file dump so a systematic
        // regression cannot drown the job log; the summary count above
        // is always complete.
        const CAP: usize = 200;
        for f in flags.iter().take(CAP) {
            for r in &f.reasons {
                println!(
                    "  ADVISORY {}:{} line {}: |m|={:e} band_zero={:e} — {r}",
                    f.shape,
                    f.predicate,
                    f.line,
                    f.margin.abs(),
                    f.band_zero
                );
            }
        }
        if flags.len() > CAP {
            println!("  … {} more flags (capped print)", flags.len() - CAP);
        }
        total_flags += flags.len();
    }
    if total_flags > 0 {
        println!(
            "k-lint: {total_flags} ADVISORY flag(s) — printed, not failing \
             (first-iteration posture, M4 PR 8b D3; gate once the \
             baseline is trusted)"
        );
    } else {
        println!("k-lint: clean — no margin crowds a decision boundary");
    }
}
