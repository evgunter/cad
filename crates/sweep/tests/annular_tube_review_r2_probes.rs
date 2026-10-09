//! **Review r2 probes for PR #4397** (the annular one-segment tube
//! through a plate; `topo::stands` `across_edges`). Each row runs every
//! op in both orders against an independent closed form, reads it
//! through `differential::outcome` WITH THE CLOSED FORM (not the measured
//! volume), meshes it and `check_mesh`es it, and prints one line per op.
//! A row asserts only that no op is wrong: a typed refusal is printed
//! and tallied, a body that is not `SOUND` (other than the known
//! two-solids-in-a-bore T3′ class, printed `T3P`) or misses its closed
//! form is a failure.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::brick;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;

use crate::common::differential::{every_op_both_orders, outcome};

fn tol() -> Tol {
    Tol::witness()
}

/// A circle about `c` of radius `r`, first vertex at azimuth `az`, in
/// `n` equal arcs, counterclockwise when `ccw`.
fn circle(c: (f64, f64), r: f64, az: f64, ccw: bool, n: usize) -> ProfileLoop<f64> {
    let sweep = if ccw { TAU } else { -TAU };
    RawLoop::new((0..n).map(|k| {
        let a = az + sweep * k as f64 / n as f64;
        (
            Point2::new(c.0 + r * a.cos(), c.1 + r * a.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(c.0, c.1),
                radius: r,
                sweep: sweep / n as f64,
            }),
        )
    }))
}

/// A polygon loop, as given (the caller orders it).
fn poly(p: &[(f64, f64)]) -> ProfileLoop<f64> {
    RawLoop::new(
        p.iter()
            .map(|&(x, y)| (Point2::new(x, y), Segment::Line))
            .collect::<Vec<_>>(),
    )
}

/// A stadium of half-length `a` along x and radius `r` about `c`,
/// counterclockwise: two lines and two half-circle arcs.
fn stadium(c: (f64, f64), a: f64, r: f64) -> ProfileLoop<f64> {
    let p = |x: f64, y: f64| Point2::new(c.0 + x, c.1 + y);
    <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        (p(-a, -r), Segment::Line),
        (
            p(a, -r),
            Segment::Arc(Arc2 {
                centre: p(a, 0.0),
                radius: r,
                sweep: PI,
            }),
        ),
        (p(a, r), Segment::Line),
        (
            p(-a, r),
            Segment::Arc(Arc2 {
                centre: p(-a, 0.0),
                radius: r,
                sweep: PI,
            }),
        ),
    ])
    .with_tangent_joints(vec![0, 1, 2, 3])
}

fn prism(loops: Vec<ProfileLoop<f64>>, z: (f64, f64)) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: z.1 - z.0,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z.0));
    topo::transform_rigid(&body, &up, tol()).unwrap()
}

fn plate() -> Body<f64> {
    brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol())
}

/// Every op in both orders between `a` and `b`, closed forms
/// `(va, vb, vab)`. Returns (printed lines, wrong lines).
fn run(what: &str, a: Body<f64>, b: Body<f64>, v: (f64, f64, f64)) -> (Vec<String>, Vec<String>) {
    let fin = |w, x| topo::test_support::finished(w, x, tol());
    let (a, b) = (fin("a", a), fin("b", b));
    let mut lines = Vec::new();
    let mut wrong = Vec::new();
    for (op, r, want) in every_op_both_orders(&a, &b, v, tol()) {
        let row = format!("{what}: {op}");
        let mut extra = String::new();
        if let Ok(res) = r.as_ref()
            && let Some(bb) = res.body()
        {
            let m = topo::mass_properties(&bb.body, tol());
            match &m {
                Ok(m) => {
                    extra += &format!(" pad={:.1e} dv={:.1e}", m.volume_pad, m.volume - want);
                    if (m.volume - want).abs() > m.volume_pad + 1e-9 * want.max(1.0) {
                        wrong.push(format!("{row}: VOLUME {} want {want}", m.volume));
                    }
                }
                Err(e) => extra += &format!(" mass-err={e:?}"),
            }
            match mesh::tessellate(&bb.body, 5e-3, tol()) {
                Ok(mesh) => match mesh::validate::check_mesh(&mesh) {
                    Ok(()) => extra += " mesh=ok",
                    Err(e) => {
                        extra += " mesh=BAD";
                        wrong.push(format!("{row}: mesh {e:?}"));
                    }
                },
                Err(e) => extra += &format!(" mesh-refused={:.60}", format!("{e:?}")),
            }
            let solids = bb.body.solids().count();
            extra += &format!(" solids={solids}");
        }
        let line = outcome(r, want, tol());
        if line.starts_with("OK BAD")
            && !line.starts_with("OK BAD t2=true t3p=false cert=true operand=true")
        {
            wrong.push(format!("{row}: {line}"));
        }
        if line.starts_with("EMPTY WRONG") || line.starts_with("OK-UNMEASURED") {
            wrong.push(format!("{row}: {line}"));
        }
        let l = format!("{row}: {line}{extra}");
        eprintln!("R2 {l}");
        lines.push(l);
    }
    (lines, wrong)
}

