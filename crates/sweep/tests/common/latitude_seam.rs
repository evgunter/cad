//! **The same-surface latitude-seam fixtures** and the readers their
//! rows share. Two bodies: the drum whose top cap carries a latitude
//! ring (one plane in three faces, the ring between them — its two
//! half-circles are the plane's only `Chart` images with a non-zero `v`
//! channel), and the sphere authored as two
//! cocircular arcs (one sphere in four faces, a seam at `v = π/4`, cut
//! by hand as the revolve builds the arc run whole).
//! With them: the axial door's cavity of either, the meter the void
//! door's graft runs over a body, the evidence `shell` hands that
//! door, and a body's plane-chart images.
//!
//! Body authoring routes here ([`super`]'s rule). The three readers
//! ride with the bodies rather than beside [`super::orient`] because
//! none evaluates a surface — each reads a stored description, or
//! runs the kernel's own meter over one — and every suite that builds
//! one of these bodies reads it through them: `shell9_probe`,
//! `shell9_r1_probes`, `shell9_r2_probes`, `shell9_r2_dump` and
//! `revert_plane_charts`. The unit suites and their review probes
//! build THE SAME BODY or their rows are not about each other
//! ([`super::cavity`]'s rule), which is what a shared fixture buys.
//!
//! **Deliberately not absorbed**, and the whole of it:
//!
//! - `shell9_r1_probes::multi_arc_sphere` and its `two_arc_sphere`
//!   — the reviewer's own derivation of the same body from a bulge
//!   computed off the arc's geometry, kept apart under
//!   [`super::oracles`]'s rule (an independent derivation is what a
//!   probe is for), with the torus family beside it;
//! - `shell7_seam_corner`'s inline drum, whose `(r, h, t)` are that
//!   row's own closed-form inputs and read beside its oracle.

use geom::{Curve3, Surface};
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, Pcurve};
use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{corners, revolved_about_y};
use topo::readback::vertex_point;
use topo::{
    Body, EdgeKey, FaceKey, FaceSurface, HalfEdgeKey, MefSite, MevSite, SurfaceKey, VertexKey,
    VoidContainment, VoidEvidence,
};

use super::charts::hollow_moves;

/// The drum's radius.
pub const DRUM_R: f64 = 1.0;
/// The drum's height.
pub const DRUM_H: f64 = 2.0;

/// The collinear-cap drum: a cylinder of radius [`DRUM_R`] and height
/// [`DRUM_H`] whose top cap carries a latitude ring at `r/2`, so the
/// cap is one plane in three faces (two half-annuli and the inner
/// disc), the ring's two half-circles and two radial lines between
/// them `Chart` images on that plane.
///
/// The revolve builds the cap whole — a station inside a run has no
/// entity in a full revolve (`crates/sweep/README.md`, "Walls: one per
/// run") — so the ring is cut by hand through the Euler door
/// ([`ring_on_cap`]), the same plane in more faces.
pub fn collinear_cap_drum() -> Body<f64> {
    let mut body = revolved_about_y(
        corners(&[(0.0, 0.0), (DRUM_R, 0.0), (DRUM_R, DRUM_H), (0.0, DRUM_H)]),
        Revolution::Full,
        Tol::witness(),
    );
    ring_on_cap(&mut body, DRUM_H, DRUM_R, DRUM_R / 2.0);
    body
}

/// The vertex of `body` at `p`.
fn vertex_at(body: &Body<f64>, p: Point3<f64>) -> VertexKey {
    body.vertices()
        .find(|(_, v)| {
            body.get_point(v.point)
                .is_some_and(|q| q.distance(p) <= 1e-12)
        })
        .unwrap_or_else(|| panic!("no vertex at {p:?}"))
        .0
}

/// The half-edge of `face` leaving `v`.
fn leaving(body: &Body<f64>, face: FaceKey, v: VertexKey) -> HalfEdgeKey {
    body.half_edges()
        .find(|(h, he)| he.start == v && body.face_of_half_edge(*h) == Some(face))
        .unwrap_or_else(|| panic!("{face:?} has no half-edge leaving {v:?}"))
        .0
}

