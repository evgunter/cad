//! The large-K fragility lint CLI (spec D3):
//! `k-lint [--gate-rule-1-only] <fresh.csv>...`
//!
//! Three exit voices, kept structurally separate because they mean
//! different things:
//!
//! * **findings** — exit [`EXIT_FINDINGS`], with the interpretation
//!   discipline printed in full (see [`discipline`]);
//! * **harness breakage** (no inputs, unreadable file, malformed CSV)
//!   — exit [`EXIT_HARNESS`], a bare error line on stderr; the lint
//!   could not run at all, which is not a statement about geometry;
//! * **clean** — exit 0.
//!
//! # Which findings gate (M10-6)
//!
//! By default all three rules gate: findings fail the run. That is the
//! corpus rows' setting and nothing about it changed.
//!
//! `--gate-rule-1-only` narrows the GATE to rule 1 — `indeterminate`
//! and `invalid` margins — while rules 2 and 3 still print, still
//! tally, and no longer fail the run. It exists for ONE caller, the E6
//! driver's own K population, whose subdivision refines margins toward
//! zero by construction and therefore crowds the escalation band in
//! bulk without anything being wrong; `docs/K-REPORT.md`'s recourse 2
//! is the demotion this implements, and ci.yml's step carries the
//! recorded justification.
//!
//! It is deliberately NOT `--advisory`: rule 1 is the trigger E6 names
//! for re-opening the K question, so a flag the caller cannot demote
//! is exactly the point. A per-rule TALLY prints on every run, gated
//! or not, so "zero rule-1 flags" is a reported number rather than an
//! inference from a green.
//!
//! Thresholds + baseline provenance: `lib.rs` module docs.

use k_lint::{Reason, lint_csv};

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
         \x20    recorded justification (ci.yml + local-scripts/ci-local.sh together — the\n\
         \x20    hosted and local rows must not drift).\n\
         \n\
         Changing geometry to get under a lint threshold is the one forbidden move.\n"
    )
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    // The one flag, recognised before the paths so a caller cannot
    // accidentally lint a file called `--gate-rule-1-only`.
    let gate_rule_1_only = args.iter().any(|a| a == "--gate-rule-1-only");
    args.retain(|a| a != "--gate-rule-1-only");
    if let Some(bad) = args.iter().find(|a| a.starts_with("--")) {
        eprintln!("k-lint: unknown option {bad}");
        std::process::exit(EXIT_HARNESS);
    }
    let paths: Vec<String> = args;
    if paths.is_empty() {
        eprintln!("k-lint: usage: k-lint [--gate-rule-1-only] <k-probe-csv>...");
        std::process::exit(EXIT_HARNESS);
    }
    let mut total_flags = 0usize;
    // Per-RULE totals across every input, printed unconditionally.
    // Rule 1's count is the number this row exists to report, and a
    // number nobody prints is a number nobody reads.
    let mut per_rule = [0usize; 4];
    // The symbolic column, summed the same way and printed the same
    // way: a decision the identity tier answered is not a rule sample
    // (lib.rs, `lint_sample`), and a population that is mostly such
    // decisions would otherwise read as "0 flagged" with no hint that
    // most of its rows never met a threshold at all.
    let mut total_scanned = 0usize;
    let mut total_symbolic = 0usize;
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
        // The per-file tally, by rule and then by reason. A FLAG can
        // carry several reasons; each is counted, so the reason counts
        // sum to at least the flag count and the two are labelled
        // differently rather than conflated.
        let mut file_rule = [0usize; 4];
        let mut file_reason = [0usize; Reason::ALL.len()];
        for f in &flags {
            for r in &f.reasons {
                file_rule[usize::from(r.rule())] += 1;
                let idx = Reason::ALL
                    .iter()
                    .position(|c| c == r)
                    .expect("Reason::ALL lists every variant");
                file_reason[idx] += 1;
            }
        }
        for (i, c) in file_rule.iter().enumerate() {
            per_rule[i] += c;
        }
        total_scanned += scanned;
        total_symbolic += scan.symbolic;
        say(format_args!(
            "k-lint: {path}: {scanned} samples ({} symbolic_zero, {} classified), {} flagged \
             — rule 1 (undecided/invalid): {}, rule 2 (near a threshold): {}, rule 3 (below a \
             floor): {}",
            scan.symbolic,
            scanned - scan.symbolic,
            flags.len(),
            file_rule[1],
            file_rule[2],
            file_rule[3]
        ));
        for (r, c) in Reason::ALL.iter().zip(file_reason) {
            if c > 0 {
                say(format_args!("    rule {} — {r}: {c}", r.rule()));
            }
        }
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
    say(format_args!(
        "k-lint: TOTAL over {} file(s): {total_scanned} samples ({total_symbolic} \
         symbolic_zero, {} classified), rule 1 (undecided/invalid) {}, rule 2 (near a \
         threshold) {}, rule 3 (below a floor) {}",
        paths.len(),
        total_scanned - total_symbolic,
        per_rule[1],
        per_rule[2],
        per_rule[3]
    ));
    // WHICH flags decide the exit. Rule 1 always does; rules 2 and 3
    // do unless this caller demoted them, and the demotion is stated
    // in the output rather than inferred from a green.
    let gating = if gate_rule_1_only {
        say(format_args!(
            "k-lint: --gate-rule-1-only — rules 2 and 3 are ADVISORY for this caller \
             (docs/K-REPORT.md recourse 2; the justification is at the calling step). \
             Rule 1 is NOT demotable: it is the trigger E6 names."
        ));
        per_rule[1]
    } else {
        total_flags
    };
    if gating > 0 {
        // stderr, and stderr only: this verdict must survive a closed
        // or redirected stdout — it is the reason the row is red.
        eprint!("{}", discipline(gating));
        std::process::exit(EXIT_FINDINGS);
    }
    if total_flags > 0 {
        say(format_args!(
            "k-lint: {total_flags} advisory flag(s), none of them rule 1 — the population \
             crowds thresholds but every margin was DECIDED"
        ));
        return;
    }
    say(format_args!(
        "k-lint: clean — no margin crowds a decision boundary"
    ));
}
