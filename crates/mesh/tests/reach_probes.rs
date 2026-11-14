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
    /// `Ok`, `validate::check_mesh` passes, and the mesh is the body's
    /// ([`geometry`]).
    Meshed,
    /// `UnsupportedCurve` from `chords::nurbs_chord_count`, in the chord
    /// pass, before any face is dispatched: a B-spline carrier with an
    /// interior knot of multiplicity equal to its degree.
    Kink(CurveKind),
    /// `UnsupportedCurve` from `trimmed::trim_polygon`'s crease check.
    Crease,
    /// `UnsupportedCurve` from `chords::chart_image`: a trimmed analytic
    /// face's edge of this carrier stores a projected image, which the
    /// chord pass bounds for a conic's image only.
    ConicImageOnly(CurveKind),
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
pub(super) fn loop_forms(
    body: &Body<f64>,
    lk: topo::LoopKey,
) -> Vec<(CurveKind, Option<&'static str>)> {
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

fn got(body: &Body<f64>, delta: f64) -> Got {
    use mesh::TessellateError as E;
    match mesh::tessellate(body, delta, tol()) {
        Ok(m) => match mesh::validate::check_mesh(&m) {
            Ok(()) => match geometry(body, &m, delta) {
                Ok(()) => Got::Meshed,
                Err(e) => Got::Other(format!("meshed, and {e}")),
            },
            Err(e) => Got::Other(format!("meshed, and check_mesh refuses: {e:?}")),
        },
        Err(E::UnsupportedCurve { edge, note }) if note.contains("C⁰ kink") => {
            Got::Kink(carrier_of(body, edge))
        }
        Err(E::UnsupportedCurve { note, .. })
            if note.contains("crosses a C⁰ crease of the face between two chord points") =>
        {
            Got::Crease
        }
        Err(E::UnsupportedCurve { edge, note })
            if note.contains("bounds the UV speed of a conic's image only") =>
        {
            Got::ConicImageOnly(carrier_of(body, edge))
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

/// **The mesh is the body's**, which `check_mesh` cannot see:
///
/// - it encloses the body's own volume to within its chordal deficit,
///   `δ` times the body's area, both read off the B-rep
///   (`validate::check_volume`). The volume is not read on a body with a
///   spline face: its mass properties cost seconds a δ there, and the
///   rows meshing such bodies here are about their creases;
/// - on every analytic curved face, each triangle:
///   - **winds outward**: its normal, as the mesh winds it, has a
///     positive dot with the face's outward normal at the foot of its
///     centroid, read here from the surface's own closed form
///     ([`outward`]), not from the kernel's normal the lane's winding
///     census reads. A face meshed as its complement, or wound inward,
///     misses the volume by a face's worth; a triangle folded over a
///     chord beside it points into the surface;
///   - **lies within δ of the surface**: its true distance, sampled on a
///     barycentric grid ([`distance`]), is at most `δ`. That reds a mesh
///     that is truly past δ, not a certificate that under-reports: a
///     lattice sized at its own steps stays within δ whatever the
///     certificate says, so a certificate is held to the truth in its own
///     row (`mesh`'s `chart_table` tests,
///     `the_cone_and_torus_certificates_hold_the_true_deviation`);
///   - **is not on one line**: `2·area/L²`, `L` its longest side, is at
///     least [`MIN_SHAPE`], so a triangle along one straight line of the
///     surface (a cone's ruling) reds here; a thin triangle does not.
fn geometry(body: &Body<f64>, mesh: &mesh::Mesh, delta: f64) -> Result<(), String> {
    if !body
        .faces()
        .any(|(fk, _)| matches!(kind_of(body, fk), SurfaceKind::Nurbs | SurfaceKind::Approx))
    {
        let at_rest = sweep::test_support::at_rest(body, tol());
        let props = topo::test_support::pinned_volume("the meshed body", &at_rest, tol());
        mesh::validate::check_volume(mesh, props.volume, props.surface_area, delta)
            .map_err(|e| format!("its volume misses the body's: {e:?}"))?;
    }
    for patch in &mesh.patches {
        let surface = body.face_surface(patch.face).expect("a live face");
        let sense = body.get_face(patch.face).expect("a live face").sense;
        let Some(_) = outward(surface, sense, Point3::origin()) else {
            continue;
        };
        for t in &patch.triangles {
            let [a, b, c] = t.map(|i| mesh.positions[i as usize]);
            let normal = (b - a).cross(c - a);
            let at = |x: f64, y: f64| {
                let z = 1.0 - x - y;
                Point3::new(
                    x * a.x + y * b.x + z * c.x,
                    x * a.y + y * b.y + z * c.y,
                    x * a.z + y * b.z + z * c.z,
                )
            };
            let centroid = at(1.0 / 3.0, 1.0 / 3.0);
            let kind = surface.kind();
            let outward = outward(surface, sense, centroid).expect("an analytic face");
            // A zero dot, or NaN where the foot is not one point, is not
            // outward.
            if normal.dot(outward).partial_cmp(&0.0) != Some(core::cmp::Ordering::Greater) {
                return Err(format!(
                    "a {kind:?} face's triangle {t:?} winds against the face"
                ));
            }
            let longest = (b - a).norm().max((c - b).norm()).max((a - c).norm());
            let shape = normal.norm() / (longest * longest);
            if shape
                .partial_cmp(&MIN_SHAPE)
                .is_none_or(core::cmp::Ordering::is_lt)
            {
                return Err(format!(
                    "a {kind:?} face's triangle {t:?} has 2·area/L² {shape:e}, under {MIN_SHAPE:e}"
                ));
            }
            const N: usize = 6;
            #[allow(clippy::cast_precision_loss)]
            let worst = (0..=N)
                .flat_map(|i| (0..=N - i).map(move |j| (i as f64 / N as f64, j as f64 / N as f64)))
                .map(|(x, y)| distance(surface, at(x, y)).expect("an analytic face"))
                .fold(0.0f64, |w, d| if d.is_nan() { d } else { w.max(d) });
            if worst
                .partial_cmp(&(delta * (1.0 + 1e-9)))
                .is_none_or(core::cmp::Ordering::is_gt)
            {
                return Err(format!(
                    "a {kind:?} face's triangle {t:?} lies {worst:e} off its surface, past δ {delta:e}"
                ));
            }
        }
    }
    Ok(())
}

/// **The least `2·area/L²` [`geometry`] admits on an analytic curved
/// face: a triangle that is not on one line.** Three points of one cone
/// ruling are exactly collinear in the cone's development
/// (`mesh::trimmed`, "The development"), so the lane cannot build that
/// triangle, and one built in the chart reads about `1e-17` (rounding).
/// It is not a bound on thin triangles: nothing bounds those. Beside a
/// ruling a triangle's ratio goes as `(h/L)²`, and valid accepted meshes
/// read `8·10⁻⁷` (a funnel cut `10⁻⁶` above its apex), `3·10⁻⁸` to
/// `8·10⁻⁷` (a frustum whose small rim has radius `2·10⁻⁷`…`5·10⁻⁷`,
/// whole or split, δ `5·10⁻³`) and `1.6·10⁻⁸` (a needle funnel of radius
/// `0.002` at δ `10⁻⁶`)
/// (`work/tesslane/a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md`).
const MIN_SHAPE: f64 = 1e-12;

/// **The outward normal of an analytic curved surface at the foot of
/// `p`**, from the surface's closed form (`geom::Surface`'s variant
/// docs), signed by the face's `sense`: a cylinder's radial, a cone's
/// `radial·cos α ∓ axis·sin α` on the `v > 0` and `v < 0` nappes, with
/// `radial` the direction of `p` from the axis, a sphere's direction from its centre, a torus's or a
/// spindle's from the point of its tube's centre circle nearest `p`.
/// `None` on a plane or a spline surface.
fn outward(surface: &geom::Surface<f64>, sense: bool, p: Point3<f64>) -> Option<Vec3<f64>> {
    use geom::Surface as S;
    let radial = |w: Vec3<f64>, axis: Vec3<f64>| w - axis * w.dot(axis);
    let n = match *surface {
        S::Cylinder { origin, axis, .. } => radial(p - origin, axis),
        S::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let w = p - apex;
            let r = radial(w, axis);
            // Off the axis on either nappe: the chart's `−(radial(u)·cos α
            // − axis·sin α)` at `v < 0`, whose point lies along
            // `−radial(u)`.
            let tilt = if w.dot(axis) < 0.0 { -1.0 } else { 1.0 };
            r * (half_angle.cos() / r.norm()) - axis * (tilt * half_angle.sin())
        }
        S::Sphere { center, .. } => p - center,
        S::Torus {
            center,
            axis,
            major_radius,
            ..
        }
        | S::Spindle {
            center,
            axis,
            major_radius,
            ..
        } => {
            let w = p - center;
            let r = radial(w, axis);
            w - r * (major_radius / r.norm())
        }
        _ => return None,
    };
    Some(if sense { n } else { -n })
}

/// **The distance from `p` to an analytic curved surface**, from its
/// closed form: to a cylinder's axis less its radius, to the nearer of a
/// cone's two nappes (the nearer ray of its meridian section, or the
/// apex), to a sphere's centre less its radius, to a torus's or a
/// spindle's tube centre circle less its minor radius. `None` on a plane
/// or a spline surface.
fn distance(surface: &geom::Surface<f64>, p: Point3<f64>) -> Option<f64> {
    use geom::Surface as S;
    let split = |w: Vec3<f64>, axis: Vec3<f64>| {
        let h = w.dot(axis);
        (h, (w - axis * h).norm())
    };
    Some(match *surface {
        S::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => (split(p - origin, axis).1 - radius).abs(),
        S::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (h, rho) = split(p - apex, axis);
            let (s, c) = (half_angle.sin(), half_angle.cos());
            // The meridian section is two rays from the apex,
            // `t·(s, ±c)`, `t ≥ 0` in `(ρ, h)`.
            [1.0, -1.0]
                .map(|sign| {
                    let t = (rho * s + h * sign * c).max(0.0);
                    (rho - t * s).hypot(h - t * sign * c)
                })
                .into_iter()
                .fold(f64::INFINITY, f64::min)
        }
        S::Sphere { center, radius, .. } => ((p - center).norm() - radius).abs(),
        S::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        }
        | S::Spindle {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (h, rho) = split(p - center, axis);
            ((rho - major_radius).hypot(h) - minor_radius).abs()
        }
        _ => return None,
    })
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
    try_revolved(pts).unwrap()
}

/// [`revolved`], with the revolve's refusal as text.
fn try_revolved(pts: &[(f64, f64)]) -> Result<Body<f64>, String> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full, tol())
        .map(|r| r.body.into_body())
        .map_err(|e| format!("{e:?}"))
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