/// The `z` sign side of a wall face about `+y` that crosses height
/// `h` at radius `rr`: which of `(0, h, ±rr)` it holds.
fn wall_side(body: &Body<f64>, face: FaceKey, h: f64, rr: f64) -> f64 {
    let band = geom_core::Band::linear(Tol::witness()).expect("band");
    let held = |z: f64| {
        topo::curved_face_containment(body, face, Point3::new(0.0, h, z), band).unwrap()
            == Some(topo::FaceContainment::In)
    };
    match (held(rr), held(-rr)) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        other => panic!("{face:?} holds (0, {h}, ±{rr}) as {other:?}, not one side"),
    }
}

/// A half-circle of radius `rr` about the `y` axis at height `h`, from
/// angle 0 (`+x`) to angle π, on the side `z` takes the sign `side`,
/// at rest in the chart of `surface`.
fn half_circle(h: f64, rr: f64, side: f64, surface: SurfaceKey) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart(surface),
        carrier: Curve3::Circle {
            center: Point3::new(0.0, h, 0.0),
            // u × (−y·side) = side·z for u = +x.
            axis: Vec3::new(0.0, -side, 0.0),
            radius: rr,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        param_start: 0.0,
        param_end: core::f64::consts::PI,
    }
}

/// A line from `p` to `q` at rest in the chart of `surface`.
fn chart_line(p: Point3<f64>, q: Point3<f64>, surface: SurfaceKey) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::chart(surface),
        ..EdgeCurveSpec::line_between(p, q)
    }
}

/// Cuts a latitude ring of radius `rr` into a full revolve's whole cap
/// at height `h` (rim radius `r`, about `+y`): a radial line from each
/// rim vertex in to the ring and the ring's two half-circles, every
/// edge a `Chart` image on the cap's plane and every new face on it.
pub fn ring_on_cap(body: &mut Body<f64>, h: f64, r: f64, rr: f64) {
    let tol = Tol::witness();
    let cap = body
        .faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - h).abs() <= 1e-12)
        })
        .expect("the cap")
        .0;
    let plane = body.get_face(cap).unwrap().surface;
    let (rim_a, rim_b) = (Point3::new(r, h, 0.0), Point3::new(-r, h, 0.0));
    let (a, b) = (Point3::new(rr, h, 0.0), Point3::new(-rr, h, 0.0));
    let (va, vb) = (vertex_at(body, rim_a), vertex_at(body, rim_b));
    let from_a = leaving(body, cap, va);
    let side = {
        let e = body
            .get_edge(body.get_half_edge(from_a).unwrap().edge)
            .unwrap();
        let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        c.mid_point().z.signum()
    };
    let spoke = body
        .mev(
            MevSite::Fan {
                he1: from_a,
                he2: from_a,
            },
            a,
            chart_line(rim_a, a, plane),
            tol,
        )
        .expect("the radial line in from the rim");
    let ring = body
        .mev(
            MevSite::Fan {
                he1: spoke.he_minus,
                he2: spoke.he_minus,
            },
            b,
            half_circle(h, rr, side, plane),
            tol,
        )
        .expect("the ring's first half");
    let from_b = leaving(body, cap, vb);
    body.mef(
        MefSite::Chords {
            he1: ring.he_minus,
            he2: from_b,
        },
        chart_line(b, rim_b, plane),
        FaceSurface::Inherit,
        tol,
    )
    .expect("the radial line out to the rim");
    let inner = body.face_of_half_edge(ring.he_plus).unwrap();
    let to_rim = leaving(body, inner, ring.vertex);
    body.mef(
        MefSite::Chords {
            he1: ring.he_plus,
            he2: to_rim,
        },
        half_circle(h, rr, -side, plane),
        FaceSurface::Inherit,
        tol,
    )
    .expect("the ring's second half");
}

