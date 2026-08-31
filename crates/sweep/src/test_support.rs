//! Test-support vocabulary: fixtures this crate's tests build on,
//! kept in one place so no suite mints its own copy.
//!
//! # The gate (stated once for this crate)
//!
//! The module declaration in `lib.rs` carries
//! `#[cfg(any(test, feature = "test-support"))]`. Both arms are
//! consumers that must be able to name these items:
//!
//! - **`test`** — the in-crate `mod tests` pins. (Cargo unifies the
//!   self dev-dependency's features into that build too, so this arm is
//!   belt-and-braces: it keeps the gate true of any in-crate test build
//!   without depending on how features resolve.) The sites several of
//!   them cover are private to their modules, so those pins cannot
//!   live in `tests/`; hosting the fixtures here rather than inside
//!   one of the test modules that uses them keeps neither module the
//!   owner of the other's fixture.
//! - **`feature = "test-support"`** — the `tests/` binaries that build
//!   on these fixtures. A `tests/` file is a **separate crate** that
//!   links the library as an ordinary dependency, so it can name
//!   neither a `#[cfg(test)]` item (that cfg is off when the library is
//!   built as a dependency) nor a `pub(crate)` one. `cfg(test)` alone
//!   therefore cannot serve as this module's gate: it is exactly what
//!   made six integration suites each declare their own `cube` (S52).
//!   The feature is off by default and turned on only from
//!   **`[dev-dependencies]`** — this crate's self dev-dependency
//!   (`sweep = { path = ".", features = ["test-support"] }`), and the
//!   same spelling in `mesh` and `step-export`, whose suites meter the
//!   [`swept_elbow`] this crate builds. So it is on exactly when some
//!   crate's TESTS compile the library, and off for every non-test
//!   build of every dependent.
//!
//!   A fixture only earns a place here once a consumer OUTSIDE this
//!   crate needs it or a second suite inside it does; the narrower
//!   homes, and the rule that routes between them, are stated in
//!   `sweep`'s own `tests/common` module.
//!
//! Existence and visibility coincide here, so one gate states both:
//! nothing in this module has a non-test consumer, unlike `topo`'s
//! `test_support_impl`, whose `ArenaCounts` the debug postcondition
//! also needs and which is therefore compiled wider than it is exported.
//!
//! Neither arm is satisfied by `cargo build [--release]`, so this
//! module is absent from every shipped build. `cargo test --release`
//! satisfies `test` — which is why `cfg(debug_assertions)` cannot
//! serve as the gate either.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom::NurbsCurve3;
use geom_core::{Affine3, Band, Point2, Point3, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{Body, EdgeKey};

use crate::blend::battery::{BlendRequest, Link, run_battery};
use crate::skin::{Section, segment_curve};
use crate::{Extrusion, Lofted, SketchSegment, extrude, sweep_body};
use geom_core::Tol;

/// The cube side the in-crate pins build on, meters.
pub const L: f64 = 1.0;
/// The blend radius, meters.
pub const R: f64 = 0.1;

/// An axis-aligned cube of side `l` with a corner at the origin:
/// eight trivalent corners, every one of them geometrically CONVEX.
pub fn cube(l: f64, tol: Tol) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(0.0, 0.0), (l, 0.0), (l, l), (0.0, l)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    extrude(&profile, Extrusion::Distance(l), tol).unwrap().body
}

/// Every edge of `body` resolved by the fillet battery, in edge
/// order.
pub fn all_links(body: &Body<f64>, tol: Tol) -> Vec<Link<f64>> {
    let tol = tol.get();
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let verdict = run_battery(
        &BlendRequest {
            body,
            edges,
            size: R,
        },
        Band::new(tol.eps, tol.k * tol.eps).unwrap(),
    )
    .expect("the battery resolves every edge of a cube");
    let mut links: Vec<Link<f64>> = verdict
        .chains
        .iter()
        .flat_map(|c| c.links().cloned())
        .collect();
    links.sort_by_key(|l| l.edge);
    links
}

/// A closed sketch loop revolved about the sketch **y-axis** — the one
/// home for the revolve fixtures the rim suites build on. Five suites
/// each carried a byte-identical copy of this before the fix pass; that
/// is the S52 shape the module header names, and the copies drift.
pub fn revolved_about_y(
    verts: Vec<ProfileVertex<f64>>,
    rev: crate::Revolution<f64>,
    tol: Tol,
) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![ProfileLoop::new(verts)])
        .validate(tol)
        .unwrap();
    let axis = crate::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    crate::revolve(&profile, axis, rev, tol).unwrap().body
}

/// **The dome**: a sphere zone of radius `r` from the equator up 45°,
/// on a flat base annulus, bored on-axis at `r/2` so the profile stays
/// ANNULAR — which is what makes the full revolve mint one wall per
/// profile segment and CLOSED latitude rims. Its equator is the
/// canonical one-edge closed plane–sphere rim.
pub fn dome(r: f64, tol: Tol) -> Body<f64> {
    revolved_about_y(dome_profile(r), crate::Revolution::Full, tol)
}

