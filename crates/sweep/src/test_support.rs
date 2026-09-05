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
    one_edge_rim(body, hits[0])
}

/// **The one-edge rim `seed` belongs to, through the kernel door.**
///
/// The fixture-side spelling for a body whose latitude rims are single
/// closed edges: a suite names the arc it means by whatever analytic
/// handle its fixture states — radius, station, support kinds — and
/// this asks [`topo::query::rim_of`] whether that arc IS the rim,
/// rather than assuming it. Returns the door's answer, so the key a
/// row blends is the key the door named.
///
/// # Panics
///
/// If the door refuses, or answers with more than one arc: either is a
/// statement about the fixture, and a fixture that stopped minting a
/// one-edge rim should say so loudly rather than blend a different set.
#[must_use]
pub fn one_edge_rim(body: &Body<f64>, seed: EdgeKey) -> EdgeKey {
    let rim = topo::query::rim_of(body, seed)
        .unwrap_or_else(|e| panic!("the selected arc is a whole rim, got {e}"));
    match rim[..] {
        [only] => only,
        ref many => panic!("this fixture's rim is one closed edge, got {many:?}"),
    }
}

/// **Every arc of the latitude rim at radius `rim_r` and station
/// `rim_y`** — a FIXTURE SELECTION that names one of the rim's arcs,
/// and the kernel door that hands back the rest.
///
/// A rim a chart seam has SPLIT is several edges, and the fillet verbs
/// take exactly its set: adding one edge more refuses `TangentialEdge`
/// at margin zero, one edge fewer stops at a seam vertex. Producing
/// that set is [`topo::query::rim_of`]'s job and not a fixture's, so
/// this scan does the half only a fixture can do — find the arc the
/// suite means, by the radius and station it stated analytically — and
/// asks the door for the rim whole.
///
/// The scan still reads the co-surface exclusion, because it is
/// choosing a SEED: a sphere's seam meridian is a great circle that can
/// share a rim's radius and centre exactly, and seeding the door with
/// one refuses `CoSurface` rather than naming the rim beside it.
///
/// Comparison is against a fixed `1e-9`: fixtures state their rims
/// analytically, so this is a fixture-selection tolerance and not a
/// kernel predicate. The door it feeds carries no tolerance at all.
///
/// Generic over the scalar so the interval lane selects its rims through
/// the same door: the comparison reads both bounds of the stored
/// enclosure, which at `f64` is the value itself. The bound is the SOLE
/// `Bounds` the scope rule allows a driver to write (`Real` comes with
/// it); nothing here decides — a fixture selector reads.
///
/// # Panics
///
/// If the door refuses the arc this scan chose. Every refusal is a
/// statement about the FIXTURE (its rim is open, its arcs are not one
/// rim), so it is louder as a panic here than as an empty answer.
/// A radius and station no arc sits at stays an empty answer, which is
/// what a suite asserting a rim's absence means by it.
#[must_use]
pub fn rim_arcs_at<T: Bounds>(body: &Body<T>, rim_r: f64, rim_y: f64) -> Vec<EdgeKey> {
    match arcs_at(body, rim_r, rim_y).first() {
        None => Vec::new(),
        Some(seed) => topo::query::rim_of(body, *seed).unwrap_or_else(|e| {
            panic!("the rim at radius {rim_r}, station {rim_y} is one rim, got {e}")
        }),
    }
}

/// **The circle edges at radius `rim_r` and station `rim_y` whose two
/// supports are different surfaces**, in key order — the raw scan, and
/// deliberately NOT a rim.
///
/// [`rim_arcs_at`] seeds the rim door from this, and one suite wants
/// the scan itself: a PARTIALLY revolved body's equator is a set of
/// open arcs on one circle that no rim door will hand back, because
/// they are not one. Selecting them is a fixture's job; what the door
/// says about them is the row's subject.
///
/// The `1e-9` and the sole [`Bounds`] bound are [`rim_arcs_at`]'s, for
/// its reasons.
#[must_use]
pub fn arcs_at<T: Bounds>(body: &Body<T>, rim_r: f64, rim_y: f64) -> Vec<EdgeKey> {
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
    let minted_edges: Vec<EdgeKey> = rec
        .rim_trims
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.meridian_remnants.iter().map(|(e, _)| *e))
        .chain(rec.slits.iter().map(|(e, _)| *e))
        .collect();
    let minted_vertices: Vec<topo::VertexKey> = rec
        .rim_feet
        .iter()
        .map(|(v, _)| *v)
        .chain(rec.meridian_splits.iter().map(|(v, _)| *v))
        .collect();
    let mut banded: Vec<EdgeKey> = rec
        .bands
        .iter()
        .flat_map(|(_, edges)| edges.iter().copied())
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

