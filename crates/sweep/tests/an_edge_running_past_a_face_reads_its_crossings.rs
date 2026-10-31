//! **An edge running past the face it lies in is divided where the two
//! lines meet, and an edge of the face's boundary on its own line is no
//! crossing.**
//!
//! The plate `x ∈ [−1, 3]`, `y ∈ [−½, ½]`, `z ∈ [0, 1]` less the diamond
//! `|x − ¾| + |y| < ¼`, whose edge at `(1, 0)` runs along `z`, less a
//! cutter: a `(y, z)` profile on `x = 1` extruded along `+x`. The
//! pocket's wall `x = 1` holds the diamond's edge where the profile
//! covers `y = 0`, and the edge's two faces both leave it into the
//! wall's material, so `A ∖ B` parts it there (`topo::boolean::slit`).
//!
//! - **A shallow crossing**: the profile's side tilted θ off the edge,
//!   crossing it at `z = 0.6`, from θ = 0.1 down to 1e-6. Each `A ∖ B`
//!   builds sound against the closed form, or refuses as
//!   [`SHALLOW_REFUSALS`] names it at the run's ε: the crossing is read
//!   where the two lines meet, so a near-parallel pair is decided at the
//!   band, never divided at a point off either line.
//! - **A collinear boundary edge**: the profile an L whose inner corner's
//!   edge lies on the diamond's edge over `z ∈ [0.6, 0.75]`, its bottom on
//!   the wall's bottom edge or through the plate's bottom. The two edges
//!   run along each other, and `A ∖ B` refuses `EdgeInFace`
//!   ([`EdgeInFaceGap::Overlap`]) at every ε.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, RigidMotion, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::ExtrudeSide;
use sweep::test_support::{finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::{AtRestBody, Body, BooleanError, BooleanResult, EdgeInFaceGap, SolidContainment};

fn tol() -> Tol {
    Tol::witness()
}

fn poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect())
}

fn extrude_on(
    what: &str,
    plane: SketchPlane<f64>,
    loops: Vec<ProfileLoop<f64>>,
    h: f64,
) -> AtRestBody<f64> {
    let profile = Profile::new(plane, loops).validate(tol()).unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished(what, body, tol())
}

/// The plate less the diamond.
fn plate() -> AtRestBody<f64> {
    extrude_on(
        "plate",
        sketch_at(0.0),
        vec![
            poly(&[(-1.0, -0.5), (3.0, -0.5), (3.0, 0.5), (-1.0, 0.5)]),
            poly(&[(1.0, 0.0), (0.75, 0.25), (0.5, 0.0), (0.75, -0.25)]),
        ],
        1.0,
    )
}

/// A cutter: the `(y, z)` profile on `x = 1` extruded along `+x` by 3.
fn cutter(yz: &[(f64, f64)]) -> AtRestBody<f64> {
    let plane = SketchPlane::from_frame(RigidMotion::axes_yz(Point3::new(1.0, 0.0, 0.0)));
    extrude_on("cutter", plane, vec![poly(yz)], 3.0)
}

/// A margin off the plate less the diamond, positive outside.
fn in_plate(q: Point3<f64>) -> f64 {
    let bx = (-1.0 - q.x)
        .max(q.x - 3.0)
        .max(-0.5 - q.y)
        .max(q.y - 0.5)
        .max(-q.z)
        .max(q.z - 1.0);
    let diamond = ((q.x - 0.75).abs() + q.y.abs() - 0.25) / std::f64::consts::SQRT_2;
    bx.max(-diamond)
}

/// Sample points through the plate, the pocket and the cutter.
fn grid() -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for i in 0..23 {
        for j in 0..9 {
            for k in 0..9 {
                out.push(Point3::new(
                    -0.93 + 0.2131 * f64::from(i),
                    -0.57 + 0.1373 * f64::from(j),
                    0.031 + 0.1171 * f64::from(k),
                ));
            }
        }
    }
    out
}

