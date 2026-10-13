//! **A point touch is read where its carriers touch** (`crates/topo/README.md`
//! C3): a ball resting on a slab's face, on another ball, or on a slab's
//! edge. Each body is the two solids grafted side by side with one
//! record of the touching cells, and tier 3′ reads that record: the
//! touch point on the carriers' closed-form lane, coincident within ε,
//! strictly inside both cells, `κ_rel` definitely positive.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick};
use topo::{Body, Cell, ContactRecords, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

/// `a` with `b` grafted beside it, and `b`'s sphere faces in `a`'s keys.
fn graft(mut a: Body<f64>, b: &Body<f64>) -> (Body<f64>, Vec<topo::FaceKey>) {
    let keys = topo::graft_disjoint_all_keyed(&mut a, b).unwrap();
    let spheres = b
        .faces()
        .filter(|(_, f)| b.get_surface(f.surface).unwrap().kind() == SurfaceKind::Sphere)
        .map(|(k, _)| keys.face(k).unwrap())
        .collect();
    (a, spheres)
}

/// The face of `body` on the plane z = `z`, outward up.
fn top(body: &Body<f64>, z: f64) -> topo::FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && (origin.z - z).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .unwrap()
}

/// What tier 3′ says of the one record of `x` and `y`.
#[derive(Debug, PartialEq, Eq)]
enum Read {
    /// No finding refuses it.
    Confirmed,
    Stale,
    Escalated,
    Unsupported,
}

fn read(body: &Body<f64>, x: Cell, y: Cell) -> Read {
    let mut records = ContactRecords::default();
    records.extend([topo::Cited::new(
        topo::Contact::new(body, x, y).unwrap(),
        topo::Cites::decided(0),
    )]);
    let errors = topo::validate_pseudomanifold(body, &records, tol())
        .err()
        .unwrap_or_default();
    if errors
        .iter()
        .any(|e| matches!(e, ValidationError::StaleContactDeclaration { .. }))
    {
        Read::Stale
    } else if errors
        .iter()
        .any(|e| matches!(e, ValidationError::CensusUnsupported { .. }))
    {
        Read::Unsupported
    } else if errors
        .iter()
        .any(|e| matches!(e, ValidationError::CensusEscalated { .. }))
    {
        Read::Escalated
    } else {
        Read::Confirmed
    }
}

/// Each record of a ball's faces against `cell`, sorted.
fn reads(body: &Body<f64>, balls: &[topo::FaceKey], cell: Cell) -> Vec<String> {
    let mut out: Vec<String> = balls
        .iter()
        .map(|&f| format!("{:?}", read(body, Cell::Face(f), cell)))
        .collect();
    out.sort();
    out
}

fn slab() -> Body<f64> {
    brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol())
}

/// A half-unit ball whose bottom is `lift` above the slab's top face at
/// (2, 2, 1): touching, the record of the hemisphere holding that point
/// is confirmed and the other's stale; lifted half a metre, or sunk five
/// centimetres (the sphere crossing the plane), both are stale; lifted
/// 3ε, both escalate. Red where the census has no sphere–plane lane
/// (every record refused typed).
#[test]
fn a_ball_on_a_slab_face_is_read_at_its_touch_point() {
    let eps = tol().eps();
    for (what, lift, want) in [
        ("touching", 0.0, ["Confirmed", "Stale"]),
        ("half a metre up", 0.5, ["Stale", "Stale"]),
        ("sunk 5 cm", -0.05, ["Stale", "Stale"]),
        ("3ε up", 3.0 * eps, ["Escalated", "Escalated"]),
    ] {
        let ball = ball_poled_y(0.5, Vec3::new(2.0, 2.0, 1.5 + lift), tol());
        let (body, balls) = graft(slab(), &ball);
        let face = top(&body, 1.0);
        assert_eq!(reads(&body, &balls, Cell::Face(face)), want, "{what}");
    }
}

/// Two half-unit balls touching at (2, 2, 2): one pair of their faces
/// holds the point and is confirmed, the other three are stale. A ball
/// of radius 0.25 inside a unit ball, touching it from inside at
/// (2, 2, 2.5): one pair confirmed, the rest stale. (Each ball's seam
/// meridian runs through its +x side, so the points are put on z.)
#[test]
fn two_balls_touching_are_read_at_their_touch_point() {
    for (what, (r, c), big) in [
        ("outside", (0.5, Vec3::new(2.0, 2.0, 2.5)), 0.5),
        ("inside", (0.25, Vec3::new(2.0, 2.0, 2.25)), 1.0),
    ] {
        let a = ball_poled_y(big, Vec3::new(2.0, 2.0, 1.5), tol());
        let b = ball_poled_y(r, c, tol());
        let (body, fa) = graft(Body::new(), &a);
        let (body, fb) = graft(body, &b);
        let fa: Vec<_> = fa.into_iter().filter(|f| !fb.contains(f)).collect();
        let mut got: Vec<String> = fa
            .iter()
            .flat_map(|&x| fb.iter().map(move |&y| (x, y)))
            .map(|(x, y)| format!("{:?}", read(&body, Cell::Face(x), Cell::Face(y))))
            .collect();
        got.sort();
        assert_eq!(
            got,
            ["Confirmed", "Stale", "Stale", "Stale"],
            "{what}: one pair of faces holds the touch point"
        );
    }
}

/// A half-unit ball resting on the slab's top edge along y = 0, its
/// centre off the slab at 45°: the record of the hemisphere holding
/// (2, 0, 1) and that edge is confirmed, the other's stale.
#[test]
fn a_ball_on_a_slab_edge_is_read_at_its_touch_point() {
    let h = 0.5 / 2.0_f64.sqrt();
    let ball = ball_poled_y(0.5, Vec3::new(2.0, -h, 1.0 + h), tol());
    let (body, balls) = graft(slab(), &ball);
    let at = |v: topo::VertexKey| {
        let (_, p) = body.vertex_points().find(|(k, _)| *k == v).unwrap();
        [p.x, p.y, p.z]
    };
    let edge = body
        .edges()
        .find(|(_, e)| {
            let ends = [e.he_plus, e.he_minus].map(|h| at(body.get_half_edge(h).unwrap().start));
            let (p, q) = ([0.0, 0.0, 1.0], [4.0, 0.0, 1.0]);
            ends == [p, q] || ends == [q, p]
        })
        .map(|(k, _)| k)
        .unwrap();
    assert_eq!(
        reads(&body, &balls, Cell::Edge(edge)),
        ["Confirmed", "Stale"]
    );
}
