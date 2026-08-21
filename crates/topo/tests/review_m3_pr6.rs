//! M3 PR 6a ADVERSARIAL REVIEW (PR #75) — falsification suite.
//! Assignments R1–R5, R7 (R6 e2e lives in
//! `crates/stl/tests/review_m3_pr6_e2e.rs`). Each test names the claim
//! it attacks; comments record the verdict the run demonstrated.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{cube_into, mapped_cube, prism, prism_z};
use geom_core::Tol;
use geom_core::{Bounds, Decide, Point3, Vec3};
use topo::{
    Body, BooleanError, BooleanResult, ContactRecords, SplitError, SplitJoinError, SplitPart,
    SplitPlane, ValidationError, intersect, mass_properties, split, subtract, union,
    validate_pseudomanifold,
};

fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

fn plane_y<T: Decide>(c: f64, ny: f64) -> SplitPlane<T> {
    SplitPlane {
        origin: Point3::new(T::from_f64(0.0), T::from_f64(c), T::from_f64(0.0)),
        normal: Vec3::new(T::from_f64(0.0), T::from_f64(ny), T::from_f64(0.0)),
    }
}

fn body_of<T: geom_core::Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

// =================================================================
// R1 — the D7 mirror-identity pinch lane.
// =================================================================

/// The PR-3 pinch profiles, verbatim (below-pinch under +n / above-
/// pinch under +n respectively).
const MIRRORED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (6.0, 1.0),
    (7.0, 1.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (0.0, 2.0),
];
const NOTCHED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (7.0, 1.0),
    (6.0, 1.0),
    (5.0, 2.0),
    (4.0, 1.0),
    (3.0, 2.0),
    (0.0, 2.0),
];

/// A profile pinching on BOTH sides of y = 1: a bottom notch with apex
/// (4,1) pinches the BELOW pieces (MIRRORED's class) and a top notch
/// with apex (9,1) pinches the ABOVE pieces (NOTCHED's class). A valid
/// decomposition exists (2 shells above pinched at (9,1) + 2 shells
/// below pinched at (4,1) — each side is exactly what the orientation
/// table produces one-sided), but the mirror lane refuses: the direct
/// run dies at the below pinch, the mirrored rerun dies at the
/// (now-below) above pinch.
const BOTH_SIDED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (12.0, 0.0),
    (12.0, 2.0),
    (10.0, 2.0),
    (9.0, 1.0),
    (8.0, 2.0),
    (0.0, 2.0),
];
/// Single-sided controls carved out of BOTH_SIDED (same plane, same
/// tips) — these must succeed, proving the both-sided decomposition is
/// made of two independently-deliverable halves.
const BUMP_ONLY: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (12.0, 0.0),
    (12.0, 2.0),
    (0.0, 2.0),
];
const NOTCH_ONLY: &[(f64, f64)] = &[
    (0.0, 0.0),
    (12.0, 0.0),
    (12.0, 2.0),
    (10.0, 2.0),
    (9.0, 1.0),
    (8.0, 2.0),
    (0.0, 2.0),
];

/// R1(a)+(c): the both-sided pinch class. VERDICT recorded in the
/// review report: the lane REFUSES (typed `DegenerateSection`, no
/// panic, no wrong body) a body whose valid pinch decomposition exists
/// — the mirror rerun only relocates the refusal, falsifying the
/// documented "double refusal = genuine both-sided zero-area residue"
/// reading. Controls: each single-sided half succeeds under the SAME
/// plane with exact volume conservation.
fn both_sided_pinch_scenario<T: Decide + Bounds + topo::PropsQuadLane>() {
    for (profile, must_succeed) in [(BUMP_ONLY, true), (NOTCH_ONLY, true), (BOTH_SIDED, false)] {
        let fx = prism::<T>(profile, 1.0);
        let v0 = mass_properties(&fx.body, Tol::witness()).unwrap().volume;
        match split(&fx.body, &plane_y::<T>(1.0, 1.0), Tol::witness()) {
            Ok(r) => {
                assert!(must_succeed, "BOTH_SIDED unexpectedly split — re-examine");
                let (va, vb) = (
                    mass_properties(body_of(&r.above), Tol::witness())
                        .unwrap()
                        .volume,
                    mass_properties(body_of(&r.below), Tol::witness())
                        .unwrap()
                        .volume,
                );
                let d = (va + vb - v0).abs();
                assert!(d.hi() <= 1e-9, "volume conservation: {d:?}");
            }
            Err(e) => {
                assert!(!must_succeed, "single-sided control refused: {e:?}");
                // The double-refusal path must be TYPED (the direct
                // run's DegenerateSection), never a panic/wrong body.
                assert!(
                    matches!(
                        e,
                        SplitError::Join(SplitJoinError::DegenerateSection { .. })
                    ),
                    "double refusal must surface DegenerateSection, got {e:?}"
                );
            }
        }
    }
}

