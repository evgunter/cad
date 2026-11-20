//! **A face whose boundary bounds no region of its chart** refuses
//! typed, on each lane that can be handed one.
//!
//! * **planar** — a lamina: two faces on one plane, each bounded by the
//!   same two coincident edges. Their chord points coincide, so every
//!   side the triangulation realises is traversed twice, and the
//!   even-odd reading marks nothing inside. Without the refusal both
//!   faces emit no triangle and the cross-face census panics
//!   (`tessellate`'s `unpaired_chord_segment`); with assertions off the
//!   answer is `Ok` with two empty patches.
//! * **trimmed** — the spline twin of the rim-only sphere cap: a NURBS
//!   hemisphere closed in `u`, its top row collapsed to the pole, bounded
//!   by its rim alone. Its chart image is a segment, which the lane never
//!   triangulates: the rim winds the chart's period, and the face's
//!   domain door refuses the loop first. The lamina on a flat NURBS
//!   patch refuses at the trimmed lane's own zero-area gate.
//!
//! Every body comes through the Euler doors, and tier 3 refuses each
//! (asserted): `tessellate` does not re-validate, so the lanes answer
//! for themselves.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, TAU};
use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Tol, Vec3};
use mesh::TessellateError;
use std::sync::Arc;
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite, MevSite};
use topo::{Body, FaceKey};

fn z_axis() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

fn arc(circle: &Curve3<f64>, t0: f64, t1: f64) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec::arc_of_circle(circle.clone(), t0, t1).expect("a circle carrier has an arc spec")
}

/// The lamina: `mvfs` on `surface`, `mev` along `t0 → t1` of `circle`,
/// and `mef` closing the loop with the same arc, so the two faces are
/// each bounded by the same two coincident edges. `at_rest` describes
/// both edges where they rest. Returns the body and the seed face.
fn lamina(
    surface: Surface<f64>,
    circle: &Curve3<f64>,
    (t0, t1): (f64, f64),
    at_rest: bool,
) -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(circle.eval(t0), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface,
            sense: true,
        },
    )
    .unwrap();
    let first = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            circle.eval(t1),
            arc(circle, t0, t1),
            tol,
        )
        .unwrap();
    let second = body
        .mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: first.he_minus,
            },
            arc(circle, t0, t1),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    if at_rest {
        for edge in [first.edge, second.edge] {
            body.describe_at_rest(edge, tol).unwrap();
        }
    }
    topo::mint_pcurves(&mut body, tol).unwrap();
    (body, seed.face)
}

/// **The planar lamina refuses `BoundaryBoundsNoRegion`**, on two arcs
/// and four δ, with its edges as scaffolding and described at rest
/// (tier 3 refuses the first for its scaffolds, the second for its zero
/// volume). Red without the refusal: the census panics at every row.
#[test]
fn a_planar_lamina_bounds_no_region_and_refuses() {
    let tol = Tol::witness();
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: z_axis(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let unit = Curve3::circle(
        Point3::new(0.0, 0.0, 0.0),
        z_axis(),
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
    );
    let skew = Curve3::circle(
        Point3::new(0.1, 0.2, 0.0),
        z_axis(),
        0.7,
        Vec3::new(0.6, 0.8, 0.0),
    );
    for (name, circle, span) in [
        ("unit quarter", &unit, (0.0, FRAC_PI_2)),
        ("skew arc", &skew, (0.3, 2.0)),
    ] {
        for at_rest in [false, true] {
            let (body, seed) = lamina(plane.clone(), circle, span, at_rest);
            assert!(
                topo::validate_geometric(&body, tol).is_err(),
                "{name}, at rest {at_rest}: tier 3 refuses the lamina"
            );
            for delta in [2e-2, 1e-2, 5e-3, 2e-3] {
                assert_eq!(
                    mesh::tessellate(&body, delta, tol).map(|_| ()),
                    Err(TessellateError::BoundaryBoundsNoRegion { face: seed }),
                    "{name}, at rest {at_rest}, δ = {delta:e}"
                );
            }
        }
    }
}

/// The NURBS hemisphere of radius 1 about +Z: `u` the full circle as
/// four rational quarters, `v` the quarter meridian from the rim (`v =
/// 0`) to the pole (`v = 1`), where its whole control row is the pole.
fn spline_hemisphere() -> Surface<f64> {
    let w = FRAC_1_SQRT_2;
    let ring = [
        (1.0, 0.0, 1.0),
        (1.0, 1.0, w),
        (0.0, 1.0, 1.0),
        (-1.0, 1.0, w),
        (-1.0, 0.0, 1.0),
        (-1.0, -1.0, w),
        (0.0, -1.0, 1.0),
        (1.0, -1.0, w),
        (1.0, 0.0, 1.0),
    ];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for (x, y, wu) in ring {
        for (p, wv) in [
            (Point3::new(x, y, 0.0), 1.0),
            (Point3::new(x, y, 1.0), w),
            (Point3::new(0.0, 0.0, 1.0), 1.0),
        ] {
            control.push(p);
            weights.push(wu * wv);
        }
    }
    let ku = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(ku, kv, control, weights).unwrap(),
    ))
}