/// **The waist's material-adding fill, by Pappus** — the 90-degree case
/// of [`wedge_fill`], at the waist's own two generators.
///
/// In the meridian half-plane the waist vertex is `V = (x_v, y_v)`,
/// where the lower generator (from `(1, 0)`, direction `(-1, 1)/2^(1/2)`)
/// meets the upper one (to `(1, 1)`, direction `(1, 1)/2^(1/2)`). The
/// material is on the axis side, so the VOID wedge at `V` opens toward
/// `+x` between the two generators and is 90 degrees; the rim is
/// concave, and the rolling ball rests in that void.
///
/// The fill region is the curvilinear triangle bounded by the two
/// generators and the fillet arc — the kite minus the sector at the
/// ball's centre — which is exactly what [`wedge_fill`] composes for any
/// wedge. This name survives because the waist's own generator
/// directions are worth stating once; the ARITHMETIC has one home.
///
/// **Not bit-identical to the collected algebraic form it replaces**,
/// and measured rather than assumed: the composed value differs by at
/// most `2.6e-17` over `r` in `{0.02, 0.05, 0.1}` — an
/// association-order difference, on fills of order `1e-3`, far under
/// both callers' bars (H4's `1e-14` absolute and its interval twin's
/// `1e-9` enclosure width, both re-run green). Output stability may
/// choose between two spellings; it may not keep a second
/// implementation (`memories/output-stability-as-justification.md`).
#[must_use]
pub fn waist_fill(x_v: f64, r: f64) -> f64 {
    let s2 = core::f64::consts::SQRT_2;
    // The station is not an argument because it cannot matter: Pappus's
    // first moment about the axis is invariant under a shift in `y`, and
    // the wedge's shape is fixed by its two directions.
    wedge_fill((x_v, 0.0), (1.0 / s2, -1.0 / s2), (1.0 / s2, 1.0 / s2), r)
}

/// **The bowl**: a flat floor at `y = 1` from the axis out to radius 1,
/// then a lip rising to `(1.5, 1.5)` and back down the outside to the
/// base — `(0,0) (1.5,0) (1.5,1.5) (1,1) (0,1)` revolved fully.
///
/// Pole-touching, so both its discs are minted as half-discs and
/// `merge_coplanar_faces` fuses each into one face. After that repair
/// its FLOOR rim `(1, 1)` is the plane-hosted closed rim whose crossings
/// are TRIVALENT — one plane face carrying both arcs in its own outer
/// cycle — and it is CONCAVE, an inside corner whose band ADDS material.
/// That is the pairing the closed-rim suites need: every other
/// plane-hosted fixture in the tree is convex.
pub fn bowl(tol: Tol) -> Body<f64> {
    bowl_at(tol)
}

/// [`bowl`] at any scalar: the same five dyadic vertices through the
/// same doors, so the interval twin differs in the scalar and nothing
/// else.
pub fn bowl_at<T: Decide + PcurveFittedLane>(tol: Tol) -> Body<T> {
    let v =
        |x: f64, y: f64| ProfileVertex::new(Point2::new(T::from_f64(x), T::from_f64(y)), T::zero());
    revolved_about_y_at(
        vec![
            v(0.0, 0.0),
            v(1.5, 0.0),
            v(1.5, 1.5),
            v(1.0, 1.0),
            v(0.0, 1.0),
        ],
        crate::Revolution::Full,
        tol,
    )
}