#[test]
fn r1_both_sided_pinch_f64() {
    both_sided_pinch_scenario::<f64>();
}

/// R1(b): structural comparison of the two paths of the identity —
/// `split(S, +n)` vs swap(`split(S, −n)`) — on both pinch profiles
/// (each orientation exercises the mirror lane for one of them).
/// Compares volumes, shell/face/edge/vertex counts per assigned side,
/// and the section-face normal convention (above section m = −n,
/// below m = +n) that the doc claims survives the swap.
fn mirror_identity_scenario<T: Decide + Bounds + topo::PropsQuadLane>() {
    for profile in [MIRRORED, NOTCHED] {
        let fx = prism::<T>(profile, 1.0);
        let rp = split(&fx.body, &plane_y::<T>(1.0, 1.0), Tol::witness()).unwrap();
        let rn = split(&fx.body, &plane_y::<T>(1.0, -1.0), Tol::witness()).unwrap();
        // swap(split(S,−n)): its BELOW is our ABOVE.
        let pairs = [
            (body_of(&rp.above), body_of(&rn.below), "above"),
            (body_of(&rp.below), body_of(&rn.above), "below"),
        ];
        for (x, y, side) in pairs {
            assert_eq!(
                x.shells().count(),
                y.shells().count(),
                "{profile:?} {side} shells"
            );
            assert_eq!(
                x.faces().count(),
                y.faces().count(),
                "{profile:?} {side} faces"
            );
            assert_eq!(
                x.edges().count(),
                y.edges().count(),
                "{profile:?} {side} edges"
            );
            assert_eq!(
                x.vertices().count(),
                y.vertices().count(),
                "{profile:?} {side} vertices"
            );
            let d = (mass_properties(x, Tol::witness()).unwrap().volume
                - mass_properties(y, Tol::witness()).unwrap().volume)
                .abs();
            assert!(d.hi() <= 1e-9, "{side}: volume mismatch {d:?}");
        }
        // Section-normal convention: outward normals point away from
        // material, so every y=1 section face on y>1 material carries
        // −y and on y<1 material +y — under BOTH plane orientations
        // (doc: "above face m = −n, below face m = +n" per call).
        // rp.above / rn.below are the y>1 material; rp.below /
        // rn.above the y<1 material.
        for (part, want_down) in [
            (body_of(&rp.above), true),
            (body_of(&rp.below), false),
            (body_of(&rn.below), true),
            (body_of(&rn.above), false),
        ] {
            let mut found = 0;
            for (_, f) in part.faces() {
                let Some(topo::Surface::Plane { origin, normal, .. }) = part.get_surface(f.surface)
                else {
                    continue;
                };
                let on_plane = (origin.y - T::from_f64(1.0)).abs().hi() < 1e-9;
                if on_plane && normal.y.lo().abs().max(normal.y.hi().abs()) > 0.99 {
                    found += 1;
                    if want_down {
                        assert!(
                            normal.y.hi() < 0.0,
                            "{profile:?} above-side normal {:?}",
                            normal.y
                        );
                    } else {
                        assert!(
                            normal.y.lo() > 0.0,
                            "{profile:?} below-side normal {:?}",
                            normal.y
                        );
                    }
                }
            }
            assert!(found > 0, "{profile:?}: no section face found on y=1");
        }
    }
}

