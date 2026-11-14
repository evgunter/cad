//! **What `mesh::tessellate` answers on valid bodies the tracker says it
//! refuses**, measured through the public doors and pinned as measured.
//!
//! Each row builds its body the way its tracker row names (a shell, a
//! split, a boolean, a loft), meshes it at three or more δ, and asserts
//! the exact answer: `Ok` with `check_mesh` passing, or the refusal's
//! variant with what tells its raising function apart. A refusal's
//! sentence is not read as its cause; each [`Got`] arm names the
//! function that constructs it.
//!
//! The rows these pin:
//! `work/tesslane/trimmed-tessellation-lacks-torus-and-plane-arms.md`,
//! `work/tesslane/cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm.md`,
//! `work/tesslane/valid-ringed-torus-faces-build-but-cannot-be-meshed.md`,
//! `work/tesslane/a-notched-or-ringed-cylinder-wall-does-not-tessellate.md`,
//! `work/tesslane/a-line-rim-on-a-creased-loft-wall-does-not-mesh.md`,
//! and the trimmed NURBS arm's one-column bands
//! (`trimmed::per_cell_candidates` under
//! `nurbs_cert::NurbsCellGrid::band_schedule`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_6, SQRT_2};
use std::collections::HashSet;

use geom::{CurveKind, SurfaceKind};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::path::{Open, Start};
use profile::test_support::bulge_loop;
use profile::{ArcSweep, Center, Profile, ProfileLoop, RawLoop as _, SketchPlane};
use sweep::test_support::{ball_poled_z, brick, finished, revolved_about_y, stacked_at};
use sweep::{
    ExtrudeSide, Extrusion, Revolution, RevolveAxis, Section, extrude, loft_body, revolve,
};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

use crate::common::{axis_y, quad, validated};

pub(super) fn tol() -> Tol {
    Tol::witness()
}

/// One `tessellate` answer, by the function that produced it.
#[derive(Debug, PartialEq, Eq, Clone)]
enum Got {
    /// `Ok`, and `validate::check_mesh` passes.
    Meshed,
    /// `UnsupportedCurve` from `trimmed::trim_frontier` (the chart
    /// roster's fallback in `tessellate_trimmed`): the named edge's
    /// carrier, and the curved chart beside it.
    Frontier(CurveKind, SurfaceKind),
    /// `UnsupportedCurve` from `chords::nurbs_chord_count`, in the chord
    /// pass, before any face is dispatched: a B-spline carrier with an
    /// interior knot of multiplicity equal to its degree.
    Kink(CurveKind),
    /// `UnsupportedCurve` from `trimmed::trim_polygon`'s crease check.
    Crease,
    /// `RingOnCurvedFace` on a face of this chart; `true` where the
    /// face's outer loop holds lines and circles only and no projected
    /// image, so `tessellate::lane_of` sent it to the curved lane and
    /// `curved::tessellate_curved`'s ring guard is what refused.
    Ring(SurfaceKind, bool),
    /// `UnsupportedCurvedShape { NotIsoRectangle { "props_rim_level" } }`
    /// from `curved::require_iso_rectangle_face`, its source raised by
    /// `geom_brep::props`' `require_rims_at_extremes`.
    NotIsoRectangle(SurfaceKind),
    /// `Triangulation` on a face of this chart.
    Triangulation(SurfaceKind),
    /// Anything else, spelled out.
    Other(String),
}

pub(super) fn kind_of(body: &Body<f64>, fk: topo::FaceKey) -> SurfaceKind {
    body.face_surface(fk).expect("a live face").kind()
}

/// The carrier kinds and stored image forms of a loop, edge by edge.
pub(super) fn loop_forms(body: &Body<f64>, lk: topo::LoopKey) -> Vec<(CurveKind, Option<&'static str>)> {
    let topo::LoopBoundary::Cycle { first } = body.get_loop(lk).expect("a live loop").boundary
    else {
        return Vec::new();
    };
    body.loop_cycle(first)
        .expect("a loop cycle")
        .into_iter()
        .map(|hek| {
            let he = body.get_half_edge(hek).expect("a half-edge");
            let carrier = body
                .get_curve_geom(body.get_edge(he.edge).expect("an edge").curve)
                .and_then(|g| g.certified())
                .expect("a certified edge")
                .carrier()
                .kind();
            let image = body.pcurve(hek).map(|c| match c.pcurve() {
                geom_brep::Pcurve::Harmonic { .. } => "Harmonic",
                geom_brep::Pcurve::Spiric { .. } => "Spiric",
                geom_brep::Pcurve::FocalSection(_) => "FocalSection",
                geom_brep::Pcurve::Projected(_) => "Projected",
                geom_brep::Pcurve::IsoLine { .. } => "IsoLine",
                geom_brep::Pcurve::IsoArc { .. } => "IsoArc",
                geom_brep::Pcurve::Fitted(_) => "Fitted",
                geom_brep::Pcurve::General(_) => "General",
            });
            (carrier, image)
        })
        .collect()
}

