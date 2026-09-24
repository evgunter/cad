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
//!   same spelling wherever another crate's suites want a body this
//!   one already owns. So it is on exactly when some crate's TESTS
//!   compile the library, and off for every non-test build of every
//!   dependent. A crate joins by adding that one line; there is
//!   nothing else to wire.
//!
//!   **No list of those crates is kept here**, deliberately: the set
//!   changes whenever a suite wants a fixture, nothing recomputes a
//!   sentence, and a hand-written census that has gone stale is the
//!   defect this module exists to remove rather than a description of
//!   it. `scripts/gates/test-features-dev-only.sh` reads the manifests
//!   and is the authority on where the edge is; `cargo tree -e dev -i
//!   sweep` answers the same question locally.
//!
//!   A fixture only earns a place here once a consumer OUTSIDE this
//!   crate needs it or a second suite inside it does; the narrower
//!   homes, and the rule that routes between them, are stated in
//!   `sweep`'s own `tests/common` module. The same rule seats the
//!   crate-PRIVATE seams a suite reads through — [`ring_clearance`]
//!   and [`walked_chains`] — which are not fixtures but the only way a
//!   `tests/` crate can observe a `pub(crate)` phase.
//!
//! # The extrusion family
//!
//! [`extruded`] is the primitive — loops on a plane, pushed along its
//! normal — and [`prism_on`], [`prism`] and [`prism_at`] are its named
//! specializations. All of them are generic in the scalar, because the
//! `Interval` and `Probe` lanes build the same bodies as the `f64` one
//! and the only alternative is a second copy at each scalar: a
//! per-scalar copy per suite is what this family was before it was
//! one. A shape that is not here yet joins by naming the primitive and
//! its own loops — it needs no new door, no new gate and no new
//! manifest edge beyond the one its crate already has.
//!
//! **The axis-aligned box is not one of them.** [`brick`] is
//! `topo::test_support::brick`, and [`block`] and [`cube`] are its two
//! views — by extent from the origin, and with one extent. The box has
//! one construction in this tree and it lives in `topo`, below every
//! crate that wants one: a second construction here would be a second
//! body that is the same solid, differing only in the order its curve
//! arena holds twelve keys and in which way round four of its twelve
//! edges name the surfaces they intersect. Those three doors are
//! generic in the scalar like the rest of this module and take their
//! extents as `f64` at every scalar, for the reason [`corners`] gives.
//!
//! # The loft family
//!
//! The skinned bodies are `f64` only, and not by omission:
//! [`Section`](crate::Section) is `Vec<ProfileLoop<f64>>`, so
//! `loft_body` has one scalar and so does everything built on it.
//! [`loft_prism_sections`] is that family's primitive — the three
//! sections — and [`loft_prism_at`] and [`loft_prism`] are its named
//! placements.
//!
//! **The sections door is not a convenience over the body door**, and
//! the split is not arbitrary: a suite reaches for the sections when
//! the body is not what it is measuring. Two kinds do. One varies the
//! PLACEMENT — stacking against the base normal, re-spacing 1 : 2,
//! carrying the stack through a rotation — where the placement is the
//! subject and belongs on the line that reads it. The other needs the
//! PRE-BODY artifact the solid has already discarded: the [`Lofted`]
//! handoff's wall and cap keys, the geometry `loft_geometry` returns,
//! or the `(sections, places)` pair a row compares against a second
//! section set. Neither kind can be served by handing it a `Body`.
//!
//! # Editing a fixture here re-authors committed bytes
//!
//! `step-export`'s `examples/export_fixtures` regenerates that crate's
//! committed `.step` corpus from its `tests/common` module, and that
//! module's boxes are this module's, and so is its [`loft_prism`]. So
//! a change to [`brick`], [`block`], [`cube`], [`extruded`],
//! [`loft_prism`] or [`pocket_die`] changes files that are checked in,
//! and is not the tests-only edit the gate on this module might
//! suggest.
//!
//! That is a coupling accepted rather than overlooked, and it is
//! guarded: `step-export`'s `committed_fixtures_are_byte_golden` runs
//! on every PR over the same builders, so the change that would move
//! the bytes reddens the branch that makes it rather than surfacing at
//! the next regeneration. Per the repo's baseline rule the answer is
//! then to decide whether the new body is right and re-baseline
//! saying what moved -- never to restore the old bytes.
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
use geom_core::{Affine3, Band, Bounds, Decide, OrthoFrame, Point2, Point3, Real, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey, FaceKey, LoopBoundary};

use crate::blend::BlendKind;
use crate::blend::battery::{BlendRequest, Chain, Link, resolve_link, run_battery, walk_chains};
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
pub fn cube<T: Decide>(l: f64, tol: Tol) -> Body<T> {
    block(l, l, l, tol)
}

/// An axis-aligned box of extents `w` x `d` x `h` with its low corner
/// at the origin.
///
/// **The second view of [`brick`], not a second body**: the suites are
/// written in two vocabularies for one box — by bounds (`brick`) and
/// by extent from the origin (this, and [`cube`] with one extent) —
/// and both reach the same construction through the same door. The
/// alternative was seven private copies of the construction under one
/// more name, which is what this replaced.
pub fn block<T: Decide>(w: f64, d: f64, h: f64, tol: Tol) -> Body<T> {
    brick((0.0, w), (0.0, d), (0.0, h), tol)
}

/// **The pocketed die's two operands**: the unit block at
/// `(x0, y0, z0)` and the centred `0.5` cutter that opens through its
/// top face, overshooting above so the pocket is a through-mouth
/// rather than a coplanar kiss.
///
/// Handed back as a pair, not as a finished body, because the two
/// suites that build this die do different things with the boolean:
/// one wants the body, the other measures the RESULT — its contact
/// lists — and a door that returns only a `Body` cannot serve the
/// second. The geometry is the shared thing; the boolean is the
/// caller's row.
pub fn pocket_die_parts(x0: f64, y0: f64, z0: f64, tol: Tol) -> (Body<f64>, Body<f64>) {
    (
        brick((x0, x0 + 1.0), (y0, y0 + 1.0), (z0, z0 + 1.0), tol),
        brick(
            (x0 + 0.25, x0 + 0.75),
            (y0 + 0.25, y0 + 0.75),
            (z0 + 0.5, z0 + 1.5),
            tol,
        ),
    )
}

