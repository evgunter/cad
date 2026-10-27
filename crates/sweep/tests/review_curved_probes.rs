//! Review probes for PR #5853 (curved copies filling one volume): shapes
//! that only abut on one carrier, near-coincident carriers, opposed and
//! mirrored copies. Each row prints what the census reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::collections::BTreeMap;

use geom_core::{Band, Point2, RigidMotion, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, ContactRecords, EntityId, ValidationError};

/// The unit square over `[0, h]`, revolved about `y` by `turn`: a
/// cylinder of radius 1 and height `h`, its walls two half-bands on one
/// key and its caps two discs, or a wedge of it.
fn cylinder(h: f64, turn: Revolution<f64>) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, h), 0.0),
            (Point2::new(0.0, h), 0.0),
        ],
        turn,
        Tol::witness(),
    )
}

/// The unit ball, its poles on `y`.
fn ball() -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        Tol::witness(),
    )
}

/// The unit ball with its cap above `y = 1/2` cut off: a sphere face
/// that is a proper subset of the ball's, and a disc on the cut.
fn cut_ball() -> Body<f64> {
    let rim = Point2::new(0.75_f64.sqrt(), 0.5);
    revolved_about_y(
        vec![
            (Point2::new(0.0, -1.0), (core::f64::consts::PI / 6.0).tan()),
            (rim, 0.0),
            (Point2::new(0.0, 0.5), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    UnitVec3::new(v, "a row's axis", Band::linear(Tol::witness()).unwrap()).unwrap()
}

/// The pose every row reads its bodies in: a turn about an axis off
/// every world axis, then a move off the origin.
fn tilted() -> RigidMotion<f64> {
    RigidMotion::rotation_then_translation(
        unit(Vec3::new(0.3, 0.2, 1.0)),
        0.7,
        Vec3::new(0.31, -0.17, 0.42),
    )
    .unwrap()
}

/// A turn by `angle` about the bodies' own axis, `y`.
fn spun(angle: f64) -> RigidMotion<f64> {
    RigidMotion::rotation(UnitVec3::exact_y(), angle).unwrap()
}

/// `body` moved by `inner`, then by [`tilted`].
fn placed(body: &Body<f64>, inner: RigidMotion<f64>) -> Body<f64> {
    let motion = tilted().compose(inner).unwrap();
    topo::transform_rigid(body, &motion, Tol::witness()).unwrap()
}

/// `a` and `b` as two solids of one body.
fn two(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut body = a.clone();
    topo::graft_disjoint_all_keyed(&mut body, b).unwrap();
    body
}

/// What the census finds over `body` at rest, each solid its own group,
/// and the flush pairs between them.
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

/// The findings the census reads as interference between two solids.
fn interference<'a>(body: &Body<f64>, findings: &'a [ValidationError]) -> Vec<&'a ValidationError> {
    findings
        .iter()
        .filter(|f| topo::census_interference_solids(body, f).is_some())
        .collect()
}

/// The surface kind of face `f`'s carrier.
fn kind(body: &Body<f64>, f: EntityId) -> Option<geom::SurfaceKind> {
    let EntityId::Face(f) = f else { return None };
    Some(body.get_surface(body.get_face(f)?.surface)?.kind())
}

/// The undecidable face pairs on two carriers of `kind` each.
fn undecided_pairs(
    body: &Body<f64>,
    findings: &[ValidationError],
    of: geom::SurfaceKind,
) -> Vec<String> {
    findings
        .iter()
        .filter(|f| {
            matches!(f, ValidationError::CensusUndecidable { a, b, .. }
                if kind(body, *a) == Some(of) && kind(body, *b) == Some(of))
        })
        .map(|f| format!("{f:?}"))
        .collect()
}


fn same_side(body: &Body<f64>, label: &str) -> usize {
    let (recorded, flush) = at_rest(body);
    for f in recorded.findings.iter().chain(&flush.findings) {
        if let ValidationError::SameSideInterference { a, b } = f {
            let k = |x: topo::FaceKey| body.get_surface(body.get_face(x).unwrap().surface).unwrap().kind();
            eprintln!("PROBE {label} SAMESIDE {:?} {:?}", k(*a), k(*b));
        }
    }
    let n = flush
        .findings
        .iter()
        .chain(&recorded.findings)
        .filter(|f| matches!(f, ValidationError::SameSideInterference { .. }))
        .count();
    let summary: Vec<String> = recorded
        .findings
        .iter()
        .chain(&flush.findings)
        .map(|f| {
            let s = format!("{f:?}");
            s.chars().take(90).collect()
        })
        .collect();
    eprintln!(
        "PROBE {label} eps={}: same_side={n} interference={} findings={} rows={}/{} :: {summary:#?}",
        Tol::witness().eps(),
        interference(body, &recorded.findings).len(),
        recorded.findings.len(),
        recorded.rows.len(),
        flush.rows.len(),
    );
    n
}

fn half_ball() -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Partial(core::f64::consts::PI),
        Tol::witness(),
    )
}