fn carrier_of(body: &Body<f64>, ek: topo::EdgeKey) -> CurveKind {
    body.get_curve_geom(body.get_edge(ek).expect("an edge").curve)
        .and_then(|g| g.certified())
        .expect("a certified edge")
        .carrier()
        .kind()
}

/// The curved chart beside `ek`: the first of its two faces that is no
/// plane.
fn curved_side(body: &Body<f64>, ek: topo::EdgeKey) -> SurfaceKind {
    let (a, b) = topo::readback::edge_sides(body, ek)
        .expect("an edge's sides")
        .faces();
    [a, b]
        .into_iter()
        .map(|fk| kind_of(body, fk))
        .find(|k| *k != SurfaceKind::Plane)
        .unwrap_or(SurfaceKind::Plane)
}

fn got(body: &Body<f64>, delta: f64) -> Got {
    use mesh::TessellateError as E;
    match mesh::tessellate(body, delta, tol()) {
        Ok(m) => match mesh::validate::check_mesh(&m) {
            Ok(()) => Got::Meshed,
            Err(e) => Got::Other(format!("meshed, and check_mesh refuses: {e:?}")),
        },
        Err(E::UnsupportedCurve { edge, note })
            if note.contains("trimmed-tessellation-lacks-torus-and-plane-arms") =>
        {
            Got::Frontier(carrier_of(body, edge), curved_side(body, edge))
        }
        Err(E::UnsupportedCurve { edge, note }) if note.contains("C⁰ kink") => {
            Got::Kink(carrier_of(body, edge))
        }
        Err(E::UnsupportedCurve { note, .. })
            if note.contains("crosses a C⁰ crease of the face between two chord points") =>
        {
            Got::Crease
        }
        Err(E::RingOnCurvedFace { face }) => {
            let outer = body.get_face(face).expect("the refused face").outer;
            let iso = loop_forms(body, outer).iter().all(|(carrier, image)| {
                matches!(carrier, CurveKind::Line | CurveKind::Circle)
                    && *image != Some("Projected")
            });
            Got::Ring(kind_of(body, face), iso)
        }
        Err(E::UnsupportedCurvedShape {
            face,
            source:
                geom_brep::props::PropsError::NotIsoRectangle {
                    what: "props_rim_level",
                },
        }) => Got::NotIsoRectangle(kind_of(body, face)),
        Err(E::Triangulation { face }) => Got::Triangulation(kind_of(body, face)),
        Err(e) => Got::Other(format!("{e:?}")),
    }
}

/// `body` answers `want` at every δ.
fn assert_at_every(what: &str, body: &Body<f64>, deltas: &[f64], want: &Got) {
    for &delta in deltas {
        assert_eq!(&got(body, delta), want, "{what}, δ = {delta:e}");
    }
}

pub(super) type Run = (&'static str, Result<BooleanResult<f64>, BooleanError>);

/// Every op of `a` and `b` in both member orders.
pub(super) fn six(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> [Run; 6] {
    [
        ("A ∪ B", topo::union(a, b, tol())),
        ("B ∪ A", topo::union(b, a, tol())),
        ("A ∩ B", topo::intersect(a, b, tol())),
        ("B ∩ A", topo::intersect(b, a, tol())),
        ("A ∖ B", topo::subtract(a, b, tol())),
        ("B ∖ A", topo::subtract(b, a, tol())),
    ]
}

/// Every op of `a` and `b` builds a body, and op `k` answers `want[k]`
/// at every δ.
fn assert_six(
    what: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    deltas: &[f64],
    want: &[Got; 6],
) {
    for ((op, run), want) in six(a, b).into_iter().zip(want) {
        let label = format!("{what}, {op}");
        let built = run.unwrap_or_else(|e| panic!("{label}: the op builds, got {e:?}"));
        let body = built
            .body()
            .unwrap_or_else(|| panic!("{label}: a body, not the empty result"));
        assert_at_every(&label, &body.body, deltas, want);
    }
}

pub(super) fn posed(what: &str, raw: Body<f64>, pose: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        what,
        topo::transform_rigid(&raw, &geom_core::test_support::rigid(&pose, tol())).unwrap(),
        tol(),
    )
}

/// A disc of radius `r` about `(0, 0, z0)` extruded `depth` along `z`.
pub(super) fn rod(r: f64, z0: f64, depth: f64) -> Body<f64> {
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol()).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp.into()])
        .validate(tol())
        .unwrap();
    let side = ExtrudeSide::Along;
    extrude(&profile, Extrusion::Distance { depth, side }, tol())
        .unwrap()
        .body
        .into_body()
}

