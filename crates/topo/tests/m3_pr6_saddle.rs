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
//!    The nearest reachable behavior is a typed refusal from that
//!    lane (`ClassificationInvariant`, pinned below); no silent
//!    mispair exists in the right-prism corpus.
//! 2. **Tilted planar operands: guard unwitnessed (swept).** With a
//!    linearly-mapped (tilted — no vertical edges) cube corner placed
//!    on an L-prism's reflex wedge edge interior and reflex cap
//!    corner, a 24-case tilt sweep produced only clean `Seamed`
//!    successes (internally gated by tier 1–2 + the volume backstop)
//!    — every realized crossing set paired B-adjacent. The
//!    non-convex-link interleaving the guard defends against did not
//!    materialize on this corpus.
//! 3. **Nearest non-success (pinned).** One tilt family refuses typed
//!    deeper in the pipeline (`JoinDesync`: "pair B edge has not
//!    exactly one surviving end") — loud, operands untouched, never a
//!    wrong body; pinned below as the guard-adjacent frontier of the
//!    M3 envelope (a saddle-class configuration the join does not yet
//!    realize).
//!
//! The guard is NOT proved unreachable for arbitrary tier-2 planar
//! operands — it stays armed, and this file is the standing hunt
//! fixture (any future firing lands here).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod common;
use common::{line, plane, prism_z};
use geom_core::{Point3, Vec3};
use topo::{Body, BooleanError, FaceSurface, MefSite, MevSite, union};

/// A cube built like common::geometric_cube but through an arbitrary
/// point transform (tilted operands are outside the prism builder).
fn mapped_cube(map: impl Fn(f64, f64, f64) -> Point3<f64>) -> Body<f64> {
    let (a, b, cc, d) = (
        map(0.0, 0.0, 0.0),
        map(1.0, 0.0, 0.0),
        map(1.0, 1.0, 0.0),
        map(0.0, 1.0, 0.0),
    );
    let (a1, b1, c1, d1) = (
        map(0.0, 0.0, 1.0),
        map(1.0, 0.0, 1.0),
        map(1.0, 1.0, 1.0),
        map(0.0, 1.0, 1.0),
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    let e_ab = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            line(a, b),
        )
        .unwrap();
    let strut = |body: &mut Body<f64>, at, from, to| {
        body.mev(MevSite::Fan { he1: at, he2: at }, to, line(from, to))
            .unwrap()
    };
    let e_bc = strut(&mut body, e_ab.he_minus, b, cc);
    let e_cd = strut(&mut body, e_bc.he_minus, cc, d);
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bottom = body
        .mef(
            MefSite::Chords {
                he1: he_dc,
                he2: e_ab.he_plus,
            },
            line(d, a),
            FaceSurface::New(plane(&[a, d, cc, b])),
        )
        .unwrap();
    let e_aa = strut(&mut body, e_ab.he_plus, a, a1);
    let e_bb = strut(&mut body, e_bc.he_plus, b, b1);
    let e_cc = strut(&mut body, e_cd.he_plus, cc, c1);
    let e_dd = strut(&mut body, f_bottom.he_plus, d, d1);
    let f_front = body
        .mef(
            MefSite::Chords {
                he1: e_aa.he_minus,
                he2: e_bb.he_minus,
            },
            line(a1, b1),
            FaceSurface::New(plane(&[a, b, b1, a1])),
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_bb.he_minus,
            he2: e_cc.he_minus,
        },
        line(b1, c1),
        FaceSurface::New(plane(&[b, cc, c1, b1])),
    )
    .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_cc.he_minus,
            he2: e_dd.he_minus,
        },
        line(c1, d1),
        FaceSurface::New(plane(&[cc, d, d1, c1])),
    )
    .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_dd.he_minus,
            he2: f_front.he_plus,
        },
        line(d1, a1),
        FaceSurface::New(plane(&[d, a, a1, d1])),
    )
    .unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(plane(&[a1, b1, c1, d1])))
        .unwrap();
    common::describe_as_intersections(&mut body);
    body
}

/// The L-prism with its reflex wedge edge along z at (2, 2).
fn l_prism() -> Body<f64> {
    prism_z::<f64>(
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
    )
    .body
}

/// Part 1's pin: prism × prism at the reflex corner — the collinear
/// vertical edges force the edge-edge lane, which refuses typed
/// (never reaching a 4-survivor v-v pairing site, never mispairing
/// silently).
#[test]
fn prism_reflex_kiss_takes_edge_edge_lane() {
    let a = l_prism();
    let b = prism_z::<f64>(&[(2.0, 2.0), (3.0, 1.0), (4.5, 2.0), (3.0, 3.0)], 0.0, 1.0).body;
    let err = union(&a, &b).unwrap_err();
    assert!(
        matches!(err, BooleanError::ClassificationInvariant { .. }),
        "expected the edge-edge lane's typed refusal, got {err:?}"
    );
}

/// Part 3's pin: the tilted saddle-class corner that the join does
/// not yet realize refuses typed (JoinDesync), operands untouched —
/// the nearest reachable non-success to the pairing guard.
#[test]
fn tilted_saddle_corner_refuses_typed() {
    let a = l_prism();
    let b = mapped_cube(|x, y, z| {
        let (e1, e2, e3) = (
            Vec3::new(0.9, -0.6, 0.5),
            Vec3::new(0.7, 0.8, -0.55),
            Vec3::new(-0.45, 0.5, 0.9),
        );
        Point3::new(
            2.0 + x * e1.x + y * e2.x + z * e3.x,
            2.0 + x * e1.y + y * e2.y + z * e3.y,
            0.5 + x * e1.z + y * e2.z + z * e3.z,
        )
    });
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let err = union(&a, &b).unwrap_err();
    assert!(
        matches!(
            err,
            BooleanError::JoinDesync { .. } | BooleanError::PairingMismatch { .. }
        ),
        "expected the typed saddle-frontier refusal, got {err:?}"
    );
    assert_eq!(format!("{a:?}"), a0, "operand A untouched");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched");
}

/// Part 2's sweep: tilted cube corners on the reflex edge interior
/// (zc = 0.5 — the v-on-e site the reduction refines to v-v) and the
/// reflex cap corner (zc = 1.0 — the direct v-v site), 24 tilts.
/// Every case must end in a clean gated success or a typed refusal —
/// no silent wrongness; a `PairingMismatch` appearing here would be
/// the D8 witness (none has materialized on this corpus).
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
            let b = mapped_cube(map);
            match union(&a, &b) {
                // Gated success (tier 1–2 + volume backstop inside).
                Ok(_) => {}
                Err(BooleanError::PairingMismatch { .. }) => {
                    panic!(
                        "D8 witness found at zc={zc} k={k}: promote this \
                         tilt to a named firing fixture"
                    );
                }
                // Any other refusal is typed and loud — acceptable.
                Err(_) => {}
            }
        }
    }
}
