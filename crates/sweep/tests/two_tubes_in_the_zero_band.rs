//! **Two tubes tangent within the zero band answer alike in both
//! operand orders**: two bodies about parallel axes whose walls are
//! tangent `0.5·ε` apart, read through every op in both orders. The
//! first tube's seam ruling lies on the tangent line, so the second
//! body's rim circles touch its wall mid-arc. Sweeping the first tube's
//! edges first splits those circles where the seam crosses their cap
//! planes; sweeping the second's first meets each whole circle, which
//! settles once the other direction has split it
//! (`boolean::reduce::settle_deferred`). ∪ is the two solids with the
//! operands' summed volume and one record set, ∖ the first operand, ∩
//! empty.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::revolved_about_y;
use topo::{BooleanResult, BooleanResultKind};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn at_rest(b: topo::Body<f64>) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(b, tol()).unwrap()
}

/// A body about `y` from radius `inner` (0 for a rod) to `outer` over
/// `y ∈ [lo, hi]`, swept by `turn` (full where `None`) and then turned
/// by `spin` about `y`, its axis moved to `(x, ·, 0)`.
fn about_y(
    (inner, outer): (f64, f64),
    (lo, hi): (f64, f64),
    turn: Option<f64>,
    spin: f64,
    x: f64,
) -> topo::Body<f64> {
    let profile = vec![
        (Point2::new(inner, lo), 0.0),
        (Point2::new(outer, lo), 0.0),
        (Point2::new(outer, hi), 0.0),
        (Point2::new(inner, hi), 0.0),
    ];
    let rev = turn.map_or(sweep::Revolution::Full, sweep::Revolution::Partial);
    let body = revolved_about_y(profile, rev, tol());
    let place = Affine3::translation(Vec3::new(x, 0.0, 0.0))
        * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), spin);
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&place, tol()), tol()).unwrap()
}

fn turned(b: topo::Body<f64>, angle: f64) -> topo::Body<f64> {
    let t = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), angle);
    topo::transform_rigid(&b, &geom_core::test_support::rigid(&t, tol()), tol()).unwrap()
}

/// A point rounded far above the band, so both orders' cells compare.
fn at(body: &topo::Body<f64>, v: topo::VertexKey) -> String {
    let (_, p) = body.vertex_points().find(|(k, _)| *k == v).unwrap();
    format!("({:.6}, {:.6}, {:.6})", p.x, p.y, p.z)
}

/// Each record as its cells' places: a vertex's point, an edge's end
/// points, a face's carrier kind.
fn records(r: &topo::BooleanBody<f64>) -> Vec<String> {
    let body: &topo::Body<f64> = &r.body;
    let cell = |c: topo::Cell| match c {
        topo::Cell::Vertex(v) => format!("vertex {}", at(body, v)),
        topo::Cell::Edge(e) => {
            let mut ends: Vec<String> = body
                .half_edges()
                .filter(|(_, h)| h.edge == e)
                .map(|(_, h)| at(body, h.start))
                .collect();
            ends.sort();
            format!("edge {ends:?}")
        }
        topo::Cell::Face(f) => {
            let face = body.get_face(f).unwrap();
            format!("face {:?}", body.get_surface(face.surface).unwrap().kind())
        }
    };
    let mut rows: Vec<String> = r
        .contacts
        .cell_pairs()
        .map(|(x, y)| {
            let mut pair = [cell(x), cell(y)];
            pair.sort();
            pair.join(" / ")
        })
        .collect();
    rows.sort();
    rows
}