/// Cuts a latitude ring at height `h` into a full revolve's curved wall
/// on `surface` (a cylinder or a cone about `+y`, split by its two
/// line meridians into half-bands): each meridian split at its middle,
/// and each half-band split there by a half-circle `Chart` image on the
/// wall, so the wall is one surface in four faces.
pub fn ring_on_wall(body: &mut Body<f64>, surface: SurfaceKey) {
    ring_on_wall_where(body, surface, |_, t0, t1| Some(0.5 * (t0 + t1)), true);
}

/// [`ring_on_wall`] at height `y`, on any wall whose meridians climb
/// monotonically in `y` (a cylinder, a cone, a sphere's pole-to-pole
/// half-bands), and again on a wall a ring already cut: the two
/// meridian pieces that cross `y` split there, and the two faces that
/// then hold both new vertices split between them.
pub fn ring_on_wall_at(body: &mut Body<f64>, surface: SurfaceKey, y: f64) {
    ring_on_wall_where(body, surface, crossing(y), true);
}

/// A meridian piece's split parameter at height `y`, bisected on its
/// carrier; `None` for a piece that does not cross `y`.
fn crossing(y: f64) -> impl Fn(&Curve3<f64>, f64, f64) -> Option<f64> {
    move |c, mut lo, mut hi| {
        let (y_lo, y_hi) = (c.eval(lo).y, c.eval(hi).y);
        if (y_lo - y) * (y_hi - y) >= 0.0 {
            return None;
        }
        let rising = y_hi > y_lo;
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if (c.eval(mid).y < y) == rising {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Some(0.5 * (lo + hi))
    }
}

/// The shared cut: `at(carrier, t0, t1)` picks each meridian's split
/// parameter, `None` for a meridian piece the ring does not cross;
/// with `halves`, each face then holding both new vertices splits
/// between them by a half-circle `Chart` image.
fn ring_on_wall_where(
    body: &mut Body<f64>,
    surface: SurfaceKey,
    at: impl Fn(&Curve3<f64>, f64, f64) -> Option<f64>,
    halves: bool,
) {
    let tol = Tol::witness();
    let on_wall = |b: &Body<f64>, he: HalfEdgeKey| {
        b.face_of_half_edge(he)
            .is_some_and(|f| b.get_face(f).unwrap().surface == surface)
    };
    let meridians: Vec<(EdgeKey, f64)> = body
        .edges()
        .filter(|(_, e)| on_wall(body, e.he_plus) && on_wall(body, e.he_minus))
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let (t0, t1) = c.params();
            at(c.carrier(), t0, t1).map(|t| (k, t))
        })
        .collect();
    assert_eq!(meridians.len(), 2, "the ring crosses two meridians");
    let mut mids = Vec::new();
    for (m, t) in meridians {
        mids.push(
            body.split_edge(m, t, tol)
                .expect("the meridian splits")
                .vertex,
        );
    }
    if !halves {
        return;
    }
    let point = |b: &Body<f64>, v: VertexKey| *b.get_point(b.get_vertex(v).unwrap().point).unwrap();
    // Angle 0 (`+x`) first.
    if point(body, mids[0]).x < point(body, mids[1]).x {
        mids.swap(0, 1);
    }
    let (p0, h) = (point(body, mids[0]), point(body, mids[0]).y);
    let holds = |b: &Body<f64>, f: FaceKey, v: VertexKey| {
        b.half_edges()
            .any(|(h, he)| he.start == v && b.face_of_half_edge(h) == Some(f))
    };
    let bands: Vec<FaceKey> = body
        .faces()
        .filter(|(k, f)| {
            f.surface == surface && holds(body, *k, mids[0]) && holds(body, *k, mids[1])
        })
        .map(|(k, _)| k)
        .collect();
    for band in bands {
        let side = wall_side(body, band, h, p0.x);
        body.mef(
            MefSite::Chords {
                he1: leaving(body, band, mids[0]),
                he2: leaving(body, band, mids[1]),
            },
            half_circle(h, p0.x, side, surface),
            FaceSurface::Inherit,
            tol,
        )
        .expect("the half-band splits at the ring");
    }
}

