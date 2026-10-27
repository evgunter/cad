//! **A tube's rim vertex touching a turned face**, through tier 3′.
//!
//! The tube is a full revolve about `y` of the annulus `0.5 ≤ ρ ≤ 1`,
//! `y ∈ [0, 1]`; its outer rim vertex `v` is the lone vertex of a
//! closed circle edge. A cube of side 10 stands on the plane through
//! `v` with unit normal `m = (cos θ, sin θ, 0)`, on `m`'s side. The
//! plane holds the rim tangent `±z` and leans over the rim, so the tube
//! lies on the plane's far side and touches the cube at `v` only.
//!
//! The cube's touching face is turned off every world axis, so its
//! world reach box takes in tube faces strictly on the plane's far
//! side. The census backstop clears those pairs along the face's own
//! normal, where the gap is the plane's offset less the tube face's
//! support along `m`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol};
use topo::test_support::mapped_cube;
use topo::{AtRestBody, Body, ContactRecords, EntityId, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

const SIDE: f64 = 10.0;

/// One pose per rim: the rim vertex's height and `θ = (k + 0.37)·15°`.
/// Both lean over the rim (top `k = 0`, bottom `k = 21`).
const POSES: [(&str, f64, usize); 2] = [("top", 1.0, 0), ("bottom", 0.0, 21)];

fn tube() -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.5, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.5, 1.0), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

/// The cube of pose `(yv, k)` on the plane through the rim vertex moved
/// `gap` along `m`, centred on that point across the plane, on `m`'s
/// side.
fn cube(yv: f64, k: usize, gap: f64) -> Body<f64> {
    let th = (k as f64 + 0.37) * std::f64::consts::TAU / 24.0;
    let m = [th.cos(), th.sin(), 0.0];
    let v = [1.0 + gap * m[0], yv + gap * m[1], 0.0];
    // In-plane axes: `u = ẑ` and `w = m × u`.
    let u = [0.0, 0.0, 1.0];
    let w = [m[1], -m[0], 0.0];
    mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (SIDE * (x - 0.5), SIDE * (y - 0.5), SIDE * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    )
}

/// How many arm-1 refusals of `errors` name a cylinder face.
fn cylinder_pairs_refused(body: &Body<f64>, errors: &[ValidationError]) -> usize {
    curved_pairs_refused(body, errors, |s| {
        matches!(s, geom::Surface::Cylinder { .. })
    })
}

/// The union of the touching pose, in both operand orders: the exact
/// volume, one cited vertex-on-face record, and tier 3′ passes. Without
/// the record the same body refuses: the touch is real, and the
/// census cannot clear a contact nothing recorded.
#[test]
fn a_tube_rim_vertex_touching_a_turned_face_unions_past_tier_3_prime() {
    let t = AtRestBody::validate(tube(), tol()).unwrap();
    let exact = SIDE.powi(3) + std::f64::consts::PI * (1.0 - 0.25);
    for (rim, yv, k) in POSES {
        let c = AtRestBody::validate(cube(yv, k, 0.0), tol()).unwrap();
        for (order, p, q) in [("tube ∪ cube", &t, &c), ("cube ∪ tube", &c, &t)] {
            let tag = format!("{rim} rim, k = {k}, {order}");
            let r = topo::union(p, q, tol()).unwrap();
            let bb = r
                .body()
                .unwrap_or_else(|| panic!("{tag}: the union is a body"));
            let volume =
                topo::test_support::pinned_volume("the measured body", &bb.body, tol()).volume;
            assert!(
                (volume - exact).abs() < 1e-6,
                "{tag}: the union's volume is the cube's and the tube's, got {volume}"
            );
            let vf = bb.contacts.vertex_face().count();
            assert_eq!(vf, 1, "{tag}: the rim vertex on the cube face is recorded");
            assert_eq!(
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
                Ok(()),
                "{tag}: tier 3′ passes with the record"
            );
            let errors = topo::validate_pseudomanifold(&bb.body, &ContactRecords::default(), tol())
                .expect_err("an unrecorded touch must refuse");
            assert!(
                cylinder_pairs_refused(&bb.body, &errors) > 0,
                "{tag}: without the record, the cube face against the outer wall it \
                 touches is refused, got {errors:?}"
            );
        }
    }
}

/// The same cube moved 0.05 off the rim: two solids, no contact, and
/// tier 3′ passes. Every tube face lies at least 0.05 behind the
/// cube's plane along `m`, and no world axis separates the cube's face
/// from the tube.
#[test]
fn a_turned_face_clear_of_a_tube_rim_passes_tier_3_prime() {
    let t = AtRestBody::validate(tube(), tol()).unwrap();
    for (rim, yv, k) in POSES {
        let c = AtRestBody::validate(cube(yv, k, 0.05), tol()).unwrap();
        let tag = format!("{rim} rim, k = {k}, 0.05 off");
        let r = topo::union(&t, &c, tol()).unwrap();
        let bb = r
            .body()
            .unwrap_or_else(|| panic!("{tag}: the union is a body"));
        assert_eq!(bb.body.solids().count(), 2, "{tag}: two separated solids");
        assert_eq!(
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()),
            Ok(()),
            "{tag}: tier 3′ passes"
        );
    }
}