/// **The pocketed die**: [`pocket_die_parts`] subtracted. Exact volume
/// `0.875`; the top face carries a ring, the pocket mouth.
///
/// `f64` only, for the reason [`rod_with_flat`] gives: the boolean
/// door's scalar bound is a compound this file is not ratified to
/// spell.
pub fn pocket_die(x0: f64, y0: f64, z0: f64, tol: Tol) -> Body<f64> {
    let (block, cutter) = pocket_die_parts(x0, y0, z0, tol);
    topo::subtract(&block, &cutter, tol)
        .expect("the die's pocket cuts")
        .body()
        .expect("the die is a body")
        .body
        .clone()
}

/// An axis-aligned box spanning `x` x `y` x `z`, as the half-open
/// intervals `(lo, hi)` — the plainest body in the kernel and the one
/// its acceptance suites reach for first.
///
/// **`topo`'s construction, named here.** The box is built by the
/// Euler sequence `topo::test_support::brick` runs, not by this
/// module's extrusion primitive; this door exists so that a suite
/// which already depends on `sweep` does not reach past it for the
/// plainest body there is.
pub fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64), tol: Tol) -> Body<T> {
    topo::test_support::brick(x, y, z, tol)
}

/// The square of side `l` with a corner at the origin, counter-clockwise
/// from that corner, as profile vertices — the one spelling of the block
/// outline the fixtures here build on when they need the loop rather
/// than the body.
fn square<T: Decide>(l: f64) -> Vec<ProfileVertex<T>> {
    corners(&[(0.0, 0.0), (l, 0.0), (l, l), (0.0, l)])
}

