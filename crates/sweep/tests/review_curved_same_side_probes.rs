//! Review probes for PR #5853 (the curved aligned face-pair arm): rows
//! that must NOT read as `SameSideInterference` (faces of one carrier
//! that only abut, mirrored halves, nearly coincident carriers, a shaft
//! in its bore), rows that must (coincident tubes, a mirrored copy), and
//! a cost row (a grid of abutting pin stacks). Every body is tilted off
//! the axes and moved off the origin.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use geom_core::{Band, Point2, Point3, RigidMotion, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, ContactRecords, ValidationError};

const Q: f64 = core::f64::consts::FRAC_PI_8; // tan(θ/4) bulge of a quarter arc is tan(π/8)

fn rect(r0: f64, r1: f64, y0: f64, y1: f64, turn: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(r0, y0), 0.0),
            (Point2::new(r1, y0), 0.0),
            (Point2::new(r1, y1), 0.0),
            (Point2::new(r0, y1), 0.0),
        ],
        turn,
        Tol::witness(),
    )
}

fn ball(turn: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        turn,
        Tol::witness(),
    )
}

fn lower_hemisphere() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, -1.0), Q.tan()),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

fn upper_hemisphere() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), Q.tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    UnitVec3::new(v, "axis", Band::linear(Tol::witness()).unwrap()).unwrap()
}

fn tilted() -> RigidMotion<f64> {
    RigidMotion::rotation_then_translation(
        unit(Vec3::new(0.3, 0.2, 1.0)),
        0.7,
        Vec3::new(0.31, -0.17, 0.42),
    )
    .unwrap()
}

fn spun(angle: f64) -> RigidMotion<f64> {
    RigidMotion::rotation(UnitVec3::exact_y(), angle).unwrap()
}

fn moved(v: Vec3<f64>) -> RigidMotion<f64> {
    RigidMotion::translation(v).unwrap()
}

fn placed(body: &Body<f64>, inner: RigidMotion<f64>) -> Body<f64> {
    let motion = tilted().compose(inner).unwrap();
    topo::transform_rigid(body, &motion, Tol::witness()).unwrap()
}

fn mirrored(body: &Body<f64>, normal: Vec3<f64>) -> Body<f64> {
    let r = topo::Reflection::across(Point3::new(0.0, 0.0, 0.0), normal, Tol::witness()).unwrap();
    topo::mirror_body(body, &r, Tol::witness()).unwrap()
}

fn many(parts: &[Body<f64>]) -> Body<f64> {
    let mut body = parts[0].clone();
    for p in &parts[1..] {
        topo::graft_disjoint_all_keyed(&mut body, p).unwrap();
    }
    body
}

fn at_rest(body: &Body<f64>) -> (topo::RecordedAtRest, topo::FlushAtRest) {
    let groups: BTreeMap<_, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
    let recorded = topo::record_at_rest(
        body,
        &ContactRecords::default(),
        |records| {
            topo::validate_pseudomanifold(body, records, Tol::witness())
                .err()
                .unwrap_or_default()
        },
        &|s| groups.get(&s).copied(),
        0,
    );
    let band = Band::linear(Tol::witness()).unwrap();
    let flush =
        <f64 as topo::AtRestPolicy>::flush_at_rest(body, &|s| groups.get(&s).copied(), band);
    (recorded, flush)
}

fn same_side(r: &topo::RecordedAtRest, f: &topo::FlushAtRest) -> usize {
    r.findings
        .iter()
        .chain(&f.findings)
        .filter(|e| matches!(e, ValidationError::SameSideInterference { .. }))
        .count()
}

fn interference(body: &Body<f64>, r: &topo::RecordedAtRest) -> usize {
    r.findings
        .iter()
        .filter(|f| topo::census_interference_solids(body, f).is_some())
        .count()
}

fn summary(r: &topo::RecordedAtRest, f: &topo::FlushAtRest) -> String {
    let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
    for e in r.findings.iter().chain(&f.findings) {
        let s = format!("{e:?}");
        let k = s.split([' ', '{', '(']).next().unwrap().to_owned();
        *kinds.entry(k).or_default() += 1;
    }
    format!(
        "{kinds:?} rows={} flush_rows={}",
        r.rows.len(),
        f.rows.len()
    )
}