/// The cube pushed 0.05 into the tube and grafted beside it, so the
/// two materials overlap: the cube's face and the outer wall it cuts
/// share points, so no direction separates them, and the pair refuses.
#[test]
fn a_turned_face_cutting_a_tube_wall_is_never_cleared() {
    for (rim, yv, k) in POSES {
        let tag = format!("{rim} rim, k = {k}, 0.05 in");
        let mut body = tube();
        topo::graft_disjoint(&mut body, &cube(yv, k, -0.05)).unwrap();
        let errors = topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol())
            .expect_err("overlapping instances must refuse");
        assert!(
            cylinder_pairs_refused(&body, &errors) > 0,
            "{tag}: the cube face against the wall it cuts is refused, got {errors:?}"
        );
    }
}

/// How many arm-1 refusals of `errors` name a face on `kind`.
fn curved_pairs_refused(
    body: &Body<f64>,
    errors: &[ValidationError],
    kind: fn(&geom::Surface<f64>) -> bool,
) -> usize {
    let on = |id: &EntityId| match id {
        EntityId::Face(f) => body
            .get_face(*f)
            .and_then(|d| body.get_surface(d.surface))
            .is_some_and(kind),
        _ => false,
    };
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusUndecidable { a, b, what, .. }
                    if what.contains("a curved face of one is within reach of the other")
                        && (on(a) || on(b))
            )
        })
        .count()
}

/// The tube's pose cubes pushed `1e-7` and `1e-6` into the wall — a
/// hundred and a thousand ε, where the 0.05 row above would stay green
/// with each reach under-claimed by up to 0.025. The face pair still
/// refuses.
#[test]
fn a_turned_face_cutting_a_tube_wall_by_a_few_hundred_eps_is_never_cleared() {
    for (rim, yv, k) in POSES {
        for depth in [1e-7, 1e-6] {
            let tag = format!("{rim} rim, k = {k}, {depth} in");
            let mut body = tube();
            topo::graft_disjoint(&mut body, &cube(yv, k, -depth)).unwrap();
            let errors = topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol())
                .expect_err("overlapping instances must refuse");
            assert!(
                cylinder_pairs_refused(&body, &errors) > 0,
                "{tag}: the cube face against the wall it cuts is refused, got {errors:?}"
            );
        }
    }
}

/// A ring torus about `y` (major radius 2, minor 1, centred at the
/// origin): a circle profile revolved whole.
fn torus() -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(3.0, 0.0), 1.0)],
        sweep::Revolution::Full,
        tol(),
    )
}

/// The cube of side 10 standing on the plane with unit normal `m`
/// tangent to [`torus`], moved `gap` along `m`, centred on the tangent
/// point across the plane, on `m`'s side. The torus's support along
/// `m` is `R·|m⊥| + r`, reached at `R·m⊥/|m⊥| + r·m`.
fn cube_on_torus(m: [f64; 3], gap: f64) -> Body<f64> {
    let n = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt();
    let m = [m[0] / n, m[1] / n, m[2] / n];
    let s = (m[0] * m[0] + m[2] * m[2]).sqrt();
    let v = [
        2.0 * m[0] / s + (1.0 + gap) * m[0],
        (1.0 + gap) * m[1],
        2.0 * m[2] / s + (1.0 + gap) * m[2],
    ];
    // In-plane axes: `u = m × ŷ / |m⊥|` and `w = m × u`.
    let u = [-m[2] / s, 0.0, m[0] / s];
    let w = [
        m[1] * u[2] - m[2] * u[1],
        m[2] * u[0] - m[0] * u[2],
        m[0] * u[1] - m[1] * u[0],
    ];
    mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (SIDE * (x - 0.5), SIDE * (y - 0.5), SIDE * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    )
}

/// Normals off every world axis and off the torus's own: each leans
/// the plane over the tube's crest away from the seam circles.
const TORUS_NORMALS: [[f64; 3]; 2] = [[0.6, 0.5, 0.62], [-0.3, -0.7, 0.45]];

fn is_torus(s: &geom::Surface<f64>) -> bool {
    matches!(s, geom::Surface::Torus { .. })
}

/// A turned face touching a torus at one point, or cutting it by `1e-7`
/// or `1e-6`, grafted with no record: the face pair refuses. No edge of
/// either side crosses the other, so arm 1 is the only arm that sees
/// the meeting.
#[test]
fn a_turned_face_touching_or_cutting_a_torus_is_never_cleared() {
    for m in TORUS_NORMALS {
        for gap in [0.0, -1e-7, -1e-6] {
            let tag = format!("m = {m:?}, gap {gap}");
            let mut body = torus();
            topo::graft_disjoint(&mut body, &cube_on_torus(m, gap)).unwrap();
            let errors = topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol())
                .expect_err("a meeting nothing recorded must refuse");
            assert!(
                curved_pairs_refused(&body, &errors, is_torus) > 0,
                "{tag}: the cube face against the torus face it meets is refused, got {errors:?}"
            );
        }
    }
}

/// The same cubes moved `0.5` off the torus: tier 3′ passes, cleared
/// along the face's own normal (no world axis separates the pair). Not
/// nearer: the whole torus's extent along a row is
/// `(R + r)·|a⊥| + r·|aᵢ|` (`boxes::torus_extent`), which overshoots the
/// support `R·|a⊥| + r` by about 0.37 at the first normal, and a gap of
/// `1e-4` refuses.
#[test]
fn a_turned_face_clear_of_a_torus_passes_tier_3_prime() {
    for m in TORUS_NORMALS {
        let tag = format!("m = {m:?}, 0.5 off");
        let mut body = torus();
        topo::graft_disjoint(&mut body, &cube_on_torus(m, 0.5)).unwrap();
        assert_eq!(
            topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol()),
            Ok(()),
            "{tag}: tier 3′ passes"
        );
    }
}