/// Profile vertices from xy pairs, every bulge zero — the straight
/// polygon vocabulary the suites' own copies of these builders are
/// written in.
///
/// Takes `f64` pairs at every scalar, like [`waisted_at`]: a fixture's
/// outline is a set of chosen constants, and a chosen constant is an
/// `f64` whatever the lane's arithmetic is.
pub fn corners<T: Decide>(pts: &[(f64, f64)]) -> Vec<ProfileVertex<T>> {
    pts.iter()
        .map(|&(x, y)| ProfileVertex::new(Point2::new(T::from_f64(x), T::from_f64(y)), T::zero()))
        .collect()
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
pub fn revolved_about_y_at<T: Decide + topo::AtRestPolicy>(
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

/// **The ONE edge of the rim at radius `rim_r` and station `rim_y`** —
/// [`rim_arcs_at`]'s answer for a fixture whose rim is a single closed
/// edge, in the shape a caller that holds one key needs.
///
/// Carries no scan and no window of its own: the selection is
/// [`arcs_at`]'s and the rim is [`topo::query::rim_of`]'s. What it adds
/// is the fixture's claim that this rim is ONE edge, said once here
/// instead of at every suite that wants a key rather than a set.
///
/// `f64` where [`rim_arcs_at`] is generic: the interval lane blends the
/// rim as the SET the door hands back, so the key shape has no caller
/// there and buys no [`Bounds`] bracket read of its own.
///
/// # Panics
///
/// If the rim at that radius and station is not exactly one edge — an
/// empty answer (no arc sits there) included, because a caller holding
/// a key has no way to say "no rim".
#[must_use]
pub fn one_edge_rim_at(body: &Body<f64>, rim_r: f64, rim_y: f64) -> EdgeKey {
    match rim_arcs_at(body, rim_r, rim_y)[..] {
        [only] => only,
        ref many => {
            panic!("the rim at radius {rim_r}, station {rim_y} is one closed edge, got {many:?}")
        }
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
/// **The station is `center.y` and nothing else**: this home names a
/// latitude of a body poled along +y, which is the axis
/// [`revolved_about_y`] mints and every fixture it builds. A body poled
/// along z states its stations in `center.z`, so a z-poled fixture
/// cannot ask this question here and rolls its own scan —
/// `seed-finder-home-reads-only-the-y-station` carries the sites and
/// the two shapes a fix could take. The premise is pinned by
/// `review_blend4_r4_probes::arcs_at_reads_the_y_station_and_nothing_else`.
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
pub fn waisted_at<T: Decide + topo::AtRestPolicy>(tol: Tol) -> Body<T> {
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
pub fn ball_poled_z_at<T: Decide + topo::AtRestPolicy>(r: T, c: Vec3<T>, tol: Tol) -> Body<T> {
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
/// arm's table: the door tests KIND membership before it tests any
/// symmetry, and no arm traces a torus. The refusal is therefore on
/// KIND — not on the spine, whose shape the fixture does not settle.
/// Its torus wall meets the base plane in a LATITUDE circle, so that
/// rim's rolling ball does have a circular spine and the pair is two
/// coaxial surfaces of revolution; the canal-surface lane owns it
/// anyway, because the arm that would mint its torus band does not
/// exist. It is what `FILLET3_SPINE_KIND_RECOURSE` is refused on.
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

/// **The extrusion door**: the closed `loops` on `plane`, extruded
/// `h` along the plane normal.
///
/// This is the whole family's primitive, and the reason it is spelled
/// here rather than inside one of them. Every body fixture in this
/// repo that is "a sketch pushed along its normal" — the cube, the
/// brick, the L-prism, the holed plate, the skewed block, the turned
/// box — is these six lines with a different loop set, and they were
/// copied rather than called because the six lines are shorter than
/// the reach to a home. A fixture joins by naming this door and its
/// own loops; nothing about it is specific to a shape, a scalar or a
/// crate, so a shape that is not in this module today needs no
/// redesign to move here, only a name.
pub fn extruded<T: Decide>(
    plane: SketchPlane<T>,
    loops: Vec<ProfileLoop<T>>,
    h: T,
    tol: Tol,
) -> Body<T> {
    let pf = Profile::new(plane, loops)
        .validate(tol)
        .expect("the fixture's profile is a valid loop set");
    extrude(&pf, Extrusion::Distance(h), tol)
        .expect("the fixture's profile extrudes")
        .body
}

/// The K funnel name a FIXTURE's authored frame axes are decided
/// under. One name for both axes of [`sketch_from_axes`]: which axis a
/// refusal is about is the refusal's own field, and a fixture that
/// refuses has a bug rather than a story.
const FIXTURE_FRAME_AXIS: &str = "fixture_frame_axis";

/// **A sketch plane from an authored pair**, orthonormalized at the
/// run's band — the fixtures' one spelling of the frame mint, for a
/// plane that is tilted, rotated or deliberately noisy.
///
/// An axis-aligned fixture does not come here: the exact world frames
/// (`OrthoFrame::axes_xy` and its two siblings) need no decision, and
/// [`sketch_at`] is the xy one at a station.
///
/// # Panics
///
/// If the band cannot be formed, or if the two directions span no
/// plane — a fixture whose axes are parallel is a broken fixture, not
/// a case under test.
pub fn sketch_from_axes<T: Decide>(
    o: Point3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    tol: Tol,
) -> SketchPlane<T> {
    SketchPlane::from_frame(
        OrthoFrame::gram_schmidt(
            o,
            u,
            v,
            FIXTURE_FRAME_AXIS,
            Band::linear(tol).expect("the fixture's tolerance forms a band"),
        )
        .expect("the fixture's two axes span a plane"),
    )
}

/// **The spine frame the tube doors take**: ring centre, spine axis,
/// and the reference radial the window's angles start from.
///
/// The axis is decided and KEPT — it is the frame's `w`, stored
/// verbatim — and the reference yields whatever component of it lies
/// along the axis.
///
/// # Panics
///
/// If the band cannot be formed, if the axis has no direction, or if
/// the reference lies along it. A fixture that wants to exercise one
/// of those refusals calls the mint itself.
pub fn tube_frame<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    tol: Tol,
) -> geom_core::OrthoFrame<T> {
    OrthoFrame::from_axis_and_reference(
        center,
        axis,
        u_ref,
        FIXTURE_FRAME_AXIS,
        Band::linear(tol).expect("the fixture's tolerance forms a band"),
    )
    .expect("the fixture's spine axis has a direction and its reference radial is off it")
}

/// The world xy sketch plane lifted to station `z0`, the placement
/// every axis-aligned fixture here extrudes from.
pub fn sketch_at<T: Decide>(z0: T) -> SketchPlane<T> {
    SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(T::zero(), T::zero(), z0)))
}

/// **A prism on an arbitrary sketch plane**: one closed loop of
/// `verts`, extruded `h` along that plane's normal.
pub fn prism_on<T: Decide>(
    plane: SketchPlane<T>,
    verts: Vec<ProfileVertex<T>>,
    h: T,
    tol: Tol,
) -> Body<T> {
    extruded(plane, vec![ProfileLoop::new(verts)], h, tol)
}

/// **A prism**: one closed profile loop extruded `h` along `+z`.
///
/// The twelfth copy of this four-line helper in the crate's suites was
/// what got it homed. Takes the vertices rather than a shape so the
/// L-prism, the arc-sided prism and the turned box are all one door;
/// panics on an invalid loop, which is a fixture bug, not an outcome.
pub fn prism<T: Decide>(verts: Vec<ProfileVertex<T>>, h: T, tol: Tol) -> Body<T> {
    prism_at(verts, T::zero(), h, tol)
}

/// [`prism`] with its sketch plane lifted to station `z0`: the loop
/// is extruded from `z0` up by `h`. The one home of the lifted
/// extrusion, so a fixture that stacks a prism on or into another body
/// does not re-spell the plane.
pub fn prism_at<T: Decide>(verts: Vec<ProfileVertex<T>>, z0: T, h: T, tol: Tol) -> Body<T> {
    prism_on(sketch_at(z0), verts, h, tol)
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

// ---------------------------------------------------------------------
// The loft prism — the corpus's one NON-AFFINE skinned body.
// ---------------------------------------------------------------------

/// The prism loft's end section: the square `[−1, 1]²`.
pub const PRISM_SQUARE: [(f64, f64); 4] = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];

/// Its middle section: the trapezoid whose two bottom corners flare by
/// ±d, d = 0.375.
///
/// The flare is the whole fixture. It makes the middle section a
/// NON-AFFINE image of the ends, so the four walls are genuinely
/// curved degree-2 nets in v rather than ruled strips — which is why
/// this body, and not a box, is what the NURBS-wall suites measure.
/// `d` is dyadic, so every corner is exact and the ±1.375 spelling and
/// the `±(1 + d)` one are the same bits.
pub const PRISM_TRAPEZOID: [(f64, f64); 4] =
    [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];

/// The v-degree the prism's three sections are interpolated at: one
/// quadratic Bézier span through all three, which is the lowest degree
/// that can bend.
pub const PRISM_V_DEGREE: usize = 2;

/// The heights [`loft_prism`] places its sections at: unit spacing.
pub const PRISM_Z: [f64; 3] = [0.0, 1.0, 2.0];

/// A closed four-line quad section (one loop) — the plainest INTEGRAL
/// profile: unit weights, no arc anywhere.
fn quad_section(pts: [(f64, f64); 4]) -> Section {
    vec![ProfileLoop::polygon(
        pts.iter().map(|&(x, y)| Point2::new(x, y)),
    )]
}

/// **The prism's three sections**, bottom to top: square, trapezoid,
/// square.
///
/// The door for the suites that keep their own PLACEMENTS, which is
/// the suites for which the finished solid is not the subject.
///
/// Two kinds ask for them. One varies the PLACEMENT and is measuring
/// that: the reversed-stacking probe stacks these down the base
/// normal, the STEP fold re-places them at 1 : 2 spacing. The other
/// keeps the standard [`PRISM_Z`] placement but needs what
/// [`loft_prism`] has already thrown away — the [`Lofted`] handoff's
/// wall and cap keys, the geometry `loft_geometry` returns, or the
/// `(sections, places)` pair a row carries beside a second section
/// set to compare the two. A `Body` cannot serve either.
pub fn loft_prism_sections() -> Vec<Section> {
    vec![
        quad_section(PRISM_SQUARE),
        quad_section(PRISM_TRAPEZOID),
        quad_section(PRISM_SQUARE),
    ]
}

/// **Placements: `zs` as pure `+z` translations** — the one home for
/// the four lines every stacked fixture would otherwise re-spell.
///
/// It sits here rather than in a suite because it was seven spellings
/// when this door was written: `sweep/tests/common`'s `stacked`, which
/// now delegates here, and a private `at_z` in each of five `tests/`
/// suites, byte-identical to one another. Only a `src/` home is
/// reachable from all of them — a `tests/` module is a different crate
/// to every other crate's suites, and the fixtures here are placed by
/// `mesh`, `step-export` and `tools/tess-meter` as well.
///
/// Not specific to the loft: [`prism_at`] and [`brick`] place their
/// own sketch planes, and a fixture that stacks anything joins by
/// naming this.
pub fn stacked_at(zs: &[f64]) -> Vec<Affine3<f64>> {
    zs.iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect()
}

/// [`loft_prism`] with its sections placed at `zs` instead of
/// [`PRISM_Z`]. The spacing is the only thing the fold varies, so it
/// is the only thing this door takes.
pub fn loft_prism_at(zs: &[f64], tol: Tol) -> Body<f64> {
    crate::loft_body::<f64>(&loft_prism_sections(), &stacked_at(zs), PRISM_V_DEGREE, tol)
        .expect("the prism's sections skin")
        .body
}

/// **The loft prism**: [`loft_prism_sections`] stacked at [`PRISM_Z`]
/// and skinned at [`PRISM_V_DEGREE`] — 4 described non-rational NURBS
/// walls, 2 planar caps, NURBS seam carriers on the 4 wall–wall edges,
/// V = 9 m³ exactly (derived in `sweep/tests/m6_loft_body.rs`).
///
/// The corpus's first NURBS-walled body, and the one every downstream
/// suite reaches for when it needs curved walls without an arc: the
/// mesher's goldens and budget rows, the STEP fixture, the editor's
/// corpus document and the tessellation meter are all measuring THIS
/// solid, and were each rebuilding it.
pub fn loft_prism(tol: Tol) -> Body<f64> {
    loft_prism_at(&PRISM_Z, tol)
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

/// **Naming totality on any blend result — open bands, corners and
/// closed-rim bands — in every direction the birth records owe.**
///
/// (a) Every output face, edge and vertex is a recorded mint or a
/// survivor of the source; (b) every recorded retirement names a SOURCE
/// key that did not survive; (c) every source entity ABSENT from the
/// output is a recorded retirement — a dead edge or vertex, or an edge a
/// band or blend row replaced; (d) every recorded mint is PRESENT in the
/// output (a stale row naming an entity a later split subdivided away
/// is the shape this direction exists for); (e) no mint is recorded
/// twice, and no mint reuses a source key — except a split FRAGMENT
/// (`meridian_remnants`, `slits`), whose key may be its own parent's,
/// because `split_edge` hands the parent key to one child. (c) is the
/// direction (a) and (b) do not imply: a retirement the surgery forgets
/// to record is invisible to both and to the census delta alike. Also:
/// the band and blend rows together name exactly `requested`. The
/// per-row COUNTS (feet, splits, retired seam vertices) stay in the
/// rows, because they are the fixture's, not the walk's.
pub fn assert_naming_totality<T: Real>(
    source: &Body<T>,
    out: &Blended<T>,
    requested: &[EdgeKey],
    what: &str,
) {
    let rec = out
        .naming
        .as_ref()
        .unwrap_or_else(|| panic!("{what}: the surgery records its births"));
    // Every birth row of every band — the blank phase's, the ruled
    // band's and the rim phase's — so the walk is total over whatever
    // the request carved.
    let mut minted_faces: Vec<FaceKey> = rec
        .blends
        .iter()
        .map(|(f, _)| *f)
        .chain(rec.corners.iter().map(|(f, _)| *f))
        .chain(rec.bands.iter().map(|(f, _)| *f))
        .collect();
    let mut minted_edges: Vec<EdgeKey> = rec
        .rim_trims
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.meridian_remnants.iter().map(|(e, _)| *e))
        .chain(rec.slits.iter().map(|(e, _)| *e))
        .chain(rec.trims.iter().map(|(e, _, _)| *e))
        .chain(rec.arcs.iter().map(|(e, _, _)| *e))
        .collect();
    let mut minted_vertices: Vec<topo::VertexKey> = rec
        .rim_feet
        .iter()
        .map(|(v, _)| *v)
        .chain(rec.meridian_splits.iter().map(|(v, _)| *v))
        .chain(rec.feet.iter().map(|(v, _, _)| *v))
        .collect();
    // (e) recorded once each.
    fn once<K: Ord + Copy>(v: &mut Vec<K>, what: &str, kind: &str) {
        let n = v.len();
        v.sort_unstable();
        v.dedup();
        assert_eq!(n, v.len(), "{what}: a {kind} mint was recorded twice");
    }
    once(&mut minted_faces, what, "face");
    once(&mut minted_edges, what, "edge");
    once(&mut minted_vertices, what, "vertex");
    // (e) no mint reuses a key — the split fragments excepted, whose
    // key may be their own parent's and never an unrelated survivor's.
    for f in &minted_faces {
        assert!(
            source.get_face(*f).is_none(),
            "{what}: a minted face reused a key: {f:?}"
        );
    }
    let fragments: Vec<(EdgeKey, EdgeKey)> = rec
        .meridian_remnants
        .iter()
        .chain(rec.slits.iter())
        .copied()
        .collect();
    for e in &minted_edges {
        match fragments.iter().find(|(k, _)| k == e) {
            Some((_, parent)) => assert!(
                e == parent || source.get_edge(*e).is_none(),
                "{what}: a split fragment carries a key that is neither its parent's nor fresh: {e:?}"
            ),
            None => assert!(
                source.get_edge(*e).is_none(),
                "{what}: a minted edge reused a key: {e:?}"
            ),
        }
    }
    for v in &minted_vertices {
        assert!(
            source.get_vertex(*v).is_none(),
            "{what}: a minted vertex reused a key: {v:?}"
        );
    }
    // (d) every mint is present.
    for f in &minted_faces {
        assert!(
            out.body.get_face(*f).is_some(),
            "{what}: a recorded face mint is absent: {f:?}"
        );
    }
    for e in &minted_edges {
        assert!(
            out.body.get_edge(*e).is_some(),
            "{what}: a recorded edge mint is absent from the output (a superseded row survived): {e:?}"
        );
    }
    for v in &minted_vertices {
        assert!(
            out.body.get_vertex(*v).is_some(),
            "{what}: a recorded vertex mint is absent: {v:?}"
        );
    }
    // (a)
    for (k, _) in out.body.faces() {
        assert!(
            minted_faces.contains(&k) || source.get_face(k).is_some(),
            "{what}: output face {k:?} is neither minted nor a survivor"
        );
    }
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
    // The edges a band replaced: a closed chain's arcs (`bands`) or an
    // open link's edge (`blends`) — together, exactly the request.
    let mut banded: Vec<EdgeKey> = rec
        .bands
        .iter()
        .flat_map(|(_, edges)| edges.iter().copied())
        .chain(rec.blends.iter().map(|(_, e)| *e))
        .collect();
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
    for (k, _) in source.faces() {
        assert!(
            out.body.get_face(k).is_some(),
            "{what}: source face {k:?} vanished — a support shrinks, it does not die"
        );
    }
    banded.sort_unstable();
    let mut want = requested.to_vec();
    want.sort_unstable();
    assert_eq!(
        banded, want,
        "{what}: the band and blend rows name exactly the requested edges"
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
pub fn bowl_at<T: Decide + topo::AtRestPolicy>(tol: Tol) -> Body<T> {
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
pub fn hemisphere_on_flat_base_at<T: Decide + topo::AtRestPolicy>(r: T, tol: Tol) -> Body<T> {
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

/// **The plane×sphere corner where the ball rests OUTSIDE the sphere**,
/// by Pappus — the sphere of radius `big_r` centred at the origin
/// meeting the plane `y = 0` at the rim of radius `big_r`, with the
/// material on the far side of the sphere from that centre and the
/// fillet ball EXTERNALLY tangent to it at radius `r`.
///
/// The sibling of [`plane_sphere_cut`], and a different closed form: the
/// ball's centre is `‖C‖ = R + r` rather than `R − r`, so
/// `C = (sqrt((R+r)^2 - r^2), r)`, the feet are `F_a = (C_x, 0)` and
/// `F_b = C·R/(R+r)` — and `F_b` now lies BETWEEN the centre and `C`,
/// so the sphere's arc between the chord `K`–`F_b` and itself bulges
/// INTO the kite `K, F_a, C, F_b` and its segment is SUBTRACTED, where
/// the internal case adds it. The sector at `C` is subtracted in both.
///
/// It is the boss's dome rim and the dimple's, exactly: the two are
/// mirror images through the rim's own plane, so one form serves both
/// and the SIGN is the material side — the dome rim's 270° material
/// wedge makes it CONCAVE and the band adds this region, the dimple's
/// 90° wedge makes it CONVEX and the band removes it.
#[must_use]
pub fn plane_sphere_external_cut(big_r: f64, r: f64) -> f64 {
    let k = (big_r, 0.0);
    let cx = ((big_r + r).powi(2) - r.powi(2)).sqrt();
    let c = (cx, r);
    let fa = (cx, 0.0);
    let scale = big_r / (big_r + r);
    let fb = (c.0 * scale, c.1 * scale);
    pappus::pappus_volume(&[
        (1.0, pappus::triangle(k, fa, c)),
        (1.0, pappus::triangle(k, c, fb)),
        (-1.0, pappus::segment((0.0, 0.0), big_r, k, fb)),
        (-1.0, pappus::sector(c, r, fa, fb)),
    ])
}

/// **The boss**: a cylinder of radius 1 and height 1 whose flat top runs
/// in to radius 0.5, where a hemisphere of radius 0.5 rises to the pole
/// — `(0,0) (1,0) (1,1) (0.5,1)[bulge tan(π/8)] (0,1.5)` revolved fully.
/// `up` false is its DIMPLE twin, the same hemisphere dug into the top
/// instead (bulge `−tan(π/8)`, apex `(0, 0.5)`).
///
/// Pole-touching, so every wall is minted as two half-bands; the caller
/// decides whether to repair. After `merge_coplanar_faces` the flat top
/// is ONE plane ANNULUS carrying THREE closed rims of three shapes at
/// once, which is why it is the fixture: its BASE rim `(1, 0)` is a
/// hostless annulus on a ring-free host, its TOP OUTER rim `(1, 1)` is a
/// hostless annulus on a host that also carries a RING, and its DOME rim
/// `(0.5, 1)` is that ring and so a LADDER. Census after the repair:
/// `V=7 E=10 F=6`.
pub fn boss(up: bool, tol: Tol) -> Body<f64> {
    // A quarter turn: `tan(theta/4)` at `theta = pi/2`.
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.5, 1.0), if up { q } else { -q }),
            ProfileVertex::new(Point2::new(0.0, if up { 1.5 } else { 0.5 }), 0.0),
        ],
        crate::Revolution::Full,
        tol,
    )
}

/// **The boss with its flat top narrowed** to outer radius `rr`, so the
/// dome rim's widened trim circle can be made to spill past the host's
/// circular outer boundary: `(0,0) (rr,0) (rr,1) (0.5,1)[dome] (0,1.5)`
/// revolved fully. The dome stays at radius 0.5, so the ladder rim's
/// containment margin against that boundary is `rr − √((0.5 + r)² − r²)`
/// and `rr` is the dial. Pole-touching; the caller repairs.
pub fn narrowed_boss(rr: f64, tol: Tol) -> Body<f64> {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(rr, 0.0), 0.0),
            ProfileVertex::new(Point2::new(rr, 1.0), 0.0),
            ProfileVertex::new(Point2::new(0.5, 1.0), q),
            ProfileVertex::new(Point2::new(0.0, 1.5), 0.0),
        ],
        crate::Revolution::Full,
        tol,
    )
}