fn tube_vs_plate(what: &str, loops: Vec<ProfileLoop<f64>>, area: f64, z: (f64, f64)) -> Vec<String> {
    let v = (area * (z.1 - z.0), 36.0, area * (z.1.min(1.0) - z.0.max(0.0)).max(0.0));
    run(what, prism(loops, z), plate(), v).1
}

fn annulus(c: (f64, f64), ro: f64, ao: f64, ri: f64, ai: f64) -> Vec<ProfileLoop<f64>> {
    vec![circle(c, ro, ao, true, 1), circle(c, ri, ai, false, 1)]
}

fn check(wrong: Vec<String>) {
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Thin rings, r 1 and 0.999, at aligned, opposed and skew azimuths;
/// sunk and through.
#[test]
fn r2_thin_ring() {
    let mut w = Vec::new();
    let area = PI * (1.0 - 0.999f64.powi(2));
    for (ao, ai) in [(0.0, 0.0), (0.0, PI), (1.0, 4.0), (0.3, 0.3 + PI / 2.0)] {
        for z in [(0.5, 2.5), (-0.5, 1.5)] {
            w.extend(tube_vs_plate(
                &format!("thin ({ao},{ai}) z{z:?}"),
                annulus((0.0, 0.0), 1.0, ao, 0.999, ai),
                area,
                z,
            ));
        }
    }
    check(w);
}

/// The inner circle off centre, tangent-close to the outer: gaps 1e-4
/// and 1e-6, its vertex at the near point, the far point, and the
/// outer's vertex opposed.
#[test]
fn r2_tangent_close_inner() {
    let mut w = Vec::new();
    let area = PI * (1.0 - 0.25);
    for gap in [1e-4, 1e-6] {
        let c = 0.5 - gap;
        for (ao, ai) in [(0.0, 0.0), (PI, 0.0), (0.0, PI), (PI, PI)] {
            let loops = vec![
                circle((0.0, 0.0), 1.0, ao, true, 1),
                circle((c, 0.0), 0.5, ai, false, 1),
            ];
            w.extend(tube_vs_plate(
                &format!("tangent gap={gap} ({ao},{ai})"),
                loops,
                area,
                (0.5, 2.5),
            ));
        }
    }
    check(w);
}

/// `parts` tubes united (each an annulus or a disc, concentric), all
/// over `z`, then every op against the plate.
fn nested(what: &str, parts: &[&[(f64, f64)]], z: (f64, f64)) -> Vec<String> {
    let fin = |w, b| topo::test_support::finished(w, b, tol());
    let mut area = 0.0;
    let mut acc: Option<Body<f64>> = None;
    for part in parts {
        let mut loops = Vec::new();
        for (i, &(r, az)) in part.iter().enumerate() {
            loops.push(circle((0.0, 0.0), r, az, i % 2 == 0, 1));
            area += if i % 2 == 0 { 1.0 } else { -1.0 } * PI * r * r;
        }
        let t = prism(loops, z);
        acc = Some(match acc {
            None => t,
            Some(b) => topo::union(&fin("acc", b), &fin("part", t), tol())
                .unwrap()
                .body()
                .expect("disjoint tubes unite")
                .body
                .clone()
                .into_body(),
        });
    }
    let v = (area * (z.1 - z.0), 36.0, area * (z.1.min(1.0) - z.0.max(0.0)));
    run(&format!("{what} z{z:?}"), acc.unwrap(), plate(), v).1
}

/// Three, four, five and six nested one-segment circles, neighbours
/// opposed; sunk and through.
#[test]
fn r2_nested_rings() {
    let mut w = Vec::new();
    for z in [(0.5, 2.5), (-0.5, 1.5)] {
        w.extend(nested("3 circles", &[&[(1.0, 0.0), (0.75, PI)], &[(0.5, 0.0)]], z));
        w.extend(nested("4 circles", &[&[(1.0, 0.0), (0.75, PI)], &[(0.5, 0.0), (0.25, PI)]], z));
        w.extend(nested(
            "5 circles",
            &[&[(1.0, 0.0), (0.8, PI)], &[(0.6, 0.0), (0.4, PI)], &[(0.2, 0.0)]],
            z,
        ));
        w.extend(nested(
            "6 circles",
            &[
                &[(1.0, 0.0), (0.85, PI)],
                &[(0.7, 0.0), (0.55, PI)],
                &[(0.4, 0.0), (0.25, PI)],
            ],
            z,
        ));
    }
    check(w);
}

/// One-segment circles mixed with polygons in one face.
#[test]
fn r2_mixed_circles_and_polygons() {
    let mut w = Vec::new();
    let z = (0.5, 2.5);
    // A square with a one-segment circular hole, its vertex towards a corner and away.
    for az in [PI / 4.0, 0.0, PI] {
        let loops = vec![
            poly(&[(-1.2, -1.2), (1.2, -1.2), (1.2, 1.2), (-1.2, 1.2)]),
            circle((0.0, 0.0), 1.0, az, false, 1),
        ];
        w.extend(tube_vs_plate(&format!("square-o circle-i az={az}"), loops, 2.4 * 2.4 - PI, z));
    }
    // A one-segment circle with a square hole, the circle's vertex on a diagonal or not.
    for az in [PI / 4.0, 0.0, 5.0 * PI / 4.0] {
        let loops = vec![
            circle((0.0, 0.0), 1.0, az, true, 1),
            poly(&[(-0.6, -0.6), (-0.6, 0.6), (0.6, 0.6), (0.6, -0.6)]),
        ];
        w.extend(tube_vs_plate(&format!("circle-o square-i az={az}"), loops, PI - 1.44, z));
    }
    // A circle with a circle hole and a triangle hole.
    let loops = vec![
        circle((0.0, 0.0), 1.5, PI, true, 1),
        circle((-0.6, 0.0), 0.4, 0.0, false, 1),
        poly(&[(0.4, -0.3), (0.4, 0.3), (1.0, 0.0)]),
    ];
    w.extend(tube_vs_plate(
        "circle-o circle+tri holes",
        loops,
        PI * 2.25 - PI * 0.16 - 0.5 * 0.6 * 0.6,
        z,
    ));
    check(w);
}

/// A slotted (stadium) region face: stadium outer, a one-segment circle
/// or an inner stadium hole; every vertex lies on the tube's walls.
#[test]
fn r2_slotted_region() {
    let mut w = Vec::new();
    let st = |a: f64, r: f64| 4.0 * a * r + PI * r * r;
    for az in [PI / 2.0, 0.0, PI] {
        let loops = vec![
            stadium((0.0, 0.0), 1.0, 0.6),
            circle((0.0, 0.0), 0.3, az, false, 1),
        ];
        w.extend(tube_vs_plate(&format!("slot-o circle-i az={az}"), loops, st(1.0, 0.6) - PI * 0.09, (0.5, 2.5)));
    }
    // A thin slot ring: inner stadium reversed (clockwise) by reversing points.
    let inner = {
        let a = 0.9;
        let r = 0.5;
        let p = |x: f64, y: f64| Point2::new(x, y);
        <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
            (p(-a, -r), Segment::Arc(Arc2 { centre: p(-a, 0.0), radius: r, sweep: -PI })),
            (p(-a, r), Segment::Line),
            (p(a, r), Segment::Arc(Arc2 { centre: p(a, 0.0), radius: r, sweep: -PI })),
            (p(a, -r), Segment::Line),
        ])
        .with_tangent_joints(vec![0, 1, 2, 3])
    };
    let loops = vec![stadium((0.0, 0.0), 1.0, 0.6), inner];
    for z in [(0.5, 2.5), (-0.5, 1.5)] {
        w.extend(tube_vs_plate(&format!("slot ring z{z:?}"), loops.clone(), st(1.0, 0.6) - st(0.9, 0.5), z));
    }
    check(w);
}

