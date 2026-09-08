//! **SHELL-10's acceptance: the doors' closing pcurve pass is
//! scope-sized.**
//!
//! A simultaneous door writes over the solids a move set names, and
//! since this unit it re-derives the pcurve rows of those solids' faces
//! and no others. These rows count what the pass mints and compare the
//! out-of-scope solid's rows bit for bit, on a two-solid body built
//! through the public doors.
//!
//! The scope's own construction and the out-of-scope-corruption row are
//! `topo`'s (`offset_together::scope_walks`) — they need the crate's
//! raw corruption door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use topo::{Body, FaceKey, SolidKey};

use crate::shell8_common::{band, beside, charts_of, deep_dump, faces_of, tol};
use crate::shell9_rows::rows;
use crate::verbs_shell::{boxy, vessel};

/// The stored rows of `solid`'s faces, in half-edge-slot order.
fn rows_of(body: &Body<f64>, solid: SolidKey) -> Vec<String> {
    let mine: Vec<FaceKey> = faces_of(body, solid);
    body.pcurves()
        .filter(|(he, _)| {
            let lp = body.get_half_edge(*he).unwrap().parent_loop;
            mine.contains(&body.get_loop(lp).unwrap().face)
        })
        .map(|(he, c)| format!("he {he:?} params {:?} pcurve {:?}", c.params(), c.pcurve()))
        .collect()
}

/// **`mint_pcurves_of` mints exactly the scope's rows.** A vessel
/// beside a vessel: both solids carry rows, and the subset pass over
/// one solid's faces returns that solid's row count and leaves the
/// other's rows bit-identical — the same reading `mint_pcurves` over
/// the whole body produces for the solid it did visit.
#[test]
fn the_subset_pass_mints_the_scopes_rows_and_no_others() {
    let pair = beside(&vessel(1.0, 2.0), &vessel(1.0, 2.0), 10.0);
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    assert_eq!(solids.len(), 2);

    let mut whole = pair.clone();
    topo::mint_pcurves(&mut whole, tol()).expect("the whole-body pass mints");
    let want_a = rows_of(&whole, solids[0]);
    let want_b = rows_of(&whole, solids[1]);
    assert!(
        !want_a.is_empty() && !want_b.is_empty(),
        "both solids carry rows, or this row measures nothing"
    );

    let mut scoped = whole.clone();
    let in_scope = faces_of(&scoped, solids[0]);
    let minted =
        topo::mint_pcurves_of(&mut scoped, &in_scope, tol()).expect("the subset pass mints");
    println!(
        "[10] subset pass: minted={minted} scope rows={} other rows={} total={}",
        want_a.len(),
        want_b.len(),
        whole.pcurves().count()
    );
    assert_eq!(minted, want_a.len(), "exactly the scope's rows");
    assert_eq!(rows_of(&scoped, solids[0]), want_a, "same rows, same bits");
    assert_eq!(rows_of(&scoped, solids[1]), want_b, "the other's untouched");
    assert_eq!(scoped.pcurves().count(), whole.pcurves().count());
}

/// **Through the door.** The axial simultaneous door on a vessel beside
/// a box re-derives the vessel's rows and leaves the box's — every row,
/// and the whole deep reading SHELL-8 pinned — exactly as it found
/// them. The whole-body pass produced the same bits by re-deriving
/// them; this pins that it no longer derives them at all.
#[test]
fn the_axial_door_re_mints_its_scopes_rows_and_reads_no_others() {
    let pair = beside(&vessel(1.0, 2.0), &boxy(2.0, 3.0, 4.0), 10.0);
    let solids: Vec<SolidKey> = pair.solids().map(|(k, _)| k).collect();
    let (ves, bx) = (solids[0], solids[1]);
    let mut before = pair.clone();
    topo::mint_pcurves(&mut before, tol()).expect("the operand's rows mint");

    let box_rows = rows_of(&before, bx);
    let box_deep = deep_dump(&before, bx);
    let all_before = rows(&before);

    let moves: Vec<topo::ChartMove<f64>> = charts_of(&before, ves)
        .into_iter()
        .map(|faces| topo::ChartMove {
            faces,
            distance: -0.05,
        })
        .collect();
    let mut after = before.clone();
    topo::offset_charts_together(&mut after, &moves, band(), tol()).expect("the vessel offsets");

    println!(
        "[10] axial door: rows before={} after={} box rows={}",
        all_before.len(),
        rows(&after).len(),
        box_rows.len()
    );
    assert_eq!(rows_of(&after, bx), box_rows, "the box's rows, bit for bit");
    assert_eq!(deep_dump(&after, bx), box_deep, "and the box itself");
    assert!(
        !rows_of(&after, ves).is_empty(),
        "the vessel's rows are there"
    );
}