/// **The boss with its dome grown** to radius `a`, so the ring the flat
/// top carries can be made to reach into the strip the top rim's carve
/// excises: `(0,0) (1,0) (1,1) (a,1)[dome] (0,1+a)` revolved fully. The
/// outer radius stays 1, so the hostless annulus rim's containment margin
/// at fillet radius `r` is `(1 − r) − a` and `a` is the dial.
/// Pole-touching; the caller repairs.
pub fn domed_boss(a: f64, tol: Tol) -> Body<f64> {
    let q = (core::f64::consts::FRAC_PI_2 / 4.0).tan();
    revolved_about_y(
        vec![
            ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 0.0), 0.0),
            ProfileVertex::new(Point2::new(1.0, 1.0), 0.0),
            ProfileVertex::new(Point2::new(a, 1.0), q),
            ProfileVertex::new(Point2::new(0.0, 1.0 + a), 0.0),
        ],
        crate::Revolution::Full,
        tol,
    )
}

/// **The bored cylinder** — one extrude, no boolean. The outer loop is
/// the unit circle with its two vertices at azimuths `outer_phi` and
/// `outer_phi + π`; the INNER loop is a circle of radius `a` centred at
/// `(d, 0)` with its two vertices on the `x` axis. Extruded 1 along `z`.
///
/// Its top cap is one plane face whose OUTER cycle is the top rim (a
/// hostless annulus, mate the outer wall's two half-bands) and whose one
/// RING is the bore's top rim (a ladder rim inside a circular outer
/// boundary). The bore's closest approach to the outer rim is its vertex
/// at `(d + a, 0)`, and with `outer_phi` off the outer rim's own sample
/// lattice (`outer_phi + k·22.5°`) NO sample sits at azimuth 0 — so
/// predicate 2's sampled gap is strictly larger than the true
/// `1 − (d + a)` and the exact ring-clearance forms answer instead. That
/// misalignment is why this fixture reaches the closed-form backstop
/// where every coaxial one is screened first.
pub fn bored_cylinder(a: f64, d: f64, outer_phi: f64, tol: Tol) -> Body<f64> {
    let v = |x: f64, y: f64, b: f64| ProfileVertex::new(Point2::new(x, y), b);
    let outer = ProfileLoop::new(vec![
        v(outer_phi.cos(), outer_phi.sin(), 1.0),
        v(-outer_phi.cos(), -outer_phi.sin(), 1.0),
    ]);
    let inner = ProfileLoop::new(vec![v(d + a, 0.0, -1.0), v(d - a, 0.0, -1.0)]);
    extruded(SketchPlane::xy(), vec![outer, inner], 1.0, tol)
}