/// **`body` is `A ∖ B` of the closed forms**: its volume `want`, tier 3
/// and 3′ against `records`, and its material at [`grid`] where both
/// margins read clear of 1e-3.
fn sound(what: &str, r: &topo::BooleanBody<f64>, want: f64, in_b: &dyn Fn(Point3<f64>) -> f64) {
    let band = Band::linear(tol()).unwrap();
    let v = topo::test_support::pinned_volume(what, &r.body, tol()).volume;
    assert!(
        (v - want).abs() < 1e-6,
        "{what}: volume {v}, closed form {want}"
    );
    assert_eq!(
        topo::validate_geometric(&r.body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&r.body, &r.contacts, tol()).err(),
        None,
        "{what}: tier 3′"
    );
    for q in grid() {
        let (a, b) = (in_plate(q), in_b(q));
        if a.abs() < 1e-3 || b.abs() < 1e-3 {
            continue;
        }
        let want = if a < 0.0 && b > 0.0 {
            SolidContainment::In
        } else {
            SolidContainment::Out
        };
        let got = topo::point_in_solid(&r.body, q, band, tol()).unwrap();
        assert_eq!(got, want, "{what}: material at {q:?}");
    }
}

/// The tilts of [`a_crossing_at_a_shallow_angle_builds_or_refuses_typed`].
const TILTS: [f64; 6] = [1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6];

/// The cells of [`a_crossing_at_a_shallow_angle_builds_or_refuses_typed`]
/// that refuse, as `(ε, θ, refusal kind)`; every other builds. At 1e-6
/// the side tilted 1e-5 reads in band against the wall where the
/// diamond's edge meets it (whether the face holds a point), and at 1e-6
/// it lies within the band of the edge's line, which the result gate
/// refuses as two edges on one line bounding overlapping material.
const SHALLOW_REFUSALS: &[(f64, f64, &str)] =
    &[(1e-6, 1e-5, "Escalated"), (1e-6, 1e-6, "LineWedges")];

/// **A crossing at a shallow angle builds sound or refuses typed** at
/// every tilt (module docs).
#[test]
fn a_crossing_at_a_shallow_angle_builds_or_refuses_typed() {
    let eps = tol().eps();
    let a = plate();
    let mut moved = Vec::new();
    for theta in TILTS {
        let t = f64::tan(theta);
        let b = cutter(&[
            (-0.35 * t, 0.25),
            (0.6, 0.25),
            (0.6, 0.75),
            (0.15 * t, 0.75),
        ]);
        let in_b = move |q: Point3<f64>| {
            (1.0 - q.x)
                .max(q.x - 4.0)
                .max(q.y - 0.6)
                .max(0.25 - q.z)
                .max(q.z - 0.75)
                .max(((q.z - 0.6) * t - q.y) * theta.cos())
        };
        let what = format!("θ = {theta:e}, ε = {eps:e}");
        let pinned = SHALLOW_REFUSALS
            .iter()
            .find(|&&(e, th, _)| e == eps && th == theta)
            .map(|&(_, _, kind)| kind);
        match (topo::subtract(&a, &b, tol()), pinned) {
            (Ok(BooleanResult::Body(r)), None) => {
                let wedge = 2.0 * (0.25 + 0.05 * t);
                sound(&what, &r, 3.875 - wedge, &in_b);
            }
            (Err(e), Some(kind))
                if format!("{:?}", e.kind()) == kind
                    && !e.to_string().contains(geom_core::KERNEL_DEFECT_ENDING) => {}
            (other, want) => moved.push(format!(
                "{what}: {}, where it {}",
                match other {
                    Ok(_) => "builds".to_owned(),
                    Err(e) => format!("refuses {:?}: {e}", e.kind()),
                },
                want.map_or("builds".to_owned(), |k| format!("refuses {k}"))
            )),
        }
    }
    assert!(moved.is_empty(), "{moved:#?}");
}

/// **A boundary edge on the edge's own line refuses as an overlap** at
/// every ε (module docs): the L standing on the wall's bottom edge, and
/// through the plate's bottom, where the collinear edge is the only one
/// the edge meets.
#[test]
fn a_boundary_edge_on_the_edges_line_refuses_as_an_overlap() {
    let a = plate();
    for (what, bottom) in [
        ("on the wall's bottom", 0.25),
        ("through the plate's bottom", -0.2),
    ] {
        let b = cutter(&[
            (-0.6, bottom),
            (0.6, bottom),
            (0.6, 0.75),
            (0.0, 0.75),
            (0.0, 0.6),
            (-0.6, 0.6),
        ]);
        match topo::subtract(&a, &b, tol()) {
            Err(BooleanError::EdgeInFace {
                gap: EdgeInFaceGap::Overlap,
                ..
            }) => {}
            other => panic!(
                "the L {what}: {:?}, where it refuses Overlap",
                other.map(|_| ())
            ),
        }
    }
}
