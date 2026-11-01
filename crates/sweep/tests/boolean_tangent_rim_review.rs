//! **PR 5968 review: a boolean tangent to a rim at a vertex, off the
//! pins.** The ring's same-pierce tie is read one order down
//! (`vtxfac.rs` `germ_order`, `geom_brep::tied_germ_turn`); the PR's
//! pins are the unit hole at `t = ±0.4, ±0.9`. These rows move every
//! axis the sign reading depends on: the tilt, the hole's radius, the
//! material's side of the rim (a hole, whose rim is convex, against a
//! pocket floor and a boss foot, whose rims are concave, and the solid
//! cylinder), and the winding. The brick `z ≤ z₀` (world) has its top
//! face through the turned rim vertex, where its trace on the rim's
//! face is tangent to the rim.
//!
//! Each op's body, wherever one builds, is held to the closed-form
//! volume, tiers or no tiers: a body that builds at the wrong volume
//! is a wrong answer. A refusal is recorded, not failed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, TAU};

use super::a_split_through_a_wrap_edges_vertex::{hoof, square_hoof};
use std::panic::{AssertUnwindSafe, catch_unwind};

use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};

fn tol() -> Tol {
    Tol::witness()
}

/// As [`hoof`] over the disc of radius `r`.
fn disc_hoof(r: f64, f0: f64, px: f64, py: f64) -> f64 {
    r * r * hoof(f0, px * r, py * r)
}

/// A prism over a region, `z ∈ [lo, hi]`, cut below `z = f0 + px·x +
/// py·y`: the volume under it.
fn prism_below(region: &dyn Fn(f64, f64, f64) -> f64, lo: f64, hi: f64, g: (f64, f64, f64)) -> f64 {
    let (f0, px, py) = g;
    region(f0 - lo, px, py) - region(f0 - hi, px, py)
}

fn circle(r: f64, phase: f64, sweep: f64, arcs: usize) -> ProfileLoop<f64> {
    RawLoop::new((0..arcs).map(|k| {
        let at = phase + TAU * k as f64 / arcs as f64 * sweep.signum();
        (
            Point2::new(r * at.cos(), r * at.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(0.0, 0.0),
                radius: r,
                sweep: sweep / arcs as f64,
            }),
        )
    }))
}

fn square() -> ProfileLoop<f64> {
    let p = |x: f64, y: f64| Point2::new(x, y);
    RawLoop::polygon([p(-2.0, -2.0), p(2.0, -2.0), p(2.0, 2.0), p(-2.0, 2.0)])
}

/// `loops` extruded over `z ∈ [lo, lo + h]`.
fn extruded(loops: Vec<ProfileLoop<f64>>, lo: f64, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    if lo == 0.0 {
        return body;
    }
    let lift = Affine3::translation(Vec3::new(0.0, 0.0, lo));
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&lift, tol()), tol()).unwrap()
}

