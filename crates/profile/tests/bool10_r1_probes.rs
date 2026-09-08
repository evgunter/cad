//! R1 review probes for BOOL-10 (PR #2135, frozen head 3f8163dd8).
//!
//! Each row pins something the review executed. A row whose assertion
//! records a DEFECT says so in its doc comment and asserts the defect,
//! so the fix pass flips the assertion rather than deleting the row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{ArcSide, ArcSweep, Bulge, Center, ClosedLoop, Open, Start, Sweep, Via};
use std::f64::consts::FRAC_PI_8;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn t() -> Tol {
    Tol::witness()
}

/// Every vertex of a closed loop, as raw bits.
fn bits(l: &ClosedLoop<f64>) -> Vec<(u64, u64, u64)> {
    l.loop_
        .vertices()
        .iter()
        .map(|v| {
            (
                v.pos().x.to_bits(),
                v.pos().y.to_bits(),
                v.bulge().to_bits(),
            )
        })
        .collect()
}

// ------------------------------------------------------------------
// 1. The placement contract, attacked past the PR's four fixtures.
// ------------------------------------------------------------------

/// **The reversal identity on a NON-axis-aligned chord at ODD n.** The
/// PR pins n = 4 and 5 on "an unsymmetric arc"; this walks n = 2..=9
/// over tilted chords and major-arc bulges (> 1) in both windings, and
/// asserts every station is bit-identical to its reversed twin (the
/// reversed leg's station list read backwards).
#[test]
fn reversal_is_bit_identical_over_odd_even_major_and_cw() {
    for (ax, ay, bx, by) in [
        (0.3, -0.7, 1.9, 0.4),
        (-1.0, -1.0, 1.0, 1.0),
        (0.0, 0.0, 2.0, 0.0),
    ] {
        for bulge in [0.25, 0.5, 1.0, 1.7, 3.0, -0.4, -1.0, -2.5] {
            for n in 2..=9usize {
                let fwd = Open
                    .at(p2(ax, ay))
                    .arc_to(
                        Bulge {
                            p: p2(bx, by),
                            b: bulge,
                        }
                        .split(n),
                        t(),
                    )
                    .unwrap()
                    .line_to(Start, t())
                    .unwrap();
                let rev = Open
                    .at(p2(bx, by))
                    .arc_to(
                        Bulge {
                            p: p2(ax, ay),
                            b: -bulge,
                        }
                        .split(n),
                        t(),
                    )
                    .unwrap()
                    .line_to(Start, t())
                    .unwrap();
                let f: Vec<_> = bits(&fwd)[1..n].to_vec();
                let mut r: Vec<_> = bits(&rev)[1..n].to_vec();
                r.reverse();
                for (k, (a, b)) in f.iter().zip(r.iter()).enumerate() {
                    assert_eq!(
                        (a.0, a.1),
                        (b.0, b.1),
                        "station {k} of n={n} bulge={bulge} chord \
                         ({ax},{ay})->({bx},{by}) is not bit-identical \
                         under reversal"
                    );
                }
            }
        }
    }
}

/// **Every station lies on the leg's own carrier**, to within the
/// leg's own rounding: |station − centre| − R against the chord-and-
/// bulge closed form the kernel itself uses.
#[test]
fn stations_sit_on_the_legs_own_carrier() {
    let mut worst = 0.0f64;
    for bulge in [0.2, 0.5, 1.0, 1.5, 2.5, -0.7, -1.0, -3.0] {
        for n in 2..=8usize {
            let a = p2(0.3, -0.7);
            let b = p2(1.9, 0.4);
            let closed = Open
                .at(a)
                .arc_to(Bulge { p: b, b: bulge }.split(n), t())
                .unwrap()
                .line_to(Start, t())
                .unwrap();
            let d = b - a;
            let l = d.norm_squared().sqrt();
            let n_hat = geom_core::Vec2::new(-d.y, d.x) * (1.0 / l);
            let mid = Point2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
            let centre = mid + n_hat * (l * (1.0 - bulge * bulge) / (4.0 * bulge));
            let r = (a - centre).norm_squared().sqrt();
            for v in closed.loop_.vertices().iter().take(n).skip(1) {
                let e = ((v.pos() - centre).norm_squared().sqrt() - r).abs();
                worst = worst.max(e);
            }
        }
    }
    assert!(
        worst < 1e-13,
        "a declared station left the leg's own carrier by {worst}"
    );
}