/// [`dome`]'s profile, so a suite can revolve it PARTIALLY for the
/// differential pair.
pub fn dome_profile(r: f64) -> Vec<ProfileVertex<f64>> {
    let a45 = core::f64::consts::FRAC_1_SQRT_2;
    let bulge = (core::f64::consts::FRAC_PI_4 / 4.0).tan();
    vec![
        ProfileVertex::new(Point2::new(0.5 * r, 0.0), 0.0),
        ProfileVertex::new(Point2::new(r, 0.0), bulge),
        ProfileVertex::new(Point2::new(r * a45, r * a45), 0.0),
        ProfileVertex::new(Point2::new(0.5 * r, r * a45), 0.0),
    ]
}

/// The one CLOSED plane–sphere rim of `body` whose circle carrier has
/// radius `rim_r` (to 1e-6). Selection is by the analytically known
/// radius, not by uniqueness: the dome carries two such rims.
///
/// # Panics
///
/// If the body does not carry exactly one.
pub fn closed_plane_sphere_rim(body: &Body<f64>, rim_r: f64) -> EdgeKey {
    let hits: Vec<EdgeKey> = body
        .edges()
        .filter_map(|(k, e)| {
            let start = body.get_half_edge(e.he_plus)?.start;
            if Some(start) != body.half_edge_end(e.he_plus) {
                return None;
            }
            let surf = |he| -> Option<geom::Surface<f64>> {
                let l = body.get_half_edge(he)?.parent_loop;
                let f = body.get_loop(l)?.face;
                body.get_surface(body.get_face(f)?.surface).cloned()
            };
            let (a, b) = (surf(e.he_plus)?, surf(e.he_minus)?);
            let ps = |x: &geom::Surface<f64>, y: &geom::Surface<f64>| {
                matches!(x, geom::Surface::Plane { .. })
                    && matches!(y, geom::Surface::Sphere { .. })
            };
            if !(ps(&a, &b) || ps(&b, &a)) {
                return None;
            }
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match *c.carrier() {
                geom::Curve3::Circle { radius, .. } if (radius - rim_r).abs() < 1e-6 => Some(k),
                _ => None,
            }
        })
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "exactly one closed plane–sphere rim of radius {rim_r}"
    );
    hits[0]
}

/// **Every arc of the latitude rim at radius `rim_r` and station
/// `rim_y`**, in key order — the selector four suites had each
/// hand-rolled a copy of.
///
/// A rim a chart seam has SPLIT is several edges, and the fillet verbs
/// take exactly its set: adding one edge more refuses `TangentialEdge`
/// at margin zero, one edge fewer stops at a seam vertex. So the scan
/// has two halves and the second is the one that is easy to omit:
///
/// 1. circular carriers on the given radius and centre station, and
/// 2. **only those whose two supports are DIFFERENT surfaces**. A
///    sphere's seam meridian is a great circle that can share a rim's
///    radius and centre exactly, so a radius scan alone returns the
///    chart seams too — and a request carrying one of those refuses on
///    the co-surface tangency before any rim door is reached.
///
/// Comparison is against a fixed `1e-9`: fixtures state their rims
/// analytically, so this is a fixture-selection tolerance and not a
/// kernel predicate. There is no PUBLIC door for this yet (the kernel
/// offers no "give me this rim's arcs" selector; that gap is
/// evgunter/cad issue 1246, filed on two independent consumer reports),
/// which is exactly why the test-side copy is homed here rather than
/// left in four suites.
#[must_use]
pub fn rim_arcs_at(body: &Body<f64>, rim_r: f64, rim_y: f64) -> Vec<EdgeKey> {
    let surface_of = |he| -> Option<topo::SurfaceKey> {
        let l = body.get_half_edge(he)?.parent_loop;
        Some(body.get_face(body.get_loop(l)?.face)?.surface)
    };
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            let geom::Curve3::Circle { radius, center, .. } = *c.carrier() else {
                return None;
            };
            if (radius - rim_r).abs() >= 1e-9 || (center.y - rim_y).abs() >= 1e-9 {
                return None;
            }
            (surface_of(e.he_plus)? != surface_of(e.he_minus)?).then_some(k)
        })
        .collect()
}

/// **The #935 zone**: a sphere zone off the equator — sphere `R = 2`
/// about the origin, sliced at `y = −0.5` and `y = 1`, bored on-axis
/// at `bore` — the body issue 935 was filed on. Annular, so the full
/// revolve mints four walls and every latitude rim as ONE closed edge;
/// its two sphere rims share the sphere wall, its cap rims the caps.
///
/// Its rims, for [`rim_arcs_at`]: sphere-lo `(√3.75, −0.5)`, sphere-hi
/// `(√3, 1)`, bore-lo `(bore, −0.5)`, bore-hi `(bore, 1)`.
pub fn sphere_zone(bore: f64, rev: crate::Revolution<f64>, tol: Tol) -> Body<f64> {
    let big_r = 2.0f64;
    let (y_lo, y_hi) = (-0.5f64, 1.0f64);
    let x_lo = (big_r.powi(2) - y_lo.powi(2)).sqrt();
    let x_hi = (big_r.powi(2) - y_hi.powi(2)).sqrt();
    let th_lo = (y_lo / big_r).asin();
    let th_hi = (y_hi / big_r).asin();
    let bulge = ((th_hi - th_lo) / 4.0).tan();
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(bore, y_lo), 0.0),
            ProfileVertex::new(Point2::new(x_lo, y_lo), bulge),
            ProfileVertex::new(Point2::new(x_hi, y_hi), 0.0),
            ProfileVertex::new(Point2::new(bore, y_hi), 0.0),
        ],
        rev,
        tol,
    )
}