pub(super) fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
        .into_body()
}

/// The frustum of radius `0.5 → 1` over `y ∈ [0, 1]`.
pub(super) fn widening() -> Body<f64> {
    revolved(&[(0.0, 0.0), (0.5, 0.0), (1.0, 1.0), (0.0, 1.0)])
}

// ------------------------------------------------- spiric, conic trims

/// The tour's torus-walled vessel meridian revolved a quarter turn
/// (`sweep`'s `spiric_rim` rows build the same body).
pub(super) fn vessel_quarter() -> Body<f64> {
    let tol = tol();
    let (r_foot, r_band, r_neck) = (5.0 / 64.0, 9.0 / 64.0, 7.0 / 64.0);
    let (y_foot, y_shoulder, y_mouth) = (4.0 / 64.0, 12.0 / 64.0, 24.0 / 64.0);
    let (h_tube, r_bellied) = (8.0 / 64.0, 6.0 / 64.0);
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(r_foot, 0.0), tol)
        .unwrap()
        .line_to(Point2::new(r_foot, y_foot), tol)
        .unwrap()
        .line_to(Point2::new(r_band, y_foot), tol)
        .unwrap()
        .arc_to(
            Center {
                c: Point2::new(r_bellied, h_tube),
                winding: ArcSweep::Ccw,
                p: Point2::new(r_band, y_shoulder),
            },
            tol,
        )
        .unwrap()
        .line_to(Point2::new(r_neck, y_shoulder), tol)
        .unwrap()
        .line_to(Point2::new(r_neck, y_mouth), tol)
        .unwrap()
        .line_to(Point2::new(0.0, y_mouth), tol)
        .unwrap()
        .line_to(Start, tol)
        .unwrap()
        .into();
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, Revolution::Partial(FRAC_PI_2), tol)
        .unwrap()
        .body
        .into_body()
}