fn lune(turn: f64) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Partial(turn),
        Tol::witness(),
    )
}

/// The ball's cap above `y = 1/2`: the complement on the sphere of
/// `cut_ball`'s sphere face, meeting it along the rim circle only.
fn upper_cap() -> Body<f64> {
    let rim = Point2::new(0.75_f64.sqrt(), 0.5);
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.5), 0.0),
            (rim, (core::f64::consts::PI / 12.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

fn tube(r0: f64, r1: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(r0, 0.0), 0.0),
            (Point2::new(r1, 0.0), 0.0),
            (Point2::new(r1, 1.0), 0.0),
            (Point2::new(r0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

fn wedge_r(r: f64) -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Partial(core::f64::consts::PI),
        Tol::witness(),
    )
}

fn shifted(v: Vec3<f64>) -> RigidMotion<f64> {
    RigidMotion::translation(v).unwrap()
}

/// Shapes on one carrier that share only boundary: never interference.
#[test]
fn review_abutting_shapes_are_no_same_side() {
    let pi = core::f64::consts::PI;
    let base = placed(&cylinder(1.0, Revolution::Full), spun(0.0));
    let stacked_spun = topo::transform_rigid(
        &cylinder(1.0, Revolution::Full),
        &tilted()
            .compose(shifted(Vec3::new(0.0, 1.0, 0.0)))
            .unwrap()
            .compose(spun(1.1))
            .unwrap(),
        Tol::witness(),
    )
    .unwrap();
    let wedge = cylinder(1.0, Revolution::Partial(pi));
    let mut bad = Vec::new();
    for (label, body) in [
        ("stacked spun", two(&base, &stacked_spun)),
        (
            "lunes",
            two(&placed(&half_ball(), spun(0.0)), &placed(&half_ball(), spun(pi))),
        ),
        (
            "caps at a circle",
            two(&placed(&cut_ball(), spun(0.0)), &placed(&upper_cap(), spun(0.9))),
        ),
        (
            "caps at a circle, seams met",
            two(&placed(&cut_ball(), spun(0.0)), &placed(&upper_cap(), spun(0.0))),
        ),
        (
            "rod in tube (opposed)",
            two(&base, &placed(&tube(1.0, 2.0), spun(0.7))),
        ),
        (
            "halves, quarter wedges at seam",
            two(
                &placed(&cylinder(1.0, Revolution::Partial(pi / 2.0)), spun(0.0)),
                &placed(&cylinder(1.0, Revolution::Partial(pi / 2.0)), spun(pi / 2.0)),
            ),
        ),
        (
            "reflex wedge and its complement",
            two(
                &placed(&cylinder(1.0, Revolution::Partial(1.5 * pi)), spun(0.0)),
                &placed(&cylinder(1.0, Revolution::Partial(0.5 * pi)), spun(1.5 * pi)),
            ),
        ),
        (
            "reflex lune and its complement",
            two(
                &placed(&lune(1.5 * pi), spun(0.0)),
                &placed(&lune(0.5 * pi), spun(1.5 * pi)),
            ),
        ),
        (
            "halves mirrored",
            two(&placed(&wedge, spun(0.0)), &{
                let refl = topo::Reflection::across(
                    geom_core::Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Tol::witness(),
                )
                .unwrap();
                let m = topo::mirror_body(&wedge, &refl, Tol::witness()).unwrap();
                placed(&m, spun(0.0))
            }),
        ),
    ] {
        if same_side(&body, label) != 0 {
            bad.push(label);
        }
    }
    assert!(bad.is_empty(), "false same-side: {bad:?}");
}

/// Near-coincident half-cylinders that abut, the second's radius or axis
/// off by k·eps: never interference.
#[test]
fn review_near_coincident_abutting_halves_are_no_same_side() {
    let pi = core::f64::consts::PI;
    let eps = Tol::witness().eps();
    let mut bad = Vec::new();
    for k in [0.5, 3.0, 10.0, 100.0, 1000.0] {
        let d = k * eps;
        let a = placed(&wedge_r(1.0), spun(0.0));
        for (label, b) in [
            ("radius", placed(&wedge_r(1.0 + d), spun(pi))),
            (
                "axis +z",
                placed(
                    &topo::transform_rigid(&wedge_r(1.0), &shifted(Vec3::new(0.0, 0.0, d)), Tol::witness()).unwrap(),
                    spun(pi),
                ),
            ),
            (
                "axis -z",
                placed(
                    &topo::transform_rigid(&wedge_r(1.0), &shifted(Vec3::new(0.0, 0.0, -d)), Tol::witness()).unwrap(),
                    spun(pi),
                ),
            ),
        ] {
            let label = format!("halves {label} off by {k} eps");
            // `axis +z` moves the second half INTO the first by d: a
            // genuine sliver overlap, a same-side pair past the band.
            let overlaps = label.contains("+z") && k >= 10.0;
            if (same_side(&two(&a, &b), &label) != 0) != overlaps {
                bad.push(label);
            }
        }
    }
    assert!(bad.is_empty(), "false or missed same-side: {bad:?}");
}

/// Copies that do overlap on one carrier: one same-side each (the
/// positive path's sanity, including a mirrored copy).
#[test]
fn review_overlapping_shapes_are_one_same_side() {
    let pi = core::f64::consts::PI;
    let base = placed(&cylinder(1.0, Revolution::Full), spun(0.0));
    let refl = topo::Reflection::across(
        geom_core::Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Tol::witness(),
    )
    .unwrap();
    let mirrored = placed(
        &topo::mirror_body(&cylinder(1.0, Revolution::Full), &refl, Tol::witness()).unwrap(),
        spun(0.0),
    );
    let flipped = topo::transform_rigid(
        &cylinder(1.0, Revolution::Full),
        &tilted()
            .compose(shifted(Vec3::new(0.0, 1.0, 0.0)))
            .unwrap()
            .compose(RigidMotion::rotation(UnitVec3::exact_x(), pi).unwrap())
            .unwrap(),
        Tol::witness(),
    )
    .unwrap();
    let mut bad = Vec::new();
    for (label, body) in [
        ("mirrored copy", two(&base, &mirrored)),
        ("flipped copy", two(&base, &flipped)),
        (
            "half wedges a quarter apart",
            two(
                &placed(&cylinder(1.0, Revolution::Partial(pi)), spun(0.0)),
                &placed(&cylinder(1.0, Revolution::Partial(pi)), spun(pi / 2.0)),
            ),
        ),
        (
            "lunes a quarter apart",
            two(&placed(&half_ball(), spun(0.0)), &placed(&half_ball(), spun(pi / 2.0))),
        ),
        (
            "overlapping heights",
            two(
                &base,
                &topo::transform_rigid(
                    &cylinder(1.0, Revolution::Full),
                    &tilted().compose(shifted(Vec3::new(0.0, 0.5, 0.0))).unwrap().compose(spun(0.3)).unwrap(),
                    Tol::witness(),
                )
                .unwrap(),
            ),
        ),
    ] {
        if same_side(&body, label) != 1 {
            bad.push(label);
        }
    }
    eprintln!("PROBE positive misses: {bad:?}");
}

/// Cost probe: an n×n grid of tangent cylinders (radius 1, pitch 2), each
/// its own solid, so every neighbour's walls are curved candidates of
/// one kind; and the same grid with a coincident copy of each pin.
#[test]
fn review_pin_grid_cost() {
    let pin = cylinder(1.0, Revolution::Full);
    for (n, copies) in [(4, false), (6, false), (4, true)] {
        let mut body: Option<Body<f64>> = None;
        for i in 0..n {
            for j in 0..n {
                let at = topo::transform_rigid(
                    &pin,
                    &tilted()
                        .compose(shifted(Vec3::new(2.0 * i as f64, 0.0, 2.0 * j as f64)))
                        .unwrap(),
                    Tol::witness(),
                )
                .unwrap();
                for _ in 0..(1 + usize::from(copies)) {
                    match &mut body {
                        None => body = Some(at.clone()),
                        Some(b) => {
                            topo::graft_disjoint_all_keyed(b, &at).unwrap();
                        }
                    }
                }
            }
        }
        let body = body.unwrap();
        let t = std::time::Instant::now();
        let (recorded, _) = at_rest(&body);
        eprintln!(
            "PROBE grid {n}x{n} copies={copies}: {:?}, findings={} same_side={}",
            t.elapsed(),
            recorded.findings.len(),
            recorded
                .findings
                .iter()
                .filter(|f| matches!(f, ValidationError::SameSideInterference { .. }))
                .count()
        );
    }
}