/// **A near-full-period sweep.** A bulge approaching the full turn
/// drives θ = 4·atan(b) toward 2π; the split must still mint exactly
/// its declared stations and declare each a tangent joint. The
/// CLOSING line's junction turn goes indeterminate at the tighter ε
/// rows for the most extreme bulges (that is the closer's band, not
/// the split's), so a chain that does not build is skipped rather
/// than asserted on.
#[test]
fn a_near_full_period_sweep_still_places_its_stations() {
    let mut built = 0usize;
    for bulge in [0.99, 1.0, 1.01, 10.0, 100.0, 1e6] {
        for n in [2usize, 3, 5] {
            let Ok(closed) = Open
                .at(p2(1.0, 0.0))
                .arc_to(
                    Bulge {
                        p: p2(-1.0, 0.0),
                        b: bulge,
                    }
                    .split(n),
                    t(),
                )
                .and_then(|open| open.line_to(Start, t()))
            else {
                continue;
            };
            built += 1;
            assert_eq!(closed.loop_.vertices().len(), n + 1);
            let joints: Vec<usize> = (1..n).collect();
            assert_eq!(
                closed.loop_.tangent_joints(),
                joints.as_slice(),
                "bulge={bulge} n={n}: the interior stations are the joints"
            );
        }
    }
    assert!(built >= 9, "too few near-full-period chains built: {built}");
}

/// **A `Center` leg split about its AUTHORED centre, CW.** The pole of
/// a CW semicircle must be as exact as the CCW one the PR pins.
#[test]
fn a_cw_center_split_places_its_pole_exactly() {
    let closed = Open
        .at(p2(1.0, 0.0))
        .arc_to(
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Cw,
                p: p2(-1.0, 0.0),
            }
            .split(2),
            t(),
        )
        .unwrap()
        .line_to(Start, t())
        .unwrap();
    let pole = closed.loop_.vertices()[1].pos();
    assert_eq!(
        (pole.x.to_bits(), pole.y.to_bits()),
        (0.0f64.to_bits(), (-1.0f64).to_bits()),
        "the CW pole is not (0, -1) bit-exact: {pole:?}"
    );
}

/// **The equator fixture's bits, re-measured.** `(0.5, 0)` exact and
/// both piece bulges `tan(π/8)` = 0x3fda827999fcef32, as claimed.
#[test]
fn the_equator_fixture_bits_are_as_claimed() {
    let closed = Open
        .at(p2(0.0, -0.5))
        .arc_to(
            Bulge {
                p: p2(0.0, 0.5),
                b: 1.0,
            }
            .split(2),
            t(),
        )
        .unwrap()
        .line_to(Start, t())
        .unwrap();
    let v = closed.loop_.vertices();
    assert_eq!(
        (v[1].pos().x.to_bits(), v[1].pos().y.to_bits()),
        (0.5f64.to_bits(), 0.0f64.to_bits())
    );
    assert_eq!(v[0].bulge().to_bits(), 0x3fda_8279_99fc_ef32);
    assert_eq!(v[1].bulge().to_bits(), 0x3fda_8279_99fc_ef32);
    assert_eq!(FRAC_PI_8.tan().to_bits(), 0x3fda_8279_99fc_ef32);
    assert_eq!(closed.loop_.tangent_joints(), &[1]);
}

// ------------------------------------------------------------------
// 2. The wrapper's admissibility: what the seal actually admits.
// ------------------------------------------------------------------

/// **DEFECT (this row asserts the defect).** The PR argues that "a
/// split on a fused verb's spec is a MISSING IMPL … unrepresentable
/// rather than refused", and `verbs.rs`'s `Split` doc says the modes
/// that also serve a fused verb "have no split there". They do:
/// `TangentIncoming` is implemented for `Split<S>` by a BLANKET impl
/// and `arc_fillet` takes `S: TangentIncoming<T>`, so
/// `Sweep{..}.split(n)` compiles as a fused verb's incoming spec and
/// the declared count is SILENTLY DROPPED — in the emitted geometry
/// and in the recorded step alike.
#[test]
fn a_declared_split_on_a_fused_incoming_is_silently_ignored() {
    let chain = |n: Option<usize>| {
        let sweep = Sweep {
            r: 2.0,
            side: ArcSide::Left,
            angle: 0.8,
        };
        let dir = Open.at(p2(0.0, 0.0)).angle(0.0, t()).unwrap();
        let opened = match n {
            None => dir.arc_fillet(sweep, 0.25, t()),
            Some(n) => dir.arc_fillet(sweep.split(n), 0.25, t()),
        }
        .unwrap();
        opened
            .toward(-1.0, 0.0, t())
            .unwrap()
            .to(p2(0.0, 3.0), t())
            .unwrap()
            .line_to(Start, t())
            .unwrap()
    };
    let plain = chain(None);
    let split = chain(Some(5));
    assert_eq!(
        bits(&plain),
        bits(&split),
        "a declared split on a fused incoming changed the geometry — \
         if this reds, the hole this row records has been closed"
    );
    assert_eq!(
        plain.loop_.tangent_joints(),
        split.loop_.tangent_joints(),
        "the declared stations were never minted"
    );
}

