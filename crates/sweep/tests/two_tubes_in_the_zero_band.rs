//! **Two bodies tangent along a seam within the zero band answer alike
//! in both operand orders**: walls about parallel axes, or a rod on a
//! puck's face, tangent `0.5·ε` apart, read through every op in both
//! orders. One body's seam ruling lies on the tangent line, so the
//! other body's rim circles touch it mid-arc. Sweeping the seamed
//! body's edges first splits those circles where the seam crosses
//! their cap planes; sweeping the other's first meets each whole
//! circle, which settles once the other direction has split it
//! (`boolean::reduce::settle_deferred`). ∪ is the two solids with the
//! operands' summed volume, one record set and one set of tier-3′
//! findings, ∖ the first operand, ∩ empty.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use topo::{BooleanResult, BooleanResultKind};

use crate::common::operands::{about_y, turned_about_z};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn at_rest(b: topo::Body<f64>) -> topo::AtRestBody<f64> {
    topo::AtRestBody::validate(b, tol()).unwrap()
}

/// A point rounded far above the band, so both orders' cells compare.
fn at(body: &topo::Body<f64>, v: topo::VertexKey) -> String {
    let (_, p) = body.vertex_points().find(|(k, _)| *k == v).unwrap();
    format!("({:.6}, {:.6}, {:.6})", p.x, p.y, p.z)
}

fn face_kind(body: &topo::Body<f64>, f: topo::FaceKey) -> String {
    let face = body.get_face(f).unwrap();
    format!("{:?}", body.get_surface(face.surface).unwrap().kind())
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
        topo::Cell::Face(f) => format!("face {}", face_kind(body, f)),
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

/// Tier 3′'s findings on the union: an undecided cross-solid pair as
/// its two faces' carrier kinds, anything else whole, so a finding that
/// appears or goes reddens the row.
fn findings(r: &topo::BooleanBody<f64>) -> Vec<String> {
    let body: &topo::Body<f64> = &r.body;
    let side = |e: topo::EntityId| match e {
        topo::EntityId::Face(f) => face_kind(body, f),
        topo::EntityId::Solid(_) => "Solid".to_owned(),
        other => format!("{other:?}"),
    };
    let mut out: Vec<String> = topo::validate_pseudomanifold(body, &r.contacts, tol())
        .err()
        .unwrap_or_default()
        .into_iter()
        .map(|f| match f {
            topo::ValidationError::CensusUndecidable { a, b, .. } => {
                let mut pair = [side(a), side(b)];
                pair.sort();
                format!("undecided {} / {}", pair[0], pair[1])
            }
            other => format!("{other:?}"),
        })
        .collect();
    out.sort();
    out
}

/// What the union of a pose must carry, alike in both orders.
struct Want {
    /// The number of records.
    records: usize,
    /// Tier 3′'s findings, as [`findings`] spells them.
    findings: &'static [&'static str],
}

