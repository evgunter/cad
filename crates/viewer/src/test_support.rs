//! **The fixture doors a test in this crate authors a document with** —
//! literals of each dimension, the world xy frame, an axis-aligned
//! rectangle, and one edit or insert through the document's own `apply`.
//!
//! Module kind: **vocabulary** — it names no driver type and no toolkit
//! type; every door is a pure function over `pncad`'s document values.
//!
//! ONE text, compiled into two crates. `lib.rs` declares it
//! `#[cfg(test)]`, so this crate's unit-test modules reach it as
//! `crate::test_support`; and `tests/common/mod.rs` mounts this same
//! file by `#[path]` and re-exports every door, so the integration
//! suites' `common::len` and a unit test's `test_support::len` are one
//! definition rather than two that can drift. That is also why nothing
//! here names `crate::` or `viewer::`: the path would mean a different
//! crate in each of the two binaries, so the file speaks only through
//! `pncad`, which both of them depend on.
//!
//! No door here carries an oracle. Each is the spelling of a value the
//! document vocabulary already has, and a row that reads one asserts
//! about what it built with it.

// One instance per binary; no single consumer uses all of it, and the
// `app`-gated modules that read most of it are absent from a
// default-feature build.
#![allow(dead_code)]
#![allow(unreachable_pub)]
// why: root Cargo.toml, the `unreachable_pub` stanza
// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]

use pncad::document::{
    Datum, Dimension, Doc, DocEdit, Expr, LoopProgram, Node, ProfileProgram, RecipeNodeId,
    RefusingReach, apply,
};
use pncad::geom_core::Tol;

// --- literals -------------------------------------------------------

/// A length literal.
pub fn len(metres: f64) -> Expr {
    Expr::literal(metres, Dimension::Length).expect("a finite length")
}

/// A length literal that remembers it was WRITTEN in millimetres —
/// `len` lowers canonically and carries no notation, which is what a
/// row about the unit a literal keeps cannot use.
pub fn len_mm(metres: f64) -> Expr {
    Expr::literal_with_unit(metres, Dimension::Length, pncad::prelude::MM.def())
        .expect("a finite length")
}

/// A dimensionless literal.
pub fn scl(value: f64) -> Expr {
    Expr::literal(value, Dimension::Scalar).expect("a finite scalar")
}

/// An angle literal.
pub fn ang(radians: f64) -> Expr {
    Expr::literal(radians, Dimension::Angle).expect("a finite angle")
}

/// Three length literals — a datum origin, a translation.
pub fn len3(v: [f64; 3]) -> [Expr; 3] {
    [len(v[0]), len(v[1]), len(v[2])]
}

/// Three dimensionless literals — a normal, a direction, an axis.
pub fn scl3(v: [f64; 3]) -> [Expr; 3] {
    [scl(v[0]), scl(v[1]), scl(v[2])]
}

/// Two length literals — a point in a sketch frame's own coordinates.
pub fn len2(v: [f64; 2]) -> [Expr; 2] {
    [len(v[0]), len(v[1])]
}

/// Two dimensionless literals — a direction in a sketch frame.
pub fn scl2(v: [f64; 2]) -> [Expr; 2] {
    [scl(v[0]), scl(v[1])]
}

// --- the document's own edit door -----------------------------------
//
// Authored through `apply`, in the order a user would: a fixture that
// reached past it would be testing a document the edit vocabulary
// cannot produce. The fixtures are part-less — no mate, no cluster — so
// the reach is the refusing one and is never asked.

/// Apply one edit, answering the new document and any minted id.
pub fn edited(
    doc: &Doc<ProfileProgram>,
    edit: DocEdit<ProfileProgram>,
    tol: Tol,
) -> (Doc<ProfileProgram>, Option<RecipeNodeId>) {
    let applied = apply(doc, &edit, tol, &RefusingReach).expect("the fixture's edit applies");
    (applied.doc, applied.record.minted)
}

/// Insert a node through the document's own door, answering the new
/// document and the minted id.
pub fn inserted(
    doc: &Doc<ProfileProgram>,
    node: Node<ProfileProgram>,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, minted) = edited(doc, DocEdit::InsertNode { node }, tol);
    (doc, minted.expect("an insert mints an id"))
}

// --- the nodes a fixture sketches with ------------------------------

/// A sketch frame node's payload.
pub fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> Node<ProfileProgram> {
    Node::Datum(Datum::Frame {
        origin: len3(origin),
        u: scl3(u),
        v: scl3(v),
    })
}

/// The world xy frame's payload — the plane these fixtures sketch on.
pub fn xy_frame() -> Node<ProfileProgram> {
    frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
}

/// An axis-aligned rectangular loop, `w` by `h`, its lower-left
/// corner at `origin` in the plane's own coordinates, counter-clockwise
/// from that corner — the loop [`rectangle`] draws, for a
/// `SessionOp::AddProfile` that takes loops rather than a node.
///
/// # Panics
///
/// Unless `w` and `h` are both positive: a non-positive side would
/// turn the winding or collapse the loop, and "lower-left,
/// counter-clockwise" would stop being true of what it returns.
pub fn rectangle_loop(origin: [f64; 2], w: f64, h: f64) -> LoopProgram {
    assert!(
        w > 0.0 && h > 0.0,
        "a rectangle has positive sides: {w} x {h}"
    );
    let [x0, y0] = origin;
    LoopProgram::polygon([(x0, y0), (x0 + w, y0), (x0 + w, y0 + h), (x0, y0 + h)])
        .expect("finite corners")
}

/// An axis-aligned rectangular profile node's payload on `plane`:
/// [`rectangle_loop`] drawn on it. `square` is this with two equal
/// sides at the plane origin, and a fixture whose block sits elsewhere
/// moves `origin`.
pub fn rectangle(plane: RecipeNodeId, origin: [f64; 2], w: f64, h: f64) -> Node<ProfileProgram> {
    Node::Profile(ProfileProgram {
        plane,
        loops: vec![rectangle_loop(origin, w, h)],
        ids: Vec::new(),
    })
}

/// A square profile node's payload on `plane`, `side` metres on a side,
/// at the plane origin.
pub fn square(plane: RecipeNodeId, side: f64) -> Node<ProfileProgram> {
    rectangle(plane, [0.0, 0.0], side, side)
}
