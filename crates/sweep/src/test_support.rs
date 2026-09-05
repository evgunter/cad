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

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::NurbsCurve3;
use geom_brep::PcurveFittedLane;
use geom_core::{Affine3, Band, Bounds, Decide, Point2, Point3, Real, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary};

use crate::blend::battery::{BlendRequest, Link, run_battery};
use crate::blend::build::Blended;
pub use crate::blend::surgery::ring_clearance_for_tests as ring_clearance;
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
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let verdict = run_battery(
        &BlendRequest {
            body,
            edges,
            size: R,
        },
        Band::linear(tol).unwrap(),
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
    revolved_about_y_at(verts, rev, tol)
}

/// [`revolved_about_y`] at any scalar the revolve door takes — the
/// interval twins build their fixtures through this, so the two lanes
/// differ in the scalar and in nothing else. The bound is the door's
/// own (`crate::revolve`'s), carrying no bracket read of its own.
pub fn revolved_about_y_at<T: Decide + PcurveFittedLane>(
    verts: Vec<ProfileVertex<T>>,
    rev: crate::Revolution<T>,
    tol: Tol,
) -> Body<T> {
    let profile = Profile::new(SketchPlane::<T>::xy(), vec![ProfileLoop::new(verts)])
        .validate(tol)
        .unwrap();
    let axis = crate::RevolveAxis {
        origin: Point2::new(T::zero(), T::zero()),
        dir: Vec2::new(T::zero(), T::one()),
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
///
/// Generic over the scalar so the interval lane selects its rims through
/// the same door: the comparison reads both bounds of the stored
/// enclosure, which at `f64` is the value itself. The bound is the SOLE
/// `Bounds` the scope rule allows a driver to write (`Real` comes with
/// it); nothing here decides — a fixture selector reads.
#[must_use]
pub fn rim_arcs_at<T: Bounds>(body: &Body<T>, rim_r: f64, rim_y: f64) -> Vec<EdgeKey> {
    let surface_of = |he| -> Option<topo::SurfaceKey> {
        let l = body.get_half_edge(he)?.parent_loop;
        Some(body.get_face(body.get_loop(l)?.face)?.surface)
    };
    let near = |x: T, want: f64| (x.lo() - want).abs() < 1e-9 && (x.hi() - want).abs() < 1e-9;
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            let geom::Curve3::Circle { radius, center, .. } = *c.carrier() else {
                return None;
            };
            if !near(radius, rim_r) || !near(center.y, rim_y) {
                return None;
            }
            (surface_of(e.he_plus)? != surface_of(e.he_minus)?).then_some(k)
        })
        .collect()
}

/// **The waisted body**: two cones meeting at radius `0.5` —
/// `(0,0)→(1,0)→(0.5,0.5)→(1,1)→(0,1)` revolved fully. The waist rim is
/// CONCAVE (the void wedge at the waist vertex is 90°) and the base and
/// top rims are convex; pole-touching, so every rim is a pair of arcs
/// meeting at chart-seam vertices. The closed-rim suites' concave
/// fixture, beside its own convex twins.
///
/// Its rims, for [`rim_arcs_at`]: waist `(0.5, 0.5)`, base `(1, 0)`,
/// top `(1, 1)`. Its volume is two frusta, `7π/12`.
pub fn waisted(tol: Tol) -> Body<f64> {
    waisted_at(tol)
}

/// [`waisted`] at any scalar: the same five dyadic vertices (every one
/// exactly representable, so the fixture's enclosures are points at a
/// certified scalar) through the same doors.
pub fn waisted_at<T: Decide + PcurveFittedLane>(tol: Tol) -> Body<T> {
    let v =
        |x: f64, y: f64| ProfileVertex::new(Point2::new(T::from_f64(x), T::from_f64(y)), T::zero());
    revolved_about_y_at(
        vec![
            v(0.0, 0.0),
            v(1.0, 0.0),
            v(0.5, 0.5),
            v(1.0, 1.0),
            v(0.0, 1.0),
        ],
        crate::Revolution::Full,
        tol,
    )
}