/// The disc of radius `r` revolved a quarter turn about `x = 1.2`.
pub(super) fn klein_elbow(r: f64) -> Body<f64> {
    let disc = bulge_loop(vec![
        (Point2::new(-r, 0.0), 1.0),
        (Point2::new(r, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![disc])
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(1.2, 0.0),
        dir: Vec2::new(0.0, -1.0),
    };
    revolve(&profile, axis, Revolution::Partial(-FRAC_PI_2), tol())
        .unwrap()
        .body
        .into_body()
}

/// **A hollowed partial revolve refuses at its cavity's torus wall.**
/// `topo::shell` builds both hollows; each has plane caps and a torus
/// wall bounded by spirics. The refusal is the trimmed lane's chart
/// roster (`trim_frontier`), on a spiric edge of the torus wall, at
/// every δ. Every plane cap a spiric bounds is the planar lane's
/// (`tessellate::lane_of` sends every plane there), and the stored
/// image on the wall's spiric half-edges is `Pcurve::Spiric`.
#[test]
fn a_spiric_bounded_torus_wall_refuses_at_the_chart_roster() {
    let deltas = [5e-3, 1e-3, 2e-4];
    let want = Got::Frontier(CurveKind::Spiric, SurfaceKind::Torus);
    for (what, operand, t) in [
        ("the sectioned vessel", vessel_quarter(), 1.0 / 128.0),
        ("the klein elbow", klein_elbow(0.275), 0.02),
    ] {
        let hollow = topo::shell(&finished("the operand", operand, tol()), t, tol())
            .unwrap_or_else(|e| panic!("{what} hollows, got {e:?}"))
            .body;
        let spiric_faces: Vec<(SurfaceKind, Vec<Option<&str>>)> = hollow
            .faces()
            .filter_map(|(fk, face)| {
                let images: Vec<_> = loop_forms(&hollow, face.outer)
                    .into_iter()
                    .filter(|(carrier, _)| *carrier == CurveKind::Spiric)
                    .map(|(_, image)| image)
                    .collect();
                (!images.is_empty()).then(|| (kind_of(&hollow, fk), images))
            })
            .collect();
        assert!(
            spiric_faces
                .iter()
                .any(|(kind, _)| *kind == SurfaceKind::Plane),
            "{what}: a plane cap carries a spiric edge: {spiric_faces:?}"
        );
        for (kind, images) in &spiric_faces {
            let stored = match kind {
                SurfaceKind::Torus => Some("Spiric"),
                _ => None,
            };
            assert!(
                matches!(kind, SurfaceKind::Plane | SurfaceKind::Torus)
                    && images.iter().all(|i| *i == stored),
                "{what}: a spiric bounds planes (no stored image) and torus walls \
                 (`Pcurve::Spiric`) only: {spiric_faces:?}"
            );
        }
        assert_at_every(what, &hollow, &deltas, &want);
    }
}

/// **A cone face cut by a tilted plane refuses at the chart roster.**
/// Both frusta of `sweep`'s `reach_cone_split` rows, split through
/// `(0, 0.5, 0)` by the plane of normal `(sin 0.3, cos 0.3, 0)`: each
/// half's cone faces carry the section `Ellipse` with its exact
/// `Pcurve::FocalSection` image stored, and `trim_frontier` refuses.
/// The widening frustum under C1's tilted brick (`sweep`'s
/// `cone_join_lane`) refuses the same way in all six ops.
#[test]
fn a_cone_face_bounded_by_a_section_ellipse_refuses_at_the_chart_roster() {
    let want = Got::Frontier(CurveKind::Ellipse, SurfaceKind::Cone);
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(0.3f64.sin(), 0.3f64.cos(), 0.0),
        tol(),
    );
    let narrowing = revolved(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]);
    for (name, frustum) in [("narrowing", narrowing), ("widening", widening())] {
        let split = topo::splitting::split(&finished("the frustum", frustum, tol()), &plane, tol())
            .unwrap_or_else(|e| panic!("the {name} frustum splits, got {e:?}"));
        for (side, part) in [("above", &split.above), ("below", &split.below)] {
            let what = format!("the {name} frustum, {side}");
            let half = part.body().unwrap_or_else(|| panic!("{what}: a body"));
            let images: Vec<_> = half
                .faces()
                .filter(|(fk, _)| kind_of(half, *fk) == SurfaceKind::Cone)
                .flat_map(|(_, face)| loop_forms(half, face.outer))
                .filter(|(carrier, _)| *carrier == CurveKind::Ellipse)
                .collect();
            assert!(
                !images.is_empty()
                    && images
                        .iter()
                        .all(|(_, image)| *image == Some("FocalSection")),
                "{what}: the cone faces' ellipse half-edges store the focal-section image: \
                 {images:?}"
            );
            assert_at_every(&what, half, &[5e-3, 1e-3, 2e-4], &want);
        }
    }
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        10f64.to_radians(),
    );
    let c1_brick = posed(
        "C1's brick",
        brick((-3.0, 3.0), (-6.0, 0.0), (-3.0, 3.0), tol()),
        Affine3::translation(Point3::new(0.0, 0.5, 0.0) - Point3::origin()) * turn,
    );
    assert_six(
        "C1 (A the cone)",
        &finished("the frustum", widening(), tol()),
        &c1_brick,
        &[2e-2, 5e-3, 1e-3],
        &[
            want.clone(),
            want.clone(),
            want.clone(),
            want.clone(),
            want.clone(),
            want,
        ],
    );
}

/// **A ball across a cylinder's wall refuses in the chord pass.** The
/// first pose of `sweep`'s `cylinder_sphere_frame` rows: the section is
/// a B-spline carrier, on a cylinder face and a sphere face that store
/// its `Pcurve::Projected` image. `chords::nurbs_chord_count` refuses
/// the carrier itself, a C⁰ kink, before any face is dispatched, so
/// neither the cylinder arm nor the chart roster is reached.
#[test]
fn a_sphere_face_bounded_by_a_spline_section_refuses_in_the_chord_pass() {
    let cylinder = finished("the cylinder", rod(0.5, -1.0, 2.0), tol());
    let r = 0.3;
    let ball = posed(
        "the ball",
        revolved_about_y(
            vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
            Revolution::Full,
            tol(),
        ),
        Affine3::translation(Vec3::new(0.5, 0.0, 0.0)),
    );
    for (op, run) in [
        ("cylinder ∩ ball", topo::intersect(&cylinder, &ball, tol())),
        ("cylinder ∪ ball", topo::union(&cylinder, &ball, tol())),
    ] {
        let built = run.unwrap_or_else(|e| panic!("{op}: the op builds, got {e:?}"));
        let body = &built.body().unwrap_or_else(|| panic!("{op}: a body")).body;
        let spline_faces: Vec<(SurfaceKind, Vec<Option<&str>>)> = body
            .faces()
            .filter_map(|(fk, face)| {
                let images: Vec<_> = loop_forms(body, face.outer)
                    .into_iter()
                    .filter(|(carrier, _)| *carrier == CurveKind::Nurbs)
                    .map(|(_, image)| image)
                    .collect();
                (!images.is_empty()).then(|| (kind_of(body, fk), images))
            })
            .collect();
        for kind in [SurfaceKind::Cylinder, SurfaceKind::Sphere] {
            assert!(
                spline_faces.iter().any(|(k, images)| *k == kind
                    && images.iter().all(|i| *i == Some("Projected"))),
                "{op}: a {kind:?} face is bounded by the spline section through its \
                 projected image: {spline_faces:?}"
            );
        }
        assert_at_every(op, body, &[2e-2, 5e-3, 1e-3], &Got::Kink(CurveKind::Nurbs));
    }
}

