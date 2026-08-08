//! The large-K fragility lint CLI (spec D3): `k-lint <fresh.csv>...`
//!
//! A GATE: findings fail the run. Three exit voices, kept structurally
//! separate because they mean different things:
//!
//! * **findings** — exit [`EXIT_FINDINGS`], with the interpretation
//!   discipline printed in full (see [`discipline`]);
//! * **harness breakage** (no inputs, unreadable file, malformed CSV)
//!   — exit [`EXIT_HARNESS`], a bare error line on stderr; the lint
//!   could not run at all, which is not a statement about geometry;
//! * **clean** — exit 0.
//!
//! Thresholds + baseline provenance: `lib.rs` module docs.

use k_lint::lint_csv;

/// The lint ran and found margins crowding a decision boundary.
const EXIT_FINDINGS: i32 = 2;

/// The lint could not run: no inputs, unreadable file, malformed CSV.
/// Distinct from [`EXIT_FINDINGS`] on purpose — blurring the two would
/// let a sweep-format drift read as a geometry finding, or vice versa.
const EXIT_HARNESS: i32 = 1;

/// Stdout write guard: a closed pipe downstream (`k-lint … | head`)
/// must end the run quietly, not panic — and must NOT change the
/// verdict. Once stdout is gone we stop printing to it and keep
/// scanning; the exit code comes from the findings, never from whether
/// the log reached its reader. Everything loud goes to stderr.
fn say(args: std::fmt::Arguments<'_>) {
    use std::io::Write as _;
    use std::sync::atomic::{AtomicBool, Ordering};
    static CLOSED: AtomicBool = AtomicBool::new(false);
    if CLOSED.load(Ordering::Relaxed) {
        return;
    }
    if writeln!(std::io::stdout(), "{args}").is_err() {
        CLOSED.store(true, Ordering::Relaxed);
    }
}

/// The failure message. Leads with the interpretation discipline
/// because the tempting wrong move is a real one: a fired lint is
/// evidence about the MARGIN DISTRIBUTION, and geometry nudged until
/// the lint goes quiet destroys exactly that evidence.
fn discipline(total_flags: usize) -> String {
    format!(
        "\nk-lint: GATE FAILED — the margin distribution changed: {total_flags} margin(s) \
         crowd a decision\nboundary that the committed baseline says should be empty.\n\
         \n\
         If the flagged margins are REAL, INTENDED geometry, do NOT change the geometry\n\
         to silence this lint. A fired lint is evidence ABOUT THE MARGIN DISTRIBUTION\n\
         — possibly that the threshold or the baseline is stale — not a geometry defect.\n\
         \n\
         Recourse, in order:\n\
         \x20 1. Re-derive the baseline and the thresholds per the snapshot-contract\n\
         \x20    runbook: docs/K-REPORT.md, \"M7 addendum (2026-08-07): the large-K\n\
         \x20    lint's floor refresh\", which re-derives BASELINE_FLOOR_MARGIN, the\n\
         \x20    percentile choice and the eps-coupled ratio against a fresh sweep.\n\
         \x20 2. If re-derivation is not warranted, demote this row to advisory with a\n\
         \x20    recorded justification (ci.yml + scripts/ci-local.sh together — the\n\
         \x20    hosted and local rows must not drift).\n\
         \n\
         Changing geometry to get under a lint threshold is the one forbidden move.\n"
    )
}

fn main() {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("k-lint: usage: k-lint <k-probe-csv>...");
        std::process::exit(EXIT_HARNESS);
    }
    let mut total_flags = 0usize;
    for path in &paths {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("k-lint: cannot read {path}: {e}");
                std::process::exit(EXIT_HARNESS);
            }
        };
        let scan = match lint_csv(&text) {
            Ok(r) => r,
            Err(e) => {
                eprintln!(
                    "k-lint: {path}:{}: malformed sweep row (harness breakage): {}",
                    e.line, e.text
                );
                std::process::exit(EXIT_HARNESS);
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
                    "  FLAG {}:{} line {}: |m|={:e} band_zero={:e} — {r}",
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
        // stderr, and stderr only: this verdict must survive a closed
        // or redirected stdout — it is the reason the row is red.
        eprint!("{}", discipline(total_flags));
        std::process::exit(EXIT_FINDINGS);
    }
    say(format_args!(
        "k-lint: clean — no margin crowds a decision boundary"
    ));
}
