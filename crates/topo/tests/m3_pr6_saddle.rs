//! M3 PR 6a, D8: the saddle fixture obligation for the 15.11 pairing
//! guard (`PairingMismatch`, `boolean/insert.rs` F12 guards: B-cyclic
//! adjacency + run-side agreement). PR 4's review noted the guard was
//! stressed only by planar 4-crossing fixtures and asked for a
//! saddle-vertex fixture (non-convex neighborhood where A-consecutive
//! ≠ B-consecutive pairing is geometrically realizable) that either
//! witnesses the guard firing or argues unreachability. Outcome here
//! — the honest middle, in three parts:
//!
//! 1. **Right prisms cannot reach the guard (proved).** Every vertex
//!    of a right prism is a profile corner and carries a vertical
//!    edge through its position. Two prisms sharing a vertex position
//!    therefore always hold COLLINEAR vertical edges there, so the
//!    site enters the reduction's edge-edge (Table II/III) lane —
//!    never a pure v-v contact with four transversal germ survivors.
//!    That lane reads the L-prism's reflex wedge by its extent and
//!    builds the union (pinned below); no silent mispair exists in the
//!    right-prism corpus.
//! 2. **Tilted planar operands (swept).** With a linearly-mapped
//!    (tilted — no vertical edges) cube corner placed on an L-prism's
//!    reflex wedge edge interior and reflex cap corner, a 24-case tilt
//!    sweep ends every union in a gated success or a typed refusal
//!    other than the guard's.
//! 3. **A four-germ saddle corner (pinned).** One tilt crosses the
//!    reflex corner four times. It builds every op, in both operand
//!    orders, at the volume clipping the tilted cube to the L-prism's
//!    two boxes gives.
//!
//! The guard cannot fire on two simple links: A's runs on one side of
//! B are disjoint arcs of one disk B's link bounds, so A's pairs never
//! cross in B's walk order. At four crossings they are adjacent there
//! too; at six they may nest, and the nested pairing builds
//! (`insert`'s module docs,
//! `join_pierce_runs_sweep::six_crossing_corners_build_every_op`). The
//! hunts here stop on any `PairingMismatch` so that it is read: it is
//! a bug at any number of crossings.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common;
use common::{finished, mapped_cube, prism_z};
use geom_core::Point3;
use geom_core::Tol;
use topo::{
    AtRestBody, BooleanDeclarations, BooleanError, BooleanResult, intersect_with, mass_properties,
    subtract_with, union, union_with,
};

/// The L-prism with its reflex wedge edge along z at (2, 2).
fn l_prism() -> AtRestBody<f64> {
    let l = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 2.0),
            (2.0, 2.0),
            (2.0, 4.0),
            (0.0, 4.0),
        ],
        0.0,
        1.0,
        Tol::witness(),
    )
    .body;
    finished("the L-prism", l, Tol::witness())
}

