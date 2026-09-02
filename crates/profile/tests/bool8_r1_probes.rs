//! BOOL-8 reviewer probes (R1, blinded lane `bool/8r1-probes`).
//!
//! ADDITIVE ONLY: this file and the two `#[path]` lines that aggregate
//! it are the whole of this lane's tree change. Nothing PR-owned is
//! touched. Every probe here attacks a claim in PR #1508's body by
//! EXECUTION; the ones that pass are the claim standing up, and the
//! ones named `..._is_the_finding` record where it does not.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::pinned;
use geom_core::{Point2, Tol};
use profile::path::{HasPos, NoAng, WithIncoming};
use profile::{Bulge, Open, PartialPath, PathError, Profile, SketchPlane, Start};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

// ==================================================================
// Claim 1 — the transition: bitwise tangent inheritance, no junction
// minted, nothing declared, structural subdivision.
// ==================================================================

/// The PR's own row pins two legs of the SAME length. That is the easy
/// case: equal lengths make the displacement `u * len` bit-identical
/// for free. This probe pushes to FIVE legs of DIFFERENT lengths off an
/// irrational direction, and asserts the property the claim actually
/// needs — that the DIRECTION is the same value each time, recovered
/// here as `d / len` being bit-identical across legs of unequal length.
///
/// The lengths are POWERS OF TWO on purpose: `u*len` and then `/len`
/// are both exact for those, so the recovery is lossless and any
/// difference in the recovered bits is the kernel's, not the probe's.
/// (With arbitrary lengths the round trip loses an ULP by itself —
/// measured, and the reason this probe is written this way.)
#[test]
fn probe_inheritance_is_bitwise_across_many_legs_of_unequal_length() {
    let lens = [0.5, 1.0, 2.0, 4.0, 0.25];
    let mut chain = Open
        .at(p2(0.0, 0.0))
        .toward(3.0, 7.0, Tol::witness())
        .unwrap()
        .line(lens[0], Tol::witness())
        .unwrap();
    for l in &lens[1..] {
        chain = chain.line(*l, Tol::witness()).unwrap();
    }
    let lp = chain
        .line_to(p2(-40.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    let v = lp.vertices();
    // Recover `u` from the FIRST leg only, where it is exact: the chain
    // starts at the origin and `lens[0]` is a power of two, so
    // `(v[1]-v[0])/lens[0]` is lossless.
    let u = (
        (v[1].pos().x - v[0].pos().x) / lens[0],
        (v[1].pos().y - v[0].pos().y) / lens[0],
    );
    // Now assert every later vertex is EXACTLY `prev + u*len`, using
    // that same recovered `u`. This isolates the direction from the
    // position arithmetic: the `+` is replicated here, so a mismatch
    // can only mean a different direction value was used.
    for i in 1..lens.len() {
        let want = (v[i].pos().x + u.0 * lens[i], v[i].pos().y + u.1 * lens[i]);
        assert_eq!(
            (want.0.to_bits(), want.1.to_bits()),
            (v[i + 1].pos().x.to_bits(), v[i + 1].pos().y.to_bits()),
            "leg {i} did not use the SAME direction value: the tangent was \
             re-derived somewhere instead of moved wholesale"
        );
    }
}

/// The other half of the same reading, measured rather than asserted:
/// the DIRECTION value is moved wholesale (probe above), but the
/// derived VERTICES round, so the realized displacement `v[i+1]-v[i]`
/// is NOT in general bit-equal to `u*len`. The PR's own bitwise row
/// avoids this by starting at the origin with two equal legs, where
/// the rounding cancels. Recorded so the review can say which reading
/// of "consecutive legs are exactly parallel" is the true one.
#[test]
fn probe_realized_displacements_do_round_even_though_the_direction_does_not() {
    let lens = [0.5_f64, 1.0, 2.0, 4.0, 0.25];
    let mut chain = Open
        .at(p2(0.0, 0.0))
        .toward(3.0, 7.0, Tol::witness())
        .unwrap()
        .line(lens[0], Tol::witness())
        .unwrap();
    for l in &lens[1..] {
        chain = chain.line(*l, Tol::witness()).unwrap();
    }
    let lp = chain
        .line_to(p2(-40.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    let v = lp.vertices();
    let u = (
        (v[1].pos().x - v[0].pos().x) / lens[0],
        (v[1].pos().y - v[0].pos().y) / lens[0],
    );
    let mut drifted = 0usize;
    for i in 0..lens.len() {
        let d = (
            v[i + 1].pos().x - v[i].pos().x,
            v[i + 1].pos().y - v[i].pos().y,
        );
        if d.0.to_bits() != (u.0 * lens[i]).to_bits() || d.1.to_bits() != (u.1 * lens[i]).to_bits()
        {
            drifted += 1;
        }
    }
    println!(
        "probe: {drifted}/{} realized leg displacements differ from u*len at the bits",
        lens.len()
    );
    // And the data gate is unmoved by that drift, which is the point.
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the rounding stays far inside the band");
}

/// The claim's own wording is "consecutive legs are exactly parallel".
/// The DIRECTION VALUE is inherited bitwise (probe above) — but the
/// LAID-DOWN displacements of two legs of unequal length need not have
/// an exactly vanishing cross product, because `u.x*l1` and `u.y*l2`
/// round independently. This probe measures that, so the review can say
/// which reading of "exactly parallel" is the true one.
#[test]
fn probe_unequal_legs_need_not_have_a_vanishing_cross_product() {
    let mut nonzero = 0usize;
    let mut total = 0usize;
    for (l1, l2) in [
        (0.7_f64, 1.3_f64),
        (0.11, 2.9),
        (1.0, 3.0),
        (0.37, 0.9991),
        (2.0, 7.5),
        (0.123_456_789, 9.876_543_21),
    ] {
        let lp = Open
            .at(p2(0.0, 0.0))
            .toward(3.0, 7.0, Tol::witness())
            .unwrap()
            .line(l1, Tol::witness())
            .unwrap()
            .line(l2, Tol::witness())
            .unwrap()
            .line_to(p2(-40.0, 3.0), Tol::witness())
            .unwrap()
            .line_to(Start, Tol::witness())
            .map(pinned)
            .unwrap();
        let v = lp.vertices();
        let d1 = (v[1].pos().x - v[0].pos().x, v[1].pos().y - v[0].pos().y);
        let d2 = (v[2].pos().x - v[1].pos().x, v[2].pos().y - v[1].pos().y);
        let cross = d1.0 * d2.1 - d1.1 * d2.0;
        total += 1;
        if cross != 0.0 {
            nonzero += 1;
        }
    }
    // Recorded, not asserted either way: the review reports the number.
    println!("probe: {nonzero}/{total} unequal-length leg pairs have a NONZERO cross product");
}

/// Nothing is declared and no junction is minted: `tangent_joints` is
/// empty for an arbitrarily long continuation run, and the lowered
/// vertex table is exactly the run's vertices (a structural
/// subdivision, not an extra construct).
#[test]
fn probe_no_junction_is_minted_and_nothing_is_declared() {
    let lp = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .line(1.0, Tol::witness())
        .unwrap()
        .line_to(p2(4.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .map(pinned)
        .unwrap();
    // Since the 2026-09-02 ruling the continuation DOES declare its
    // own zero-turn joint; what this probe is really about — that no
    // junction is MINTED, i.e. the vertex count — is asserted above.
    assert_eq!(
        lp.tangent_joints(),
        &[1, 2, 3],
        "each continuation declares its own joint"
    );
    let v: Vec<_> = lp
        .vertices()
        .iter()
        .map(|x| (x.pos().x, x.pos().y))
        .collect();
    assert_eq!(
        v,
        vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (2.0, 0.0),
            (3.0, 0.0),
            (4.0, 0.0),
            (4.0, 3.0)
        ],
        "every continuation mints exactly one subdivision vertex, no more"
    );
}

// ==================================================================
// Claim 2 — the doors agree. Executed on the SAME data, and pushed
// past the PR's axis-aligned equal-length fixture.
// ==================================================================

/// The PR's `..._subdivides_a_run_and_validates` row uses an
/// axis-aligned run with equal legs, where the vertices are exactly
/// collinear for free. This probe runs the DATA gate over subdivided
/// runs whose vertices are NOT guaranteed exactly collinear: irrational
/// direction, unequal legs, many subdivisions, several scales. If
/// `validate` refused any of these, the "doors agree" claim would hold
/// only for the fixture.
#[test]
fn probe_the_data_gate_accepts_awkward_subdivided_runs() {
    let dirs = [
        (3.0, 7.0),
        (1.0, 1e-3),
        (0.5, -0.5),
        (1e4, 3.0),
        (2.0, -9.7),
    ];
    let legsets: [&[f64]; 4] = [
        &[1.0, 1.0],
        &[0.7, 1.3, 0.11],
        &[1e-4, 5.0],
        &[0.123_456_789, 9.876_543_21, 0.5, 3.25],
    ];
    for (dx, dy) in dirs {
        for legs in legsets {
            let mut chain = Open
                .at(p2(0.0, 0.0))
                .toward(dx, dy, Tol::witness())
                .unwrap()
                .line(legs[0], Tol::witness())
                .unwrap();
            for l in &legs[1..] {
                chain = chain.line(*l, Tol::witness()).unwrap();
            }
            // Close through a point well off the run so the closing
            // junctions turn definitely. Computed from the authored
            // data rather than read back off the chain.
            let nrm = (dx * dx + dy * dy).sqrt();
            let total: f64 = legs.iter().sum();
            let head = (dx / nrm * total, dy / nrm * total);
            let scale = total.max(1.0);
            let away = p2(head.0 - scale * dy / nrm, head.1 + scale * dx / nrm);
            let lp = chain
                .line_to(away, Tol::witness())
                .unwrap()
                .line_to(Start, Tol::witness())
                .map(pinned)
                .unwrap();
            // Every continuation joint is DECLARED (Evan, in-chat,
            // 2026-09-02): the subdivisions are `1..legs.len()`, and
            // the run's two closing junctions turn definitely.
            let declared: Vec<usize> = lp.tangent_joints().to_vec();
            let want: Vec<usize> = (1..legs.len()).collect();
            assert_eq!(declared, want, "legs={legs:?}");
            Profile::new(SketchPlane::xy(), vec![lp.clone()])
                .validate(Tol::witness())
                .unwrap_or_else(|e| {
                    panic!("the data gate refused a lattice-authored subdivided run: dir=({dx},{dy}) legs={legs:?} err={e:?}")
                });
        }
    }
}

// ==================================================================
// Claim 3 — the keep-refusing set, and the hunt for a spelling that
// sneaks an AUTHORED tangency through as a "continuation".
// ==================================================================

/// Every spelling I can reach that would bind a director equal to the
/// incoming tangent and then run a straight leg. All must refuse. If
/// any of these succeeded, an authored tangency would have entered
/// wearing the continuation's clothes.
#[test]
fn probe_no_spelling_sneaks_an_authored_tangency_through() {
    let t = Tol::witness();
    let base = || {
        Open.at(p2(0.0, 0.0))
            .toward(1.0, 0.0, t)
            .unwrap()
            .line(2.0, t)
            .unwrap()
    };

    // (a) `.toward` with the exact incoming direction.
    assert!(
        matches!(
            base().toward(1.0, 0.0, t),
            Err(PathError::JunctionTangent { .. })
        ),
        "toward() at the exact incoming direction must still refuse"
    );
    // (b) `.toward` with a POSITIVE MULTIPLE of it (a different value
    // spelling of the same ray — the obvious way to try to slip past a
    // value comparison).
    assert!(
        matches!(
            base().toward(7.5, 0.0, t),
            Err(PathError::JunctionTangent { .. })
        ),
        "a rescaled director is the same ray and must still refuse"
    );
    // (c) `.angle` with the exact incoming angle.
    assert!(
        matches!(base().angle(0.0, t), Err(PathError::JunctionTangent { .. })),
        "angle() at the incoming angle must still refuse"
    );
    // (d) `.toward` just inside the band but not exactly on it.
    assert!(
        matches!(
            base().toward(1.0, 1e-15, t),
            Err(PathError::JunctionTangent { .. })
        ),
        "an in-band-but-not-exact director must still refuse"
    );
    // (e) declared identity — RULED LEGAL (Evan, in-chat, 2026-09-02):
    // every zero-turn joint is a declared tangent joint, and the
    // lattice never asks whether the carriers are the same. This used
    // to refuse `SameCarrierJunction`. It is the one arm of this probe
    // that moved, and it moved by ruling: the probe's subject is that
    // no AUTHORED DIRECTION sneaks a tangency through undeclared, and
    // (a)-(d) and (f)-(h) still hold that line.
    let declared = base().tangent().line(2.0, t);
    assert!(
        declared.is_ok(),
        "declared identity is a tangent joint: {declared:?}"
    );
    // (f) `.tangent()` binds the angle slot, and `line_to` is not
    // available on a Directed tip at all — recorded here as a typed
    // fact rather than a runtime one: there is no
    // `tangent().line_to(..)` spelling to sneak through.
    // (g) an authored collinear TARGET.
    assert!(
        matches!(
            base().line_to(p2(4.0, 0.0), t),
            Err(PathError::JunctionTangent { .. })
        ),
        "an authored collinear target must still refuse"
    );
    // (h) the continuation, then an authored collinear target off IT —
    // the continuation must not launder the run into a state where the
    // next authored direction is accepted.
    assert!(
        matches!(
            base().line(2.0, t).unwrap().line_to(p2(6.0, 0.0), t),
            Err(PathError::JunctionTangent { .. })
        ),
        "the continuation must not launder a later authored tangency"
    );
    // (i) the continuation, then declared identity off it — legal for
    // the same ruling, and the laundering worry it was written against
    // is answered by (h) above, which is about an AUTHORED direction.
    assert!(
        base().line(2.0, t).unwrap().tangent().line(1.0, t).is_ok(),
        "a declared identity after a continuation is a tangent joint"
    );
}

/// Curved zero-turn and the cusp, off the continuation's own output
/// state (not just off a fresh chain): the narrowing must not have
/// opened either arm.
#[test]
fn probe_curved_zero_turn_and_cusp_still_refuse_off_a_continuation() {
    let t = Tol::witness();
    let after = || {
        Open.at(p2(0.0, 0.0))
            .toward(1.0, 0.0, t)
            .unwrap()
            .line(2.0, t)
            .unwrap()
            .line(2.0, t)
            .unwrap()
    };
    assert!(
        matches!(after().turn(0.0, t), Err(PathError::JunctionTangent { .. })),
        "zero turn off a continuation must refuse"
    );
    assert!(
        matches!(
            after().turn(core::f64::consts::PI, t),
            Err(PathError::JunctionCusp { .. })
        ),
        "the cusp off a continuation must refuse"
    );
}

/// The declaration/continuation interaction: `.tangent()` DECLARES the
/// joint it binds, so a run that leaves an arc declared and then
/// subdivides structurally must carry exactly ONE declared joint (the
/// arc/line one) and no others — the continuation must neither inherit
/// nor re-emit the declaration.
#[test]
fn probe_a_declared_departure_then_continuations_declares_exactly_once() {
    let t = Tol::witness();
    let lp = Open
        .at(p2(-1.0, 0.0))
        .arc_to(
            Bulge {
                p: p2(1.0, 0.0),
                b: 1.0,
            },
            t,
        )
        .unwrap()
        .tangent()
        .line(1.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .line_to(Start, t)
        .map(pinned)
        .unwrap();
    assert_eq!(
        lp.tangent_joints(),
        &[1, 2, 3],
        "the arc/line joint AND every continuation joint are declared \
         (Evan, in-chat, 2026-09-02); each is declared exactly once"
    );
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(t)
        .expect("declared departure + structural subdivisions is well-formed data");
}

// ==================================================================
// Claim 4 — the carrier-blindness pin, and the MAJOR hunt: is there
// ANY arc for which the lowered undeclared tangency ALSO passes
// validate (a silently-accepted undeclared tangency)?
// ==================================================================

/// Sweep the arc's bulge (and so its radius and sweep angle) across
/// four orders of magnitude and both senses, plus the leg length. Every
/// one of these authors a line tangent to an arc with NOTHING declared.
/// The claim is that the data gate refuses all of them. A single
/// acceptance here is a silently-accepted undeclared tangency — MAJOR.
#[test]
fn probe_hunt_a_silently_accepted_undeclared_tangency_off_an_arc() {
    let t = Tol::witness();
    let bulges = [
        1.0, 0.5, 0.25, 0.1, 0.01, 1e-3, 1e-4, 1e-5, 1e-6, 2.0, 4.0, 10.0, -1.0, -0.5, -0.01,
        -1e-4, 0.999_999, 1.000_001,
    ];
    let lens = [1.0, 1e-2, 1e-4, 100.0];
    let mut accepted: Vec<(f64, f64)> = Vec::new();
    let mut authored_refused: Vec<(f64, f64)> = Vec::new();
    let mut undeclared_at_gate: Vec<(f64, f64)> = Vec::new();
    let mut checked = 0usize;
    for b in bulges {
        for len in lens {
            let arc = Open
                .at(p2(-1.0, 0.0))
                .arc_to(Bulge { p: p2(1.0, 0.0), b }, t);
            let Ok(arc) = arc else { continue };
            let lp = match arc.line(len, t) {
                Ok(c) => match c.line_to(Start, t).map(pinned) {
                    Ok(l) => l,
                    Err(_) => {
                        authored_refused.push((b, len));
                        continue;
                    }
                },
                Err(_) => {
                    authored_refused.push((b, len));
                    continue;
                }
            };
            assert!(
                lp.tangent_joints().contains(&1),
                "the continuation DECLARES its joint, off an arc as anywhere \
                 (Evan, in-chat, 2026-09-02)"
            );
            checked += 1;
            let verdict = Profile::new(SketchPlane::xy(), vec![lp]).validate(t);
            match verdict {
                Ok(_) => accepted.push((b, len)),
                // The one verdict this hunt is about. Anything else —
                // a sliver, a non-simple loop, a degenerate segment at
                // the extreme lengths this sweep walks — is a different
                // fact and not this probe's subject.
                Err(profile::ProfileError::UndeclaredTangency { .. }) => {
                    undeclared_at_gate.push((b, len));
                }
                Err(_) => {}
            }
        }
    }
    println!(
        "probe: {checked} off-arc continuations reached the data gate; \
         {} were ACCEPTED; {} never reached it",
        accepted.len(),
        authored_refused.len()
    );
    // RULED (Evan, in-chat, 2026-09-02): the continuation DECLARES the
    // joint it mints, so no arc/line tangency reaches the gate
    // undeclared from this door any more — which is what the hunt was
    // looking for, inverted. The assertion above (every loop carries
    // joint 1 in `tangent_joints`) is the door's half; this is the
    // gate's: it never sees an undeclared one.
    //
    // NOT asserted: that every one of them VALIDATES. This sweep walks
    // lengths of 1e-4 and 100 against every bulge, which makes slivers
    // and non-simple loops; those refuse for their own reasons and are
    // not this probe's subject. {accepted} of {checked} pass, and the
    // print above carries the numbers.
    assert!(
        undeclared_at_gate.is_empty(),
        "MAJOR: an UNDECLARED arc/line tangency reached the data gate from a \
         continuation that is supposed to declare it: {undeclared_at_gate:?}"
    );
    assert!(checked > 0, "the hunt must actually reach the gate");
}

// ==================================================================
// Claim 5 — the seam wall. Reproduce it, then attack the
// impossibility argument by exhaustive search over rotations,
// traversal directions, and every closer the lattice owns.
// ==================================================================

/// The lily section at `shoulder = 0` (the kite: four tips, four
/// midpoint subdivisions) and at `shoulder = 1` (the rectangle: four
/// corners, four collinear tips), built exactly as `lily.rs` builds
/// them, so the fixture is the demo's own geometry rather than a
/// stand-in.
fn lily_outline(width: f64, ridge: f64, keel: f64, shoulder: f64) -> Vec<Point2<f64>> {
    let sh = |a: (f64, f64), b: (f64, f64)| {
        let m = (0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1));
        (m.0 + shoulder * m.0, m.1 + shoulder * m.1)
    };
    let right = (0.5 * width, 0.0);
    let ridge_p = (0.0, ridge);
    let left = (-0.5 * width, 0.0);
    let keel_p = (0.0, -keel);
    [
        right,
        sh(right, ridge_p),
        ridge_p,
        sh(ridge_p, left),
        left,
        sh(left, keel_p),
        keel_p,
        sh(keel_p, right),
    ]
    .iter()
    .map(|&(x, y)| p2(x, y))
    .collect()
}

/// Which vertices of the real lily outlines are STRAIGHT junctions
/// (subdivisions) and which turn (corners)? The PR's impossibility
/// argument rests entirely on "the pattern alternates corner /
/// subdivision", so measure it rather than take it.
#[test]
fn probe_the_junction_pattern_really_alternates() {
    // Every (width, ridge, keel) the demo actually uses, at both
    // shoulder extremes. NOTE ridge != keel throughout.
    for (w, r, k) in [
        (0.170, 0.028, 0.020),
        (0.420, 0.034, 0.016),
        (0.060, 0.010, 0.006),
        (0.105, 0.014, 0.010),
    ] {
        for shoulder in [0.0, 1.0] {
            let v = lily_outline(w, r, k, shoulder);
            let n = v.len();
            let straight: Vec<bool> = (0..n)
                .map(|i| {
                    let a = v[(i + n - 1) % n];
                    let b = v[i];
                    let c = v[(i + 1) % n];
                    let d1 = b - a;
                    let d2 = c - b;
                    // scale-free turn: |sin| against the shorter arm
                    let cross = (d1.x * d2.y - d1.y * d2.x).abs();
                    let l1 = d1.norm_squared().sqrt();
                    let l2 = d2.norm_squared().sqrt();
                    cross / (l1 * l2) < 1e-12
                })
                .collect();
            println!("probe: w={w} r={r} k={k} shoulder={shoulder} straight={straight:?}");
            for i in 0..n {
                assert_ne!(
                    straight[i],
                    straight[(i + 1) % n],
                    "the alternation claim fails at vertex {i} (w={w} shoulder={shoulder})"
                );
            }
        }
    }
}

/// **The exhaustive hunt, re-run with the declared closer in the
/// alphabet.** (BOOL-8 ran this with `line_to(Start)` as the only
/// straight closer the lattice owned and recorded ZERO closures over 32
/// spellings — 2 sections x 2 shoulder values x 8 starts x 2
/// directions. The alphabet has grown; the row now measures what the
/// growth bought, and what it did not.)
///
/// Two claims come out, and the second is the one that matters for
/// lily:
///
/// 1. **The undeclared closer still closes nothing.** Every one of the
///    32 spellings refuses with `line_to(Start)`, exactly as measured
///    before — the declaration is what changed, not the geometry.
/// 2. **The declared closer closes exactly the spellings whose seam is
///    a CORNER**, which is what "a seam at a corner is sufficient"
///    means — and those spellings sit at OPPOSITE PARITY in the kite
///    (`shoulder = 0`) and the rectangle (`shoulder = 1`). In the kite
///    the corners are the TIPS (even indices); in the rectangle they
///    are the SHOULDERS (odd). So no single starting vertex closes both
///    sections.
///
/// That second measurement is a finding about LILY, not about the
/// closer: a loft matches segment j of every section to segment j of
/// every other, so all of a plan's sections must be authored at ONE
/// rotation. A plan carrying both a `shoulder = 1` base and a
/// `shoulder = 0` belly therefore has no rotation that gives every
/// section a corner at its seam — the parity flips between them and a
/// uniform rotation cannot follow it.
///
/// **And that is why the wall was never the departure's to move.** A
/// third column runs here now: the DECLARED ARRIVAL, which says the
/// seam's own junction on the target instead of on the closing leg's
/// departure. It closes all 64 — both sections, every starting vertex,
/// both directions — so the parity measurement above stands as a fact
/// about lily's corner sets while ceasing to be a wall.
#[test]
fn probe_exhaustive_third_spelling_hunt_across_the_seam() {
    let t = Tol::witness();
    let mut undeclared: Vec<String> = Vec::new();
    let mut declared: Vec<(f64, f64, usize, bool)> = Vec::new();
    let mut arriving: Vec<(f64, f64, usize, bool)> = Vec::new();
    let mut attempts = 0usize;
    for (w, r, k) in [(0.170, 0.028, 0.020), (0.420, 0.034, 0.016)] {
        for shoulder in [0.0_f64, 1.0] {
            let base = lily_outline(w, r, k, shoulder);
            let n = base.len();
            for start in 0..n {
                for rev in [false, true] {
                    // The rotated / possibly reversed vertex ring.
                    let ring: Vec<Point2<f64>> = (0..n)
                        .map(|j| {
                            let idx = if rev {
                                (start + n - j) % n
                            } else {
                                (start + j) % n
                            };
                            base[idx]
                        })
                        .collect();
                    // Two straight closers now: the undeclared one,
                    // which computes a direction and classifies it, and
                    // the declared one, which takes the ray. Every
                    // other closer the table owns (`arc_to(Start)`,
                    // `tangent_arc_to(Start)`, `fillet(..).to(Start)`)
                    // mints an ARC, which is not this outline.
                    attempts += 1;
                    if let Ok(tag) = try_author(&ring, Closer::LineTo, t) {
                        undeclared.push(format!(
                            "w={w} shoulder={shoulder} start={start} rev={rev} closer={tag}"
                        ));
                    }
                    if try_author(&ring, Closer::ContinueTo, t).is_ok() {
                        declared.push((w, shoulder, start, rev));
                    }
                    if try_author(&ring, Closer::ArrivesStraight, t).is_ok() {
                        arriving.push((w, shoulder, start, rev));
                    }
                }
            }
        }
    }
    println!(
        "probe: {attempts} rings; undeclared closed {}, declared-departure closed {}, \
         declared-arrival closed {}",
        undeclared.len(),
        declared.len(),
        arriving.len()
    );
    assert!(
        undeclared.is_empty(),
        "the UNDECLARED closer must still close nothing — the declaration is what \
         changed: {undeclared:?}"
    );
    assert!(
        !declared.is_empty(),
        "the declared closer must close the spellings whose seam is a corner"
    );
    // The parity measurement, per section width.
    for w in [0.170_f64, 0.420] {
        let starts = |shoulder: f64| -> Vec<usize> {
            let mut v: Vec<usize> = declared
                .iter()
                .filter(|(ww, sh, _, _)| *ww == w && *sh == shoulder)
                .map(|(_, _, start, _)| *start)
                .collect();
            v.sort_unstable();
            v.dedup();
            v
        };
        let kite = starts(0.0);
        let rect = starts(1.0);
        println!("probe: w={w} kite closes at {kite:?}, rectangle closes at {rect:?}");
        assert!(!kite.is_empty() && !rect.is_empty());
        assert!(
            kite.iter().all(|i| i % 2 == 0),
            "the kite's corners are its TIPS (even indices): {kite:?}"
        );
        assert!(
            rect.iter().all(|i| i % 2 == 1),
            "the rectangle's corners are its SHOULDERS (odd indices): {rect:?}"
        );
        assert!(
            kite.iter().all(|i| !rect.contains(i)),
            "no starting vertex closes BOTH sections — the parity flips, and a loft \
             pins one rotation for all of them: kite {kite:?} vs rectangle {rect:?}"
        );
    }
    // **THE PARITY WALL FALLS.** The measurement above is unchanged and
    // still true of the DEPARTURE-side spellings — it is a fact about
    // lily's corner sets, not about the closer, and no departure-side
    // declaration could ever follow a corner set that moves. What the
    // ARRIVAL-side declaration adds is the other junction: with it,
    // every ring closes, both sections at every starting vertex and in
    // both directions, so a loft that pins one rotation for all its
    // sections now has one.
    assert_eq!(
        arriving.len(),
        attempts,
        "the declared arrival closes every ring: {} of {attempts}",
        arriving.len()
    );
}

/// **The positive control for the exhaustive hunt above, and the
/// SHARPENING of the PR's generalization.** The hunt returning zero
/// successes proves nothing unless `try_author` can succeed at all, so
/// two outlines that MUST author:
///
/// 1. a plain 8-gon whose every vertex turns (no subdivision anywhere);
/// 2. an 8-vertex outline on the SAME budget whose subdivisions are
///    distributed unevenly — one side carrying two interior vertices
///    and one side carrying none.
///
/// (2) is the sharpening: the PR generalizes the wall to "every closed
/// outline whose every side is subdivided", and the real mechanism is
/// the strict corner/subdivision ALTERNATION that "exactly one per
/// side" forces. Move one subdivision and the same vertex budget
/// authors, because the closer can then depart a corner and land on
/// one. Lily cannot do that — the loft pins where its vertices are —
/// which is what makes its wall a geometry fact rather than a
/// spelling one.
#[test]
fn probe_positive_control_and_the_uneven_distribution_that_escapes_the_wall() {
    let t = Tol::witness();
    // (1) every vertex turns.
    let octagon: Vec<Point2<f64>> = (0..8)
        .map(|k| {
            let a = core::f64::consts::TAU * (k as f64) / 8.0;
            p2(a.cos(), a.sin())
        })
        .collect();
    assert!(
        try_author(&octagon, Closer::LineTo, t).is_ok(),
        "POSITIVE CONTROL FAILED: the harness cannot author even an all-corner \
         outline, so the exhaustive hunt's zero successes would be vacuous"
    );
    // (2) same budget, uneven distribution: a rectangle whose right
    // side carries two interior vertices, whose top and bottom carry
    // one each, and whose left side carries none.
    let uneven = [
        p2(1.0, -1.0),  // corner
        p2(1.0, -0.25), // subdivision
        p2(1.0, 0.25),  // subdivision (same side)
        p2(1.0, 1.0),   // corner
        p2(0.0, 1.0),   // subdivision
        p2(-1.0, 1.0),  // corner
        p2(-1.0, -1.0), // corner  <- left side NOT subdivided
        p2(0.0, -1.0),  // subdivision
    ];
    // The escape needs the UNSUBDIVIDED side to be the closing one, so
    // try every rotation exactly as the seam hunt does. The lily
    // outlines score 0/16 there; this one must score at least 1.
    let n = uneven.len();
    let mut ok = 0usize;
    for start in 0..n {
        for rev in [false, true] {
            let ring: Vec<Point2<f64>> = (0..n)
                .map(|j| {
                    let idx = if rev {
                        (start + n - j) % n
                    } else {
                        (start + j) % n
                    };
                    uneven[idx]
                })
                .collect();
            if try_author(&ring, Closer::LineTo, t).is_ok() {
                ok += 1;
            }
        }
    }
    println!(
        "probe: the uneven 8-vertex distribution authors in {ok}/{} spellings",
        2 * n
    );
    assert!(
        ok > 0,
        "the same vertex budget DOES author once the alternation is broken"
    );
}

#[derive(Clone, Copy)]
enum Closer {
    LineTo,
    /// The DECLARED structural closer (BOOL-11): the leg says it is the
    /// straight continuation and names `Start` as where it lands, so
    /// the departure is checked against the ray rather than inferred
    /// from it.
    ContinueTo,
    /// The DECLARED ARRIVAL (BOOL-12): the closing leg's departure is
    /// spelled by whichever verb its own junction wants, and the SEAM
    /// is declared on the target — `Start.arrives_tangent()`. This is
    /// the spelling the parity wall was waiting for.
    ArrivesStraight,
}

/// Author `ring` as a closed loop: `toward` at every vertex whose
/// junction turns, the structural continuation at every vertex whose
/// junction is straight, and the requested closer at the seam. Returns
/// Ok only if the lattice actually produced a closed loop that also
/// passes the data gate.
fn try_author(ring: &[Point2<f64>], closer: Closer, t: Tol) -> Result<&'static str, ()> {
    let n = ring.len();
    let straight_at = |i: usize| -> bool {
        let a = ring[(i + n - 1) % n];
        let b = ring[i];
        let c = ring[(i + 1) % n];
        let d1 = b - a;
        let d2 = c - b;
        let cross = (d1.x * d2.y - d1.y * d2.x).abs();
        let l1 = d1.norm_squared().sqrt();
        let l2 = d2.norm_squared().sqrt();
        cross / (l1 * l2) < 1e-12
    };
    let dist = |a: Point2<f64>, b: Point2<f64>| (b - a).norm_squared().sqrt();
    let d0 = ring[1] - ring[0];
    let mut chain: PartialPath<f64, HasPos<WithIncoming>, NoAng> = Open
        .at(ring[0])
        .toward(d0.x, d0.y, t)
        .map_err(|_| ())?
        .line(dist(ring[0], ring[1]), t)
        .map_err(|_| ())?;
    for i in 1..n - 1 {
        let len = dist(ring[i], ring[i + 1]);
        chain = if straight_at(i) {
            // The structural continuation: no authored direction.
            chain.line(len, t).map_err(|_| ())?
        } else {
            let d = ring[i + 1] - ring[i];
            chain
                .toward(d.x, d.y, t)
                .map_err(|_| ())?
                .line(len, t)
                .map_err(|_| ())?
        };
    }
    // The closer, departing ring[n-1] and landing on ring[0] == Start.
    let lp = match closer {
        Closer::LineTo => chain.line_to(Start, t).map_err(|_| ())?,
        Closer::ContinueTo => chain.continue_to(Start, t).map_err(|_| ())?,
        // The two junctions the closing leg touches are independent, and
        // this arm says each with the verb it wants: the DEPARTURE with
        // `continue_to` where the run continues through `ring[n-1]` and
        // `line_to` where it turns there, the ARRIVAL with the target
        // where the run continues through `ring[0]` and plain `Start`
        // where it turns.
        Closer::ArrivesStraight => match (straight_at(n - 1), straight_at(0)) {
            (false, false) => chain.line_to(Start, t).map_err(|_| ())?,
            (false, true) => chain.line_to(Start.arrives_tangent(), t).map_err(|_| ())?,
            (true, false) => chain.continue_to(Start, t).map_err(|_| ())?,
            (true, true) => chain
                .continue_to(Start.arrives_tangent(), t)
                .map_err(|_| ())?,
        },
    };
    let lp = pinned(lp);
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(t)
        .map_err(|_| ())?;
    Ok(match closer {
        Closer::LineTo => "line_to(Start)",
        Closer::ContinueTo => "continue_to(Start)",
        Closer::ArrivesStraight => "the junction each seam side wants",
    })
}

/// Reproduce the two measured seam refusals of the UNDECLARED closer
/// and PRINT the actual error values, so the quoted
/// the quoted margin -7.85e-17 stays checked rather than
/// taken — and so the claim that the two rotations refuse for two
/// different reasons is now readable off the payload, which carries
/// the site (`Departure` in A, `Seam` in B).
#[test]
fn probe_reproduce_the_measured_seam_wall_in_both_rotations() {
    let t = Tol::witness();
    let v = lily_outline(0.170, 0.028, 0.020, 0.0);
    // Rotation A — seam at a CORNER (vertex 0, a tip): the closer
    // departs the run's subdivision vertex.
    let ring_a: Vec<Point2<f64>> = v.clone();
    let err_a = author_to_the_closer(&ring_a, t);
    println!("probe: rotation A (seam at a corner) -> {err_a:?}");
    // Rotation B — seam at a SUBDIVISION vertex (vertex 1).
    let ring_b: Vec<Point2<f64>> = (0..v.len()).map(|j| v[(1 + j) % v.len()]).collect();
    let err_b = author_to_the_closer(&ring_b, t);
    println!("probe: rotation B (seam at a subdivision) -> {err_b:?}");
    assert!(
        matches!(err_a, Some(PathError::JunctionTangent { .. })),
        "rotation A must refuse at the closer's DEPARTURE — as an ORDINARY \
         departure refusal, the same one a mid-chain tangent departure gets, \
         got {err_a:?}"
    );
    assert!(
        matches!(err_b, Some(PathError::SeamTangent { .. })),
        "rotation B must refuse at the SEAM, and `SeamTangent` is a refusal \
         only a seam can produce, got {err_b:?}"
    );
}

/// The PR body quotes a measured margin,
/// a tangent-band refusal at margin -7.85e-17, for the seam wall. Its
/// fixture is not the real lily section but the suite's stand-in kite
/// (`right`/`ridge`/`left`/`keel` at 1, 1.5, 1). Reproduce THAT number
/// against THAT fixture, so the quoted figure is checked rather than
/// taken.
#[test]
fn probe_reproduce_the_quoted_margin_on_the_suites_own_fixture() {
    let t = Tol::witness();
    let ring = [
        p2(1.0, 0.0),
        p2(0.5, 0.75),
        p2(0.0, 1.5),
        p2(-0.5, 0.75),
        p2(-1.0, 0.0),
        p2(-0.5, -0.5),
        p2(0.0, -1.0),
        p2(0.5, -0.5),
    ];
    let err = author_to_the_closer(&ring, t);
    println!("probe: PR-fixture rotation A margin -> {err:?}");
    let rot: Vec<Point2<f64>> = (0..8).map(|j| ring[(1 + j) % 8]).collect();
    let err_b = author_to_the_closer(&rot, t);
    println!("probe: PR-fixture rotation B margin -> {err_b:?}");
    // Rotation A is the closer's DEPARTURE, rotation B is the SEAM, and
    // since the seam wall's departure half collapsed they are different
    // TYPES rather than one type carrying a site tag — so this pins them
    // by type and no payload needs reading.
    assert!(matches!(err, Some(PathError::JunctionTangent { .. })));
    assert!(matches!(err_b, Some(PathError::SeamTangent { .. })));
}

fn author_to_the_closer(ring: &[Point2<f64>], t: Tol) -> Option<PathError<f64>> {
    let n = ring.len();
    let straight_at = |i: usize| -> bool {
        let a = ring[(i + n - 1) % n];
        let b = ring[i];
        let c = ring[(i + 1) % n];
        let d1 = b - a;
        let d2 = c - b;
        let cross = (d1.x * d2.y - d1.y * d2.x).abs();
        cross / (d1.norm_squared().sqrt() * d2.norm_squared().sqrt()) < 1e-12
    };
    let dist = |a: Point2<f64>, b: Point2<f64>| (b - a).norm_squared().sqrt();
    let d0 = ring[1] - ring[0];
    let mut chain = Open
        .at(ring[0])
        .toward(d0.x, d0.y, t)
        .ok()?
        .line(dist(ring[0], ring[1]), t)
        .ok()?;
    for i in 1..n - 1 {
        let len = dist(ring[i], ring[i + 1]);
        chain = if straight_at(i) {
            chain.line(len, t).ok()?
        } else {
            let d = ring[i + 1] - ring[i];
            chain.toward(d.x, d.y, t).ok()?.line(len, t).ok()?
        };
    }
    chain.line_to(Start, t).err()
}