// ---------------------------------------------------------------- rings

/// **A torus wall holding a spiric ring refuses at the curved lane's
/// ring guard.** The donut `(2, 0.5)` about `y` and the bar through its
/// tube (`sweep`'s `germ_plane_torus_spiric`, pose B2): the unions and
/// the donut less the bar keep the ovals as rings of torus faces whose
/// outer loops are iso circles, so `lane_of` sends them to the curved
/// lane, whose ring guard refuses. The other three ops put the spirics
/// on an outer loop and refuse at the chart roster.
#[test]
fn a_torus_wall_holding_a_spiric_ring_refuses_at_the_curved_lanes_ring_guard() {
    let torus = bulge_loop(vec![
        (Point2::new(2.0, -0.5), 1.0),
        (Point2::new(2.0, 0.5), 1.0),
    ]);
    let donut = revolve(&validated(vec![torus]), axis_y(), Revolution::Full, tol())
        .unwrap()
        .body
        .into_body();
    let bar = brick::<f64>((-0.05, 0.05), (0.25, 0.35), (1.0, 3.0), tol());
    let ring = Got::Ring(SurfaceKind::Torus, true);
    let frontier = Got::Frontier(CurveKind::Spiric, SurfaceKind::Torus);
    assert_six(
        "the donut and B2 (A the donut)",
        &finished("the torus", donut, tol()),
        &finished("B2", bar, tol()),
        &[2e-2, 5e-3, 1e-3],
        &[
            ring.clone(),
            ring.clone(),
            frontier.clone(),
            frontier.clone(),
            ring,
            frontier,
        ],
    );
}

/// **A ball face holed by a tilted tube's rim.** `sweep`'s
/// `a_tube_ending_on_a_ball` pose "tilted 1.2": the rim crosses neither
/// seam meridian and stays a ring of one half-band. Ball ∪ tube and
/// ball ∖ tube refuse at the curved lane's ring guard (the ring's
/// projected circles are not on the outer loop, which is what
/// `trimmed::has_tilted_circle` reads). Tube ∪ ball refuses
/// `Triangulation` first, on the OTHER half-band, which has no ring:
/// the ball's own faces refuse it ([`a_z_poled_ball_of_radius_root_two`]).
#[test]
fn a_ball_face_holding_a_tilted_ring_refuses_at_the_curved_lanes_ring_guard() {
    let tube = posed(
        "the tube",
        rod(1.0, 1.0, 2.0),
        Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), 1.2),
    );
    let ball = finished("the ball", ball_poled_z(SQRT_2, Vec3::zero(), tol()), tol());
    let ring = Got::Ring(SurfaceKind::Sphere, true);
    assert_six(
        "the tilted tube on the ball (A the tube)",
        &tube,
        &ball,
        &[2e-2, 5e-3, 1e-3],
        &[
            Got::Triangulation(SurfaceKind::Sphere),
            ring.clone(),
            Got::Meshed,
            Got::Meshed,
            Got::Meshed,
            ring,
        ],
    );
}

/// **The `z`-poled ball of radius `√2` does not mesh; radius 1 does.**
/// `sweep::test_support::ball_poled_z` turns a revolved ball a quarter
/// turn about `x`. At radius `√2` both of its faces refuse
/// `Triangulation`: `curved::tessellate_curved`'s `insert` closure hands
/// the CDT a pole entry whose azimuth is `−4.8e−49`, which `spade`
/// refuses as a coordinate. No ring and no trim is involved.
#[test]
fn a_z_poled_ball_of_radius_root_two() {
    let deltas = [2e-2, 5e-3];
    assert_at_every(
        "the z-poled ball, r = √2",
        &ball_poled_z(SQRT_2, Vec3::zero(), tol()),
        &deltas,
        &Got::Triangulation(SurfaceKind::Sphere),
    );
    for r in [1.0, 0.3] {
        assert_at_every(
            &format!("the z-poled ball, r = {r}"),
            &ball_poled_z(r, Vec3::zero(), tol()),
            &deltas,
            &Got::Meshed,
        );
    }
}