#[test]
fn r1_mirror_identity_structural_f64() {
    mirror_identity_scenario::<f64>();
}

#[cfg(feature = "interval")]
mod interval_r1 {
    use super::*;

    #[test]
    fn r1_interval_lane() {
        both_sided_pinch_scenario::<geom_core::Interval>();
        mirror_identity_scenario::<geom_core::Interval>();
    }
}

// =================================================================
// R2 — census completeness: coincidence classes with a deliberately
// vertex-free / minimal witness skeleton.
// =================================================================

/// The prompt's prime candidate miss: a coplanar face-face AREA
/// overlap whose boundaries cross ONLY edge×edge (no vertex of either
/// face touches the other entity anywhere) — a plus/cross of two
/// slabs occupying the same z range. The census has no face-face
/// sweep; the claim is the skeleton lanes catch it. Expect LOUD:
/// EdgeEdgeCross (in-plane boundary crossings) and/or EdgeFaceOverlap.
#[test]
fn r2_coplanar_plus_overlap_detected() {
    let mut body = mapped_cube(|x, y, z| Point3::new(3.0 * x, 1.0 + y, z));
    cube_into(&mut body, |x, y, z| Point3::new(1.0 + x, 3.0 * y, z));
    let errors =
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()).unwrap_err();
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{errors:?}"
    );
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: topo::CensusContact::EdgeEdgeCross { .. },
                ..
            }
        )),
        "expected in-plane edge-edge crossings: {errors:?}"
    );
}

/// Identical coincident faces (full flush overlap, all-vv skeleton):
/// two unit cubes stacked flush at z = 1 as one two-shell body.
/// (a) Undeclared: must be LOUD. (b) With the four corner vv pairs
/// fabricated as declarations: does 3′ CERTIFY a full face-on-face
/// area contact from 4 vertex records alone? Executed answer feeds
/// the report (the PR calls skeleton-certification deliberate).
#[test]
fn r2_flush_stack_full_overlap() {
    let mut body = mapped_cube(Point3::new);
    cube_into(&mut body, |x, y, z| Point3::new(x, y, 1.0 + z));
    let errors =
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()).unwrap_err();
    assert!(!errors.is_empty(), "flush stack must not pass undeclared");
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{errors:?}"
    );
    // Fabricate the 4 corner vv declarations from the census's own
    // findings and re-validate.
    let mut contacts = ContactRecords::default();
    for e in &errors {
        if let ValidationError::UndeclaredContact {
            contact: topo::CensusContact::VertexVertex { a, b },
            ..
        } = e
        {
            contacts.vv.push(topo::VvContact { a: *a, b: *b });
        }
    }
    assert_eq!(contacts.vv.len(), 4, "expected the 4 corner kisses");
    let verdict = validate_pseudomanifold(&body, &contacts, Tol::witness());
    // Executed outcome recorded in the report: if Ok, a full-area
    // flush contact is certifiable from 4 vv records (skeleton
    // posture, per PR body); if Err, list what still fires.
    match verdict {
        Ok(()) => eprintln!("R2: flush stack CERTIFIES from 4 vv records"),
        Err(errs) => eprintln!("R2: flush stack still refuses: {errs:?}"),
    }
}

/// Containment with vertices exactly ON the boundary: a diamond prism
/// balanced on a cube's top face with its 4 base vertices resting on
/// the top face's EDGE interiors (region containment, no interior
/// vertex, no boundary crossing). Expect LOUD via the undeclarable
/// VertexOnEdge lane regardless of any declarations.
#[test]
fn r2_inscribed_diamond_vertices_on_edges() {
    let mut body = mapped_cube(|x, y, z| Point3::new(2.0 * x, 2.0 * y, 2.0 * z));
    // Diamond prism z ∈ [2,3]: base corners (1,0,2),(2,1,2),(1,2,2),(0,1,2)
    // — each on an edge interior of the cube's top face.
    cube_into(&mut body, |x, y, z| {
        Point3::new(1.0 + x - y, x + y, 2.0 + z)
    });
    let errors =
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()).unwrap_err();
    assert!(
        errors.iter().any(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: topo::CensusContact::VertexOnEdge { .. },
                ..
            }
        )),
        "expected VertexOnEdge findings: {errors:?}"
    );
}