/// Every op in both orders: ∪ two solids of volume `vx + vy` carrying
/// one record set, the two rims' touches on the seam (the seam's span
/// in the wall is not recorded:
/// `a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`);
/// ∖ the first operand; ∩ empty.
fn every_op_in_both_orders(
    pose: &str,
    (x, vx): (topo::Body<f64>, f64),
    (y, vy): (topo::Body<f64>, f64),
) {
    let (x, y) = (at_rest(x), at_rest(y));
    let mut unions = Vec::new();
    for (order, p, q, vp) in [("x, y", &x, &y, vx), ("y, x", &y, &x, vy)] {
        let what = |op: &str| format!("{pose}: {op} in order {order}");
        match topo::union(p, q, tol()) {
            Ok(BooleanResult::Body(u)) => {
                assert_eq!(u.body.solids().count(), 2, "{}: two solids", what("∪"));
                let v = topo::mass_properties(&u.body, tol()).unwrap().volume;
                assert!(
                    (v - (vx + vy)).abs() <= 1e-9 * (vx + vy),
                    "{}: volume {v} against the closed form {}",
                    what("∪"),
                    vx + vy
                );
                let findings = topo::validate_pseudomanifold(&u.body, &u.contacts, tol())
                    .err()
                    .unwrap_or_default();
                assert!(
                    findings
                        .iter()
                        .all(|f| matches!(f, topo::ValidationError::CensusUndecidable { .. })),
                    "{}: 3′ reads every record; only the cross-solid curved search is \
                     undecided: {findings:?}",
                    what("∪")
                );
                unions.push(records(&u));
            }
            other => panic!("{}: {:?}", what("∪"), other.map(|_| ())),
        }
        match topo::subtract(p, q, tol()) {
            Ok(BooleanResult::Body(d)) => {
                assert_eq!(d.kind, BooleanResultKind::OperandA, "{}", what("∖"));
                let v = topo::mass_properties(&d.body, tol()).unwrap().volume;
                assert!(
                    (v - vp).abs() <= 1e-9 * vp,
                    "{}: volume {v} against the closed form {vp}",
                    what("∖")
                );
            }
            other => panic!("{}: {:?}", what("∖"), other.map(|_| ())),
        }
        let meet = topo::intersect(p, q, tol());
        assert!(
            matches!(meet, Ok(BooleanResult::Empty)),
            "{}: {:?}",
            what("∩"),
            meet.map(|_| ())
        );
    }
    assert_eq!(
        unions[0].len(),
        2,
        "{pose}: ∪ records the touch at the rims, one record each: {:?}",
        unions[0]
    );
    assert_eq!(
        unions[0], unions[1],
        "{pose}: ∪ records alike in both orders"
    );
}

/// A tube `r ∈ [0.3, 1]`, `y ∈ [0, 1]`, beside a shorter one about the
/// axis `x = 2 + 0.5·ε`.
#[test]
fn two_tubes_side_by_side() {
    let gap = 0.5 * band().zero();
    every_op_in_both_orders(
        "two tubes",
        (about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0), PI * 0.91),
        (
            about_y((0.3, 1.0), (0.2, 0.8), None, 0.0, 2.0 + gap),
            PI * 0.91 * 0.6,
        ),
    );
}

/// A quarter tube spun `3π/4` so its wall faces the tube, both turned
/// 0.5 rad about `z` off the world's axes.
#[test]
fn a_quarter_tube_beside_a_tube_turned_off_axis() {
    let gap = 0.5 * band().zero();
    every_op_in_both_orders(
        "quarter tube",
        (
            turned(about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0), 0.5),
            PI * 0.91,
        ),
        (
            turned(
                about_y(
                    (0.3, 1.0),
                    (0.2, 0.8),
                    Some(FRAC_PI_2),
                    1.5 * FRAC_PI_2,
                    2.0 + gap,
                ),
                0.5,
            ),
            PI * 0.91 * 0.6 / 4.0,
        ),
    );
}

/// A quarter rod `r = 0.2` in the tube's bore, spun `−π/4` so its wall
/// faces the bore's ruling at `x = 0.3`.
#[test]
fn a_quarter_rod_in_the_bore() {
    let gap = 0.5 * band().zero();
    every_op_in_both_orders(
        "quarter rod",
        (about_y((0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0), PI * 0.91),
        (
            about_y(
                (0.0, 0.2),
                (0.2, 0.8),
                Some(FRAC_PI_2),
                -0.5 * FRAC_PI_2,
                0.1 - gap,
            ),
            PI * 0.04 * 0.6 / 4.0,
        ),
    );
}