/// **Claim 3: a reading the new rung newly decides.** A one-segment
/// cylinder (or annular tube) flush inside a taller coaxial one of the
/// same radii: every vertex, rim and seam of the short one lies on the
/// tall one's wall, so only a point inside a cap decides; the caps are
/// one-vertex discs (or opposed annuli).
#[test]
fn r2_flush_coaxial_cylinder() {
    let mut w = Vec::new();
    for (sa, sb) in [(0.0, 0.0), (0.0, PI), (1.0, 2.5)] {
        let a = prism(vec![circle((0.0, 0.0), 1.0, sa, true, 1)], (0.0, 1.0));
        let b = prism(vec![circle((0.0, 0.0), 1.0, sb, true, 1)], (-1.0, 2.0));
        let (lines, wr) = run(&format!("flush disc ({sa},{sb})"), a, b, (PI, 3.0 * PI, PI));
        let _ = lines;
        w.extend(wr);
    }
    for (sa, sb) in [(0.0, PI), (PI, 0.0)] {
        let a = prism(annulus((0.0, 0.0), 1.0, sa, 0.5, sa + PI), (0.0, 1.0));
        let b = prism(annulus((0.0, 0.0), 1.0, sb, 0.5, sb + PI), (-1.0, 2.0));
        let ar = PI * 0.75;
        let (_, wr) = run(&format!("flush annulus ({sa},{sb})"), a, b, (ar, 3.0 * ar, ar));
        w.extend(wr);
    }
    check(w);
}