fn op(o: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<Body<f64>, String> {
    let a = topo::test_support::finished("a", a.clone(), tol());
    let b = topo::test_support::finished("b", b.clone(), tol());
    match boolean_op_with(
        o,
        &a,
        &b,
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    ) {
        Ok(out) => out
            .body()
            .map(|r| r.body.clone().into_body())
            .ok_or_else(|| "empty".to_string()),
        Err(e) => Err(format!("{e:?}")),
    }
}

/// A shape: its body, its rim vertex (local), and its volume under
/// `z = g(x, y)` (local).
struct Shape {
    name: String,
    body: Body<f64>,
    vertex: [f64; 3],
    below: Box<dyn Fn((f64, f64, f64)) -> f64>,
}

fn shapes(r: f64, sweep: f64, arcs: usize) -> Vec<Result<Shape, String>> {
    let phase = FRAC_PI_2;
    let disc = move |f0: f64, px: f64, py: f64| disc_hoof(r, f0, px, py);
    let mut out = Vec::new();
    // A hole: convex rim on the bottom face.
    out.push(Ok(Shape {
        name: format!("hole r {r} arcs {arcs}"),
        body: extruded(vec![square(), circle(r, phase, sweep, arcs)], 0.0, 2.0),
        vertex: [0.0, r, 0.0],
        below: Box::new(move |g| {
            prism_below(&square_hoof, 0.0, 2.0, g) - prism_below(&disc, 0.0, 2.0, g)
        }),
    }));
    // The solid cylinder: convex rim, material inside.
    out.push(Ok(Shape {
        name: format!("cylinder r {r} arcs {arcs}"),
        body: extruded(vec![circle(r, phase, sweep, arcs)], 0.0, 2.0),
        vertex: [0.0, r, 0.0],
        below: Box::new(move |g| prism_below(&disc, 0.0, 2.0, g)),
    }));
    // A pocket z ∈ [1, 2]: concave rim at its floor.
    out.push(
        op(
            BooleanOp::Subtract,
            &extruded(vec![square()], 0.0, 2.0),
            &extruded(vec![circle(r, phase, sweep, arcs)], 1.0, 2.0),
        )
        .map(|body| Shape {
            name: format!("pocket r {r} arcs {arcs}"),
            body,
            vertex: [0.0, r, 1.0],
            below: Box::new(move |g| {
                prism_below(&square_hoof, 0.0, 2.0, g) - prism_below(&disc, 1.0, 2.0, g)
            }),
        })
        .map_err(|e| format!("pocket r {r} arcs {arcs}: setup {e}")),
    );
    // A boss z ∈ [1, 2] on a plate z ∈ [0, 1]: concave rim at its foot.
    out.push(
        op(
            BooleanOp::Union,
            &extruded(vec![square()], 0.0, 1.0),
            &extruded(vec![circle(r, phase, sweep, arcs)], 0.5, 1.5),
        )
        .map(|body| Shape {
            name: format!("boss r {r} arcs {arcs}"),
            body,
            vertex: [0.0, r, 1.0],
            below: Box::new(move |g| {
                prism_below(&square_hoof, 0.0, 1.0, g) + prism_below(&disc, 1.0, 2.0, g)
            }),
        })
        .map_err(|e| format!("boss r {r} arcs {arcs}: setup {e}")),
    );
    out
}

/// Every row's outcome, and the wrong answers.
fn sweep_rows(radii: &[f64], tilts: &[f64]) -> (Vec<String>, Vec<String>) {
    let half = 5.0;
    let (mut table, mut wrong) = (Vec::new(), Vec::new());
    for &r in radii {
        for sweep in [TAU, -TAU] {
            for shape in [1, 2].into_iter().flat_map(|arcs| shapes(r, sweep, arcs)) {
                let shape = match shape {
                    Ok(s) => s,
                    Err(e) => {
                        table.push(format!("SETUP {e}"));
                        continue;
                    }
                };
                let whole = (shape.below)((1e3, 0.0, 0.0));
                for &t in tilts {
                    let turn =
                        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), t);
                    let lift = |p: [f64; 3]| turn.transform_point(Point3::new(p[0], p[1], p[2])).z;
                    let origin = lift([0.0; 3]);
                    let n = [
                        lift([1.0, 0.0, 0.0]) - origin,
                        lift([0.0, 1.0, 0.0]) - origin,
                        lift([0.0, 0.0, 1.0]) - origin,
                    ];
                    let z0 = lift(shape.vertex);
                    let d = z0 - origin;
                    let g = (d / n[2], -n[0] / n[2], -n[1] / n[2]);
                    let below = (shape.below)(g);
                    let a = topo::transform_rigid(
                        &shape.body,
                        &geom_core::test_support::rigid(&turn, tol()),
                        tol(),
                    )
                    .unwrap();
                    let b = sweep::test_support::brick(
                        (-half, half),
                        (-half, half),
                        (-half, z0),
                        tol(),
                    );
                    let slab = 4.0 * half * half * (z0 + half);
                    for (o, name, closed) in [
                        (BooleanOp::Subtract, "A−B", whole - below),
                        (BooleanOp::Intersect, "A∩B", below),
                        (BooleanOp::Union, "A∪B", slab + whole - below),
                    ] {
                        let what = format!("{}, sweep {sweep:+.0}, t {t:+}, {name}", shape.name);
                        let a = topo::test_support::finished("a", a.clone(), tol());
                        let b = topo::test_support::finished("b", b.clone(), tol());
                        let got = boolean_op_with(
                            o,
                            &a,
                            &b,
                            &topo::BooleanInputs::default(),
                            SweepStrategy::Realized,
                            tol(),
                        );
                        let line = match got {
                            Err(e) => {
                                let s = format!("{e:?}");
                                format!("REFUSE {}", &s[..s.len().min(160)])
                            }
                            Ok(out) => match out.body() {
                                None => {
                                    if closed.abs() > 1e-9 {
                                        wrong.push(format!("{what}: empty where {closed}"));
                                    }
                                    "EMPTY".into()
                                }
                                Some(res) => {
                                    let body = &res.body;
                                    let t1 = topo::validate(body).is_ok();
                                    let t2 = topo::validate_closed(body).is_ok();
                                    let t3 = topo::validate_geometric(body, tol()).is_ok();
                                    let t3p =
                                        topo::validate_pseudomanifold(body, &res.contacts, tol());
                                    let m = catch_unwind(AssertUnwindSafe(|| {
                                        topo::test_support::pinned_volume("v", body, tol())
                                    }));
                                    let vol = match m {
                                        Ok(m) => {
                                            let off = (m.volume - closed).abs();
                                            if off > m.volume_pad + 1e-9 * closed.abs() {
                                                wrong.push(format!("{what}: volume {} ± {} vs closed {closed} (off {off:e})", m.volume, m.volume_pad));
                                                format!("WRONG VOLUME off {off:e}")
                                            } else {
                                                "volume ok".into()
                                            }
                                        }
                                        Err(_) => "volume refused".into(),
                                    };
                                    format!(
                                        "BUILD t1 {t1} t2 {t2} t3 {t3} t3' {} {vol}",
                                        match t3p {
                                            Ok(_) => "ok".to_string(),
                                            Err(e) => {
                                                let s = format!("{e:?}");
                                                s[..s.len().min(100)].to_string()
                                            }
                                        }
                                    )
                                }
                            },
                        };
                        table.push(format!("{what}: {line}"));
                    }
                }
            }
        }
    }
    (table, wrong)
}