/// The unit sphere authored as two cocircular arcs meeting at the
/// latitude `π/4`: one sphere in four faces, a same-surface seam. The
/// revolve builds the arc run as one wall (`crates/sweep/README.md`,
/// "Walls: one per run"), so the seam is cut through the Euler door as
/// the revolve declares a latitude rim ([`latitude_on_full_wall`]).
pub fn two_arc_sphere() -> Body<f64> {
    use core::f64::consts::{FRAC_PI_2, PI};
    let r = 1.0;
    let v = PI / 4.0;
    let (s, c) = v.sin_cos();
    let station = Point2::new(r * c, r * s);
    let mut body = revolved_about_y(
        vec![
            (Point2::new(0.0, -r), ((FRAC_PI_2 + v) / 4.0).tan()),
            (station, ((FRAC_PI_2 - v) / 4.0).tan()),
            (Point2::new(0.0, r), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    let sphere = body
        .faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .expect("the sphere")
        .1
        .surface;
    latitude_on_full_wall(&mut body, sphere, station);
    body
}

/// The axial door's cavity of `body` at wall thickness `t` — every
/// chart moved inward through `offset_charts_together` — asserted
/// tier-3 valid, which is the door's own contract and the premise of
/// every row that reverts or grafts the cavity.
pub fn door_cavity(body: &Body<f64>, t: f64) -> Body<f64> {
    let mut cavity = body.clone();
    let band = geom_core::Band::linear(Tol::witness()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(body, t), band, Tol::witness())
        .expect("the door takes it");
    assert_eq!(
        topo::validate_geometric(&cavity, Tol::witness()),
        Ok(()),
        "cavity tier 3"
    );
    cavity
}

/// Every edge of `body` re-certified exactly as the void door's graft
/// does (`combine.rs`'s recertify arm: the description with its image
/// verbatim, carrier and interval verbatim, endpoints from `he_plus`,
/// surfaces from the body itself), and the refusals. A `Scaffold`
/// edge is skipped, as the graft skips it.
///
/// A MIRROR of that arm, and it can drift from it: if the graft's
/// meter changes, this reads a different meter. The rows that also
/// call `insert_voids` on the same cavity (the door's own verdict)
/// are what catch a drift — this reader's refusals would disagree
/// with a door that no longer refuses, or refuses more.
pub fn graft_recertify_failures(body: &Body<f64>) -> Vec<(EdgeKey, CertifyError)> {
    let band = geom_core::Band::linear(Tol::witness()).expect("band");
    body.edges()
        .filter_map(|(ek, e)| {
            let curve = body.get_curve_geom(e.curve)?.certified()?;
            if matches!(curve.description(), geom_brep::EdgeDescription::Scaffold(_)) {
                return None;
            }
            let start_v = body.get_half_edge(e.he_plus)?.start;
            let end_v = body.half_edge_end(e.he_plus)?;
            EdgeCurve::certify(
                curve.restated_spec(),
                vertex_point(body, start_v).expect("a live vertex"),
                vertex_point(body, end_v).expect("a live vertex"),
                |sk| body.get_surface(sk).cloned(),
                band,
            )
            .err()
            .map(|err| (ek, err))
        })
        .collect()
}

/// The evidence `shell` hands the void door — every cavity shell
/// `Carried { Positive }` (`shell.rs`, "The evidence"). A restatement,
/// and it can drift: a door that starts demanding a different
/// certificate refuses these rows at `insert_voids` while `shell`'s
/// own rows (`shell7_seam_corner`) keep passing, which is the signal.
pub fn void_evidence(cavity: &Body<f64>) -> VoidEvidence {
    VoidEvidence {
        shells: cavity
            .shells()
            .map(|(k, _)| {
                (
                    k,
                    VoidContainment::Carried {
                        sign: geom_core::Sign::Positive,
                    },
                )
            })
            .collect(),
    }
}

/// Every edge described as a `Chart` image on a PLANE, with its image,
/// in the body's edge order.
pub fn plane_images(body: &Body<f64>) -> Vec<(EdgeKey, Pcurve<f64>)> {
    body.edges()
        .filter_map(|(k, e)| {
            let curve = body.get_curve_geom(e.curve)?.certified()?;
            let c = curve.description().chart()?;
            matches!(body.get_surface(c.surface), Some(Surface::Plane { .. }))
                .then(|| (k, c.pcurve.clone()))
        })
        .collect()
}

/// `station` (sketch plane `xy`) turned by `a` about `+y`.
fn turned(station: Point2<f64>, a: f64) -> Point3<f64> {
    let (sin, cos) = a.sin_cos();
    Point3::new(station.x * cos, station.y, -station.x * sin)
}

/// Cuts the latitude arc a revolve about `+y` (sketch plane `xy`)
/// strikes at `station` from azimuth `from` through `theta`, between
/// two vertices already at its ends inside one wall face on `surface`
/// (the face holding the arc's midpoint): the revolve's own
/// `RevolvedPoint` declaration, then at rest in the wall's chart.
fn latitude_arc(
    body: &mut Body<f64>,
    surface: SurfaceKey,
    station: Point2<f64>,
    from: f64,
    theta: f64,
) -> EdgeKey {
    let tol = Tol::witness();
    let (va, vb) = (
        vertex_at(body, turned(station, from)),
        vertex_at(body, turned(station, from + theta)),
    );
    let band = geom_core::Band::linear(tol).expect("band");
    let mid = turned(station, from + theta / 2.0);
    let face = body
        .faces()
        .find(|(k, f)| {
            f.surface == surface
                && topo::curved_face_containment(body, *k, mid, band).unwrap()
                    == Some(topo::FaceContainment::In)
        })
        .expect("a wall face holds the latitude's midpoint")
        .0;
    let (he1, he2) = (leaving(body, face, va), leaving(body, face, vb));
    let (sin, cos) = from.sin_cos();
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Scaffold(geom_brep::MappedCurve::RevolvedPoint {
            point: station,
            place: geom_core::Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                from,
            ),
            axis_origin: Point3::new(0.0, 0.0, 0.0),
            axis_dir: Vec3::new(0.0, 1.0, 0.0),
            angle: theta,
        }),
        carrier: Curve3::Circle {
            center: Point3::new(0.0, station.y, 0.0),
            // The rotation's own sense: `+y × +x = −z`, the way a
            // positive turn about `+y` carries `+x`.
            axis: Vec3::new(0.0, theta.signum(), 0.0),
            radius: station.x,
            u_ref: Vec3::new(cos, 0.0, -sin),
        },
        param_start: 0.0,
        param_end: theta.abs(),
    };
    let edge = body
        .mef(
            MefSite::Chords { he1, he2 },
            spec,
            FaceSurface::Inherit,
            tol,
        )
        .expect("the latitude splits the wall")
        .edge;
    body.describe_at_rest(edge, surface, tol)
        .expect("the latitude rests in the wall's chart");
    edge
}

/// Cuts the latitude a full revolve about `+y` (sketch plane `xy`)
/// would have struck at `station` into its curved wall on `surface`,
/// split by its two meridians into π-bands: each meridian split at the
/// station, then one half-period [`latitude_arc`] per band, as the
/// revolve declares its two rims.
pub fn latitude_on_full_wall(body: &mut Body<f64>, surface: SurfaceKey, station: Point2<f64>) {
    ring_on_wall_where(body, surface, crossing(station.y), false);
    let pi = core::f64::consts::PI;
    latitude_arc(body, surface, station, 0.0, pi);
    latitude_arc(body, surface, station, pi, pi);
}
