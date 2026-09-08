//! **Every stored pcurve row of a body, as text — one instrument, two
//! readers.** [`rows`] is the bit-for-bit form (`{:?}` of every number
//! is its shortest round-trip spelling, so equal text is equal bits):
//! `shell9_probe` compares two bodies' rows with it, and [`print_rows`]
//! is the dump form the corpora (`shell5_r1_dump`, `shell7_dump`,
//! `shell8_dump`) call on every body they dump, for a base/head diff
//! (`--nocapture`, grep `[rows]`, sort, diff). The row below shells
//! `verbs_shell`'s fixtures the same way and asserts each BUILDS.
//!
//! The closing mint re-derives every row of every body `shell`
//! returns: on a body whose transferred rows were content-correct the
//! re-derived row is bit-identical, and a row that moves names a body
//! that was carrying a stale row tier 3 did not see.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Vec3;
use topo::Body;

use super::shell7_common::{face_of_he, tol};
use super::shell8_common::cap;
use super::verbs_shell::{boxy, tube, vessel};

/// One line per stored row, in half-edge-slot order: the half-edge,
/// its face, the parameter window and the image.
pub(crate) fn rows(body: &Body<f64>) -> Vec<String> {
    body.pcurves()
        .map(|(he, cache)| {
            format!(
                "he {he:?} face {:?} params {:?} pcurve {:?}",
                face_of_he(body, he),
                cache.params(),
                cache.pcurve()
            )
        })
        .collect()
}

/// [`rows`] printed under `label`, with the count.
pub(crate) fn print_rows(label: &str, body: &Body<f64>) {
    let rows = rows(body);
    for row in &rows {
        println!("[rows] {label}: {row}");
    }
    println!("[rows] {label}: {} rows", rows.len());
}

/// The top cap of a single-shell body revolved about `y`.
fn top(body: &Body<f64>, y: f64) -> Vec<topo::FaceKey> {
    let shell = body.shells().next().expect("a shell").0;
    cap(body, shell, Vec3::new(0.0, 1.0, 0.0), y)
}

fn shelled(label: &str, body: &Body<f64>, t: f64, open: &[topo::FaceKey]) {
    let out = topo::shell_open(body, t, open, tol()).unwrap_or_else(|e| panic!("{label}: {e}"));
    print_rows(label, &out.body);
}

/// `verbs_shell`'s fixtures, sealed and opened at their top; every one
/// builds.
#[test]
fn shell9_rows_verbs_shell_corpus() {
    let b = boxy(2.0, 3.0, 4.0);
    shelled("box sealed", &b, 0.25, &[]);
    let v = vessel(1.0, 2.0);
    shelled("vessel sealed", &v, 0.2, &[]);
    shelled("vessel opened top", &v, 0.2, &top(&v, 2.0));
    let u = tube(0.6, 1.0, 2.0);
    shelled("tube sealed", &u, 0.1, &[]);
    shelled("tube opened top", &u, 0.1, &top(&u, 2.0));
    let hollow = topo::shell(&v, 0.2, tol()).expect("hollows").body;
    shelled("hollow vessel shelled again", &hollow, 0.05, &[]);
}
