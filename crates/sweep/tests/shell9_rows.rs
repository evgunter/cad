//! **The `shell` verb's fixtures, shelled and dumped row by row.** The
//! row below shells the block and [`crate::common::shell_operands`]'
//! vessel and tube sealed and opened at their top, prints
//! every stored pcurve row of the result through
//! [`crate::common::pcurve_rows::print_rows`] (a corpus for a
//! base/head diff, taken as that module says), and asserts each one
//! BUILDS.
//!
//! The closing mint re-derives every row of every body `shell`
//! returns: on a body whose transferred rows were content-correct the
//! re-derived row is bit-identical, and a row that moves names a body
//! that was carrying a stale row tier 3 did not see.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::block;
use topo::Body;

use super::common::pcurve_rows::print_rows;
use super::common::shell_operands::{tube, vessel};
use super::shell7_common::tol;
use super::shell8_common::cap;

/// The top cap of a single-shell body revolved about `y`.
fn top(body: &Body<f64>, y: f64) -> Vec<topo::FaceKey> {
    let shell = body.shells().next().expect("a shell").0;
    cap(body, shell, Vec3::new(0.0, 1.0, 0.0), y)
}

fn shelled(label: &str, body: &Body<f64>, t: f64, open: &[topo::FaceKey]) {
    let out = topo::shell_open(body, t, open, tol()).unwrap_or_else(|e| panic!("{label}: {e}"));
    print_rows(label, &out.body);
}

/// The `shell` verb's fixtures, sealed and opened at their top; every
/// one builds.
#[test]
fn shell9_rows_verbs_shell_corpus() {
    let b = block(2.0, 3.0, 4.0, Tol::witness());
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