/// A tube whose cap sits exactly on the plate's top (z ∈ [1, 3]) or
/// fills its thickness (z ∈ [0, 1]): coincident caps on the plate's
/// faces, opposed vertices.
#[test]
fn r2_flush_caps_on_plate() {
    let mut w = Vec::new();
    let area = PI * 0.75;
    for z in [(1.0, 3.0), (0.0, 1.0), (0.0, 2.0)] {
        w.extend(tube_vs_plate(
            &format!("flush-cap z{z:?}"),
            annulus((0.0, 0.0), 1.0, 0.0, 0.5, PI),
            area,
            z,
        ));
    }
    check(w);
}

/// Past the reach: a ring thinner than `L/4096` beside every edge
/// (r 1 and 0.9995; the outer edge's `L` is 4, so its shortest step is
/// 4/4096 ≈ 9.8e-4 > 5e-4). Measured, not asserted beyond "no wrong
/// body": what each op does at the cliff.
#[test]
fn r2_ring_thinner_than_the_last_step() {
    let mut w = Vec::new();
    for ri in [0.9995, 0.9999] {
        let area = PI * (1.0 - ri * ri);
        for (ao, ai) in [(0.0, 0.0), (0.0, PI)] {
            w.extend(tube_vs_plate(
                &format!("thinner ri={ri} ({ao},{ai})"),
                annulus((0.0, 0.0), 1.0, ao, ri, ai),
                area,
                (0.5, 2.5),
            ));
        }
    }
    check(w);
}

/// What tier 3′ says about the two-solids-in-a-bore results: the cause,
/// printed (the PR names it `CensusUndecidable`).
#[test]
fn r2_t3p_cause_of_the_bore_results() {
    let fin = |w, b| topo::test_support::finished(w, b, tol());
    let t = fin("t", prism(annulus((0.0, 0.0), 1.0, 0.0, 0.999, PI), (-0.5, 1.5)));
    let p = fin("p", plate());
    let r = topo::subtract(&p, &t, tol()).unwrap();
    let bb = r.body().unwrap();
    let e = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol());
    eprintln!("R2T3P thin through B∖A: {:.300}", format!("{e:?}"));
}
