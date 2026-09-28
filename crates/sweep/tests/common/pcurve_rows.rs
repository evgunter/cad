//! **Every stored pcurve row of a body, as text — one instrument, two
//! readers.** [`rows`] is the bit-for-bit form (`{:?}` of every number
//! is its shortest round-trip spelling, so equal text is equal bits):
//! `shell9_probe`, `shell10_r2_dump` and `shell10_scoped_walks` compare
//! two bodies' rows with it, and [`print_rows`] is the dump form the
//! corpora (`shell5_r1_dump`, `shell7_dump`, `shell8_dump`,
//! `shell9_rows`) call on every body they dump, for a base/head diff.
//! What a suite CHECKS of a body it built, reading stored data and
//! evaluating nothing, so it routes here beside [`super::cap_rims`]
//! ([`super`]'s routing rule).
//!
//! **How to take that diff, exactly.** `--nocapture` prints from every
//! test thread into one stream, so two suites' lines interleave and a
//! line can be split across a write; both SHELL-10 review lanes hit
//! the same spurious one-line diff that way. Run the corpora with
//! `-- --test-threads=1 --nocapture`, filter with a grep ANCHORED at
//! the tag (`grep -E '^\[rows\]'`, not a bare `[rows]`), then sort and
//! diff. A diff of one line whose neighbours are identical is this
//! artefact, not a moved row — re-run single-threaded before reading
//! it as a finding.
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - `shell9_r2_dump`'s `rows(label, body)`, a different dump — its own
//!   `[r2rows]` tag, no face column and a tier-3 line — which that
//!   suite's diff is taken over;
//! - `revert_periodic_wrap`'s `rows`, which keys each row's whole
//!   `Debug` (certificate included) by its half-edge for that suite's
//!   own comparisons.

use topo::Body;

/// One line per stored row, in half-edge-slot order: the half-edge,
/// its face, the parameter window and the image.
pub fn rows(body: &Body<f64>) -> Vec<String> {
    body.pcurves()
        .map(|(he, cache)| {
            format!(
                "he {he:?} face {:?} params {:?} pcurve {:?}",
                body.face_of_half_edge(he).unwrap(),
                cache.params(),
                cache.pcurve()
            )
        })
        .collect()
}

/// [`rows`] printed under `label`, with the count.
pub fn print_rows(label: &str, body: &Body<f64>) {
    let rows = rows(body);
    for row in &rows {
        println!("[rows] {label}: {row}");
    }
    println!("[rows] {label}: {} rows", rows.len());
}