/// **A hollowed partial revolve meshes, its cavity's torus wall on the
/// torus row.** `topo::shell` builds both hollows; each has plane caps
/// and a torus wall bounded by spirics, whose half-edges store
/// `Pcurve::Spiric`. The trimmed lane meshes the wall from its chart row
/// at every δ, and the mesh is the body's ([`geometry`]). Every plane cap
/// a spiric bounds is the planar lane's (`tessellate::lane_of` sends every
/// plane there).
#[test]
fn a_spiric_bounded_torus_wall_meshes_on_the_torus_row() {
    let deltas = [5e-3, 1e-3, 2e-4];
    let want = Got::Meshed;
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

/// **A cone face cut by a tilted plane meshes on the cone row.** Both
/// frusta of `sweep`'s `reach_cone_split` rows, split through
/// `(0, 0.5, 0)` by the plane of normal `(sin 0.3, cos 0.3, 0)`: each
/// half's cone faces carry the section `Ellipse` with its exact
/// `Pcurve::FocalSection` image stored, and the trimmed lane meshes them
/// from the face's chart row; the mesh is the body's ([`geometry`]).
/// The widening frustum under C1's tilted brick (`sweep`'s
/// `cone_join_lane`) meshes the same way in all six ops.
#[test]
fn a_cone_face_bounded_by_a_section_ellipse_meshes_on_the_cone_row() {
    let want = Got::Meshed;
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
            let nappe = if name == "narrowing" { -1.0 } else { 1.0 };
            assert!(
                nappes(half).iter().all(|&s| s == nappe),
                "{what}: its cone faces lie on the {nappe} nappe: {:?}",
                nappes(half)
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

/// **A cone face through its apex meshes on the cone row.** The funnel
/// (apex at the origin, rim of radius 1 at `y = 1`) split by the tilted
/// plane of [`a_cone_face_bounded_by_a_section_ellipse_meshes_on_the_cone_row`]:
/// below the cut each half face's loop runs up one ruling from the apex,
/// along the section ellipse, and down the other ruling back to it, so its
/// polygon meets the apex row at two azimuths and keeps the apex there
/// twice (`crate::chart_table`'s singular-row rule). Both halves mesh at
/// every δ, and the mesh is the body's ([`geometry`]).
#[test]
fn a_cone_face_through_its_apex_meshes_on_the_cone_row() {
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(0.3f64.sin(), 0.3f64.cos(), 0.0),
        tol(),
    );
    let funnel = revolved(&[(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    let split = topo::splitting::split(&finished("the funnel", funnel, tol()), &plane, tol())
        .unwrap_or_else(|e| panic!("the funnel splits, got {e:?}"));
    for (side, part, rulings) in [("above", &split.above, 2), ("below", &split.below, 2)] {
        let what = format!("the funnel, {side}");
        let half = part.body().unwrap_or_else(|| panic!("{what}: a body"));
        let loops: Vec<Vec<CurveKind>> = half
            .faces()
            .filter(|(fk, _)| kind_of(half, *fk) == SurfaceKind::Cone)
            .map(|(_, face)| {
                loop_forms(half, face.outer)
                    .into_iter()
                    .map(|(c, _)| c)
                    .collect()
            })
            .collect();
        let apex_loops = loops
            .iter()
            .filter(|l| !l.contains(&CurveKind::Circle))
            .count();
        assert!(
            loops.len() == 2
                && loops.iter().all(|l| {
                    l.contains(&CurveKind::Ellipse)
                        && l.iter().filter(|c| **c == CurveKind::Line).count() == rulings
                })
                && apex_loops == if side == "below" { 2 } else { 0 },
            "{what}: two half faces, each an ellipse and two rulings, through the apex \
             below the cut and to the rim above it: {loops:?}"
        );
        assert_eq!(
            nappes(half),
            vec![1.0; 2],
            "{what}: both faces on the v > 0 nappe"
        );
        assert_at_every(&what, half, &[2e-2, 1.5e-2, 5e-3, 1e-3, 2e-4], &Got::Meshed);
    }
}

/// **The nappe each cone face lies on**: the sign of `(p − apex)·axis`
/// at its loop's vertices off the apex, `+1` where the chart's `v > 0`.
/// A face with vertices on both is reported `0`.
fn nappes(body: &Body<f64>) -> Vec<f64> {
    let points: std::collections::HashMap<_, _> = body.vertex_points().collect();
    body.faces()
        .filter_map(|(fk, face)| {
            let geom::Surface::Cone { apex, axis, .. } = *body.face_surface(fk)? else {
                return None;
            };
            let topo::LoopBoundary::Cycle { first } = body.get_loop(face.outer)?.boundary else {
                return None;
            };
            let heights: Vec<f64> = body
                .loop_cycle(first)?
                .into_iter()
                .map(|h| {
                    (points[&body.get_half_edge(h).expect("a half-edge").start] - apex).dot(axis)
                })
                .filter(|&h| h != 0.0)
                .collect();
            Some(if heights.iter().all(|&h| h > 0.0) {
                1.0
            } else if heights.iter().all(|&h| h < 0.0) {
                -1.0
            } else {
                0.0
            })
        })
        .collect()
}

/// **A cone face through its apex meshes at every scale and pose, and
/// on either nappe.** The trimmed lane triangulates a cone face in its
/// development (`mesh::trimmed`, "The development"), where the apex is
/// one point and a ruling a ray from it, so no triangle lies along a
/// ruling ([`geometry`]'s shape bound reds one), and the lattice, the
/// certificates and the fold census read a chart whose steps are the
/// row's at any scale. Each body here refused before that:
///
/// - the funnel of [`a_cone_face_through_its_apex_meshes_on_the_cone_row`]
///   scaled by `10⁻²`, `10²`, `300` and `10³`, translated far from the
///   origin, and turned and translated, each half at `δ` of `5·10⁻³`,
///   `10⁻³` and `2·10⁻⁴` of its size;
/// - a needle funnel, `(0, 0)`, `(0.02, 1)`, `(0, 1)`, at `δ = 10⁻³` and
///   `2·10⁻⁴`, whose triangles are the lattice's thinnest;
/// - a shifted funnel, `(0, 133.3)`, `(612.3, 911.1)`, `(0, 911.1)`, at
///   `δ = 0.18`, cut at `y = 533`;
/// - two funnels apex to apex, unioned: the lower one's faces lie on
///   their surface's `v < 0` nappe and run through its apex.
#[test]
fn a_cone_face_through_its_apex_meshes_at_every_scale_and_pose() {
    let n = Vec3::new(0.3f64.sin(), 0.3f64.cos(), 0.0);
    let turn = Affine3::rotation_about_axis(
        Point3::new(0.3, 0.1, 0.2),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    );
    let far = Affine3::translation(Vec3::new(700.3, -577.7, 123.4));
    let unit = |k: f64| vec![(0.0, 0.0), (k, k), (0.0, k)];
    // A revolve the kernel refuses at the run's ε (at ε = 1e-12 the cone
    // certificate of the ×300, ×10³ and shifted funnels is past its
    // limit, `KernelLimit`, at their extent) is a
    // typed door refusal, printed and skipped, as `issue303`'s rows do; at
    // the default ε every body builds.
    let build = |pts: &[(f64, f64)], pose: Option<Affine3<f64>>| {
        let raw = try_revolved(pts)?;
        Ok::<_, String>(match pose {
            Some(pose) => posed("the funnel", raw, pose),
            None => finished("the funnel", raw, tol()),
        })
    };
    // (name, body, a point of the cut, its normal, the δ it meshes at)
    type Case = (
        String,
        Result<AtRestBody<f64>, String>,
        Point3<f64>,
        Vec3<f64>,
        Vec<f64>,
    );
    let cases: Vec<Case> = [1e-2, 1e2, 3e2, 1e3]
        .into_iter()
        .map(|k| {
            (
                format!("the funnel ×{k:e}"),
                build(&unit(k), None),
                Point3::new(0.0, 0.5 * k, 0.0),
                n,
                vec![5e-3 * k, 1e-3 * k, 2e-4 * k],
            )
        })
        .chain(
            [("translated", far), ("turned and translated", far * turn)].map(|(name, pose)| {
                (
                    format!("the funnel, {name}"),
                    build(&unit(1.0), Some(pose)),
                    pose.transform_point(Point3::new(0.0, 0.5, 0.0)),
                    pose.transform_vec(n),
                    vec![5e-3, 1e-3, 2e-4],
                )
            }),
        )
        .chain([
            (
                "the needle funnel".to_string(),
                build(&[(0.0, 0.0), (0.02, 1.0), (0.0, 1.0)], None),
                Point3::new(0.0, 0.5, 0.0),
                n,
                vec![1e-3, 2e-4],
            ),
            (
                "the shifted funnel".to_string(),
                build(&[(0.0, 133.3), (612.3, 911.1), (0.0, 911.1)], None),
                Point3::new(0.0, 533.0, 0.0),
                n,
                vec![0.18],
            ),
        ])
        .collect();
    for (name, body, at, normal, deltas) in cases {
        let body = match body {
            Ok(body) => body,
            Err(e) => {
                assert!(
                    tol().eps() < geom_core::tolerance::DEFAULT_EPS && e.contains("KernelLimit"),
                    "{name}: the revolve refuses at ε {:e} other than at the cone \
                     certificate's kernel limit below the default ε: {e}",
                    tol().eps()
                );
                eprintln!(
                    "{name}: the revolve refuses at ε {:e}, skipped: {e}",
                    tol().eps()
                );
                continue;
            }
        };
        let plane = topo::test_support::split_plane(at, normal, tol());
        let split = topo::splitting::split(&body, &plane, tol())
            .unwrap_or_else(|e| panic!("{name} splits, got {e:?}"));
        for (side, part) in [("above", &split.above), ("below", &split.below)] {
            let what = format!("{name}, {side}");
            let half = part.body().unwrap_or_else(|| panic!("{what}: a body"));
            assert_at_every(&what, half, &deltas, &Got::Meshed);
        }
    }
    let up = finished("the upper funnel", revolved(&unit(1.0)), tol());
    let down = finished(
        "the lower funnel",
        revolved(&[(0.0, 0.0), (0.0, -1.0), (1.0, -1.0)]),
        tol(),
    );
    let union = topo::union(&up, &down, tol()).expect("the funnels unite at their apex");
    let union = &union.body().expect("a body").body;
    let mut on = nappes(union);
    on.sort_by(f64::total_cmp);
    assert_eq!(
        on,
        vec![-1.0, -1.0, 1.0, 1.0],
        "two faces on each nappe, none on both"
    );
    assert_at_every(
        "the funnels apex to apex",
        union,
        &[2e-2, 5e-3, 1e-3],
        &Got::Meshed,
    );
}

/// **A cone face ending near its apex meshes at every δ.** The funnel of
/// [`a_cone_face_through_its_apex_meshes_on_the_cone_row`] cut by the same
/// tilted plane through `(0, h, 0)`: the half above the cut has a section
/// ellipse within `h` of the apex, down to `h = 10⁻⁶`. Both halves mesh at
/// every δ from `2·10⁻²` to `5·10⁻⁵`, and the mesh is the body's
/// ([`geometry`]: within δ in truth, sampled).
///
/// Two things hold it. The cone's certificate is the flat triangle's own
/// deviation (`mesh`'s `cert::cert_cone`), so a triangle joining the
/// section's small end to a lattice row far round the axis is charged
/// what it sags, at its small radius; and the arm's rows are graded to
/// their own radius (`chart_table`'s `lattice_rows`), so the row next to
/// the small end stands a certified chord from it. Under rows uniform in
/// `v`, `h = 3·10⁻³` and `10⁻³` at `δ = 5·10⁻⁵` are truly past δ
/// (`1.63δ`, `1.23δ`) and refuse.
#[test]
fn a_cone_face_ending_near_its_apex_meshes_at_every_delta() {
    let n = Vec3::new(0.3f64.sin(), 0.3f64.cos(), 0.0);
    let deltas = [2e-2, 5e-3, 1e-3, 2e-4, 5e-5];
    for h in [0.02, 1e-2, 3e-3, 1e-3, 1e-4, 1e-6] {
        let body = finished(
            "the funnel",
            revolved(&[(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
            tol(),
        );
        let plane = topo::test_support::split_plane(Point3::new(0.0, h, 0.0), n, tol());
        let split = topo::splitting::split(&body, &plane, tol())
            .unwrap_or_else(|e| panic!("the funnel cut at {h:e} splits, got {e:?}"));
        for (side, part) in [("above", &split.above), ("below", &split.below)] {
            let what = format!("the funnel cut at h = {h:e}, {side}");
            let half = part.body().unwrap_or_else(|| panic!("{what}: a body"));
            assert_at_every(&what, half, &deltas, &Got::Meshed);
        }
    }
}

/// **A ball across a cylinder's wall refuses in the chord pass.** The
/// first pose of `sweep`'s `cylinder_sphere_frame` rows: the section is
/// two B-spline carriers, on a cylinder face and a sphere face that
/// store their `Pcurve::Projected` images. The sphere face is the
/// trimmed lane's sphere row's, and the first section edge the chord
/// pass reaches refuses there: the chord pass bounds a projected image's
/// speed for a conic's image only (`chords::chart_image`), before the
/// other edge's carrier is read. That other carrier has an interior knot
/// of full multiplicity, a C⁰ kink `chords::nurbs_chord_count` refuses.
/// The bound the conic-only rule keeps unread is eleven orders loose on
/// this image
/// (`work/tesslane/a-projected-spline-images-speed-bound-on-a-sphere-is-eleven-orders-loose.md`).
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
        assert_at_every(
            op,
            body,
            &[2e-2, 5e-3, 1e-3],
            &Got::ConicImageOnly(CurveKind::Nurbs),
        );
    }
}

// ---------------------------------------------------------------- rings

/// **A torus wall holding a spiric ring refuses at the curved lane's
/// ring guard.** The donut `(2, 0.5)` about `y` and the bar through its
/// tube (`sweep`'s `germ_plane_torus_spiric`, pose B2): the unions and
/// the donut less the bar keep the ovals as rings of torus faces whose
/// outer loops are iso circles, so `lane_of` sends them to the curved
/// lane, whose ring guard refuses. The other three ops put the spirics
/// on an outer loop, and the trimmed lane meshes them on the torus row.
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
    assert_six(
        "the donut and B2 (A the donut)",
        &finished("the torus", donut, tol()),
        &finished("B2", bar, tol()),
        &[2e-2, 5e-3, 1e-3],
        &[
            ring.clone(),
            ring.clone(),
            Got::Meshed,
            Got::Meshed,
            ring,
            Got::Meshed,
        ],
    );
}

/// **A ball face holed by a tilted tube's rim.** `sweep`'s
/// `a_tube_ending_on_a_ball` pose "tilted 1.2": the rim crosses neither
/// seam meridian and stays a ring of one half-band. Both unions and
/// ball ∖ tube refuse at the curved lane's ring guard (the ring's
/// projected circles are not on the outer loop, which is what
/// `trimmed::has_tilted_circle` reads).
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
            ring.clone(),
            ring.clone(),
            Got::Meshed,
            Got::Meshed,
            Got::Meshed,
            ring,
        ],
    );
}

/// **The `z`-poled ball meshes at every radius.**
/// `sweep::test_support::ball_poled_z` turns a revolved ball a quarter
/// turn about `x`, which leaves its chart axis at `(0, cos(π/2), 1)`
/// with `cos(π/2) = 6.1e−17` in floats. The seam meridian's column is
/// `atan2(p · v_ref, p · u_ref)` at the meridian's mid point, and
/// `p · v_ref` is the residue of an exact zero. At `r = 1` and `0.3`
/// it rounds to `0.0`; at `r = √2` and `0.1` it is a nonzero below
/// spade's coordinate floor (`u = −4.8e−49` at `√2`), which the crate's
/// CDT feed (`cdt::spade_point`) enters as `0.0`.
#[test]
fn a_z_poled_ball_of_radius_root_two() {
    let deltas = [2e-2, 5e-3];
    for r in [SQRT_2, 0.1, 1.0, 0.3] {
        assert_at_every(
            &format!("the z-poled ball, r = {r}"),
            &ball_poled_z(r, Vec3::zero(), tol()),
            &deltas,
            &Got::Meshed,
        );
    }
}

/// `body` meshes at `delta`, passes `check_mesh`, and its signed volume.
fn meshed_volume(what: &str, body: &Body<f64>, delta: f64) -> f64 {
    let m = mesh::tessellate(body, delta, tol())
        .unwrap_or_else(|e| panic!("{what}: meshes at δ = {delta:e}, got {e:?}"));
    mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{what}: check_mesh: {e:?}"));
    mesh::validate::signed_volume(&m)
}

/// **A ball, turned and placed every way a user turns one, meshes and
/// encloses the ball**, at about forty radii over `[0.07, 40]`. Every
/// mesh vertex lies on the sphere, so the mesh is an inscribed
/// polyhedron: its volume is at most `4/3·π·r³` and short of it by at
/// most the sphere's area times δ. `check_mesh` alone cannot see a
/// wrong region; the bracket can.
#[test]
fn a_turned_ball_meshes_its_whole_volume_at_every_radius() {
    use core::f64::consts::{FRAC_PI_3, PI};
    let n = 40;
    let rot =
        |axis: Vec3<f64>, angle: f64| Affine3::rotation_about_axis(Point3::origin(), axis, angle);
    let diagonal = Vec3::new(1.0, 1.0, 0.0) * (1.0 / SQRT_2);
    for k in 0..n {
        #[allow(clippy::cast_precision_loss)]
        let r = 0.07 * (40.0f64 / 0.07).powf(k as f64 / (n - 1) as f64);
        let delta = r / 50.0;
        let z = ball_poled_z(r, Vec3::zero(), tol());
        let poses: [(&str, Body<f64>); 8] = [
            ("poled z", z.clone()),
            (
                "poled x",
                sweep::test_support::ball_poled(r, Vec3::zero(), Vec3::unit_x(), tol()),
            ),
            (
                "then ¼ turn about y",
                (*posed("y", z.clone(), rot(Vec3::unit_y(), FRAC_PI_2))).clone(),
            ),
            (
                "then ¼ turn about x",
                (*posed("x", z.clone(), rot(Vec3::unit_x(), FRAC_PI_2))).clone(),
            ),
            (
                "then π/3 about z",
                (*posed("π/3", z.clone(), rot(Vec3::unit_z(), FRAC_PI_3))).clone(),
            ),
            (
                "then 3π/2 about z",
                (*posed("3π/2", z.clone(), rot(Vec3::unit_z(), 1.5 * PI))).clone(),
            ),
            (
                "then ¼ turn about (1,1,0)/√2",
                (*posed("diag", z.clone(), rot(diagonal, FRAC_PI_2))).clone(),
            ),
            (
                "off the origin",
                ball_poled_z(r, Vec3::new(0.3, -1.7, 2.9), tol()),
            ),
        ];
        let ball = 4.0 / 3.0 * PI * r.powi(3);
        let slack = 4.0 * PI * r * r * delta;
        for (pose, body) in &poses {
            let what = format!("ball r = {r}, {pose}");
            let v = meshed_volume(&what, body, delta);
            assert!(
                v <= ball && v >= ball - slack,
                "{what}: volume {v} outside [{}, {ball}]",
                ball - slack
            );
        }
    }
}

/// **A slab cut by a ball whose chart axis is an ulp off the cut's
/// normal** (`ball_poled_z`'s pole is `(0, 6.1e−17, 1)` against the
/// slab's `z` face) meshes, and its volume is the slab's less the cap,
/// `π·h²·(3r − h)/3`, within the cap's area times δ: every vertex of
/// the cavity lies on the sphere or on the cut circle, so the cavity
/// mesh is inscribed in the cap.
#[test]
fn a_slab_cut_by_an_ulp_tilted_ball_meshes_its_volume() {
    use core::f64::consts::PI;
    let slab = finished(
        "the slab",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let (r, h, delta) = (0.09, 0.05, 1e-3);
    let ball = finished(
        "the ball",
        ball_poled_z(r, Vec3::new(0.5, 0.5, 1.0 + r - h), tol()),
        tol(),
    );
    let cut = topo::subtract(&slab, &ball, tol()).expect("the cut builds");
    let body = &cut.body().expect("a body").body;
    let v = meshed_volume("the slab less the cap", body, delta);
    let cap = PI * h * h * (3.0 * r - h) / 3.0;
    let slack = 2.0 * PI * r * h * delta;
    assert!(
        v >= 1.0 - cap && v <= 1.0 - cap + slack,
        "volume {v} outside [{}, {}]",
        1.0 - cap,
        1.0 - cap + slack
    );
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
/// outer loop by an ellipse, and the trimmed lane meshes them on the cone
/// row.
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
    assert_six(
        "T1 (A the cone)",
        &cone,
        &wedge,
        &[2e-2, 5e-3, 1e-3],
        &[
            ring.clone(),
            ring.clone(),
            Got::Meshed,
            Got::Meshed,
            ring,
            Got::Meshed,
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