/// **A bar through a pipe's wall**, the three bars of
/// `work/tesslane/a-notched-or-ringed-cylinder-wall-does-not-tessellate.md`
/// and the brick turned 0.3 rad about `x` (`sweep`'s `verbs_germarms`).
/// The notched walls refuse at the curved lane's shape door
/// (`require_iso_rectangle_face`, on `props_rim_level`); the holed
/// walls, rings of rulings and arcs or of ellipse arcs alike, refuse at
/// the curved lane's ring guard, their outer loops being iso. ∩ and
/// bar ∖ pipe mesh; the turned bar's do so through the trimmed lane's
/// cylinder arm.
#[test]
fn a_notched_or_holed_pipe_wall_refuses_at_the_curved_lanes_doors() {
    let pipe = finished("the pipe", rod(1.0, -2.0, 4.0), tol());
    let deltas = [5e-3, 1e-3, 2e-4];
    let notch = Got::NotIsoRectangle(SurfaceKind::Cylinder);
    let ring = Got::Ring(SurfaceKind::Cylinder, true);
    let by = |wall: &Got| {
        [
            wall.clone(),
            wall.clone(),
            Got::Meshed,
            Got::Meshed,
            wall.clone(),
            Got::Meshed,
        ]
    };
    for (what, x, y, z, wall) in [
        (
            "the bar through",
            (-1.1, 1.1),
            (-0.3, 0.3),
            (-0.3, 0.3),
            &notch,
        ),
        (
            "the bar half in",
            (0.5, 1.1),
            (-0.3, 0.3),
            (-0.3, 0.3),
            &notch,
        ),
        (
            "the bar off the axis",
            (-1.1, 1.1),
            (0.15, 0.7),
            (-0.4, 0.1),
            &ring,
        ),
    ] {
        let bar = finished("the bar", brick::<f64>(x, y, z, tol()), tol());
        assert_six(
            &format!("{what} (A the pipe)"),
            &pipe,
            &bar,
            &deltas,
            &by(wall),
        );
    }
    let turned = posed(
        "the turned bar",
        brick((-1.1, 1.1), (0.2, 0.7), (-0.15, 0.15), tol()),
        Affine3::rotation_about_axis(Point3::new(0.0, 0.45, 0.0), Vec3::new(1.0, 0.0, 0.0), 0.3),
    );
    assert_six(
        "the bar turned 0.3 (A the pipe)",
        &pipe,
        &turned,
        &deltas,
        &by(&ring),
    );
}

/// **A cone wall holding a ring of ellipse arcs** (`sweep`'s
/// `cone_join_lane`, pose T1): the unions and cone ∖ box refuse at the
/// curved lane's ring guard; the other three ops trim the cone face's
/// outer loop by an ellipse and refuse at the chart roster.
#[test]
fn a_cone_wall_holding_a_ring_refuses_at_the_curved_lanes_ring_guard() {
    let height = 1.2 / FRAC_PI_6.tan();
    let cone = posed(
        "the cone",
        revolved(&[(0.0, 0.0), (1.2, 0.0), (0.0, height)]),
        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), FRAC_PI_2),
    );
    let edge = Point3::new(0.5 * 0.6, height - 1.5 * 0.6, 0.0);
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 0.0, 1.0),
        -50f64.to_radians(),
    );
    let wedge = posed(
        "the box",
        brick((0.0, 6.0), (0.0, 6.0), (-3.0, 3.0), tol()),
        Affine3::translation(edge - Point3::origin()) * turn,
    );
    let ring = Got::Ring(SurfaceKind::Cone, true);
    let frontier = Got::Frontier(CurveKind::Ellipse, SurfaceKind::Cone);
    assert_six(
        "T1 (A the cone)",
        &cone,
        &wedge,
        &[2e-2, 5e-3, 1e-3],
        &[
            ring.clone(),
            ring.clone(),
            frontier.clone(),
            frontier.clone(),
            ring,
            frontier,
        ],
    );
}

// ------------------------------------------------------- creased lofts

