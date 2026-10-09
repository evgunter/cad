//! Review r1 probes for PR 4397 (the annular one-segment tube through a
//! plate, `topo::stands` `across_edges`). Each family prints one line
//! per op in both orders — `differential::outcome`, then the mesh check
//! and the volume against a closed form re-derived here — so base and
//! head can be diffed. `cargo test -p sweep --test all r1_probe --
//! --ignored --nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Tol, Vec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::test_support::brick;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, Body, BooleanResult};

use crate::common::differential::{every_op_both_orders, outcome};

fn tol() -> Tol {
    Tol::witness()
}

type Seg = (Point2<f64>, Segment<f64>);

/// A circle about `c` of radius `r` in `arcs` equal arcs, its first
/// vertex at azimuth `az`, counterclockwise iff `ccw`.
fn circle(c: (f64, f64), r: f64, az: f64, ccw: bool, arcs: usize) -> Vec<Seg> {
    let sweep = if ccw { TAU } else { -TAU };
    let n = arcs as f64;
    (0..arcs)
        .map(|k| {
            let a = az + sweep * k as f64 / n;
            (
                Point2::new(c.0 + r * a.cos(), c.1 + r * a.sin()),
                Segment::Arc(Arc2 {
                    centre: Point2::new(c.0, c.1),
                    radius: r,
                    sweep: sweep / n,
                }),
            )
        })
        .collect()
}

/// A polygon through `pts` (given counterclockwise), reversed unless `ccw`.
fn poly(pts: &[(f64, f64)], ccw: bool) -> Vec<Seg> {
    let mut v: Vec<(f64, f64)> = pts.to_vec();
    if !ccw {
        v.reverse();
    }
    v.into_iter()
        .map(|(x, y)| (Point2::new(x, y), Segment::Line))
        .collect()
}

/// Regular `n`-gon of circumradius `r` about `c`, first vertex at `az`.
fn ngon(c: (f64, f64), r: f64, n: usize, az: f64) -> Vec<(f64, f64)> {
    (0..n)
        .map(|k| {
            let a = az + TAU * k as f64 / n as f64;
            (c.0 + r * a.cos(), c.1 + r * a.sin())
        })
        .collect()
}

/// An annular sector slot about the origin between radii `ri < ro`,
/// from azimuth `a0` sweeping `s` counterclockwise; square ends, or
/// (`bulge`) each end a quarter-turn arc bulging out of the slot, meeting
/// the walls at a corner (no tangency to declare).
fn c_slot(ri: f64, ro: f64, a0: f64, s: f64, bulge: bool) -> Vec<Seg> {
    let at = |r: f64, a: f64| Point2::new(r * a.cos(), r * a.sin());
    let o = Point2::new(0.0, 0.0);
    let arc = |r: f64, sw: f64| {
        Segment::Arc(Arc2 {
            centre: o,
            radius: r,
            sweep: sw,
        })
    };
    let (rm, h) = ((ri + ro) / 2.0, (ro - ri) / 2.0);
    let big = h * 2.0f64.sqrt();
    let a1 = a0 + s;
    // `sign` +1 bulges toward increasing azimuth (the `a1` end).
    let end = |a: f64, sign: f64| {
        if bulge {
            let t = (-a.sin(), a.cos());
            Segment::Arc(Arc2 {
                centre: Point2::new(rm * a.cos() - sign * h * t.0, rm * a.sin() - sign * h * t.1),
                radius: big,
                sweep: PI / 2.0,
            })
        } else {
            Segment::Line
        }
    };
    vec![
        (at(ro, a0), arc(ro, s)),
        (at(ro, a1), end(a1, 1.0)),
        (at(ri, a1), arc(ri, -s)),
        (at(ri, a0), end(a0, -1.0)),
    ]
}