/// The whole rim at radius `r` in the plane `z = z0` of a `z`-extruded
/// body, selected by its carrier's stored circle and by whether that
/// circle's centre is OFF the `z` axis — the discriminator a bored
/// cylinder's two coplanar rims need, since both are circles in one
/// plane.
pub fn z_rim(body: &Body<f64>, r: f64, z0: f64, off_axis: bool) -> Vec<EdgeKey> {
    let seed = body
        .edges()
        .find(|(_, e)| {
            let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
                return false;
            };
            matches!(c.carrier(), geom::Curve3::Circle { radius, center, axis, .. }
                if (radius - r).abs() < 1e-9 && (center.z - z0).abs() < 1e-9
                    && axis.z.abs() > 0.9
                    && (center.x.hypot(center.y) > 0.1) == off_axis)
        })
        .map(|(k, _)| k)
        .expect("the rim's seed edge");
    topo::query::rim_of(body, seed).expect("one rim")
}

/// The rod's radius, meters.
pub const ROD_R: f64 = 0.5;
/// The flat's distance from the rod's axis, meters.
pub const ROD_FLAT: f64 = 0.3;
/// The rod's length, meters. **Unity**, so `A_section · L` and
/// `A_section` coincide on this fixture; the factor is pinned by the
/// `L = 2.5` rod in
/// `review_fillet_h7_r2_probes::r2_the_prism_closed_form_scales_with_the_rod_length`.
pub const ROD_L: f64 = 1.0;
/// The fillet radius the rod rows carve at, meters — one home for the
/// rod's four numbers.
pub const ROD_FILLET: f64 = 0.1;

