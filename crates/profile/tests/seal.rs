//! The `ProfileLoop`/`ProfileVertex` seal, pinned from OUTSIDE the crate.
//!
//! An integration test is a separate crate, so everything here is
//! subject to the same privacy the kernel's consumers are. Two rows:
//! the read surface is complete, and nothing in this crate can
//! deserialize a loop.
//!
//! The E0451 pin — that `ProfileLoop { .. }` does not COMPILE out of
//! crate — lives where it can be executed: a `compile_fail` doctest on
//! [`profile::ProfileLoop`]. A row here could only observe the seal's
//! consequences, never a compile error.

use geom_core::Point2;
use profile::{ProfileLoop, ProfileVertex, RawLoop};

/// The read surface is COMPLETE: every accessor, exercised against a
/// door-built loop.
///
/// The completeness claim is not made by this row alone. Every
/// consumer of a loop outside `crates/profile` — sweep, mesh, stl,
/// step-import/export, editor-core, pncad, the tour, and their
/// fixtures — reads it through exactly these four accessors, so THE
/// WORKSPACE COMPILING, with zero out-of-crate field access anywhere in
/// it, is the completeness proof for the read surface. This row pins
/// what those accessors return.
#[test]
fn accessors_read_back_everything_the_doors_wrote() {
    let vs = vec![
        ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
        ProfileVertex::new(Point2::new(1.0, 0.0), 0.5),
        ProfileVertex::new(Point2::new(1.0, 1.0), -0.25),
        ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
    ];
    let lp: ProfileLoop<f64> = RawLoop::new(vs.clone());

    // ProfileVertex: pos() and bulge() return the constructor's
    // arguments, bit for bit.
    for (got, want) in lp.vertices().iter().zip(&vs) {
        assert_eq!(got.pos().x.to_bits(), want.pos().x.to_bits());
        assert_eq!(got.pos().y.to_bits(), want.pos().y.to_bits());
        assert_eq!(got.bulge().to_bits(), want.bulge().to_bits());
    }

    // ProfileLoop: vertices() is the chain in traversal order;
    // tangent_joints() is empty until declared.
    assert_eq!(lp.vertices().len(), vs.len());
    assert!(lp.tangent_joints().is_empty());

    // The declaring door round-trips through the accessor verbatim —
    // no sorting, no dedup (validation owns that; see the accessor's
    // normative docs).
    let declared = lp.with_tangent_joints(vec![2, 0, 2]);
    assert_eq!(declared.tangent_joints(), &[2, 0, 2]);

    // The polygon door: every bulge zero, chain order preserved.
    let poly: ProfileLoop<f64> = RawLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(0.0, 2.0),
    ]);
    assert_eq!(poly.vertices().len(), 3);
    assert!(poly.vertices().iter().all(|v| v.bulge() == 0.0));
    assert_eq!(poly.vertices()[1].pos().x, 2.0);

    // reversed() survives the seal: it reads and rebuilds through the
    // same private representation, and it is still an involution.
    let there_and_back = declared.reversed().reversed();
    assert_eq!(there_and_back.tangent_joints(), declared.tangent_joints());
    for (a, b) in there_and_back.vertices().iter().zip(declared.vertices()) {
        assert_eq!(a.pos().x.to_bits(), b.pos().x.to_bits());
        assert_eq!(a.bulge().to_bits(), b.bulge().to_bits());
    }
}

/// **Cannot-mint, at the source level.** Deserialization is the one
/// route that could rebuild a value field-by-field without naming a
/// door, so the seal is only worth what the absence of serde on these
/// two types is worth. This row reads the crate's own sources and
/// refuses if that absence ever stops holding.
///
/// The argument the absence completes lives at
/// `crates/editor-core/src/persist/wire.rs`: the persisted form is the
/// PROGRAM, and replay through the driver is the only path from stored
/// steps to geometry. A `#[derive(Deserialize)]` landing on either type
/// here would open a second path, silently — hence a scan, not a
/// comment.
#[test]
fn neither_type_can_be_deserialized() {
    let manifest = include_str!("../Cargo.toml");
    for line in manifest.lines() {
        let l = line.trim();
        if l.starts_with('#') {
            continue;
        }
        assert!(
            !l.contains("serde"),
            "profile's manifest names serde ({l:?}) — the crate is serde-free by policy \
             (see path/program.rs) and the cannot-mint argument rests on it"
        );
    }

    let src = include_str!("../src/lib.rs");
    let seal_offsets = [
        "pub struct ProfileVertex<T: Real>",
        "pub struct ProfileLoop<T: Real>",
    ];
    for decl in seal_offsets {
        let at = src.find(decl);
        assert!(at.is_some(), "{decl} is still declared in lib.rs");
        let at = at.unwrap_or(0);
        // The attributes and docs attached to a declaration are the
        // 40 lines above it; a derive on the type would be in there.
        let head: String = src[..at]
            .lines()
            .rev()
            .take(40)
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !head.contains("serde") && !head.contains("Serialize") && !head.contains("Deserialize"),
            "a serde attribute reached {decl} — the seal's cannot-mint claim is void"
        );
    }
    assert!(
        !src.contains("serde"),
        "profile's root module names serde — the crate is serde-free by policy"
    );
}