/// Part 1's pin: prism × prism at the reflex corner — the collinear
/// vertical edges force the edge-edge lane, which reads the L-prism's
/// 270° wedge by its extent (never reaching a 4-survivor v-v pairing
/// site). The union passes tiers 2 and 3′ and the at-rest certificate
/// at the closed form: the L's 12 plus the kite's 5/2 less their
/// common 7/6.
#[test]
fn prism_reflex_kiss_takes_edge_edge_lane() {
    let tol = Tol::witness();
    let a = l_prism();
    let b = prism_z::<f64>(
        &[(2.0, 2.0), (3.0, 1.0), (4.5, 2.0), (3.0, 3.0)],
        0.0,
        1.0,
        tol,
    )
    .body;
    let b = finished("the kissing prism", b, Tol::witness());
    // The coplanar top/bottom contacts are declared so the
    // classification reaches the edge-edge lane (undeclared, it
    // refuses earlier at the coincidence door — rung (b)).
    let bb = match topo::union_with(&a, &b, &common::flush_declarations(&a, &b, tol), tol) {
        Ok(topo::BooleanResult::Body(bb)) => bb,
        other => panic!("the reflex kiss's union: {other:?}"),
    };
    assert_eq!(topo::validate_closed(&bb.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
        Ok(()),
        "tier 3′"
    );
    assert!(
        topo::validate_geometric_certificate(&bb.body, tol).is_ok(),
        "the at-rest certificate"
    );
    let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
    assert!((v - 40.0 / 3.0).abs() < 1e-9, "volume {v}");
}

/// Part 3's pin: the tilted saddle corner crosses the L-prism's reflex
/// corner four times. Every op, in both operand orders, builds at the
/// volume clipping the tilted cube to the L-prism's two boxes gives
/// (`union_flush_onto_edge_contact::clipped_volume`), passing tiers 2
/// and 3′ and the certificate, operands untouched. Red as
/// `PairingMismatch` when two germs in one sector are ordered by the
/// other solid's sector, as a `ClassificationInvariant` when B runs a
/// null edge forward in A's order, and as `Euler(SelfLoopEdge)` when
/// the pairing starts at A's first germ whatever the op keeps of A.
#[test]
fn tilted_saddle_corner_builds_every_op() {
    let tol = Tol::witness();
    let a = l_prism();
    let (e1, e2, e3) = ([0.9, -0.6, 0.5], [0.7, 0.8, -0.55], [-0.45, 0.5, 0.9]);
    let at = |x: f64, y: f64, z: f64| -> [f64; 3] {
        core::array::from_fn(|k| [2.0, 2.0, 0.5][k] + x * e1[k] + y * e2[k] + z * e3[k])
    };
    let b = mapped_cube(
        |x, y, z| {
            let p = at(x, y, z);
            Point3::new(p[0], p[1], p[2])
        },
        tol,
    );
    let b = finished("the tilted cube", b, tol);
    // The tilted cube's six faces as vertex loops, and its volume.
    let mut faces = Vec::new();
    for (u, v, w) in [(0, 1, 2), (1, 2, 0), (2, 0, 1)] {
        for side in [0.0, 1.0] {
            let p = |s: f64, t: f64| {
                let mut c = [0.0; 3];
                (c[u], c[v], c[w]) = (side, s, t);
                at(c[0], c[1], c[2])
            };
            faces.push(vec![p(0.0, 0.0), p(1.0, 0.0), p(1.0, 1.0), p(0.0, 1.0)]);
        }
    }
    let clip = crate::union_flush_onto_edge_contact::clipped_volume;
    let vb = clip(faces.clone(), &[]);
    let common: f64 = [
        ([0.0, 0.0, 0.0], [4.0, 2.0, 1.0]),
        ([0.0, 2.0, 0.0], [2.0, 4.0, 1.0]),
    ]
    .into_iter()
    .map(|(lo, hi): ([f64; 3], [f64; 3])| {
        let planes: Vec<([f64; 3], f64)> = (0..3)
            .flat_map(|k| {
                let e: [f64; 3] = core::array::from_fn(|j| if j == k { 1.0 } else { 0.0 });
                [(e, hi[k]), (e.map(|x| -x), -lo[k])]
            })
            .collect();
        clip(faces.clone(), &planes)
    })
    .sum();
    let va = 12.0;
    assert!(common > 0.0 && common < vb, "the cube crosses the corner");
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let d = BooleanDeclarations::default();
    for (op, got, want) in [
        ("a ∪ b", union_with(&a, &b, &d, tol), va + vb - common),
        ("a ∩ b", intersect_with(&a, &b, &d, tol), common),
        ("a ∖ b", subtract_with(&a, &b, &d, tol), va - common),
        ("b ∪ a", union_with(&b, &a, &d, tol), va + vb - common),
        ("b ∩ a", intersect_with(&b, &a, &d, tol), common),
        ("b ∖ a", subtract_with(&b, &a, &d, tol), vb - common),
    ] {
        let BooleanResult::Body(out) = got.unwrap_or_else(|e| panic!("{op} refused: {e:?}")) else {
            panic!("{op} came back empty");
        };
        let v = mass_properties(&out.body, tol).unwrap().volume;
        assert!((v - want).abs() < 1e-9, "{op}: volume {v}, want {want}");
        assert_eq!(topo::validate_closed(&out.body), Ok(()), "{op}: tier 2");
        assert_eq!(
            topo::validate_pseudomanifold(&out.body, &out.contacts, tol),
            Ok(()),
            "{op}: 3′"
        );
        assert!(
            topo::validate_geometric_certificate(&out.body, tol).is_ok(),
            "{op}: the certificate"
        );
    }
    assert_eq!(format!("{a:?}"), a0, "operand A untouched");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched");
}

/// Part 2's sweep: tilted cube corners on the reflex edge interior
/// (zc = 0.5 — the v-on-e site the reduction refines to v-v) and the
/// reflex cap corner (zc = 1.0 — the direct v-v site), 24 tilts.
/// Every case must end in a clean gated success or a typed refusal —
/// no silent wrongness. A `PairingMismatch` stops the sweep to be
/// read (module docs): none has materialized on this corpus.
#[test]
fn tilt_sweep_no_silent_mispair() {
    let a = l_prism();
    for zc in [0.5f64, 1.0] {
        for k in 0..12 {
            let t = 0.3 + 0.11 * f64::from(k);
            let (c, s) = (t.cos(), t.sin());
            let map = move |x: f64, y: f64, z: f64| {
                // Rotate about z by t, tilt about x by t/2, then a
                // fixed shear pointing the corner at the notch.
                let (x1, y1) = (x * c - y * s, x * s + y * c);
                let (t2c, t2s) = ((t / 2.0).cos(), (t / 2.0).sin());
                let (y2, z2) = (y1 * t2c - z * t2s, y1 * t2s + z * t2c);
                Point3::new(
                    2.0 + 0.7 * x1 + 0.3 * y2,
                    2.0 + 0.6 * y2 - 0.2 * z2 + 0.3 * x1,
                    zc + 0.8 * z2 - 0.3 * x1,
                )
            };
            let b = finished(
                "a tilted cube",
                mapped_cube(map, Tol::witness()),
                Tol::witness(),
            );
            match union(&a, &b, Tol::witness()) {
                // Gated success (tier 1–2 + volume backstop inside).
                Ok(_) => {}
                Err(BooleanError::PairingMismatch { .. }) => {
                    panic!(
                        "PairingMismatch at zc={zc} k={k}: a bug at any number of \
                         crossings"
                    );
                }
                // Any other refusal is typed and loud — acceptable.
                Err(_) => {}
            }
        }
    }
}
