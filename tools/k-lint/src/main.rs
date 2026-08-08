//! The large-K lint CLI (M4 PR 8b, D3): `k-lint <fresh.csv>...`
//!
//! ADVISORY (first iteration): flags PRINT and the exit code stays 0 —
//! the CI row never fails on a finding. It fails (exit 1) only on
//! harness breakage: no inputs, unreadable file, malformed CSV.
//! Thresholds + baseline provenance: `lib.rs` module docs.

use k_lint::lint_csv;

/// Stdout write guard: a closed pipe downstream (`k-lint … | head`)
/// must end the run quietly, not panic — advisory output is not
/// harness breakage. Everything loud goes to stderr + exit 1.
fn say(args: std::fmt::Arguments<'_>) {
    use std::io::Write as _;
    if writeln!(std::io::stdout(), "{args}").is_err() {
        std::process::exit(0);
    }
}

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
        let scan = match lint_csv(&text) {
            Ok(r) => r,
            Err(e) => {
                eprintln!(
                    "k-lint: {path}:{}: malformed sweep row (harness breakage): {}",
                    e.line, e.text
                );
                std::process::exit(1);
            }
        };
        let (scanned, flags) = (scan.scanned, scan.flags);
        say(format_args!(
            "k-lint: {path}: {scanned} samples, {} flagged",
            flags.len()
        ));
        // Never a silent exemption: say it whenever rule (2)'s definite
        // arm ran capped at the baseline floor (lib.rs, "Rule (2)'s
        // discrimination floor").
        if let Some((raw, floor)) = scan.proximity_capped {
            say(format_args!(
                "  note: rule 2's definite arm (10^2*Keps = {raw:e}) is looser than \
                 the baseline floor ({floor:e}) at this eps row and ran capped to it \
                 — at this eps the corpus's honest fine features sit less than a \
                 decade above the escalation band; the floor is the calibrated statement"
            ));
        }
        // Print every flag, but cap the per-file dump so a systematic
        // regression cannot drown the job log; the summary count above
        // is always complete.
        const CAP: usize = 200;
        for f in flags.iter().take(CAP) {
            for r in &f.reasons {
                say(format_args!(
                    "  ADVISORY {}:{} line {}: |m|={:e} band_zero={:e} — {r}",
                    f.shape,
                    f.predicate,
                    f.line,
                    f.margin.abs(),
                    f.band_zero
                ));
            }
        }
        if flags.len() > CAP {
            say(format_args!(
                "  … {} more flags (capped print)",
                flags.len() - CAP
            ));
        }
        total_flags += flags.len();
    }
    if total_flags > 0 {
        say(format_args!(
            "k-lint: {total_flags} ADVISORY flag(s) — printed, not failing \
             (first-iteration posture, M4 PR 8b D3; gate once the \
             baseline is trusted)"
        ));
    } else {
        say(format_args!(
            "k-lint: clean — no margin crowds a decision boundary"
        ));
    }
}