/// **DEFECT (asserts the defect).** `Split`'s fields are public and
/// its `TangentIncoming` impl is blanket over `S`, so a NESTED split
/// type-checks — and the inner count is silently dropped, the outer
/// one winning.
#[test]
fn a_nested_split_silently_drops_the_inner_count() {
    let sweep = || Sweep {
        r: 1.0,
        side: ArcSide::Left,
        angle: 1.2,
    };
    let nested = profile::Split {
        spec: sweep().split(4),
        n: 2,
    };
    let n = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t())
        .unwrap()
        .arc_to(nested, t())
        .unwrap()
        .line_to(Start, t())
        .unwrap();
    let flat = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t())
        .unwrap()
        .arc_to(sweep().split(2), t())
        .unwrap()
        .line_to(Start, t())
        .unwrap();
    assert_eq!(
        bits(&n),
        bits(&flat),
        "the inner declared count was not dropped — if this reds, the \
         nesting hole this row records has been closed"
    );
}

/// The `Via` split, which the PR's own rows never exercise: it must
/// land the unsplit leg's end vertex and declare its stations.
#[test]
fn a_via_leg_splits_about_its_own_derived_carrier() {
    let via = || Via {
        q: p2(1.0, 0.6),
        p: p2(2.0, 0.0),
    };
    let plain = Open
        .at(p2(0.0, 0.0))
        .arc_to(via(), t())
        .unwrap()
        .line_to(Start, t())
        .unwrap();
    let split = Open
        .at(p2(0.0, 0.0))
        .arc_to(via().split(3), t())
        .unwrap()
        .line_to(Start, t())
        .unwrap();
    let pv = plain.loop_.vertices();
    let sv = split.loop_.vertices();
    assert_eq!(pv.len(), 2);
    assert_eq!(sv.len(), 4);
    assert_eq!(
        (pv[1].pos().x.to_bits(), pv[1].pos().y.to_bits()),
        (sv[3].pos().x.to_bits(), sv[3].pos().y.to_bits()),
        "the split leg must land on the unsplit leg's end vertex"
    );
    assert_eq!(split.loop_.tangent_joints(), &[1, 2]);
}

/// **The middle station divides by the CHORD**, which the plain leg
/// never needs: an even split's `centre + R·m̂` takes the unit normal
/// of `end − at`. A `Sweep` leg whose authored angle approaches a full
/// turn drives that chord to zero, so this row walks the angle up to
/// 2π at an even and an odd count and asserts the kernel either
/// refuses typed or produces finite stations — never a NaN vertex.
#[test]
fn a_full_turn_sweep_leg_never_places_a_non_finite_station() {
    use std::f64::consts::TAU;
    for angle in [3.0, TAU - 1e-3, TAU - 1e-9, TAU, TAU + 0.5] {
        for n in [2usize, 3, 4] {
            let built = Open
                .at(p2(0.0, 0.0))
                .angle(0.0, t())
                .and_then(|d| {
                    d.arc_to(
                        Sweep {
                            r: 1.0,
                            side: ArcSide::Left,
                            angle,
                        }
                        .split(n),
                        t(),
                    )
                })
                .and_then(|open| open.line_to(Start, t()));
            let Ok(closed) = built else {
                continue;
            };
            for v in closed.loop_.vertices() {
                assert!(
                    v.pos().x.is_finite() && v.pos().y.is_finite() && v.bulge().is_finite(),
                    "angle={angle} n={n}: a non-finite vertex {v:?}"
                );
            }
        }
    }
}
