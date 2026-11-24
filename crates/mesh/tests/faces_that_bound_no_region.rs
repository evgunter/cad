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
//!   patch refuses as the planar one does, on the same parities.
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
    lamina_of(
        surface,
        arc(circle, t0, t1),
        (circle.eval(t0), circle.eval(t1)),
        at_rest,
    )
}

/// [`lamina`] on any edge spec from `p0` to `p1`.
fn lamina_of(
    surface: Surface<f64>,
    spec: EdgeCurveSpec<f64>,
    (p0, p1): (Point3<f64>, Point3<f64>),
    at_rest: bool,
) -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p0, true).unwrap();
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
            p1,
            spec.clone(),
            tol,
        )
        .unwrap();
    let second = body
        .mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: first.he_minus,
            },
            spec,
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
        frame: geom::Frame::new(
            Point3::new(0.0, 0.0, 0.0),
            z_axis(),
            Vec3::new(1.0, 0.0, 0.0),
        ),
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
                frame: geom::Frame::new(
                    Point3::new(0.0, 0.0, 0.0),
                    -z_axis(),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
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

/// **The lamina on a flat NURBS patch refuses `BoundaryBoundsNoRegion`**
/// as the planar one does: on an arc pair, by the constraint pass's
/// parities, though its polygon's shoelace sum is exactly zero and so
/// gives it no orientation; on a line pair, whose loop is two points,
/// before any triangulation.
#[test]
fn a_spline_lamina_bounds_no_region_and_refuses() {
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
    let (a, b) = (Point3::new(0.1, 0.3, 0.0), Point3::new(0.77, 0.51, 0.0));
    for (name, (body, seed)) in [
        ("arc pair", lamina(flat.clone(), &circle, (0.3, 2.0), false)),
        (
            "line pair",
            lamina_of(flat, EdgeCurveSpec::line_between(a, b), (a, b), false),
        ),
    ] {
        assert!(
            topo::validate_geometric(&body, tol).is_err(),
            "{name}: tier 3"
        );
        for delta in [1e-1, 1e-2] {
            assert_eq!(
                mesh::tessellate(&body, delta, tol).map(|_| ()),
                Err(TessellateError::BoundaryBoundsNoRegion { face: seed }),
                "{name}, δ = {delta:e}"
            );
        }
    }
}

/// The flat bilinear NURBS patch over the unit square at `z = 0`.
fn flat_patch() -> Surface<f64> {
    let unit = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    Surface::Nurbs(Arc::new(
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
    ))
}

/// **A lens of two short arcs meshes at a coarse δ.** Two DISTINCT arcs
/// from `Q = (0.8, 0.5)` to `P = (0.2, 0.5)` on the flat NURBS patch,
/// bulging either way: each is one chord at δ = 0.1 by its sagitta, so
/// its loop would draw two points and nothing to triangulate. The chord
/// pass's short-loop floor gives each two, and the face meshes. Red
/// without the floor: the loop of two points refuses. The one witness of
/// a split between two spline faces: what the join makes is read below.
#[test]
fn a_lens_of_two_short_arcs_meshes_at_a_coarse_delta() {
    let tol = Tol::witness();
    let r = 1.09_f64.sqrt();
    let (t0, t1) = (1.0_f64.atan2(0.3), 1.0_f64.atan2(-0.3));
    let below = Curve3::circle(
        Point3::new(0.5, -0.5, 0.0),
        z_axis(),
        r,
        Vec3::new(1.0, 0.0, 0.0),
    );
    let above = Curve3::circle(
        Point3::new(0.5, 1.5, 0.0),
        -z_axis(),
        r,
        Vec3::new(1.0, 0.0, 0.0),
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(below.eval(t0), true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: flat_patch(),
            sense: true,
        },
    )
    .unwrap();
    let first = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            below.eval(t1),
            arc(&below, t0, t1),
            tol,
        )
        .unwrap();
    // `above`'s frame is mirrored, so it too runs from Q to P over
    // `t0 → t1`, bulging the other way.
    body.mef(
        MefSite::Chords {
            he1: first.he_plus,
            he2: first.he_minus,
        },
        arc(&above, t0, t1),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    topo::mint_pcurves(&mut body, tol).unwrap();
    for delta in [1e-1, 2e-2] {
        let mesh = mesh::tessellate(&body, delta, tol)
            .unwrap_or_else(|e| panic!("δ = {delta:e}: the lens meshes: {e:?}"));
        let lens = mesh
            .patches
            .iter()
            .find(|p| p.face == seed.face)
            .expect("the seed face's patch");
        assert!(
            !lens.triangles.is_empty(),
            "δ = {delta:e}: the lens has triangles"
        );
        // What the join made: the two faces are one sheet, each laying two
        // triangles on the four chord points alone, so the join splits
        // the diagonal each lays (`mesh::join`'s census), at the foot on
        // its own surface: one point a face, the same position, two
        // triangles more a face. The mesh is closed by vertex id.
        assert_eq!(
            mesh::validate::check_mesh(&mesh),
            Ok(()),
            "δ = {delta:e}: a closed manifold by vertex id"
        );
        let on_edges: std::collections::HashSet<u32> = mesh
            .boundaries
            .iter()
            .flat_map(|b| b.points.iter().copied())
            .collect();
        assert_eq!(on_edges.len(), 4, "δ = {delta:e}: two chord points an arc");
        let split: Vec<_> = (0..mesh.positions.len())
            .filter(|i| !on_edges.contains(&u32::try_from(*i).unwrap()))
            .map(|i| mesh.positions[i])
            .collect();
        assert_eq!(split.len(), 2, "δ = {delta:e}: one split point a face");
        assert!(
            split[0].same_bits(split[1]),
            "δ = {delta:e}: one sheet, one foot"
        );
        assert_eq!(
            mesh.patches
                .iter()
                .map(|p| p.triangles.len())
                .collect::<Vec<_>>(),
            [4, 4],
            "δ = {delta:e}: each face's two triangles, one side split"
        );
    }
}

/// **A face holding a ring that retraces itself refuses for its
/// orientation**, from the trimmed lane's `None` arm. The face lies on
/// the flat NURBS patch: its outer loop is a triangle, and its ring is
/// two edges along one arc, the far side of a lamina face. The outer
/// loop gives the boundary sides of odd count, so the loops bound a
/// region. The ring's two traversals meet the same chart points, so its
/// shoelace sum is exactly zero, and the ring has no orientation to read
/// against the outer loop's. The lamina face itself bounds nothing.
#[test]
fn a_face_holding_a_retraced_ring_refuses_for_its_orientation() {
    let tol = Tol::witness();
    let line = EdgeCurveSpec::line_between;
    let (p0, p1, p2, q) = (
        Point3::new(0.1, 0.1, 0.0),
        Point3::new(0.9, 0.1, 0.0),
        Point3::new(0.5, 0.9, 0.0),
        Point3::new(0.5, 0.4, 0.0),
    );
    let circle = Curve3::circle(
        Point3::new(0.5, 0.5, 0.0),
        z_axis(),
        0.1,
        Vec3::new(0.0, -1.0, 0.0),
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p0, true).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: flat_patch(),
            sense: true,
        },
    )
    .unwrap();
    let fan = |body: &mut Body<f64>, at, to, spec| {
        body.mev(MevSite::Fan { he1: at, he2: at }, to, spec, tol)
            .unwrap()
    };
    let e01 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            line(p0, p1),
            tol,
        )
        .unwrap();
    let e12 = fan(&mut body, e01.he_minus, p2, line(p1, p2));
    body.mef(
        MefSite::Chords {
            he1: e12.he_minus,
            he2: e01.he_plus,
        },
        line(p2, p0),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    // A strut from the triangle's corner into the seed face, the lamina
    // at its tip, and the strut killed so the lamina's far side is a ring.
    let strut = fan(&mut body, e01.he_plus, q, line(p0, q));
    let a = fan(
        &mut body,
        strut.he_minus,
        circle.eval(2.0),
        arc(&circle, 0.0, 2.0),
    );
    body.mef(
        MefSite::Chords {
            he1: a.he_plus,
            he2: a.he_minus,
        },
        arc(&circle, 0.0, 2.0),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    topo::mint_pcurves(&mut body, tol).unwrap();
    assert_eq!(
        body.face(seed.face).unwrap().loops().count(),
        2,
        "the seed face holds the ring"
    );
    assert!(topo::validate_geometric(&body, tol).is_err(), "tier 3");
    for delta in [1e-1, 1e-2] {
        // The seed face is first in arena order, so `tessellate` answers
        // for it; the lamina face behind it bounds nothing.
        assert_eq!(
            mesh::tessellate(&body, delta, tol).map(|_| ()),
            Err(TessellateError::TrimLoopOrientation { face: seed.face }),
            "δ = {delta:e}"
        );
    }
}