/// **The chord a flat cuts on the [`ROD_R`] circle, and the two arcs it
/// leaves** — see [`rod_chord_at`].
#[derive(Debug, Clone, Copy)]
pub struct RodChord {
    /// Half the chord's length: the flat's half-width, and the offset
    /// of each of its two ends from the foot of the perpendicular.
    pub half: f64,
    /// The bulge of the arc the flat LEAVES STANDING — the D-profile
    /// rod's cylindrical wall — traversed counter-clockwise about the
    /// circle's centre, from the chord end at `+half` to the one at
    /// `−half`.
    pub wall_bulge: f64,
    /// The bulge of the arc the flat CUTS AWAY — the section that
    /// stands on a block's top edge, or sinks into it — traversed
    /// counter-clockwise, the other way round the same two ends.
    pub section_bulge: f64,
}

/// **The chord a plane `flat` from the axis cuts on the [`ROD_R`]
/// circle.** One home for the D-profile's arithmetic: `half` is
/// `sqrt(ROD_R² − flat²)`, and each arc's bulge is `tan(sweep / 4)` of
/// the angle it subtends at the centre, the two sweeps summing to a
/// turn.
///
/// Every ruled fixture in the tree is this chord at some `flat`: the
/// D-profile rod ([`rod_d_profile_of_length_at`]) extrudes the wall
/// arc, and a rod's section standing on — or sunk into — a block's top
/// edge extrudes the section arc. `flat` may be negative (a flat past
/// the axis), and the two fields keep their meanings there: the wall
/// arc is then the shorter of the two.
#[must_use]
pub fn rod_chord_at(flat: f64) -> RodChord {
    let half = (ROD_R.powi(2) - flat.powi(2)).sqrt();
    let wall = 2.0 * (core::f64::consts::PI - half.atan2(flat));
    RodChord {
        half,
        wall_bulge: (wall / 4.0).tan(),
        section_bulge: ((core::f64::consts::TAU - wall) / 4.0).tan(),
    }
}

/// **The rod with a flat milled along it** — the `CylinderPlaneCylinder`
/// consumer: a cylinder of radius [`ROD_R`] about `z` over
/// `z ∈ [0, ROD_L]`, minus a box whose face at `x = ROD_FLAT` planes the
/// flat. Two straight creases (cylinder–plane, along the ruling), each
/// ending in the two caps — planes perpendicular to the ruling, the
/// transverse caps the ruled band is cut off at. Built through the
/// public boolean door, as a user would mill it.
///
/// `f64` only: the boolean door's scalar bound is `Decide + Bounds`, a
/// compound this file is not ratified to spell (the bracket-bound
/// allowlist is per file). The interval twin takes the same body through
/// the extrude door instead — [`rod_d_profile_at`].
///
/// Bit-identical to the pre-delegation body only because `ROD_L = 1.0`
/// makes the general form's `2·len` coincide with the old `len + 1.0`;
/// the bit-dump differential is the guard, not the arithmetic.
pub fn rod_with_flat(tol: Tol) -> Body<f64> {
    rod_with_flat_at(ROD_R, ROD_FLAT, ROD_L, 1.0, tol).unwrap_or_else(|e| panic!("{e}"))
}