/// **The spline rim-only cap never reaches the triangulation**: one
/// closed rim edge, the dome below it and the floor disc closing it.
/// The rim winds the closed chart once, so the trimmed lane's domain
/// door refuses the face (`OuterOpen`, `u` deck 1) before its UV
/// segment is a polygon. Tier 3 refuses the body for its scaffold rim.
#[test]
fn a_spline_rim_only_cap_refuses_at_the_domain_door() {
    let tol = Tol::witness();
    let rim = Curve3::circle(
        Point3::new(0.0, 0.0, 0.0),
        z_axis(),
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(rim.eval(0.0), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: spline_hemisphere(),
            sense: true,
        },
    )
    .unwrap();
    body.mef(
        MefSite::Lone {
            r#loop: seed.r#loop,
        },
        arc(&rim, 0.0, TAU),
        FaceSurface::New {
            surface: Surface::Plane {
                origin: Point3::new(0.0, 0.0, 0.0),
                normal: -z_axis(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
        tol,
    )
    .unwrap();
    topo::mint_pcurves(&mut body, tol).unwrap();
    assert!(topo::validate_geometric(&body, tol).is_err(), "tier 3");
    for delta in [1e-1, 1e-2] {
        let got = mesh::tessellate(&body, delta, tol).map(|_| ());
        assert!(
            matches!(
                &got,
                Err(TessellateError::DomainUnread { face, cause })
                    if *face == seed.face
                    && matches!(**cause, topo::pcurves::RingPlacementError::OuterOpen { .. })
            ),
            "δ = {delta:e}: {got:?}"
        );
    }
}

/// **The lamina on a flat NURBS patch refuses at the trimmed lane's
/// zero-area gate**: the two traversals' chart points coincide, so the
/// polygon's shoelace sum is exactly zero.
#[test]
fn a_spline_lamina_refuses_at_the_zero_area_gate() {
    let tol = Tol::witness();
    let unit = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let flat = Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            unit.clone(),
            unit,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
            ],
            vec![1.0; 4],
        )
        .unwrap(),
    ));
    let circle = Curve3::circle(
        Point3::new(0.5, 0.45, 0.0),
        z_axis(),
        0.3,
        Vec3::new(0.6, 0.8, 0.0),
    );
    let (body, _) = lamina(flat, &circle, (0.3, 2.0), false);
    assert!(topo::validate_geometric(&body, tol).is_err(), "tier 3");
    for delta in [1e-1, 1e-2] {
        assert_eq!(
            mesh::tessellate(&body, delta, tol).map(|_| ()),
            Err(TessellateError::MissingEntity {
                what: "degenerate trimmed boundary"
            }),
            "δ = {delta:e}"
        );
    }
}
