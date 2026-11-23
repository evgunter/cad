//! **Review probes, R1 of PR 4704** (CONTACTHOLD's flush quarter round):
//! an independent oracle for the flush and corner postures (volume,
//! surface area and solid count), the round turned about its rim line
//! by small angles either way (the boundary between a tangent pair and
//! a crossing, and between a half-turn wedge and a reflex or convex
//! one), and the same-side posture where the slab lies beyond the rim.
//! The print rows assert nothing; run with `--ignored --nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::an_arc_tangent_at_a_boolean_vertex::{quarter_round, slab, slab_between};
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanResult, mass_properties};

/// The quarter round turned by `theta` about its rim line `x = 2, z = 1`
/// (along `y`): `theta > 0` lifts its bottom off `z = 1` (the wall then
/// dips below `z = 1` at `x > 2`), `theta < 0` dips the bottom into the
/// slab (the wall lifts).
fn turned_round(theta: f64) -> AtRestBody<f64> {
    let (s, c) = theta.sin_cos();
    // R about +y: x ↦ (c, 0, −s), z ↦ (s, 0, c); about the point (2, 0, 1).
    let rx = Vec3::new(c, 0.0, -s);
    let rz = Vec3::new(s, 0.0, c);
    let centre = Vec3::new(2.0, 0.0, 1.0);
    let origin = centre - (rx * 2.0 + rz * 1.0);
    let plane = SketchPlane::from_frame(geom_core::test_support::rigid(
        &Affine3::from_parts(geom_core::Mat3::from_cols(rz, rx, Vec3::unit_y()), origin),
        geom_core::Tol::witness(),
    ));
    let b90 = (core::f64::consts::PI / 8.0).tan();
    let profile = Profile::new(
        plane,
        vec![bulge_loop(vec![
            (Point2::new(1.0, 0.5), 0.0),
            (Point2::new(1.0, 2.0), -b90),
            (Point2::new(2.0, 3.0), 0.0),
            (Point2::new(3.0, 3.0), 0.0),
            (Point2::new(3.0, 0.5), 0.0),
        ])],
    )
    .validate(Tol::witness())
    .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 4.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    finished("the turned round", body, Tol::witness())
}

#[derive(Debug, Clone, Copy)]
struct Got {
    volume: f64,
    area: f64,
    solids: usize,
}