/// **A flat-floored dome cavity with a bore** — the plane–sphere fold's
/// fourth quadrant, `(sphere pocket, chain concave)`, which no other
/// fixture reaches. A block of revolution (radius 1, height 1) holds a
/// cavity whose FLOOR is the plane `y = 0.3` out to a rim of radius
/// `0.5` and whose CEILING is the hemisphere of radius `0.5` centred on
/// the axis at the floor's level; a bore of radius `0.2` runs from where
/// it meets the ceiling up through the top, which is what keeps the body
/// one shell. Material lies BELOW the floor and OUTSIDE the sphere, so
/// the sphere face's sense is `false` (a pocket) while the rim — where
/// the floor and the ceiling meet — is the UNION of the two material
/// half-spaces, a wedge of 3π/2: CONCAVE. Boss and pip are the other
/// two mixed quadrants; the dome is the fourth.
pub fn domed_cavity(tol: Tol) -> Body<f64> {
    let (big_r, bore, floor) = (0.5_f64, 0.2_f64, 0.3_f64);
    let y_bore = floor + (big_r.powi(2) - bore.powi(2)).sqrt();
    // The ceiling arc from the bore's edge down to the rim subtends
    // `acos(bore / R)` at the centre and bulges AWAY from the centre —
    // to the left of its downward-outward chord.
    let theta = (bore / big_r).acos();
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(bore, 1.0), 0.0),
            ProfileVertex::new(Point2::new(bore, y_bore), -(theta / 4.0).tan()),
            ProfileVertex::new(Point2::new(big_r, floor), 0.0),
            ProfileVertex::new(Point2::new(0.0, floor), 0.0),
        ],
        crate::Revolution::Full,
        tol,
    )
}

/// A radius-`r` ball centred at `c` with its polar axis along `+z`: the
/// revolve puts a ball's poles on the sketch axis, and a plane×sphere
/// section against a chart whose polar axis is tilted to the plane is a
/// typed frontier of the boolean's split-join, so a ball meeting a
/// `z`-plane face is charted with its pole along that normal. The die's
/// pips (`slab ∖ ball`) and their concave twin, a boss (`slab ∪ ball`),
/// are both built from this.
pub fn ball_poled_z(r: f64, c: Vec3<f64>, tol: Tol) -> Body<f64> {
    ball_poled_z_at(r, c, tol)
}

/// [`ball_poled_z`] at any scalar the revolve and rigid-motion doors
/// take.
pub fn ball_poled_z_at<T: Decide + PcurveFittedLane>(r: T, c: Vec3<T>, tol: Tol) -> Body<T> {
    let ball = revolved_about_y_at(
        vec![
            ProfileVertex::new(Point2::new(T::zero(), -r), T::one()),
            ProfileVertex::new(Point2::new(T::zero(), r), T::zero()),
        ],
        crate::Revolution::Full,
        tol,
    );
    let poled = topo::transform_rigid(
        &ball,
        &Affine3::rotation_about_axis(
            Point3::new(T::zero(), T::zero(), T::zero()),
            Vec3::new(T::one(), T::zero(), T::zero()),
            T::from_f64(core::f64::consts::FRAC_PI_2),
        ),
        tol,
    )
    .unwrap();
    topo::transform_rigid(&poled, &Affine3::translation(c), tol).unwrap()
}

/// **The toroidal spool**: an annular meridian whose outer wall is an
/// off-axis 60° ARC, revolved about the sketch y-axis by `rev`.
///
/// That wall is a TORUS, and a torus support is outside every analytic
/// arm's table — the canal-surface lane's front door, where the rolling
/// ball's spine is neither a line nor a circle. It is what
/// `FILLET3_SPINE_KIND_RECOURSE` is refused on.
///
/// The arc is 60° about `(1.5, 0)` of radius `0.5`, so it meets the base
/// at a square corner and the top at a 30° one — neither joint tangent,
/// which is what keeps the profile's own validator out of the way. The
/// bore is on-axis at `0.5`. Takes `rev` because the partial revolve is
/// a different refusal's fixture.
pub fn spool(rev: crate::Revolution<f64>, tol: Tol) -> Body<f64> {
    let bulge = (core::f64::consts::FRAC_PI_6 / 2.0).tan();
    let (ex, ey) = (1.75, 0.25 * 3.0f64.sqrt());
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.5, 0.0), 0.0),
            ProfileVertex::new(Point2::new(2.0, 0.0), bulge),
            ProfileVertex::new(Point2::new(ex, ey), 0.0),
            ProfileVertex::new(Point2::new(0.5, ey), 0.0),
        ],
        rev,
        tol,
    )
}