fn body_of(loops: Vec<Vec<Seg>>, z: (f64, f64)) -> Body<f64> {
    let loops = loops.into_iter().map(RawLoop::new).collect();
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap_or_else(|e| panic!("profile: {e:?}"));
    let depth = Extrusion::Distance {
        depth: z.1 - z.0,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z.0));
    topo::transform_rigid(&body, &up, tol()).unwrap()
}

fn fin(w: &str, b: Body<f64>) -> AtRestBody<f64> {
    topo::test_support::finished(w, b, tol())
}

fn plate() -> AtRestBody<f64> {
    fin("plate", brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol()))
}

/// Every op both orders of `a` (volume `va`) and `b` (`vb`), common
/// volume `vab`: one line each, never panicking.
fn report(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, v: (f64, f64, f64)) {
    for (op, r, want) in every_op_both_orders(a, b, v, tol()) {
        let extra = match &r {
            Ok(BooleanResult::Body(bb)) => {
                let mesh = match mesh::tessellate(&bb.body, 5e-3, tol()) {
                    Ok(m) => match mesh::validate::check_mesh(&m) {
                        Ok(()) => "mesh=ok".to_string(),
                        Err(e) => format!("mesh=BAD {e:?}"),
                    },
                    Err(e) => format!("tess=ERR {:.60}", format!("{e:?}")),
                };
                mesh
            }
            _ => String::new(),
        };
        println!("R1 {what}: {op} => {} {extra}", outcome(r, want, tol()));
    }
}

/// A tube of `loops` over `z` (profile area `area`) against the plate.
fn vs_plate(what: &str, loops: Vec<Vec<Seg>>, area: f64, z: (f64, f64)) {
    let t = match std::panic::catch_unwind(|| fin("tube", body_of(loops, z))) {
        Ok(t) => t,
        Err(_) => {
            println!("R1 {what}: BUILD-PANIC");
            return;
        }
    };
    let over = (z.1.min(1.0) - z.0.max(0.0)).max(0.0);
    report(what, &t, &plate(), (area * (z.1 - z.0), 36.0, area * over));
}

fn annulus(ro: f64, ri: f64, ao: f64, ai: f64) -> Vec<Vec<Seg>> {
    vec![
        circle((0.0, 0.0), ro, ao, true, 1),
        circle((0.0, 0.0), ri, ai, false, 1),
    ]
}

const SUNK: (f64, f64) = (0.5, 2.5);
const THROUGH: (f64, f64) = (-0.5, 1.5);

/// Claim 1: thin rings (inner nearly the outer), opposed and aligned.
#[test]
#[ignore = "review probe"]
fn r1_probe_thin_rings() {
    for ri in [0.99, 0.999, 0.9995, 0.9999, 0.99999] {
        for (ao, ai) in [(0.0, PI), (0.0, 0.0), (1.0, 4.0)] {
            for (zn, z) in [("sunk", SUNK), ("through", THROUGH)] {
                vs_plate(
                    &format!("thin ri={ri} ({ao},{ai}) {zn}"),
                    annulus(1.0, ri, ao, ai),
                    PI * (1.0 - ri * ri),
                    z,
                );
            }
        }
    }
}

/// Claim 1: the inner circle off centre, tangent-close to the outer at
/// `+x` with gap `g`; vertices opposed, both at the close point, or
/// both away from it.
#[test]
#[ignore = "review probe"]
fn r1_probe_tangent_close() {
    for g in [1e-2, 1e-3, 1e-5, 1e-7] {
        for (ao, ai) in [(0.0, PI), (0.0, 0.0), (PI, PI), (PI, 0.0), (2.0, 5.0)] {
            let loops = vec![
                circle((0.0, 0.0), 1.0, ao, true, 1),
                circle((0.5 - g, 0.0), 0.5, ai, false, 1),
            ];
            vs_plate(&format!("close g={g} ({ao},{ai})"), loops, PI * 0.75, SUNK);
        }
    }
}