/// **The rolling ball's fill at a wedge, by Pappus** — the general form
/// [`waist_fill`] is the 90-degree case of, and the one home both the
/// `f64` rows and their interval twins read.
///
/// `k` is the profile corner and `da`, `db` the two generator
/// directions leaving it, spanning the wedge the ball rests in. The
/// ball of radius `r` touches both rays; the region between the corner
/// and its arc is the kite `k, fa, c, fb` minus the sector at `c`
/// between the feet, and Pappus revolves it.
///
/// **The wedge is the MATERIAL's on a convex rim and the VOID's on a
/// concave one**, and that is the only difference between the two
/// material sides: the region is the same shape, the carve REMOVES it
/// on the convex side and ADDS it on the concave one, so a caller
/// supplies the sign. Nothing of the kernel enters — the corner and the
/// two directions are read off the fixture's own profile.
#[must_use]
pub fn wedge_fill(k: (f64, f64), da: (f64, f64), db: (f64, f64), r: f64) -> f64 {
    let unit = |v: (f64, f64)| {
        let n = (v.0 * v.0 + v.1 * v.1).sqrt();
        (v.0 / n, v.1 / n)
    };
    let (da, db) = (unit(da), unit(db));
    let wedge = (da.0 * db.1 - da.1 * db.0)
        .abs()
        .atan2(da.0 * db.0 + da.1 * db.1);
    let t = r / (wedge / 2.0).tan();
    let d = r / (wedge / 2.0).sin();
    let bis = unit((da.0 + db.0, da.1 + db.1));
    let fa = (k.0 + t * da.0, k.1 + t * da.1);
    let fb = (k.0 + t * db.0, k.1 + t * db.1);
    let c = (k.0 + d * bis.0, k.1 + d * bis.1);
    pappus::pappus_volume(&[
        (1.0, pappus::triangle(k, fa, c)),
        (1.0, pappus::triangle(k, c, fb)),
        (-1.0, pappus::sector(c, r, fa, fb)),
    ])
}

/// **A pole-touching hemisphere of radius `r` on a flat base disc**: the
/// base `(0,0)→(r,0)` and the sphere quarter `(r,0)→(0,r)`, revolved
/// fully. The simplest plane-hosted closed rim there is — one profile
/// segment per support — and after `merge_coplanar_faces` its equator is
/// the hostless-crossing shape with a plane×sphere pair.
pub fn hemisphere_on_flat_base(r: f64, tol: Tol) -> Body<f64> {
    hemisphere_on_flat_base_at(r, tol)
}

/// [`hemisphere_on_flat_base`] at any scalar, so the interval twin
/// differs in the scalar and nothing else.
pub fn hemisphere_on_flat_base_at<T: Decide + PcurveFittedLane>(r: T, tol: Tol) -> Body<T> {
    // A quarter turn: `tan(theta/4)` at `theta = pi/2`.
    let bulge = T::from_f64((core::f64::consts::FRAC_PI_2 / 4.0).tan());
    revolved_about_y_at(
        vec![
            ProfileVertex::new(Point2::new(T::zero(), T::zero()), T::zero()),
            ProfileVertex::new(Point2::new(r, T::zero()), bulge),
            ProfileVertex::new(Point2::new(T::zero(), r), T::zero()),
        ],
        crate::Revolution::Full,
        tol,
    )
}

/// **The plane×sphere hostless carve's removed volume, by Pappus** — the
/// unit sphere of radius `big_r` centred at the origin meeting the plane
/// `y = 0` at the rim of radius `big_r`, material above the plane and
/// inside the sphere, filleted at radius `r`.
///
/// The ball rests `r` above the floor and internally tangent to the
/// sphere, so its centre is `C = (sqrt((R-r)^2 - r^2), r)` and its feet
/// are `F_a = (C_x, 0)` and `F_b = C·R/(R-r)`. The removed meridian
/// region is the kite `K, F_a, C, F_b` PLUS the circular segment of the
/// sphere's own circle between the chord `K`–`F_b` and its arc (the arc
/// bulges away from the centre, so the region holds it and the
/// straight-sided kite does not) MINUS the sector at `C` between the
/// feet. One home, because the `f64` rows and the interval twin read the
/// same truth.
#[must_use]
pub fn plane_sphere_cut(big_r: f64, r: f64) -> f64 {
    let k = (big_r, 0.0);
    let cx = ((big_r - r).powi(2) - r.powi(2)).sqrt();
    let c = (cx, r);
    let fa = (cx, 0.0);
    let scale = big_r / (big_r - r);
    let fb = (c.0 * scale, c.1 * scale);
    pappus::pappus_volume(&[
        (1.0, pappus::triangle(k, fa, c)),
        (1.0, pappus::triangle(k, c, fb)),
        (1.0, pappus::segment((0.0, 0.0), big_r, k, fb)),
        (-1.0, pappus::sector(c, r, fa, fb)),
    ])
}