/// **A prism**: one closed profile loop extruded `h` along `+z`.
///
/// The twelfth copy of this four-line helper in the crate's suites was
/// what got it homed. Takes the vertices rather than a shape so the
/// L-prism, the arc-sided prism and the turned box are all one door;
/// panics on an invalid loop, which is a fixture bug, not an outcome.
pub fn prism(verts: Vec<ProfileVertex<f64>>, h: f64, tol: Tol) -> Body<f64> {
    let pf = Profile::new(SketchPlane::xy(), vec![ProfileLoop::new(verts)])
        .validate(tol)
        .expect("the fixture's profile is a valid loop");
    extrude(&pf, Extrusion::Distance(h), tol)
        .expect("the fixture's profile extrudes")
        .body
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

// ------------------------------------------------------------------
// Shared assertion helpers for the blend suites — one home each.
// ------------------------------------------------------------------

/// The faces across the edges of `face`'s outer cycle, deduplicated —
/// a band's NEIGHBOURS, which is how a door is proved rather than
/// inferred: the ladder's band sits between one plane face and two
/// ring-free half-caps, the annulus's between the two half-band walls.
pub fn faces_around<T: Real>(body: &Body<T>, face: FaceKey) -> Vec<FaceKey> {
    let LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(face).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("a band's outer loop is a cycle")
    };
    let mut out: Vec<FaceKey> = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| {
            let mate = body.mate(he).unwrap();
            body.get_loop(body.get_half_edge(mate).unwrap().parent_loop)
                .unwrap()
                .face
        })
        .filter(|&f| f != face)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// **Naming totality on a closed-rim band, in all three directions.**
/// (a) Every output entity is a recorded mint or a survivor of the
/// source; (b) every recorded retirement names a SOURCE key that did not
/// survive; (c) every source entity ABSENT from the output is a recorded
/// retirement — a dead edge or vertex, or a rim arc a band row replaced.
/// (c) is the direction the first two do not imply: a retirement the
/// surgery forgets to record is invisible to (a) and (b) and to the
/// census delta alike. Also: the band rows name exactly `requested`.
/// The per-row COUNTS (feet, splits, retired seam vertices) stay in the
/// rows, because they are the fixture's, not the walk's.
pub fn assert_naming_totality(
    source: &Body<f64>,
    out: &Blended<f64>,
    requested: &[EdgeKey],
    what: &str,
) {
    let rec = out
        .naming
        .as_ref()
        .unwrap_or_else(|| panic!("{what}: the rim phase records its births"));
    // Every birth row of either band — the rim phase's and the open
    // bands' — so the walk is total over whatever the request carved.
    let minted_edges: Vec<EdgeKey> = rec
        .rim_trims
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.meridian_remnants.iter().map(|(e, _)| *e))
        .chain(rec.slits.iter().map(|(e, _)| *e))
        .chain(rec.trims.iter().map(|(e, _, _)| *e))
        .chain(rec.arcs.iter().map(|(e, _, _)| *e))
        .collect();
    let minted_vertices: Vec<topo::VertexKey> = rec
        .rim_feet
        .iter()
        .map(|(v, _)| *v)
        .chain(rec.meridian_splits.iter().map(|(v, _)| *v))
        .chain(rec.feet.iter().map(|(v, _, _)| *v))
        .collect();
    // The edges a band replaced: a closed chain's arcs (`bands`) or an
    // open link's edge (`blends`) — together, exactly the request.
    let mut banded: Vec<EdgeKey> = rec
        .bands
        .iter()
        .flat_map(|(_, edges)| edges.iter().copied())
        .chain(rec.blends.iter().map(|(_, e)| *e))
        .collect();
    // (a)
    for (k, _) in out.body.edges() {
        assert!(
            minted_edges.contains(&k) || source.get_edge(k).is_some(),
            "{what}: output edge {k:?} is neither minted nor a survivor"
        );
    }
    for (k, _) in out.body.vertices() {
        assert!(
            minted_vertices.contains(&k) || source.get_vertex(k).is_some(),
            "{what}: output vertex {k:?} is neither a recorded mint nor a survivor"
        );
    }
    // (b)
    for e in &rec.dead.edges {
        assert!(
            source.get_edge(*e).is_some(),
            "{what}: a retirement names a source edge, got {e:?}"
        );
        assert!(
            out.body.get_edge(*e).is_none() || minted_edges.contains(e),
            "{what}: a retired edge does not survive: {e:?}"
        );
    }
    for v in &rec.dead.vertices {
        assert!(
            source.get_vertex(*v).is_some(),
            "{what}: a retirement names a source vertex, got {v:?}"
        );
        assert!(
            out.body.get_vertex(*v).is_none(),
            "{what}: a retired vertex does not survive: {v:?}"
        );
    }
    // (c)
    for (k, _) in source.edges() {
        if out.body.get_edge(k).is_none() {
            assert!(
                rec.dead.edges.contains(&k) || banded.contains(&k),
                "{what}: source edge {k:?} vanished with no retirement recorded"
            );
        }
    }
    for (k, _) in source.vertices() {
        if out.body.get_vertex(k).is_none() {
            assert!(
                rec.dead.vertices.contains(&k),
                "{what}: source vertex {k:?} vanished with no retirement recorded"
            );
        }
    }
    banded.sort_unstable();
    let mut want = requested.to_vec();
    want.sort_unstable();
    assert_eq!(
        banded, want,
        "{what}: the band rows name exactly the requested arcs"
    );
}