/// Claim 1: three and four nested one-segment annuli, every
/// neighbouring vertex pair opposed, united one tube at a time.
#[test]
#[ignore = "review probe"]
fn r1_probe_nested_rings() {
    let three = [(1.0, 0.85), (0.7, 0.55), (0.4, 0.25)];
    let four = [(1.0, 0.88), (0.76, 0.64), (0.52, 0.4), (0.28, 0.16)];
    for (name, rings, z) in [
        ("three sunk", &three[..], SUNK),
        ("four sunk", &four[..], SUNK),
        ("three through", &three[..], THROUGH),
    ] {
        let mut acc: Option<AtRestBody<f64>> = None;
        let mut area = 0.0;
        for (k, &(ro, ri)) in rings.iter().enumerate() {
            let (ao, ai) = if k % 2 == 0 { (0.0, PI) } else { (PI, 0.0) };
            area += PI * (ro * ro - ri * ri);
            let t = fin("ring", body_of(annulus(ro, ri, ao, ai), z));
            acc = Some(match acc {
                None => t,
                Some(a) => match topo::union(&a, &t, tol()) {
                    Ok(BooleanResult::Body(bb)) => fin("rings", bb.body.into_body()),
                    other => {
                        println!(
                            "R1 nested {name}: ring union => {}",
                            outcome(other, 0.0, tol())
                        );
                        return;
                    }
                },
            });
        }
        let acc = acc.unwrap();
        let over = z.1.min(1.0) - z.0.max(0.0);
        report(
            &format!("nested {name}"),
            &acc,
            &plate(),
            (area * (z.1 - z.0), 36.0, area * over),
        );
    }
}

/// Claim 1: one-segment circles mixed with polygons in one profile.
#[test]
#[ignore = "review probe"]
fn r1_probe_mixed() {
    let hex = ngon((0.0, 0.0), 1.0, 6, 0.3);
    let hex_area = 6.0 * 0.5 * (TAU / 6.0).sin();
    let sq = ngon((0.0, 0.0), 0.5, 4, 0.7);
    let sq_area = 2.0 * 0.25;
    let tri = ngon((0.0, 0.0), 0.6, 3, 1.1);
    let tri_area = 3.0 * 0.5 * 0.36 * (TAU / 3.0).sin();
    let rows: Vec<(&str, Vec<Vec<Seg>>, f64)> = vec![
        (
            "hex outer, circle hole opposed",
            vec![
                poly(&hex, true),
                circle((0.0, 0.0), 0.5, PI + 0.3, false, 1),
            ],
            hex_area - PI * 0.25,
        ),
        (
            "hex outer, near-inscribed circle hole",
            vec![poly(&hex, true), circle((0.0, 0.0), 0.866, PI, false, 1)],
            hex_area - PI * 0.866 * 0.866,
        ),
        (
            "circle outer, square hole",
            vec![circle((0.0, 0.0), 1.0, PI, true, 1), poly(&sq, false)],
            PI - sq_area,
        ),
        (
            "circle outer, triangle hole",
            vec![circle((0.0, 0.0), 1.0, 0.0, true, 1), poly(&tri, false)],
            PI - tri_area,
        ),
        (
            "square outer near-touching circle hole",
            vec![
                poly(&[(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)], true),
                circle((0.0, 0.0), 0.999, 0.0, false, 1),
            ],
            4.0 - PI * 0.999 * 0.999,
        ),
        (
            "circle outer, circle hole and square hole",
            vec![
                circle((0.0, 0.0), 1.0, 0.0, true, 1),
                circle((-0.45, 0.0), 0.3, 0.0, false, 1),
                poly(&ngon((0.45, 0.0), 0.3, 4, 0.0), false),
            ],
            PI - PI * 0.09 - 2.0 * 0.09,
        ),
    ];
    for (name, loops, area) in rows {
        for (zn, z) in [("sunk", SUNK), ("through", THROUGH)] {
            vs_plate(&format!("mixed {name} {zn}"), loops.clone(), area, z);
        }
    }
}