// =================================================================
// R4 — the saddle frontier (D8): pin what JoinDesync is, and stress
// tilt families the 24-sweep missed, with a volume-identity oracle
// (vol(A∪B) = vol(A)+vol(B)−vol(A∩B); vol(A∖B) = vol(A)−vol(A∩B))
// that detects WRONG-RESULT outcomes the internal gates cannot.
// =================================================================

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

/// Volume of a boolean outcome: `Some(v)` when it closed (Empty = 0),
/// `None` on a typed refusal. Panics only on `PairingMismatch` — the
/// D8 witness this hunt exists for.
fn vol_of(r: Result<BooleanResult<f64>, BooleanError>, ctx: &str) -> Option<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => {
            Some(mass_properties(&b.body, Tol::witness()).unwrap().volume)
        }
        Ok(BooleanResult::Empty) => Some(0.0),
        Err(BooleanError::PairingMismatch { .. }) => {
            panic!("D8 WITNESS: PairingMismatch at {ctx}")
        }
        Err(_) => None,
    }
}

/// The implementer's frontier fixture, pinned TIGHTLY: their test
/// accepts `JoinDesync | PairingMismatch`; this one demands to know
/// which. (If it ever flips to PairingMismatch, that is the D8
/// witness and this test fails loudly to say so.)
#[test]
fn r4_frontier_is_joindesync_not_pairingmismatch() {
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
    let err = union(&a, &b, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, BooleanError::JoinDesync { .. }),
        "frontier moved: {err:?}"
    );
}

/// Families the 24-tilt sweep missed: all three OPS (they swept union
/// only), near-tangent tilts, and a rotation-only family (no shear).
/// Every closing case must satisfy the inclusion-exclusion volume
/// identities to 1e-9 — a wrong-but-gated body fails HERE even when
/// tier 1–2 pass.
#[test]
fn r4_extended_sweep_volume_identities() {
    let a = l_prism();
    let va = mass_properties(&a, Tol::witness()).unwrap().volume;
    let mut closed = 0;
    let mut refused = 0;
    for zc in [0.5f64, 1.0] {
        for family in 0..2u8 {
            for k in 0..10 {
                let t = if k < 5 {
                    0.02 + 0.03 * f64::from(k) // near-tangent band
                } else {
                    0.3 + 0.22 * f64::from(k - 5)
                };
                let (c, s) = (t.cos(), t.sin());
                let map = move |x: f64, y: f64, z: f64| {
                    let (x1, y1) = (x * c - y * s, x * s + y * c);
                    if family == 0 {
                        // Their family (rot + tilt + shear).
                        let (t2c, t2s) = ((t / 2.0).cos(), (t / 2.0).sin());
                        let (y2, z2) = (y1 * t2c - z * t2s, y1 * t2s + z * t2c);
                        Point3::new(
                            2.0 + 0.7 * x1 + 0.3 * y2,
                            2.0 + 0.6 * y2 - 0.2 * z2 + 0.3 * x1,
                            zc + 0.8 * z2 - 0.3 * x1,
                        )
                    } else {
                        // Rotation-only about z then x — corner AT the
                        // reflex site, no shear (higher symmetry).
                        let (txc, txs) = (t.cos(), t.sin());
                        let (y2, z2) = (y1 * txc - z * txs, y1 * txs + z * txc);
                        Point3::new(2.0 + x1, 2.0 + y2, zc + z2)
                    }
                };
                let b = mapped_cube(map);
                let vb = mass_properties(&b, Tol::witness()).unwrap().volume;
                let ctx = format!("zc={zc} family={family} k={k}");
                let vu = vol_of(union(&a, &b, Tol::witness()), &ctx);
                let vi = vol_of(intersect(&a, &b, Tol::witness()), &ctx);
                let vs = vol_of(subtract(&a, &b, Tol::witness()), &ctx);
                if let (Some(vu), Some(vi)) = (vu, vi) {
                    assert!(
                        (vu - (va + vb - vi)).abs() <= 1e-9,
                        "{ctx}: union identity broke: {vu} vs {va}+{vb}-{vi}"
                    );
                    closed += 1;
                }
                if let (Some(vs), Some(vi)) = (vs, vi) {
                    assert!(
                        (vs - (va - vi)).abs() <= 1e-9,
                        "{ctx}: subtract identity broke: {vs} vs {va}-{vi}"
                    );
                }
                if vu.is_none() || vi.is_none() || vs.is_none() {
                    refused += 1;
                }
            }
        }
    }
    // The sweep must EXERCISE both outcomes to mean anything.
    eprintln!("R4 sweep: {closed} identity-checked, {refused} typed refusals");
    assert!(closed > 0, "sweep never closed — fixtures miss the site");
}