/// **A recourse sentence promises the carve on EITHER material side and
/// hedges on nothing** — the one home of the pin three suites used to
/// spell as a string test each. The negative half names the hedge
/// SHAPES a conditioned clause would take, so a rewording that keeps
/// the promise but re-conditions it goes red here.
pub fn assert_promises_either_side(sentence: &str) {
    assert!(
        sentence.contains("either material side"),
        "the carve half is promised on both sides: {sentence}"
    );
    for hedge in [
        "CONVEX",
        "convex side",
        "convex only",
        "convex-only",
        "only where",
        "where the rim is",
        "not on a concave",
    ] {
        assert!(
            !sentence.contains(hedge),
            "the carve half conditions on nothing, but carries {hedge:?}: {sentence}"
        );
    }
}

/// **Pappus pieces for a meridian-plane fill or cut**: `(area, ∫x dA)`
/// of the elementary regions a fillet's cross-section decomposes into,
/// so a row can derive a revolved volume delta by hand as
/// `2π Σ ∫x dA` ([`pappus::pappus_volume`]) and compare it to the measured one.
/// Coordinates are the meridian half-plane's `(x, y)` with the axis at
/// `x = 0`. Each piece's centroid is the textbook one; the row states
/// which pieces are added and which subtracted, and why.
pub mod pappus {
    use core::f64::consts::TAU;

    /// A triangle `(a, b, c)`: area `½|(b−a)×(c−a)|`, centroid the mean.
    #[must_use]
    pub fn triangle(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> (f64, f64) {
        let area = 0.5 * ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)).abs();
        (area, area * (a.0 + b.0 + c.0) / 3.0)
    }

    /// The MINOR circular sector at `c` of radius `rho` between the rays
    /// to `f1` and `f2` (angle `θ < π`): area `½ρ²θ`, centroid
    /// `4ρ sin(θ/2) / (3θ)` from `c` along the bisector.
    #[must_use]
    pub fn sector(c: (f64, f64), rho: f64, f1: (f64, f64), f2: (f64, f64)) -> (f64, f64) {
        let (u, v) = ((f1.0 - c.0, f1.1 - c.1), (f2.0 - c.0, f2.1 - c.1));
        let theta = angle(u, v);
        let area = 0.5 * rho.powi(2) * theta;
        let d = 4.0 * rho * (theta / 2.0).sin() / (3.0 * theta);
        let bis = unit((u.0 + v.0, u.1 + v.1));
        (area, area * (c.0 + d * bis.0))
    }

    /// The circular SEGMENT of the circle centred `o` of radius `big_r`
    /// between the chord `a`–`b` and the minor arc: area
    /// `½R²(φ − sin φ)`, centroid `4R sin³(φ/2) / (3(φ − sin φ))` from
    /// `o` along the chord's bisector.
    #[must_use]
    pub fn segment(o: (f64, f64), big_r: f64, a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
        let (u, v) = ((a.0 - o.0, a.1 - o.1), (b.0 - o.0, b.1 - o.1));
        let phi = angle(u, v);
        let area = 0.5 * big_r.powi(2) * (phi - phi.sin());
        let d = 4.0 * big_r * (phi / 2.0).sin().powi(3) / (3.0 * (phi - phi.sin()));
        let bis = unit((u.0 + v.0, u.1 + v.1));
        (area, area * (o.0 + d * bis.0))
    }

    /// `2π Σ ∫x dA` over signed pieces: the revolved volume of the
    /// region they compose.
    #[must_use]
    pub fn pappus_volume(pieces: &[(f64, (f64, f64))]) -> f64 {
        TAU * pieces.iter().map(|(sign, (_, mx))| sign * mx).sum::<f64>()
    }

    fn angle(u: (f64, f64), v: (f64, f64)) -> f64 {
        let dot = u.0 * v.0 + u.1 * v.1;
        let cross = u.0 * v.1 - u.1 * v.0;
        cross.abs().atan2(dot)
    }

    fn unit(v: (f64, f64)) -> (f64, f64) {
        let n = (v.0 * v.0 + v.1 * v.1).sqrt();
        (v.0 / n, v.1 / n)
    }
}