/// Claim 1: slotted (non-circular) region faces: a C-shaped annular
/// sector slot, square or round ends, at several sweeps.
#[test]
#[ignore = "review probe"]
fn r1_probe_slots() {
    for s in [PI / 2.0, PI, 1.5 * PI, 1.9 * PI] {
        for round in [false, true] {
            let (ri, ro) = (0.5, 1.0);
            let h = (ro - ri) / 2.0;
            let big2 = 2.0 * h * h;
            let area =
                s / 2.0 * (ro * ro - ri * ri) + if round { big2 * (PI / 2.0 - 1.0) } else { 0.0 };
            for (zn, z) in [("sunk", SUNK), ("through", THROUGH)] {
                vs_plate(
                    &format!("slot s={s:.3} round={round} {zn}"),
                    vec![c_slot(ri, ro, 0.4, s, round)],
                    area,
                    z,
                );
            }
        }
    }
}

/// Claim 3: an uncut shell whose every vertex and edge lies on the other
/// operand while a disc face's interior does not — a one-segment
/// cylinder plug in a plate's one-segment bore of the same radius,
/// flush on the wall. Before the PR the disc offered no witness.
#[test]
#[ignore = "review probe"]
fn r1_probe_plug_in_bore() {
    let sq = poly(&[(-3.0, -3.0), (3.0, -3.0), (3.0, 3.0), (-3.0, 3.0)], true);
    for arcs in [1, 2] {
        for (bz, pz) in [((0.0, 1.0), (0.0, 1.0)), ((0.0, 1.0), (0.25, 0.75))] {
            for (ab, ap) in [(0.0, PI), (0.0, 0.0)] {
                let bored = fin(
                    "bored plate",
                    body_of(
                        vec![sq.clone(), circle((0.0, 0.0), 0.5, ab, false, arcs)],
                        bz,
                    ),
                );
                let plug = fin(
                    "plug",
                    body_of(vec![circle((0.0, 0.0), 0.5, ap, true, arcs)], pz),
                );
                let vp = PI * 0.25 * (pz.1 - pz.0);
                let vb = (36.0 - PI * 0.25) * (bz.1 - bz.0);
                report(
                    &format!("plug arcs={arcs} bore z={bz:?} plug z={pz:?} az=({ab},{ap})"),
                    &plug,
                    &bored,
                    (vp, vb, 0.0),
                );
            }
        }
    }
}

/// Claim 3: an operand against an identical copy of itself — every
/// witness lies on the other boundary, so the shell witness reaches the
/// `On` question. A disc face's new rung-3 witness must read `On` too.
#[test]
#[ignore = "review probe"]
fn r1_probe_identical_shells() {
    let rows: Vec<(&str, Vec<Vec<Seg>>, f64)> = vec![
        (
            "cylinder",
            vec![circle((0.0, 0.0), 0.5, 0.0, true, 1)],
            PI * 0.25,
        ),
        ("annulus", annulus(1.0, 0.5, 0.0, PI), PI * 0.75),
        (
            "thin annulus",
            annulus(1.0, 0.999, 0.0, PI),
            PI * (1.0 - 0.999 * 0.999),
        ),
    ];
    for (name, loops, area) in rows {
        let a = fin("a", body_of(loops.clone(), (0.0, 1.0)));
        let b = fin("b", body_of(loops, (0.0, 1.0)));
        report(&format!("identical {name}"), &a, &b, (area, area, area));
    }
}