/// **A line rim on a creased loft wall meshes at some δ and refuses at
/// its neighbours.** Degree-1 lofts of a square whose `+x` side bulges
/// by the listed amounts, one section a metre (`sweep`'s
/// `degree1_loft_slabs`). The refusal is `trim_polygon`'s crease check;
/// the loft with no bulge-0 section meshes at every δ.
#[test]
fn a_line_rim_on_a_creased_loft_wall_meshes_or_refuses_with_delta() {
    let corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let section = |b: f64| -> Section {
        vec![bulge_loop(
            (0..4)
                .map(|i| {
                    let (x, y) = corners[i];
                    (Point2::new(x, y), if i == 1 { b } else { 0.0 })
                })
                .collect(),
        )]
    };
    let deltas = [5e-3, 2e-3, 1e-3, 5e-4];
    let (ok, no) = (Got::Meshed, Got::Crease);
    let cases: [(&[f64], [&Got; 4]); 4] = [
        (&[0.0, 1.0, 0.0], [&ok, &ok, &no, &no]),
        (&[1.0, 0.2, 1.0], [&ok, &ok, &ok, &ok]),
        (&[1.0, 0.2, 0.0, 0.7, 1.0], [&no, &no, &ok, &ok]),
        (&[0.0, 1.0, 0.3, 0.0], [&no, &ok, &no, &no]),
    ];
    for (bulges, want) in cases {
        let sections: Vec<Section> = bulges.iter().map(|&b| section(b)).collect();
        #[allow(clippy::cast_precision_loss)]
        let zs: Vec<f64> = (0..bulges.len()).map(|i| i as f64).collect();
        let lofted = loft_body::<f64>(&sections, &stacked_at(&zs), 1, tol())
            .unwrap_or_else(|e| {
                panic!(
                    "{bulges:?} lofts: {}",
                    e.render(geom_brep::recourse::Reading::Build)
                )
            })
            .body
            .into_body();
        for (delta, want) in deltas.into_iter().zip(want) {
            assert_eq!(&got(&lofted, delta), want, "{bulges:?}, δ = {delta:e}");
        }
    }
}

// ------------------------------------------------- one-column NURBS bands

/// The section of a blade `2·scale` wide and `0.01·scale` thick, turned
/// by `turn` about the sketch origin.
pub(super) fn blade(scale: f64, turn: f64) -> Section {
    let (s, c) = turn.sin_cos();
    let p = |x: f64, y: f64| {
        let (x, y) = (x * scale, y * scale);
        (x * c - y * s, x * s + y * c)
    };
    quad([
        p(-1.0, -0.005),
        p(1.0, -0.005),
        p(1.0, 0.005),
        p(-1.0, 0.005),
    ])
}

/// A loft of [`blade`] sections at `zs`, each turned `rate·z`.
pub(super) fn lofted_blade(zs: &[f64], rate: f64, degree: usize) -> Body<f64> {
    let sections: Vec<Section> = zs.iter().map(|&z| blade(1.0, rate * z)).collect();
    loft_body::<f64>(&sections, &stacked_at(zs), degree, tol())
        .unwrap_or_else(|e| {
            panic!(
                "the blade lofts: {}",
                e.render(geom_brep::recourse::Reading::Build)
            )
        })
        .body
        .into_body()
}

/// Per spline face of a meshed blade: `(the width of its bottom rim,
/// its triangle count, how many of its triangle corners are points no
/// edge's polyline holds)`, narrow faces first.
fn blade_walls(body: &Body<f64>, delta: f64) -> Vec<(f64, usize, usize)> {
    let mesh = mesh::tessellate(body, delta, tol())
        .unwrap_or_else(|e| panic!("the blade meshes at δ = {delta:e}, got {e:?}"));
    mesh::validate::check_mesh(&mesh)
        .unwrap_or_else(|e| panic!("the blade's mesh at δ = {delta:e}: {e:?}"));
    let shared: HashSet<u32> = mesh
        .boundaries
        .iter()
        .flat_map(|b| b.points.iter().copied())
        .collect();
    let mut walls: Vec<(f64, usize, usize)> = mesh
        .patches
        .iter()
        .filter(|p| kind_of(body, p.face) == SurfaceKind::Nurbs)
        .map(|p| {
            let ids: HashSet<u32> = p.triangles.iter().flatten().copied().collect();
            let floor: Vec<Point3<f64>> = ids
                .iter()
                .map(|&i| mesh.positions[i as usize])
                .filter(|q| q.z.abs() < 1e-12)
                .collect();
            let width = floor
                .iter()
                .flat_map(|a| floor.iter().map(move |b| (*a - *b).norm()))
                .fold(0.0, f64::max);
            let interior = ids.iter().filter(|i| !shared.contains(i)).count();
            (width, p.triangles.len(), interior)
        })
        .collect();
    walls.sort_by(|a, b| a.0.total_cmp(&b.0));
    walls
}