/// **The BLEND-1 lantern**: a pole-touching solid of revolution —
/// base disk, unit-sphere belly to the 3-4-5 shoulder `(0.8, 0.6)`,
/// cone to the lip `(0.2, 1.2)`, top disk — so every wall is a pair
/// of half-bands and every latitude rim a pair of arcs meeting at
/// chart-seam vertices.
///
/// Its rims, for [`rim_arcs_at`]: neck `(1, 0)`, shoulder
/// `(0.8, 0.6)`, lip `(0.2, 1.2)`.
pub fn lantern(tol: Tol) -> Body<f64> {
    let bulge = (0.6f64.asin() / 4.0).tan();
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), bulge),
            ProfileVertex::new(Point2::new(0.8, 0.6), 0.0),
            ProfileVertex::new(Point2::new(0.2, 1.2), 0.0),
            ProfileVertex::new(Point2::new(0.0, 1.2), 0.0),
        ],
        crate::Revolution::Full,
        tol,
    )
}

// ---------------------------------------------------------------------
// The swept elbow — the corpus's one curved-path `sweep_body`.
// ---------------------------------------------------------------------

/// The elbow path's radius, meters.
pub const ELBOW_R: f64 = 3.0;
/// The elbow's square cross-section half-width, meters.
pub const ELBOW_H: f64 = 0.25;
/// The station count the elbow is skinned at.
pub const ELBOW_STATIONS: usize = 9;
/// The v-degree the elbow's stations are interpolated at.
pub const ELBOW_V_DEGREE: usize = 3;

/// **The elbow's path**: a quarter circle of radius [`ELBOW_R`] in the
/// world YZ plane, starting at the origin with tangent `+z` — so the
/// identity-placed profile, which lies in the world XY plane, is
/// already perpendicular to it — and ending at `(0, R, R)`. Its centre
/// is `(0, R, 0)` and its axis of revolution is the world-x direction
/// through that centre.
///
/// The sketch arc runs `(0,0) → (R,R)` with `bulge = tan(θ/4) =
/// tan(π/8)`, i.e. a 90° turn; the placement rotates the sketch plane
/// by −π/2 about the world y-axis, sending sketch `(x, y)` to world
/// `(0, y, x)`.
pub fn elbow_path() -> NurbsCurve3<f64> {
    segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(ELBOW_R, ELBOW_R),
            bulge: (core::f64::consts::PI / 8.0).tan(),
        },
        Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            -core::f64::consts::FRAC_PI_2,
        ),
    )
    .expect("the elbow path is a well-formed quarter arc")
}

/// The elbow's section: a `2·`[`ELBOW_H`]-sided square centred on the
/// profile plane's origin, unit weights — the plainest integral
/// profile there is.
pub fn elbow_section() -> Section {
    vec![ProfileLoop::polygon(
        [
            (-ELBOW_H, -ELBOW_H),
            (ELBOW_H, -ELBOW_H),
            (ELBOW_H, ELBOW_H),
            (-ELBOW_H, ELBOW_H),
        ]
        .into_iter()
        .map(|(x, y)| Point2::new(x, y)),
    )]
}

/// **The swept elbow** — [`elbow_section`] carried along
/// [`elbow_path`] at [`ELBOW_STATIONS`] stations, v-degree
/// [`ELBOW_V_DEGREE`]: walls of degree 1×3 whose stations lie exactly
/// on the quarter torus of square cross-section, so the body's volume
/// converges to that torus's Pappus volume `(2h)²·R·π/2`.
///
/// This is the tree's ONE curved-path swept body. It is the corpus
/// constant for the STEP fixture, the tessellation rows and the
/// skin-integrality bracket alike, and it lives here so those suites
/// meter the same solid rather than six independent re-derivations of
/// the same six constants. A suite that needs the halves separately —
/// the path under a different section, the section under a different
/// path — takes them from the two functions above.
///
/// Returns the whole [`Lofted`] handoff, because a suite that checks
/// ORIENTATION needs the wall and cap keys and not only the body;
/// [`swept_elbow`] is the body alone, for the suites that do not.
pub fn swept_elbow_lofted(tol: Tol) -> Lofted<f64> {
    sweep_body::<f64>(
        &elbow_section(),
        Affine3::identity(),
        &elbow_path(),
        ELBOW_STATIONS,
        ELBOW_V_DEGREE,
        tol,
    )
    .expect("the curved-path sweep body builds")
}

/// [`swept_elbow_lofted`]'s body alone.
pub fn swept_elbow(tol: Tol) -> Body<f64> {
    swept_elbow_lofted(tol).body
}