/// [`report`] with each order's flush pairs declared
/// (`fixtures::flush_declarations`); a refused finding prints, never
/// panics.
fn report_declared(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, v: (f64, f64, f64)) {
    let (va, vb, vab) = v;
    for (tag, x, y, vx) in [("A·B", a, b, va), ("B·A", b, a, vb)] {
        let d = match std::panic::catch_unwind(|| {
            topo::test_support::flush_declarations(x, y, tol())
        }) {
            Ok(d) => d,
            Err(_) => {
                println!("R1 {what}: {tag} => DECLARE-PANIC");
                continue;
            }
        };
        let runs = [
            ("∪", topo::union_with(x, y, &d, tol()), va + vb - vab),
            ("∩", topo::intersect_with(x, y, &d, tol()), vab),
            ("∖", topo::subtract_with(x, y, &d, tol()), vx - vab),
        ];
        for (op, r, want) in runs {
            println!(
                "R1 {what}: {tag} {op} declared => {}",
                outcome(r, want, tol())
            );
        }
    }
}

/// Claim 3, declared: the plug in the bore and the identical shells
/// with their flush pairs declared, so the op reaches the uncut-shell
/// verdict rather than refusing the coincidence.
#[test]
#[ignore = "review probe"]
fn r1_probe_declared_flush() {
    let sq = poly(&[(-3.0, -3.0), (3.0, -3.0), (3.0, 3.0), (-3.0, 3.0)], true);
    for arcs in [1, 2] {
        for (ab, ap) in [(0.0, PI), (0.0, 0.0)] {
            for pz in [(0.0, 1.0), (0.25, 0.75), (0.0, 0.5)] {
                let bored = fin(
                    "bored plate",
                    body_of(
                        vec![sq.clone(), circle((0.0, 0.0), 0.5, ab, false, arcs)],
                        (0.0, 1.0),
                    ),
                );
                let plug = fin(
                    "plug",
                    body_of(vec![circle((0.0, 0.0), 0.5, ap, true, arcs)], pz),
                );
                let vp = PI * 0.25 * (pz.1 - pz.0);
                report_declared(
                    &format!("plug arcs={arcs} plug z={pz:?} az=({ab},{ap})"),
                    &plug,
                    &bored,
                    (vp, 36.0 - PI * 0.25, 0.0),
                );
            }
            // An annular plug in an annular bore.
            let bored = fin(
                "annular bore",
                body_of(
                    vec![sq.clone(), circle((0.0, 0.0), 1.0, ab, false, arcs)],
                    (0.0, 1.0),
                ),
            );
            let core = fin(
                "core",
                body_of(vec![circle((0.0, 0.0), 0.5, ab, true, arcs)], (0.0, 1.0)),
            );
            let plate_with_core = match topo::union(&bored, &core, tol()) {
                Ok(BooleanResult::Body(bb)) => fin("holed+core", bb.body.into_body()),
                other => {
                    println!("R1 annular bore build => {}", outcome(other, 0.0, tol()));
                    continue;
                }
            };
            let ring = fin(
                "ring plug",
                body_of(
                    vec![
                        circle((0.0, 0.0), 1.0, ap, true, arcs),
                        circle((0.0, 0.0), 0.5, ap + PI, false, arcs),
                    ],
                    (0.0, 1.0),
                ),
            );
            report_declared(
                &format!("ring plug arcs={arcs} az=({ab},{ap})"),
                &ring,
                &plate_with_core,
                (PI * 0.75, 36.0 - PI * 0.75, 0.0),
            );
        }
    }
    let cyl = vec![circle((0.0, 0.0), 0.5, 0.0, true, 1)];
    let a = fin("a", body_of(cyl.clone(), (0.0, 1.0)));
    let b = fin("b", body_of(cyl, (0.0, 1.0)));
    report_declared(
        "identical cylinder",
        &a,
        &b,
        (PI * 0.25, PI * 0.25, PI * 0.25),
    );
    let ann = annulus(1.0, 0.5, 0.0, PI);
    let a = fin("a", body_of(ann.clone(), (0.0, 1.0)));
    let b = fin("b", body_of(ann, (0.0, 1.0)));
    report_declared(
        "identical annulus",
        &a,
        &b,
        (PI * 0.75, PI * 0.75, PI * 0.75),
    );
}
