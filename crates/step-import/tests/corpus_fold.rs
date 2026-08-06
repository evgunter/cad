//! **The #210 corpus fold's own acceptance row: committed-byte
//! identity, measured and classified, for the two fixtures the
//! widened exportable class brought in.**
//!
//! `roundtrip.rs` already runs `nonuniform_loft` and `swept_elbow`
//! through rows 1 and 2 with every other fixture — census and certified
//! volume against `KERNEL_*`, the tier ladder, and the fixed point
//! (second export byte-identical to the first). What it only *reports*
//! is the comparison of the FIRST re-export against the committed
//! bytes, because entity numbering and traversal may legitimately
//! differ.
//!
//! This suite makes that measurement an assertion for the two new
//! fixtures, in the shape M7-3 established for `loft_prism`
//! (`review_probes_m7_3.rs`'s V5 probe): the token streams must stay
//! ALIGNED, and every divergent pair must fall in a NAMED class. Two
//! classes are named:
//!
//! * `-0.0 → 0.0` — the documented `plus_zero` parse-hygiene idiom
//!   (`geometry.rs`): STEP's `-0.` and `0.` are distinct tokens but
//!   the same direction, and the importer normalizes. The fixed point
//!   holds from the first re-export on, which is what makes this a
//!   hygiene class and not a drift.
//! * the uncertainty record, and ONLY when the ambient ε differs from
//!   the 1e-9 the fixtures were written at (the CI matrix's 1e-6 /
//!   1e-12 rows).
//!
//! Anything else fails here by name. That is the point of the row: a
//! new corpus body is exactly where a new divergence class would first
//! appear, and it must not appear silently.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{fixture, import_body};

/// The two fixtures the #210 fold added, and their measured `-0.0`
/// token counts against the committed bytes at the corpus's ε.
///
/// The counts are DERIVED, not chosen: a `-0.0` component reaches the
/// file wherever the writer's frame construction produced a negated
/// zero, and the importer's `plus_zero` normalizes each one on the way
/// in. Pinning the number keeps the class from quietly growing.
const FOLD_FIXTURES: [(&str, usize); 2] = [("nonuniform_loft", 3), ("swept_elbow", 3)];

/// Is this divergent token pair the `plus_zero` class, EXACTLY?
///
/// The numeral must be `-0.0` on the committed side and `0.0` on the
/// re-exported side — matched whole, after splitting off the Part 21
/// punctuation that closes the enclosing list (`,`, `)`, `;`), which
/// must itself be unchanged. A substring test would have been exact on
/// today's corpus (all six divergent tokens are bare `-0.0` plus
/// punctuation) and would silently absorb a future `-0.05` at a
/// divergent position — the class-laundering this suite exists to
/// forbid, one level down.
fn is_plus_zero(committed: &str, reexported: &str) -> bool {
    let split = |t: &str| -> (String, String) {
        let numeral = t.trim_end_matches([',', ')', ';']);
        (numeral.to_owned(), t[numeral.len()..].to_owned())
    };
    let (x, x_tail) = split(committed);
    let (y, y_tail) = split(reexported);
    x == "-0.0" && y == "0.0" && x_tail == y_tail
}

#[test]
fn the_folds_committed_byte_divergence_stays_in_the_named_classes() {
    for (name, want_minus_zero) in FOLD_FIXTURES {
        let committed = fixture(name, "step");
        let (body, _eps) = import_body(name);
        let options = step_export::StepOptions {
            product_name: name.to_owned(),
            ..step_export::StepOptions::default()
        };
        let out = step_export::step_string(&body, &options).expect("re-export");

        let toks = |s: &str| -> Vec<String> { s.split_whitespace().map(str::to_owned).collect() };
        let a = toks(&committed);
        let b = toks(&out);
        assert_eq!(
            a.len(),
            b.len(),
            "{name}: token streams must stay aligned (a length change is a \
             structural divergence, not a token class)"
        );
        let diffs: Vec<(usize, &String, &String)> = a
            .iter()
            .zip(b.iter())
            .enumerate()
            .filter(|(_, (x, y))| x != y)
            .map(|(i, (x, y))| (i, x, y))
            .collect();
        let minus_zero = diffs.iter().filter(|(_, x, y)| is_plus_zero(x, y)).count();
        eprintln!(
            "{name}: {} tokens, {} differing pairs — {minus_zero} in the -0.0 class, \
             {} outside it",
            a.len(),
            diffs.len(),
            diffs.len() - minus_zero
        );
        for (i, x, y) in diffs.iter().take(12) {
            eprintln!("  {name} tok {i}: {x} -> {y}");
        }

        assert_eq!(
            minus_zero, want_minus_zero,
            "{name}: the -0.0 parse-hygiene class changed size"
        );
        for (i, x, y) in diffs.iter().filter(|(_, x, y)| !is_plus_zero(x, y)) {
            assert!(
                x.contains("1.0E-9") && y.contains("1.0E-"),
                "{name} token {i}: a divergence outside every NAMED class — \
                 report it, do not widen the classes: {x} -> {y}"
            );
            assert_ne!(
                geom_core::Tolerance::get().eps,
                1e-9,
                "{name}: at the corpus's own ε the uncertainty record must not diverge"
            );
        }
    }
}