/// **The waist's material-adding fill, by Pappus** — nothing of the
/// kernel enters.
///
/// In the meridian half-plane `(x, y)` the waist vertex is
/// `V = (x_v, y_v) = (0.5, 0.5)`, where the lower generator (from
/// `(1, 0)`, direction `(−1, 1)/√2`) meets the upper one (to `(1, 1)`,
/// direction `(1, 1)/√2`). The material is on the axis side, so the
/// VOID wedge at `V` opens toward `+x` between the two generators and
/// is `90°`; the rim is concave. The rolling ball of radius `r` rests in
/// that void, tangent to both generators: its centre is on the wedge's
/// bisector (the `+x` ray from `V`) at distance `r/sin 45° = r√2`, so
/// `C = (x_v + r√2, y_v)`, and its feet are `r` from `V` along each
/// generator, `F± = (x_v + r/√2, y_v ± r/√2)`.
///
/// The fill region is the curvilinear triangle `V, F−, F+` bounded by
/// the two generators and the fillet arc — the kite `V F− C F+` minus
/// the circular sector at `C` between the feet. The kite is two right
/// triangles of legs `r, r`, area `r²`; the sector's angle is
/// `π − π/2 = π/2`, area `πr²/4`; so the fill's area is `r²(1 − π/4)`.
///
/// Its first moment about the axis, `∫ x dA`:
/// - the kite is symmetric about `y = y_v` and each of its two
///   triangles has centroid `x = x_v + r/√2` (the mean of `x_v`,
///   `x_v + r/√2` and `x_v + r√2`), so `∫_kite x dA = r²(x_v + r/√2)`;
/// - the sector's centroid lies `4√2 r/(3π)` from `C` toward `V`
///   (`2R sin θ / 3θ` at half-angle `θ = π/4`), so
///   `∫_sector x dA = (πr²/4)(x_v + r√2) − √2 r³/3`.
///
/// Subtracting and collecting,
/// `∫_fill x dA = x_v r²(1 − π/4) + √2 r³(5/6 − π/4)`, and Pappus gives
/// `ΔV = 2π ∫_fill x dA`. Both brackets are positive, as the fill lies
/// on the `+x` side of `V`.
#[must_use]
pub fn waist_fill(x_v: f64, r: f64) -> f64 {
    2.0 * PI
        * (x_v * r.powi(2) * (1.0 - PI / 4.0) + 2f64.sqrt() * r.powi(3) * (5.0 / 6.0 - PI / 4.0))
}

/// The rod's radius, meters.
pub const ROD_R: f64 = 0.5;
/// The flat's distance from the rod's axis, meters.
pub const ROD_FLAT: f64 = 0.3;
/// The rod's length, meters.
pub const ROD_L: f64 = 1.0;