/// Every op in both orders: ∪ two solids of volume `vx + vy` carrying
/// `want`; ∖ the first operand; ∩ empty. Returns the union's records.
fn every_op_in_both_orders(
    pose: &str,
    (x, vx): (topo::Body<f64>, f64),
    (y, vy): (topo::Body<f64>, f64),
    want: &Want,
) -> Vec<String> {
    let (x, y) = (at_rest(x), at_rest(y));
    let mut unions = Vec::new();
    for (order, p, q, vp) in [("x, y", &x, &y, vx), ("y, x", &y, &x, vy)] {
        let what = |op: &str| format!("{pose}: {op} in order {order}");
        match topo::union(p, q, tol()) {
            Ok(BooleanResult::Body(u)) => {
                assert_eq!(u.body.solids().count(), 2, "{}: two solids", what("∪"));
                let v =
                    topo::test_support::pinned_volume("the measured body", &u.body, tol()).volume;
                assert!(
                    (v - (vx + vy)).abs() <= 1e-9 * (vx + vy),
                    "{}: volume {v} against the closed form {}",
                    what("∪"),
                    vx + vy
                );
                let (recs, found) = (records(&u), findings(&u));
                assert_eq!(
                    found,
                    want.findings,
                    "{}: tier 3′'s findings (records {recs:?})",
                    what("∪")
                );
                assert_eq!(recs.len(), want.records, "{}: {recs:?}", what("∪"));
                unions.push(recs);
            }
            other => panic!("{}: {:?}", what("∪"), other.map(|_| ())),
        }
        match topo::subtract(p, q, tol()) {
            Ok(BooleanResult::Body(d)) => {
                assert_eq!(d.kind, BooleanResultKind::OperandA, "{}", what("∖"));
                let v =
                    topo::test_support::pinned_volume("the measured body", &d.body, tol()).volume;
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
        unions[0], unions[1],
        "{pose}: ∪ records alike in both orders"
    );
    unions.swap_remove(0)
}

/// Two walls tangent along the seam: the rims' two touches on it are
/// recorded, and the seam's span in the other wall is not
/// (`a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`).
/// The census's cross-solid search leaves the wall pair, each wall
/// against the other's cap, and the two solids undecided.
const WALLS: Want = Want {
    records: 2,
    findings: &[
        "undecided Cylinder / Cylinder",
        "undecided Cylinder / Plane",
        "undecided Cylinder / Plane",
        "undecided Solid / Solid",
    ],
};

/// A tube `r ∈ [0.3, 1]`, `y ∈ [0, 1]`, beside a shorter one about the
/// axis `x = 2 + 0.5·ε`.
#[test]
fn two_tubes_side_by_side() {
    let gap = 0.5 * band().zero();
    every_op_in_both_orders(
        "two tubes",
        (
            about_y(tol(), (0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
            PI * 0.91,
        ),
        (
            about_y(tol(), (0.3, 1.0), (0.2, 0.8), None, 0.0, 2.0 + gap),
            PI * 0.91 * 0.6,
        ),
        &WALLS,
    );
}

/// A quarter tube spun `3π/4` so its wall faces the tube, both turned
/// 0.5 rad about `z` off the world's axes.
#[test]
fn a_quarter_tube_beside_a_tube_turned_off_axis() {
    let gap = 0.5 * band().zero();
    let tube = about_y(tol(), (0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0);
    let quarter = about_y(
        tol(),
        (0.3, 1.0),
        (0.2, 0.8),
        Some(FRAC_PI_2),
        1.5 * FRAC_PI_2,
        2.0 + gap,
    );
    every_op_in_both_orders(
        "quarter tube",
        (turned_about_z(tol(), tube, 0.5), PI * 0.91),
        (turned_about_z(tol(), quarter, 0.5), PI * 0.91 * 0.6 / 4.0),
        &WALLS,
    );
}

/// A quarter rod `r = 0.2` in the tube's bore, spun `−π/4` so its wall
/// faces the bore's ruling at `x = 0.3`.
#[test]
fn a_quarter_rod_in_the_bore() {
    let gap = 0.5 * band().zero();
    every_op_in_both_orders(
        "quarter rod",
        (
            about_y(tol(), (0.3, 1.0), (0.0, 1.0), None, 0.0, 0.0),
            PI * 0.91,
        ),
        (
            about_y(
                tol(),
                (0.0, 0.2),
                (0.2, 0.8),
                Some(FRAC_PI_2),
                -0.5 * FRAC_PI_2,
                0.1 - gap,
            ),
            PI * 0.04 * 0.6 / 4.0,
        ),
        &WALLS,
    );
}

/// A rod `r = 0.25` lying along `x` over `[−1.5, 1.5]` on a unit puck's
/// top face `y = 0` (the puck `y ∈ [−0.3, 0]`, spun 1 rad), its axis
/// `0.5·ε` above the tangency and its seam on the contact ruling, which
/// the puck's rim circle crosses at `x = ±1`. ∪ records the rim's two
/// touches on the seam and the seam lying in the top face between them
/// (the sweep's record of a line in a plane face), which the result
/// carries on its seam edge, the rod's whole length.
#[test]
fn a_rod_on_a_puck_with_its_seam_on_the_ruling() {
    let gap = 0.5 * band().zero();
    let r = 0.25;
    let rod = topo::transform_rigid(
        &about_y(tol(), (0.0, r), (-1.5, 1.5), None, 0.0, 0.0),
        &geom_core::test_support::rigid(
            &(Affine3::translation(Vec3::new(0.0, r + gap, 0.0))
                * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_z(), -FRAC_PI_2)),
            tol(),
        ),
        tol(),
    )
    .unwrap();
    let records = every_op_in_both_orders(
        "rod on puck",
        (
            about_y(tol(), (0.0, 1.0), (-0.3, 0.0), None, 1.0, 0.0),
            PI * 0.3,
        ),
        (rod, PI * r * r * 3.0),
        &Want {
            records: 3,
            findings: &[
                "undecided Cylinder / Cylinder",
                "undecided Cylinder / Cylinder",
                "undecided Cylinder / Cylinder",
                "undecided Cylinder / Cylinder",
                "undecided Cylinder / Plane",
                "undecided Cylinder / Plane",
                "undecided Solid / Solid",
            ],
        },
    );
    let in_faces: Vec<&String> = records
        .iter()
        .filter(|r| r.starts_with("edge") && r.contains("face "))
        .collect();
    // The seam's ends sit within the band of `y = 0`, which prints at
    // the sixth place at a coarse ε: read their `x` only.
    let [seam] = in_faces.as_slice() else {
        panic!("rod on puck: one edge lies in a face: {in_faces:?}");
    };
    assert!(
        seam.starts_with("edge [\"(-1.500000, ")
            && seam.contains("\"(1.500000, ")
            && seam.ends_with("/ face Plane"),
        "rod on puck: the seam lies in the puck's top: {seam}"
    );
}