/// **A rod with a flat at any radius** — [`rod_with_flat`]'s
/// construction with the rod's radius `big_r`, the flat's distance
/// `flat`, the length `len` and the cutter box's half-width
/// `cutter_half` as parameters; the cutter runs from `−len/2` to
/// `3·len/2` along `z`, so `rod_with_flat` is this at
/// `(ROD_R, ROD_FLAT, ROD_L, 1.0)`, bit for bit. The boolean door keeps
/// the cylinder's stored radius exactly `big_r`, which a D-profile
/// through the extrude door does not (it reconstructs the radius from
/// a chord that collapses as the flat nears tangency). The mill's own
/// refusal comes back as text rather than a panic, so a family walk
/// can report an unbuildable member and go on.
///
/// # Errors
///
/// The boolean door's refusal, rendered.
pub fn rod_with_flat_at(
    big_r: f64,
    flat: f64,
    len: f64,
    cutter_half: f64,
    tol: Tol,
) -> Result<Body<f64>, String> {
    let disc =
        profile::circle(Point2::new(0.0, 0.0), big_r, tol).expect("the rod's disc is a valid loop");
    let rod = extruded(SketchPlane::xy(), vec![disc.into()], len, tol);
    let square = ProfileLoop::new(
        [
            (flat, -cutter_half),
            (cutter_half, -cutter_half),
            (cutter_half, cutter_half),
            (flat, cutter_half),
        ]
        .into_iter()
        .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
        .collect(),
    );
    let cutter = extruded(sketch_at(-0.5 * len), vec![square], 2.0 * len, tol);
    Ok(topo::subtract(&rod, &cutter, tol)
        .map_err(|e| format!("the flat does not mill: {e:?}"))?
        .body()
        .expect("a body remains")
        .body
        .clone())
}

/// **The `+y` crease of a rod with a flat** (or of any body whose
/// cylinder–plane line edges are a rod's two creases): the one whose
/// `he_plus` starts above the axis. One crease, so a band asked for it
/// cannot collide with the other crease's on the flat.
pub fn rod_upper_crease(body: &Body<f64>) -> EdgeKey {
    let creases: Vec<EdgeKey> = rod_creases(body)
        .into_iter()
        .filter(|&k| {
            let e = body.get_edge(k).expect("a crease");
            let v = body.get_half_edge(e.he_plus).expect("its plus half").start;
            let p = body
                .get_point(body.get_vertex(v).expect("its start").point)
                .expect("its point");
            p.y > 0.0
        })
        .collect();
    assert_eq!(creases.len(), 1, "one crease on the +y side: {creases:?}");
    creases[0]
}

/// **The same rod with a flat, spelled as a D-profile extrude**: the
/// chord at `x = ROD_FLAT` and the major arc of the `ROD_R` circle — ONE
/// cap arc, sweeping past π, so the cap's rim is split nowhere and the
/// far foot's split parameter lies a turn off the carrier's principal
/// branch. Same creases, same caps, same closed form as
/// [`rod_with_flat`]; generic over the scalar for the interval twin
/// (the extrude door's bound is `Decide + AtRestPolicy`, no bracket).
pub fn rod_d_profile_at<T: Decide + topo::AtRestPolicy>(tol: Tol) -> Body<T> {
    rod_d_profile_of_length_at(ROD_L, tol)
}

/// [`rod_d_profile_at`] at any length — the one home for the D-rod of
/// a length other than [`ROD_L`], which the prism factor `A · L` and
/// the cap lever are pinned on.
pub fn rod_d_profile_of_length_at<T: Decide + topo::AtRestPolicy>(len: f64, tol: Tol) -> Body<T> {
    let f = T::from_f64;
    let c = rod_chord_at(ROD_FLAT);
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(f(ROD_FLAT), f(c.half)), f(c.wall_bulge)),
        ProfileVertex::new(Point2::new(f(ROD_FLAT), f(-c.half)), f(0.0)),
    ]);
    extruded(SketchPlane::<T>::xy(), vec![lp], f(len), tol)
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
    let v = (flat, (big_r.powi(2) - flat.powi(2)).sqrt());
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
    0.5 * twice - 0.5 * r.powi(2) * theta + 0.5 * big_r.powi(2) * (phi - phi.sin())
}

/// **A disc authored as `n` equal arcs, extruded `h` along `+z`** — a
/// cylinder of radius `r` about the origin whose two rims are each a
/// closed chain of `n` links meeting at `n` junctions, and whose wall
/// is `n` faces of ONE cylinder surface. Every closed-rim suite's
/// revolve fixture splits its rim at exactly one seam (two arcs), so
/// the N-link closed chain for `N ≥ 3` is this builder's shape and no
/// other's. Its raised rim sits in the cap's OUTER cycle and is
/// CONVEX. `n = 2` is the two-semicircle rim the other suites build.
///
/// # Panics
///
/// As [`cylinder_of_arcs_at`].
#[must_use]
pub fn disc_of_arcs(n: usize, r: f64, h: f64, tol: Tol) -> Body<f64> {
    cylinder_of_arcs_at(n, r, Point2::new(0.0, 0.0), 0.0, h, tol)
}

/// **A cylinder of radius `r` about `(c.x, c.y)`, from station `z0`
/// up by `h`, whose circle is authored as `n` equal arcs.** The arcs
/// start at azimuth 0 and run counter-clockwise; each spans `2π/n`,
/// so its bulge is `tan(π/(2n))` (a bulge is `tan(θ/4)` for an arc of
/// turning `θ`).
///
/// # Panics
///
/// On `n < 2` — one vertex cannot author a closed loop — or an invalid
/// profile, both fixture bugs.
#[must_use]
pub fn cylinder_of_arcs_at(
    n: usize,
    r: f64,
    c: Point2<f64>,
    z0: f64,
    h: f64,
    tol: Tol,
) -> Body<f64> {
    prism_at(arc_polygon(n, r, c), z0, h, tol)
}

/// **A block with an `n`-arc bore through it**: the `l × l × h` block
/// with a corner at the origin, bored on its centre by a circle of
/// radius `r` authored as `n` equal arcs. Each of its two bore rims is
/// a closed chain of `n` links in a RING of its cap — and CONVEX: a
/// through-bore's cap rim is a 90° material wedge, the same as the
/// disc's, on the other side of the wall.
///
/// # Panics
///
/// As [`cylinder_of_arcs_at`]; `r` reaching the block's sides is a
/// fixture bug too.
#[must_use]
pub fn bored_block_of_arcs(n: usize, l: f64, h: f64, r: f64, tol: Tol) -> Body<f64> {
    assert!(2.0 * r < l, "the bore must clear the block's sides");
    let outer = ProfileLoop::new(square(l));
    let hole = ProfileLoop::new(arc_polygon(n, r, Point2::new(l / 2.0, l / 2.0)));
    extruded(SketchPlane::xy(), vec![outer, hole], h, tol)
}