/// **The rod with a flat milled along it** — the `CylinderPlaneCylinder`
/// consumer: a cylinder of radius [`ROD_R`] about `z` over
/// `z ∈ [0, ROD_L]`, minus a box whose face at `x = ROD_FLAT` planes the
/// flat. Two straight creases (cylinder–plane, along the ruling), each
/// ending in the two caps — planes perpendicular to the ruling, the
/// transverse caps the ruled band is cut off at. Built through the
/// public boolean door, as a user would mill it.
///
/// Generic over the scalar so the interval twin builds the same body
/// through the same doors at `Interval`.
pub fn rod_with_flat_at<T: Decide + Bounds + PcurveFittedLane>(tol: Tol) -> Body<T> {
    let f = T::from_f64;
    let disc = profile::circle(Point2::new(f(0.0), f(0.0)), f(ROD_R), tol)
        .expect("the rod's disc is a valid loop");
    let rod = Profile::new(SketchPlane::<T>::xy(), vec![disc.into()])
        .validate(tol)
        .expect("the rod's profile validates");
    let rod = extrude(&rod, Extrusion::Distance(f(ROD_L)), tol)
        .expect("the rod extrudes")
        .body;
    let square = ProfileLoop::new(
        [(ROD_FLAT, -1.0), (1.0, -1.0), (1.0, 1.0), (ROD_FLAT, 1.0)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(f(x), f(y)), f(0.0)))
            .collect(),
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(f(0.0), f(0.0), f(-0.5))));
    let cutter = Profile::new(plane, vec![square])
        .validate(tol)
        .expect("the cutter's profile validates");
    let cutter = extrude(&cutter, Extrusion::Distance(f(ROD_L + 1.0)), tol)
        .expect("the cutter extrudes")
        .body;
    topo::subtract(&rod, &cutter, tol)
        .expect("the flat mills")
        .body()
        .expect("a body remains")
        .body
        .clone()
}

/// [`rod_with_flat_at`] at `f64`.
pub fn rod_with_flat(tol: Tol) -> Body<f64> {
    rod_with_flat_at(tol)
}

/// **The creases of a rod with a flat**: every straight edge whose two
/// supports are a cylinder and a plane — the ruling edges the band is
/// asked for.
pub fn rod_creases<T: Real>(body: &Body<T>) -> Vec<EdgeKey> {
    use topo::query::{self, CurveKind, CurveKindSet, SurfaceKindSet};
    query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            query::edge_carrier_matches(body, k, CurveKindSet::just(CurveKind::Line))
                && query::edge_adjacent_matches(
                    body,
                    k,
                    SurfaceKindSet::just(geom_brep::SurfaceKind::Cylinder),
                    SurfaceKindSet::just(geom_brep::SurfaceKind::Plane),
                )
        })
        .collect()
}

/// **The cross-section area a ruled band removes at one cylinder–plane
/// crease** — the prism closed form's `A_section`, so a row can derive
/// `ΔV = A_section · L` by hand and compare it to the measured volume.
///
/// In the section normal to the ruling: a circle of radius `R` about
/// the origin (the rod) cut by the line `x = flat` (the flat), the
/// crease at `V = (flat, √(R² − flat²))` on the upper side. The rolling
/// ball of radius `r` rests inside the material at distance `r` from
/// both, so its centre is `c = (flat − r, h)` with
/// `h = √((R − r)² − (flat − r)²)` — the crossing of the offset line
/// and the offset circle, which is the arm's own sheet crossing. Its
/// feet are `f_b = (flat, h)` on the flat and `f_a = c · R/(R − r)` on
/// the rod. The region the band removes is the curvilinear triangle
/// `f_b → V` (along the flat), `V → f_a` (along the rod's circle) and
/// `f_a → f_b` (along the fillet arc, concave toward `c`):
///
/// ```text
/// A = area(quad c, f_b, V, f_a)     the straight-sided hull
///   − ½ r² θ                         minus the fillet sector at c
///   + ½ R² (φ − sin φ)               plus the rod's circular segment
///                                    between chord V–f_a and its arc
/// θ = acos((flat − r)/(R − r))       the sector angle at c
/// φ = θ − acos(flat/R)               the rod arc's sweep V → f_a
/// ```
///
/// (`f_a` lies along `c` from the origin, so `angle(f_a) = θ`.) The
/// quad is traversed counter-clockwise in `(x, y)`, so its shoelace sum
/// is positive as written.
#[must_use]
pub fn rod_section_cut(big_r: f64, flat: f64, r: f64) -> f64 {
    let h = ((big_r - r).powi(2) - (flat - r).powi(2)).sqrt();
    let c = (flat - r, h);
    let f_b = (flat, h);
    let v = (flat, (big_r * big_r - flat * flat).sqrt());
    let scale = big_r / (big_r - r);
    let f_a = (c.0 * scale, c.1 * scale);
    let quad = [c, f_b, v, f_a];
    let mut twice = 0.0;
    for i in 0..4 {
        let (p, q) = (quad[i], quad[(i + 1) % 4]);
        twice += p.0 * q.1 - q.0 * p.1;
    }
    let theta = ((flat - r) / (big_r - r)).acos();
    let phi = theta - (flat / big_r).acos();
    0.5 * twice - 0.5 * r * r * theta + 0.5 * big_r * big_r * (phi - phi.sin())
}