fn read(b: &AtRestBody<f64>) -> Got {
    let m = mass_properties(
        b,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap();
    Got {
        volume: m.enclosure.volume,
        area: m.enclosure.surface_area,
        solids: b.solids().count(),
    }
}

fn outcome(r: &Result<BooleanResult<f64>, topo::BooleanError>) -> Result<Option<Got>, String> {
    match r {
        Ok(BooleanResult::Empty) => Ok(None),
        Ok(BooleanResult::Body(b)) => Ok(Some(read(&b.body))),
        Err(e) => Err(format!("{e:?}")),
    }
}

/// **The flush and corner postures against an independent oracle**:
/// volume, surface area and solid count of every op.
/// Union: `vs + vr`, area `As + Ar − 2·6` (the 1.5 × 4 resting contact
/// counted on both sides), one solid. Differences: the operand's volume,
/// area and one solid. Meet: `Empty`.
#[test]
fn flush_and_corner_against_area_and_solid_count() {
    let round = quarter_round();
    let r = read(&round);
    let contact = 1.5 * 4.0;
    for (posture, s) in [
        ("flush", slab(2.0, -1.0, 6.0)),
        ("corner", slab(2.0, 0.0, 4.0)),
    ] {
        let sv = read(&s);
        let cases = [
            (
                "slab ∪ round",
                topo::union(&s, &round, Tol::witness()),
                sv.volume + r.volume,
                sv.area + r.area - 2.0 * contact,
            ),
            (
                "slab ∖ round",
                topo::subtract(&s, &round, Tol::witness()),
                sv.volume,
                sv.area,
            ),
            (
                "round ∖ slab",
                topo::subtract(&round, &s, Tol::witness()),
                r.volume,
                r.area,
            ),
        ];
        for (op, res, vol, area) in cases {
            let tag = format!("{posture} / {op}");
            let g = outcome(&res)
                .unwrap_or_else(|e| panic!("{tag}: {e}"))
                .unwrap_or_else(|| panic!("{tag}: Empty"));
            assert!(
                (g.volume - vol).abs() <= 1e-9 * vol,
                "{tag}: volume {g:?} vs {vol}"
            );
            assert!(
                (g.area - area).abs() <= 1e-9 * area,
                "{tag}: area {g:?} vs {area}"
            );
            assert_eq!(g.solids, 1, "{tag}: one solid: {g:?}");
        }
        let meet = topo::intersect(&s, &round, Tol::witness());
        assert!(
            matches!(meet, Ok(BooleanResult::Empty)),
            "{posture}: meet {meet:?}"
        );
    }
}

/// **The round turned about its rim line by `±φ`**, printed against the
/// closed form, flush and corner, undeclared. Dipped (`θ = −φ`) its
/// bottom overlaps the slab in the triangle `1.125·tan φ` per unit
/// depth, `4.5·tan φ` in all; lifted (`θ = +φ`) it touches the slab only
/// along the rim line (its wall dips below `z = 1` past the slab's edge).
#[test]
#[ignore = "review probe: prints; run with --ignored --nocapture"]
fn r1_turned_about_the_rim() {
    let eps = Tol::witness().eps();
    for phi in [
        1e-15, 1e-12, 1e-10, 1e-9, 3e-9, 1e-8, 1e-7, 1e-5, 1e-3, 1e-1,
    ] {
        for theta in [-phi, phi] {
            let round = turned_round(theta);
            let r = read(&round);
            let ov = if theta < 0.0 { 4.5 * phi.tan() } else { 0.0 };
            for (posture, s) in [
                ("flush", slab(2.0, -1.0, 6.0)),
                ("corner", slab(2.0, 0.0, 4.0)),
            ] {
                let sv = read(&s);
                let tag = |op: &str| format!("θ={theta:e} {posture} / {op}");
                for (op, res, want) in [
                    (
                        "∪",
                        topo::union(&s, &round, Tol::witness()),
                        Some(sv.volume + r.volume - ov),
                    ),
                    (
                        "s∖r",
                        topo::subtract(&s, &round, Tol::witness()),
                        Some(sv.volume - ov),
                    ),
                    (
                        "r∖s",
                        topo::subtract(&round, &s, Tol::witness()),
                        Some(r.volume - ov),
                    ),
                    (
                        "∩",
                        topo::intersect(&s, &round, Tol::witness()),
                        if ov > 0.0 { Some(ov) } else { None },
                    ),
                ] {
                    let o = outcome(&res);
                    let verdict = match (&o, want) {
                        (Ok(Some(g)), Some(w)) => {
                            let err = (g.volume - w).abs();
                            if err <= 1e3 * eps {
                                "ok"
                            } else if err <= ov + 1e3 * eps {
                                "ok-in-band"
                            } else {
                                "WRONG"
                            }
                        }
                        (Ok(None), None) => "ok",
                        (Ok(None), Some(w)) if w <= 1e3 * eps => "ok-in-band(empty)",
                        (Ok(_), _) => "WRONG",
                        (Err(_), _) => "refused",
                    };
                    eprintln!("R1-TURN {} [{verdict}]: want {want:?}; got {o:?}", tag(op));
                }
            }
        }
    }
}

/// **The same-side posture**: the slab lies beyond the rim
/// (`x ∈ [2, 5]`), so its top and the round's wall leave the rim line
/// on the same side (`+x`) and touch only along it, and the round's
/// bottom overhangs. Slab ∖ round and round ∖ slab are the operands,
/// the meet is `Empty`; the union is the two touching along a line.
#[test]
#[ignore = "review probe: prints; run with --ignored --nocapture"]
fn r1_same_side_beyond_the_rim() {
    let round = quarter_round();
    let r = read(&round);
    for (posture, s) in [
        ("beyond", slab_between(2.0, 5.0, -1.0, 6.0)),
        ("beyond-corner", slab_between(2.0, 5.0, 0.0, 4.0)),
    ] {
        let sv = read(&s);
        for (op, res, want) in [
            (
                "∪",
                topo::union(&s, &round, Tol::witness()),
                Some(sv.volume + r.volume),
            ),
            (
                "s∖r",
                topo::subtract(&s, &round, Tol::witness()),
                Some(sv.volume),
            ),
            (
                "r∖s",
                topo::subtract(&round, &s, Tol::witness()),
                Some(r.volume),
            ),
            ("∩", topo::intersect(&s, &round, Tol::witness()), None),
        ] {
            eprintln!(
                "R1-SAME {posture} / {op}: want {want:?}; got {:?}",
                outcome(&res)
            );
        }
    }
}