/// **A wall narrower than one column of its own lattice meshes on its
/// boundary chords alone.** A blade `2 × 0.01` lofted at degree 3
/// through four sections turned 0.4 rad a metre. Each narrow wall's
/// band schedule is one band of ONE column (`nuc == 1`) and 165, 520 and
/// 1642 rows at these δ, so `per_cell_candidates` emits no point for
/// it: the face's triangles are two per boundary chord of its long
/// sides, and none of its corners is a lattice point. The wide walls
/// carry lattice points. Measured worst certificates of the narrow
/// walls: `7.0e−5`, `1.8e−5`, `5.2e−6`.
#[test]
fn a_one_column_wall_meshes_on_its_boundary_chords_alone() {
    let body = lofted_blade(&[0.0, 1.0, 2.0, 3.0], 0.4, 3);
    for (delta, narrow_triangles) in [(1e-2, 330), (1e-3, 1040), (1e-4, 3284)] {
        let walls = blade_walls(&body, delta);
        assert_eq!(walls.len(), 4, "δ = {delta:e}: four walls");
        for (width, triangles, interior) in &walls[..2] {
            assert!(
                (width - 0.01).abs() < 1e-9 && *triangles == narrow_triangles && *interior == 0,
                "δ = {delta:e}: a narrow wall is {narrow_triangles} triangles on no lattice \
                 point: width {width}, {triangles} triangles, {interior} lattice points"
            );
        }
        for (width, _, interior) in &walls[2..] {
            assert!(
                (width - 2.0).abs() < 1e-9 && *interior > 0,
                "δ = {delta:e}: a wide wall carries lattice points: width {width}, {interior}"
            );
        }
    }
}

/// **A one-column band whose rows are raised adds no point.** The blade
/// lofted at degree 2 through stations `0, 0.01, 0.02, 0.5, 1`
/// (`lofted_circle`'s uneven stations), so each wall has three bands of
/// unlike aspect and `band_schedule` raises the rows of the tall ones.
///
/// Straight, every wall's bands are `nuc == 1` with rows `1, 1, 1 → 3`,
/// and the mesh is the box's eight corners and twelve triangles at
/// every δ. Turned 0.01 rad a metre, the walls curve: at δ `1e−1` all
/// four walls are one-column bands with rows `1, 1 → 2, 2 → 5` and
/// carry six triangles each; at `5e−2`, rows `1, 1 → 2, 3 → 5` and
/// eight; at `1e−2` the narrow walls have rows `1, 2 → 3, 6 → 9` and
/// eighteen. None of those walls has a lattice point, and each meshes:
/// measured worst certificates `3.3e−3` (wide) and `1.9e−5` (narrow) at
/// δ `1e−1`, `2.5e−3` and `1.4e−5` at `5e−2`, `5.9e−6` (narrow) at
/// `1e−2`.
#[test]
fn a_one_column_band_with_raised_rows_adds_no_lattice_point() {
    let zs = [0.0, 0.01, 0.02, 0.5, 1.0];
    let straight = lofted_blade(&zs, 0.0, 2);
    for delta in [1e-2, 1e-3, 1e-4] {
        let mesh = mesh::tessellate(&straight, delta, tol())
            .unwrap_or_else(|e| panic!("the straight blade meshes at δ = {delta:e}, got {e:?}"));
        mesh::validate::check_mesh(&mesh)
            .unwrap_or_else(|e| panic!("the straight blade's mesh at δ = {delta:e}: {e:?}"));
        let triangles: usize = mesh.patches.iter().map(|p| p.triangles.len()).sum();
        assert_eq!(
            (mesh.positions.len(), triangles),
            (8, 12),
            "the straight blade at δ = {delta:e}: its corners and nothing else"
        );
    }
    let turned = lofted_blade(&zs, 0.01, 2);
    for (delta, narrow, wide) in [(1e-1, 6, Some(6)), (5e-2, 8, Some(8)), (1e-2, 18, None)] {
        let walls = blade_walls(&turned, delta);
        assert_eq!(walls.len(), 4, "δ = {delta:e}: four walls");
        for (width, triangles, interior) in &walls[..2] {
            assert!(
                (width - 0.01).abs() < 1e-9 && *triangles == narrow && *interior == 0,
                "δ = {delta:e}: a narrow wall is {narrow} triangles on no lattice point: \
                 width {width}, {triangles} triangles, {interior} lattice points"
            );
        }
        for (width, triangles, interior) in &walls[2..] {
            assert!(
                (width - 2.0).abs() < 1e-9
                    && wide.is_none_or(|n| *triangles == n && *interior == 0),
                "δ = {delta:e}: a wide wall of one column is {wide:?} triangles on no lattice \
                 point: width {width}, {triangles} triangles, {interior} lattice points"
            );
        }
    }
}