/// The sweep, printed; red on any body at the wrong volume.
#[test]
#[ignore = "review probe: run by name"]
fn a_tangent_rim_boolean_off_the_pins_builds_at_its_closed_form_or_refuses() {
    let radii: Vec<f64> = std::env::var("REVIEW_RADII")
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or_else(|_| vec![0.5, 1.0, 1.7]);
    let tilts = [0.15, 0.4, 0.7, 0.9, 1.2, -0.15, -0.4, -0.7, -0.9, -1.2];
    let (table, wrong) = sweep_rows(&radii, &tilts);
    for line in &table {
        println!("ROW {line}");
    }
    assert!(
        wrong.is_empty(),
        "{} wrong:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

/// **The brick corner on the rim vertex** (the sweep filed as
/// `work/splitdoor/a-brick-corner-at-a-holes-rim-vertex-leaves-loose-ends.md`):
/// every body it builds, with its measured volume, printed for an
/// outside quadrature to check. `CORNER arcs t spin op status volume pad`.
#[test]
#[ignore = "review probe: run by name"]
fn a_brick_corner_on_a_rim_vertex_prints_its_volumes() {
    for arcs in [1, 2] {
        let a0 = extruded(vec![square(), circle(1.0, FRAC_PI_2, TAU, arcs)], 0.0, 2.0);
        for t in [0.4, 0.9, -0.4, -0.9] {
            let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), t);
            let a =
                topo::transform_rigid(&a0, &geom_core::test_support::rigid(&turn, tol()), tol())
                    .unwrap();
            let v = Point3::new(0.0, t.cos(), t.sin());
            for spin in [FRAC_PI_4, -FRAC_PI_4, 0.3, -0.3, 2.5, -2.5] {
                let brick = sweep::test_support::brick(
                    (v.x - 6.0, v.x),
                    (v.y - 6.0, v.y),
                    (v.z - 6.0, v.z),
                    tol(),
                );
                let about = Affine3::rotation_about_axis(v, Vec3::new(0.0, 0.0, 1.0), spin);
                let b = topo::transform_rigid(
                    &brick,
                    &geom_core::test_support::rigid(&about, tol()),
                    tol(),
                )
                .unwrap();
                for (o, name) in [
                    (BooleanOp::Subtract, "sub"),
                    (BooleanOp::Intersect, "int"),
                    (BooleanOp::Union, "uni"),
                ] {
                    let fa = topo::test_support::finished("a", a.clone(), tol());
                    let fb = topo::test_support::finished("b", b.clone(), tol());
                    let got = boolean_op_with(
                        o,
                        &fa,
                        &fb,
                        &topo::BooleanInputs::default(),
                        SweepStrategy::Realized,
                        tol(),
                    );
                    let (status, vol, pad) = match got {
                        Err(e) => {
                            let s = format!("{e:?}");
                            (
                                format!(
                                    "REFUSE:{}",
                                    s.split(['{', '('])
                                        .take(3)
                                        .collect::<Vec<_>>()
                                        .join("/")
                                        .replace(' ', "")
                                ),
                                f64::NAN,
                                f64::NAN,
                            )
                        }
                        Ok(out) => match out.body() {
                            None => ("EMPTY".to_string(), 0.0, 0.0),
                            Some(res) => {
                                let ok3 = topo::validate_geometric(&res.body, tol()).is_ok();
                                let ok3p =
                                    topo::validate_pseudomanifold(&res.body, &res.contacts, tol())
                                        .is_ok();
                                let m = catch_unwind(AssertUnwindSafe(|| {
                                    topo::test_support::pinned_volume("v", &res.body, tol())
                                }));
                                let (vol, pad) = m
                                    .map(|m| (m.volume, m.volume_pad))
                                    .unwrap_or((f64::NAN, f64::NAN));
                                (format!("BUILD:t3={ok3}:t3p={ok3p}"), vol, pad)
                            }
                        },
                    };
                    println!("CORNER {arcs} {t} {spin} {name} {status} {vol:.15e} {pad:.3e}");
                }
            }
        }
    }
}