// =================================================================
// R5 — census predicates near/at the ε boundary, f64 AND Interval:
// escalation must surface typed (CensusEscalated / UndeclaredContact),
// never a silent pass, on fixtures built to straddle the band.
// =================================================================

/// A prism whose top notch apex hangs `delta` above the bottom edge:
/// vertex-edge gap = delta, exercising `pm_census_ve_line_gap` /
/// `pm_census_ve_span` (and the vv lane at delta = 0 corners). For
/// every delta at or inside ε the validator must REFUSE (finding or
/// typed escalation) — a silent Ok is the R5 falsification.
fn straddle_scenario<T: Decide + topo::PropsQuadLane>(delta: f64) {
    let fx = prism_z::<T>(
        &[
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 2.0),
            (3.0, 2.0),
            (2.0, delta),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        0.0,
        1.0,
    );
    let verdict = validate_pseudomanifold(&fx.body, &ContactRecords::default(), Tol::witness());
    let errors = verdict.expect_err("near-touch at/inside eps must be loud");
    let (mut esc, mut und) = (0, 0);
    for e in &errors {
        match e {
            ValidationError::CensusEscalated { .. } => esc += 1,
            ValidationError::UndeclaredContact { .. } => und += 1,
            _ => {}
        }
    }
    eprintln!("R5 delta={delta}: {und} findings, {esc} escalations");
    for e in &errors {
        assert!(
            matches!(
                e,
                ValidationError::UndeclaredContact { .. } | ValidationError::CensusEscalated { .. }
            ),
            "unexpected error class: {e:?}"
        );
    }
}

/// f64 lane: deltas strictly inside the band (default ε = 1e-12 per
/// the gate's tolerance rows) and exactly AT it.
#[test]
fn r5_straddle_f64() {
    for delta in [0.0, 5e-13, 1e-12] {
        straddle_scenario::<f64>(delta);
    }
}

// =================================================================
// R7 — contract surface: tamper distinction on the vf lane, and the
// closure gap's loudness on rows the acceptance suite skipped
// (reversed subtract, intersect vs second toucher).
// =================================================================