/// Rows that must not decide a `SameSideInterference` and must not read
/// as any interference.
#[test]
fn probe_faces_of_one_carrier_that_only_abut_are_no_same_side() {
    let eps = Tol::witness().eps();
    println!("eps = {eps:e}");
    let half = Revolution::Partial(core::f64::consts::PI);
    let quarter = Revolution::Partial(core::f64::consts::FRAC_PI_2);
    let wedge = rect(0.0, 1.0, 0.0, 1.0, half);
    let rows: Vec<(&str, Body<f64>)> = vec![
        (
            "hemispheres on the equator",
            many(&[
                placed(&lower_hemisphere(), spun(0.0)),
                placed(&upper_hemisphere(), spun(0.3)),
            ]),
        ),
        (
            "half balls side by side",
            many(&[
                placed(&ball(half), spun(0.0)),
                placed(&ball(half), spun(core::f64::consts::PI)),
            ]),
        ),
        (
            "quarter balls opposite (poles only)",
            many(&[
                placed(&ball(quarter), spun(0.0)),
                placed(&ball(quarter), spun(core::f64::consts::PI)),
            ]),
        ),
        (
            "quarter balls adjacent",
            many(&[
                placed(&ball(quarter), spun(0.0)),
                placed(&ball(quarter), spun(core::f64::consts::FRAC_PI_2)),
            ]),
        ),
        (
            "quarter cylinders opposite (axis only)",
            many(&[
                placed(&rect(0.0, 1.0, 0.0, 1.0, quarter), spun(0.0)),
                placed(
                    &rect(0.0, 1.0, 0.0, 1.0, quarter),
                    spun(core::f64::consts::PI),
                ),
            ]),
        ),
        (
            "half cylinder and its mirror image",
            many(&[
                placed(&wedge, spun(0.0)),
                placed(&mirrored(&wedge, Vec3::new(0.0, 0.0, 1.0)), spun(0.0)),
            ]),
        ),
        (
            "bands end to end, seams apart",
            many(&[
                placed(&rect(0.0, 1.0, 0.0, 1.0, Revolution::Full), spun(0.0)),
                placed(&rect(0.0, 1.0, 1.0, 2.0, Revolution::Full), spun(0.9)),
            ]),
        ),
        (
            "tubes end to end",
            many(&[
                placed(&rect(0.5, 1.0, 0.0, 1.0, Revolution::Full), spun(0.0)),
                placed(&rect(0.5, 1.0, 1.0, 2.0, Revolution::Full), spun(0.0)),
            ]),
        ),
        (
            "shaft in its bore (opposed)",
            many(&[
                placed(&rect(0.0, 0.5, 0.0, 1.0, Revolution::Full), spun(0.0)),
                placed(&rect(0.5, 1.0, 0.0, 1.0, Revolution::Full), spun(0.4)),
            ]),
        ),
        (
            "ball in a spherical shell's cavity (opposed)",
            many(&[
                placed(&ball(Revolution::Full), spun(0.0)),
                placed(
                    &revolved_about_y(
                        vec![
                            (Point2::new(0.0, -2.0), 1.0),
                            (Point2::new(0.0, 2.0), 0.0),
                            (Point2::new(0.0, 1.0), -1.0),
                            (Point2::new(0.0, -1.0), 0.0),
                        ],
                        Revolution::Full,
                        Tol::witness(),
                    ),
                    spun(0.0),
                ),
            ]),
        ),
    ];
    let mut bad = Vec::new();
    for (label, body) in &rows {
        let (r, f) = at_rest(body);
        let (s, i) = (same_side(&r, &f), interference(body, &r));
        println!(
            "{label}: same_side={s} interference={i} :: {}",
            summary(&r, &f)
        );
        if s != 0 || i != 0 {
            bad.push(format!(
                "{label}: same_side={s} interference={i}: {:?}",
                r.findings
            ));
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// Distinct carriers a few ε to a thousand ε apart, facing the same way,
/// whose solids share no material: a rod of radius 1 and a thin tube
/// whose outer wall stands `d` off it, and the same tube's axis moved.
#[test]
fn probe_nearly_coincident_cylinders_are_no_same_side() {
    let eps = Tol::witness().eps();
    let mut bad = Vec::new();
    for k in [3.0, 10.0, 100.0, 1000.0] {
        let d = k * eps;
        let rod = rect(0.0, 1.0, 0.0, 1.0, Revolution::Full);
        // A same-oriented outward wall at radius 1 + d, stacked on the
        // rod: the walls abut along no interior.
        let tube = rect(0.0, 1.0 + d, 1.0, 2.0, Revolution::Full);
        for (label, body) in [
            (
                "radius, stacked",
                many(&[placed(&rod, spun(0.0)), placed(&tube, spun(0.3))]),
            ),
            (
                "coaxial copies, axis off",
                many(&[
                    placed(&rod, spun(0.0)),
                    placed(&rod, moved(Vec3::new(0.6 * d, 0.0, 0.8 * d))),
                ]),
            ),
        ] {
            let (r, f) = at_rest(&body);
            let s = same_side(&r, &f);
            println!("k={k} {label}: same_side={s} :: {}", summary(&r, &f));
            // The axis-off copies genuinely overlap (almost all of their
            // volume), so a same-side finding there is right.
            if s != 0 && label.starts_with("radius") {
                bad.push(format!("k={k} {label}"));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

/// Rows that must read as one `SameSideInterference`.
#[test]
fn probe_overlapping_copies_are_one_same_side() {
    let rows: Vec<(&str, Body<f64>)> = vec![
        (
            "coincident tubes (bore walls reversed)",
            many(&[
                placed(&rect(0.5, 1.0, 0.0, 1.0, Revolution::Full), spun(0.0)),
                placed(&rect(0.5, 1.0, 0.0, 1.0, Revolution::Full), spun(0.7)),
            ]),
        ),
        (
            "cylinder and its mirror image (seam opposite)",
            many(&[
                placed(&rect(0.0, 1.0, 0.0, 1.0, Revolution::Full), spun(0.0)),
                placed(
                    &mirrored(
                        &rect(0.0, 1.0, 0.0, 1.0, Revolution::Full),
                        Vec3::new(1.0, 0.0, 0.0),
                    ),
                    spun(0.0),
                ),
            ]),
        ),
        (
            "half balls overlapping by a quarter",
            many(&[
                placed(&ball(Revolution::Partial(core::f64::consts::PI)), spun(0.0)),
                placed(
                    &ball(Revolution::Partial(core::f64::consts::PI)),
                    spun(core::f64::consts::FRAC_PI_2),
                ),
            ]),
        ),
        (
            "hemisphere inside a ball",
            many(&[
                placed(&ball(Revolution::Full), spun(0.0)),
                placed(&upper_hemisphere(), spun(0.5)),
            ]),
        ),
    ];
    for (label, body) in &rows {
        let (r, f) = at_rest(body);
        println!(
            "{label}: same_side={} interference={} :: {}",
            same_side(&r, &f),
            interference(body, &r),
            summary(&r, &f)
        );
    }
}

/// Cost: a grid of `n×n` stacks of `m` pins abutting end to end along
/// their axis, every abutting pair one carrier facing the same way.
#[test]
fn probe_pin_stack_cost() {
    let (n, m) = (3usize, 4usize);
    let pin = rect(0.0, 0.4, 0.0, 1.0, Revolution::Full);
    let mut parts = Vec::new();
    for i in 0..n {
        for j in 0..n {
            for k in 0..m {
                parts.push(placed(
                    &pin,
                    moved(Vec3::new(1.5 * i as f64, k as f64, 1.5 * j as f64)),
                ));
            }
        }
    }
    let body = many(&parts);
    let t = std::time::Instant::now();
    let (r, f) = at_rest(&body);
    println!(
        "pins {}: {:?} interference={} :: {}",
        n * n * m,
        t.elapsed(),
        interference(&body, &r),
        summary(&r, &f)
    );
}