/// **A boss on a block**: the cube of side `l` at the origin, unioned
/// with an `n`-arc cylinder of radius `r` on its centre that starts
/// inside the block at `z0 < l` and stands `h` tall, so it protrudes
/// through the top. The boss's FOOT rim, at station `l`, is a closed
/// chain of `n` links in a RING of the block's top face — and
/// CONCAVE, the material-adding twin of [`bored_block_of_arcs`]'s rim.
///
/// # Panics
///
/// On a boss that does not protrude, or a union the boolean refuses —
/// fixture bugs.
#[must_use]
pub fn boss_of_arcs(n: usize, l: f64, r: f64, z0: f64, h: f64, tol: Tol) -> Body<f64> {
    assert!(
        z0 < l && z0 + h > l,
        "the boss must start inside the block and protrude"
    );
    let boss = cylinder_of_arcs_at(n, r, Point2::new(l / 2.0, l / 2.0), z0, h, tol);
    realized(BooleanOp::Union, &cube(l, tol), &boss, tol)
}

/// **A pocket in a block**: the cube of side `l` at the origin, minus
/// an `n`-arc cylinder of radius `r` on its centre from station
/// `floor < l` up past the top. The pocket's FLOOR rim, at station
/// `floor`, is a closed chain of `n` links in the floor face's OUTER
/// cycle — and CONCAVE, the material-adding twin of [`disc_of_arcs`]'s
/// rim.
///
/// # Panics
///
/// On a floor at or above the top, or a subtract the boolean refuses —
/// fixture bugs.
#[must_use]
pub fn pocket_of_arcs(n: usize, l: f64, r: f64, floor: f64, tol: Tol) -> Body<f64> {
    assert!(
        floor < l,
        "the pocket's floor must sit below the block's top"
    );
    let tool = cylinder_of_arcs_at(n, r, Point2::new(l / 2.0, l / 2.0), floor, l, tol);
    realized(BooleanOp::Subtract, &cube(l, tol), &tool, tol)
}

/// **The realized boolean of two fixtures, as a body** — `op` through
/// the public door with no declarations and `SweepStrategy::Realized`,
/// the body unwrapped. The one spelling: a suite that builds a boss, a
/// pip or a bore calls this rather than the eleven lines it replaces.
///
/// # Panics
///
/// If the boolean refuses or leaves no body — a fixture bug, louder as
/// a panic than as an empty answer.
#[must_use]
pub fn realized(op: BooleanOp, a: &Body<f64>, b: &Body<f64>, tol: Tol) -> Body<f64> {
    boolean_op_with(
        op,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol,
    )
    .unwrap_or_else(|e| panic!("the fixture's {op:?} builds, got {e}"))
    .body()
    .expect("the fixture's boolean is a body")
    .body
    .clone()
}

/// The `n` bulged vertices of a circle of radius `r` about `c`,
/// authored as `n` equal arcs starting at azimuth 0.
fn arc_polygon(n: usize, r: f64, c: Point2<f64>) -> Vec<ProfileVertex<f64>> {
    assert!(n >= 2, "a closed loop of arcs needs at least two vertices");
    let bulge = (core::f64::consts::PI / (2.0 * n as f64)).tan();
    (0..n)
        .map(|i| {
            let th = 2.0 * core::f64::consts::PI * (i as f64) / (n as f64);
            ProfileVertex::new(Point2::new(c.x + r * th.cos(), c.y + r * th.sin()), bulge)
        })
        .collect()
}

/// **Every arc whose stored carrier is a circle centred at station
/// `z`**, in key order — the raw scan, seeded through no rim door.
///
/// The z-poled twin of [`arcs_at`]'s scan, for the extruded fixtures
/// above — the fifth z-poled scan in the tree, and the instance
/// `work/blend/seed-finder-home-reads-only-the-y-station` records
/// against the day the home reads a station on either axis. It is
/// station-only where [`arcs_at`] also filters by radius and excludes
/// co-surface circles, and that is right for THESE fixtures rather
/// than in general: each builder above mints one circle per station
/// (the seams it leaves are lines, so no co-surface circle exists),
/// which the scan checks by requiring every arc it finds to share one
/// carrier radius — two rims at one station are then a loud fixture
/// bug rather than a silent union.
///
/// Deliberately NOT routed through [`topo::query::rim_of`]: that door
/// matches arcs by bit-identical carrier circles, and `extrude` stores
/// each arc of one authored circle on its own centre and radius, so it
/// refuses every rim these builders mint
/// (`work/blend/rim-of-refuses-extruded-multi-arc-rims`).
///
/// # Panics
///
/// If the arcs at `z` do not share one radius (within `1e-9`).
#[must_use]
pub fn circle_arcs_at_z(body: &Body<f64>, z: f64) -> Vec<EdgeKey> {
    let found: Vec<(EdgeKey, f64)> = body
        .edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            match c.carrier() {
                geom::Curve3::Circle { center, radius, .. } if (center.z - z).abs() < 1e-9 => {
                    Some((k, *radius))
                }
                _ => None,
            }
        })
        .collect();
    if let Some(&(_, r0)) = found.first() {
        assert!(
            found.iter().all(|(_, r)| (r - r0).abs() < 1e-9),
            "one rim at station z = {z}: the arcs there do not share one radius"
        );
    }
    found.into_iter().map(|(k, _)| k).collect()
}

/// **A full revolve's rim is the two arcs its one seam splits it
/// into** — the fact eight suites state before carving a revolved
/// rim, said once. Two is the fixture's shape, not the door's limit:
/// the N-arc rims are `closed_chain_junctions`' subject.
///
/// # Panics
///
/// If `arcs` is not exactly two, naming `name`.
pub fn assert_full_revolve_rim(arcs: &[EdgeKey], name: &str) {
    assert_eq!(
        arcs.len(),
        2,
        "{name}: the rim is the two arcs a full revolve's one seam splits it into"
    );
}

/// **The chains the battery's walk builds from `edges`** — each link
/// resolved as a fillet of `size`, then walked into maximal chains —
/// BEFORE any predicate judges them. A suite reads a junction's vertex
/// and its two links off this on a chain the G1 check would refuse as
/// well as on one it admits, which `run_battery`'s verdict cannot show.
///
/// # Panics
///
/// If an edge does not resolve to a link: a fixture whose edge the
/// battery cannot even describe is not one to walk.
#[must_use]
pub fn walked_chains(
    body: &Body<f64>,
    edges: &[EdgeKey],
    size: f64,
    band: Band,
) -> Vec<Chain<f64>> {
    let links = edges
        .iter()
        .map(|&e| resolve_link(body, e, size, band, BlendKind::Fillet))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("every requested edge resolves to a link, got {e}"));
    walk_chains(links)
}