/// vf tamper: on the vertex-on-face kiss body, point the record at a
/// WRONG face — must fire StaleContactDeclaration (dead/witness-free
/// declaration) AND UndeclaredContact (the orphaned real rest): the
/// two directions must stay distinguishable on the vf lane too (the
/// acceptance suite only tampers vv).
#[test]
fn r7_vf_tamper_distinguishes_stale_vs_undeclared() {
    let slab = brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let tilted = mapped_cube(|x, y, z| {
        let (e1, e2, e3) = (
            Vec3::new(0.9, 0.1, 0.3),
            Vec3::new(-0.2, 0.8, 0.45),
            Vec3::new(-0.3, -0.4, 0.85),
        );
        Point3::new(
            2.0 + x * e1.x + y * e2.x + z * e3.x,
            2.0 + x * e1.y + y * e2.y + z * e3.y,
            1.0 + x * e1.z + y * e2.z + z * e3.z,
        )
    });
    let BooleanResult::Body(r) = union(&slab, &tilted, Tol::witness()).unwrap() else {
        panic!("kiss union is a body");
    };
    assert_eq!(
        validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()),
        Ok(())
    );
    let mut tampered = r.contacts.clone();
    let rests = tampered.a_on_b.len() + tampered.b_on_a.len();
    assert_eq!(rests, 1, "expected exactly one vf record");
    let rec = tampered
        .a_on_b
        .first_mut()
        .or(tampered.b_on_a.first_mut())
        .unwrap();
    let right = rec.face;
    let wrong = r
        .body
        .faces()
        .map(|(k, _)| k)
        .find(|&k| k != right)
        .unwrap();
    rec.face = wrong;
    let errors = validate_pseudomanifold(&r.body, &tampered, Tol::witness()).unwrap_err();
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. })),
        "{errors:?}"
    );
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{errors:?}"
    );
}

/// Closure rows the suite skipped: REVERSED subtract (mover minus
/// kiss assembly) and intersect against a second toucher at the same
/// locus. Every outcome must be a certified result with the exact
/// volume oracle or LOUD (typed refusal / UndeclaredContact-only
/// gate) — never silent wrongness.
#[test]
fn r7_closure_reversed_rows_loud() {
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let BooleanResult::Body(base) = union(&a, &b, Tol::witness()).unwrap() else {
        panic!("kiss base");
    };
    // Reversed subtract: mover ∖ assembly. Oracle: mover [1.5,2.5]^3
    // minus its overlap with B-cube [1,2]^3 = 1 − 0.125 = 0.875.
    let mover = brick::<f64>((1.5, 2.5), (1.5, 2.5), (1.5, 2.5));
    match subtract(&mover, &base.body, Tol::witness()) {
        Ok(BooleanResult::Body(r)) => {
            assert_eq!(
                mass_properties(&r.body, Tol::witness()).unwrap().volume,
                0.875
            );
            match validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()) {
                Ok(()) => {}
                Err(errors) => {
                    for e in &errors {
                        assert!(
                            matches!(e, ValidationError::UndeclaredContact { .. }),
                            "gate must stay in the documented gap class: {e:?}"
                        );
                    }
                }
            }
        }
        Ok(BooleanResult::Empty) => panic!("reversed subtract cannot be empty"),
        Err(e) => eprintln!("R7 reversed subtract refusal: {e:?}"),
    }
    // Intersect vs a second toucher kissing the same (1,1,1) locus.
    let toucher = brick::<f64>((0.0, 1.0), (1.0, 2.0), (1.0, 2.0));
    match intersect(&base.body, &toucher, Tol::witness()) {
        Ok(BooleanResult::Body(r)) => {
            // The intersection of the assembly with the edge-tied
            // toucher: B-cube ∩ toucher is the face-flush region of
            // measure 0 → must not silently return junk; A ∩ toucher
            // is the shared edge only. Any BODY here must certify or
            // refuse loudly.
            let v = mass_properties(&r.body, Tol::witness()).unwrap().volume;
            match validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()) {
                Ok(()) => eprintln!("R7 intersect second-toucher closed: vol {v}"),
                Err(errors) => {
                    for e in &errors {
                        assert!(
                            matches!(e, ValidationError::UndeclaredContact { .. }),
                            "{e:?}"
                        );
                    }
                }
            }
        }
        Ok(BooleanResult::Empty) => eprintln!("R7 intersect second-toucher: Empty (measure-zero)"),
        Err(e) => eprintln!("R7 intersect second-toucher refusal: {e:?}"),
    }
}

#[cfg(feature = "interval")]
mod interval_r5 {
    use super::*;

    /// Interval lane: the same deltas — the enclosure straddling the
    /// band edge must surface as typed escalation or finding, never a
    /// silent skip or a poison panic.
    #[test]
    fn r5_straddle_interval() {
        for delta in [0.0, 5e-13, 1e-12] {
            straddle_scenario::<geom_core::Interval>(delta);
        }
    }
}
