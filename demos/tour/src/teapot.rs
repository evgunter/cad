//! **The teapot** — `shell`'s designated demo, and what it measured.
//!
//! The register has named this scene since 2026-08-09: *"the Utah
//! teapot is this verb's designated demo — a vessel is a shelled
//! revolve; the demo queues behind the verb."* The verb landed at
//! #1048. This is the demo, and its job is not to look like a teapot:
//! it is to be the first thing outside `shell`'s own acceptance corpus
//! that asks the verb for a real part and to report, in numbers, what
//! came back.
//!
//! Four bodies, and they are FOUR SOLIDS rather than one. That is the
//! honest exit shape (the lily precedent), and everything below says
//! which parts of it are the kernel's answer and which are the
//! modeller's:
//!
//! - **the pot** — one `revolve` of one meridian, hollowed by `shell`
//!   into a wall and a cavity in one solid. Drawn see-through, because
//!   a cavity is invisible in an opaque render at every camera (the
//!   hollow ring's founding reason, on this verb's own shape).
//! - **the lid** — a second `revolve`, its knob's top rim rolled
//!   through the one-edge annulus band. Rendered LIFTED above the
//!   mouth: an exploded view. No mate is authored and none is implied
//!   — declared contact is M9's territory, and a scene that faked one
//!   here would be claiming a certification nothing issued.
//! - **the spout** — a cone-frustum tube, built about its own axis and
//!   placed by `transform_rigid`.
//! - **the handle** — one `tube_along_arc` window, its two roots
//!   driven through the belly wall — 11.2 mm past it, measured, which
//!   is what makes the union a real request and is also why a teapot
//!   built this way would leak (see `HANDLE_OVER`).
//!
//! # Findings this scene records (the demo-purpose rule)
//!
//! 1. **`shell`'s sealed arm survived exactly ONE junction shape, it
//!    was never about curvature, and it is now REPAIRED.** The verb
//!    replaced one chart at a time and re-anchored the neighbours'
//!    edges on carriers that had not moved yet, so a junction survived
//!    exactly when the neighbouring surface was invariant under the
//!    moved face's own offset motion: a plane normal to a cylinder's
//!    axis, both ways, and nothing else. A right prism on a TRIANGLE
//!    refused the same way as a cone frustum, which is what ruled
//!    curvature out.
//!
//!    **#1081 made the offsets SIMULTANEOUS**, in two halves. PR-2a
//!    solves an all-planar body's corner as `nᵢ·x = cᵢ` over every
//!    moved plane meeting it, so the hexagon, the bevel, the kite and
//!    the triangular prism hollow. PR-2b solves a body of REVOLUTION's
//!    corners in its own meridian half-plane, where a plane normal to
//!    the axis is a line, a cylinder is a line, a cone is a line and a
//!    sphere is a circle — so a corner is one line/line or line/circle
//!    meeting, closed form, with every further surface verified against
//!    it and the seam's azimuth carried as the conventional datum it
//!    is. **That is what un-squared this pot.** Its belly was a SPHERE
//!    ZONE and therefore on the curved side, and the meridian this
//!    scene ships is now the one the model always wanted: base, foot,
//!    ONE ARC, mouth. The three squared segments — shoulder, belly,
//!    shoulder — are gone, and so is the wall that pinned their
//!    refusal.
//!
//!    **The frontier moved rather than vanished**, and the scene's
//!    wall 1 moved with it — twice, and it is now gone. Pushing the
//!    belly's arc centre OFF the axis makes its wall a TORUS, which
//!    the meridian reduction did not know; the body fell to the
//!    per-chart loop and the C5 table refused its plane×torus pair.
//!    The reduction knows the kind now — a coaxial torus's meridian is
//!    a circle centred `(R, h_c)`, the sphere's circle with one more
//!    number — so that pot hollows and the probe below asserts it
//!    rather than pinning a refusal. A curved
//!    junction can also be TANGENT, and a tangent junction has no
//!    transversal corner to solve at all: the conditioning meter says
//!    so in the geometry's own terms, which is why the bullet still
//!    refuses where the pot's own foot-to-belly junction — the same
//!    surface pair — does not. `tests/verbs_teapot.rs` is that table,
//!    its sweep, and the sweep's stated blind spot. None of this was a
//!    gap the verb announced: `shell`'s acceptance corpus is a box, a
//!    cylinder between two caps and a tube between two caps — every
//!    fixture in the surviving class, and the class was never named.
//! 2. **The OPENED arm was wrong on every solid of revolution, and is
//!    the fixed thing this scene now ships.** This scene was specified
//!    to be `shell_open`'s first consumer past acceptance, opening the
//!    pot at its mouth so the wall's thickness shows as an annular
//!    rim. What it found was a body that passed tiers 1, 2 and 3 while
//!    each designated half-disc carried a RING that was its own cavity
//!    counterpart's boundary: reaching the axis apex the outer loop
//!    already owned, running back along that loop's seam legs. Genus 1
//!    where `topo::shell`'s docs say *"one opening gives a cup, which
//!    is genus 0"*, and the CDT refused an insertion on a mouth
//!    half-disc. That was wall 2 (#1082); the pot shipped sealed and
//!    this teapot had no opening.
//!
//!    **The mechanism was NOT "a revolve's cap is two half-discs".**
//!    The adopted review fixtures falsified that: a revolved TUBE's
//!    mouth is ONE face and was wrong too, and a partial revolve's cap
//!    is one face and touches the axis. The class is *a designated
//!    face whose cavity counterpart's boundary cannot become an
//!    interior-disjoint RING of it*, and what put a revolved cap out
//!    of reach was the REVOLVE's SEAM — an axis apex two half-discs
//!    share, or a radial slit an annular cap's loop walks twice. That
//!    is a fact about the sweep and not about the mouth, so
//!    `shell_open` retires it before the glue, through the Euler doors
//!    alone; on an annular cap the rim it then builds is TWO DISJOINT
//!    ANNULI, a face split, which the same doors express. The
//!    invariant is stated at rest as well: tier 3 refuses a ring
//!    standing on its own face's outer loop.
//!
//!    Nor was the path unvisited before:
//!    `offd2_r1_probes::probe_opened_vessel_cup` already opened a
//!    revolved vessel and checked only the things that were right —
//!    tier 3, the shell count, the volume — never the rings, the genus
//!    or the mesh. That is the transferable lesson, and that probe now
//!    checks all three. **The pot ships OPENED**, one annular rim,
//!    genus 0, meshing — and it also LEAVES AS STEP, which the sealed
//!    two-shell body could not.
//! 3. **A steam vent is what makes the lid's knob filletable.** The
//!    one-edge annulus band carves a CLOSED latitude rim, and a full
//!    revolve mints one only from an ANNULAR profile; a profile that
//!    touches the axis mints half-walls whose rims are two open arcs.
//!    So the lid is bored — a vent through the finial, which is a real
//!    teapot's answer as well as the kernel's. What is measured HERE is
//!    the positive half: the bored lid's knob rim is a closed edge and
//!    it rolls, with the band's census and its two tangency lines
//!    checked below. The negative half is measured ELSEWHERE and cited
//!    rather than re-asserted — `verbs_arms1_r1_probes::the_unbored_
//!    hemisphere_equator_refuses_typed` is the axis-touching profile's
//!    own pin, and the register's ARMS-1 row states the bound. The
//!    scene is where a PART met it while trying to be a part: the pot
//!    touches the axis at both ends, so nothing on it is a candidate
//!    and the lid had to be bored to have one.
//! 4. **The teapot is four solids because the operand gate has no arm
//!    for either join.** handle ∪ pot is torus × cylinder; spout ∪ pot
//!    is cone × plane. Both refuse `CurvedPairUnsupported`, pinned in
//!    walls 2 and 3 with the payload carried verbatim into the panel's
//!    note. Read what the second one NAMES: the spout's outer cone
//!    against a PLANE of the pot — not the belly wall the spout
//!    actually pierces. The gate is pair-scoped and box-conservative,
//!    so it reports the first pair whose boxes MAY meet, and a reader
//!    who took the refusal's text for the cause would be reading the
//!    wrong pair (the wall-7 lesson). The schedule is the banked
//!    germ-chord lanes and #1057's two C5 arms.
//! 5. **A spout the shape of a spout is not authorable at all.** What
//!    a potter draws is a swept curved section — a canal or a loft
//!    along a bent spine. `sweep_body` cannot round the U-turn a real
//!    spout takes and there is no variable-section sweep, so the shape
//!    on screen is a straight cone frustum tilted into place: a spout
//!    the way a lathe would make one. Recorded as a register note
//!    rather than worked around, because a hand-built stand-in would
//!    be evidence about this file and not about the library.
//!
//! # What this scene deliberately does NOT do
//!
//! No kernel change, no route widened, no gate softened. Every one of
//! the five findings above is a live probe or an executed table, and
//! each carries the sentence that retires it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use pncad::authoring::{p2, validated};
use pncad::geom::{Curve3, Surface};
use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec2, Vec3};
use pncad::prelude::{Open, Start, fillet_edges};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, TubeWindow, revolve, tube_along_arc};
use pncad::topo::{Body, BooleanError, EdgeKey, FaceKey, Operand};

use crate::{SceneBody, Stop, View};

// ---------------------------------------------------------------------
// The vessel's meridian. Every station is a dyadic rational in meters,
// so the profile's own arithmetic is exact and the closed forms below
// are compared against numbers no rounding entered.
// ---------------------------------------------------------------------

/// The foot's radius — the cylinder the pot stands on.
const R_FOOT: f64 = 4.0 / 64.0;
/// The belly's sphere radius — the pot's widest wall, at `Y_BELLY_C`.
const R_BELLY: f64 = 5.0 / 64.0;
/// The mouth's radius, which the lid's rim reads.
const R_NECK: f64 = 3.0 / 64.0;
/// Where the foot ends and the belly's arc begins.
const Y_FOOT: f64 = 1.0 / 64.0;
/// The belly sphere's centre, on the axis. The two junction stations
/// are the sphere's own 3-4-5 points: `(4/64, 1/64)` at the foot and
/// `(3/64, 8/64)` at the mouth, both with an exactly zero residual.
const Y_BELLY_C: f64 = 4.0 / 64.0;
/// The mouth's plane.
const Y_MOUTH: f64 = 8.0 / 64.0;

/// The wall thickness. A tenth of the belly's radius, and an eighth of
/// the narrowest wall the shell has to survive (the neck's), so every
/// per-face reach margin below is definite by a wide margin rather
/// than by a hair.
const WALL: f64 = 1.0 / 128.0;

/// The NURBS fit tolerance `shell` would hand its approximating lane.
/// Unread here: every wall of this pot is analytic, so the offsets are
/// closed forms and nothing is fitted.
const FIT_TOL: f64 = 1e-6;

// ---------------------------------------------------------------------
// The lid: a second solid of revolution, lifted.
// ---------------------------------------------------------------------

/// How far above the mouth the lid renders — the exploded-view gap.
/// No mate is authored and none is implied; see the module docs.
const LIFT: f64 = 1.0 / 32.0;
/// The lid's underside plane.
const LID_BASE: f64 = Y_MOUTH + LIFT;
/// The dome's sphere radius, and its centre's station: the 5-12-13
/// triple. What that buys is not that any of these decimals is a
/// binary-exact float — 3/64 and 13/256 are, 0.6 and 0.8 would not be
/// — but that the RESIDUALS are exactly zero: `|c − p|² − r²` for the
/// rim and for the knob's foot evaluate to 0.0 in f64, so the arc door
/// has nothing to round when it checks equidistance.
const DOME_R: f64 = 13.0 / 256.0;
/// The dome sphere's centre, BELOW the lid's underside.
const DOME_C: f64 = LID_BASE - 5.0 / 256.0;
/// The knob's radius — the 5-12-13 point of the dome circle.
const R_KNOB: f64 = 5.0 / 256.0;
/// Where the dome ends and the knob's wall begins.
const Y_KNOB: f64 = LID_BASE + 7.0 / 256.0;
/// The knob's top.
const Y_TOP: f64 = LID_BASE + 12.0 / 256.0;
/// The steam vent bored through the finial — which is also what makes
/// the lid's profile ANNULAR, and therefore what makes its latitude
/// rims closed edges. See the module docs' finding 3.
const R_VENT: f64 = 1.0 / 256.0;
/// The roll on the knob's top rim.
const ROLL: f64 = 2.0 / 256.0;

// ---------------------------------------------------------------------
// The spout and the handle.
// ---------------------------------------------------------------------

/// The spout's length along its own axis.
const SPOUT_LEN: f64 = 8.0 / 64.0;
/// The spout's outer radius at the root, and at the tip.
const SPOUT_R0: f64 = 6.0 / 256.0;
const SPOUT_R1: f64 = 3.0 / 256.0;
/// The spout's wall.
const SPOUT_WALL: f64 = 1.0 / 256.0;
/// The spout's root, inside the belly.
const SPOUT_ROOT: Point3<f64> = Point3 {
    x: -1.0 / 32.0,
    y: 3.0 / 64.0,
    z: 0.0,
};
/// The spout's axis: the 3-4-5 direction. 0.6 and 0.8 are NOT
/// binary-exact floats; what is exact is the residual — the rotation
/// matrix built from them satisfies `cᵢ·cⱼ − δᵢⱼ == 0.0` in f64, so
/// `transform_rigid`'s orthonormality decide has nothing to round.
const SPOUT_DIR: Vec3<f64> = Vec3 {
    x: -0.8,
    y: 0.6,
    z: 0.0,
};

/// The handle's spine radius: half the chord between its two roots on
/// the belly wall, so the unextended window is exactly a semicircle and
/// the handle stands one radius clear of the pot at its widest. The
/// roots span 46.875 mm of the belly's 78.125 mm of height.
const HANDLE_R: f64 = 6.0 / 256.0;
/// The handle's tube radius.
const HANDLE_TUBE: f64 = 1.0 / 128.0;
/// The handle's spine centre, ON the belly wall at the belly's own
/// mid-height — so the semicircle's two ends land on the wall and its
/// far side stands `HANDLE_R` proud of it.
const HANDLE_C: Point3<f64> = Point3 {
    x: R_BELLY,
    y: Y_BELLY_C,
    z: 0.0,
};
/// How far past the semicircle each end of the handle runs, in radians
/// of its own spine, so that each root PENETRATES the belly rather than
/// touching it — which is what makes the union attempted below a real
/// request rather than a tangency.
///
/// **It penetrates all the way through, and the geometry says it must.**
/// The spine's centre sits ON the outer wall, so at the semicircle's own
/// ends the tube already reaches `HANDLE_TUBE` inward — and this tube's
/// radius IS the wall thickness, so the cap is flush with the cavity at
/// zero overshoot and inside it at any positive one. At 0.5 rad the
/// deepest material stands `HANDLE_R·sin(0.5) + HANDLE_TUBE` = 19.0 mm
/// below the outer wall, which is 11.2 mm into the tea. Nothing
/// asserted here depends on that (the union refuses at the operand
/// gate, before any intersection work), and the scene keeps the
/// overshoot rather than thinning the handle to hide it — but a teapot
/// built this way would leak, and re-cutting it is the FIRST thing the
/// wall-3 retire note asks for if the union ever composes.
const HANDLE_OVER: f64 = 0.5;

/// The scene's chord budget.
const DELTA: f64 = 2e-4;

// ---------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------

/// One full revolve of `lp` about the sketch's own `+y` axis.
fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("the meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian fully revolves")
    .body
}

/// **The vessel's meridian**: base disc, foot, belly, mouth disc —
/// the shoulders and the belly are ONE ARC about a centre on the axis,
/// which makes the wall a SPHERE ZONE and the pot the shape a potter
/// would throw.
///
/// This is the meridian the model always wanted. Until #1081's PR-2b it
/// was wall 1, pinned by its refusal: `shell` moved one chart at a time
/// and a corner where a plane meets a sphere is not where transporting
/// it under either one alone puts it. The simultaneous door solves that
/// corner against both surfaces at once, so the arc ships.
///
/// Every station is a dyadic rational and the sphere is the 3-4-5
/// triple twice over: radius `5/64` about `(0, 4/64)`, meeting the foot
/// cylinder at `(4/64, 1/64)` and the mouth at `(3/64, 8/64)`, so both
/// junctions' residuals `|c − p|² − r²` are exactly `0.0` in f64 and
/// the closed forms below compare against numbers no rounding entered.
fn vessel_meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(R_FOOT, 0.0), tol)
        .expect("the base disc")
        .line_to(Point2::new(R_FOOT, Y_FOOT), tol)
        .expect("the foot")
        .arc_to(
            Center {
                c: Point2::new(0.0, Y_BELLY_C),
                winding: ArcSweep::Ccw,
                p: Point2::new(R_NECK, Y_MOUTH),
            },
            tol,
        )
        .expect("the belly rides a sphere centred on the axis")
        .line_to(Point2::new(0.0, Y_MOUTH), tol)
        .expect("the mouth disc")
        .line_to(Start, tol)
        .expect("the axis closes the meridian")
        .into()
}

/// **The lid's meridian**: underside annulus, dome, knob wall, knob
/// top annulus, vent bore.
fn lid_meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(Point2::new(R_VENT, LID_BASE))
        .line_to(Point2::new(R_NECK, LID_BASE), tol)
        .expect("the underside seats on the mouth rim")
        .arc_to(
            Center {
                c: Point2::new(0.0, DOME_C),
                winding: ArcSweep::Ccw,
                p: Point2::new(R_KNOB, Y_KNOB),
            },
            tol,
        )
        .expect("the dome rides a sphere centred on the axis")
        .line_to(Point2::new(R_KNOB, Y_TOP), tol)
        .expect("the knob's wall")
        .line_to(Point2::new(R_VENT, Y_TOP), tol)
        .expect("the knob's top")
        .line_to(Start, tol)
        .expect("the vent closes the meridian")
        .into()
}

/// **The spout's meridian**: an annular trapezoid — a cone frustum
/// with a cone frustum bored out of it, one wall thick.
fn spout_meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(Point2::new(SPOUT_R0 - SPOUT_WALL, 0.0))
        .line_to(Point2::new(SPOUT_R0, 0.0), tol)
        .expect("the root annulus")
        .line_to(Point2::new(SPOUT_R1, SPOUT_LEN), tol)
        .expect("the outer cone")
        .line_to(Point2::new(SPOUT_R1 - SPOUT_WALL, SPOUT_LEN), tol)
        .expect("the tip annulus")
        .line_to(Start, tol)
        .expect("the bore closes the meridian")
        .into()
}

/// The placement that takes the spout's own `+y` axis onto
/// [`SPOUT_DIR`] and its root onto [`SPOUT_ROOT`]. The rotation is the
/// 3-4-5 turn about `+z`, so every entry is exact.
fn spout_placement() -> Affine3<f64> {
    Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(SPOUT_DIR.y, -SPOUT_DIR.x, 0.0),
            SPOUT_DIR,
            Vec3::unit_z(),
        ),
        Vec3::new(SPOUT_ROOT.x, SPOUT_ROOT.y, SPOUT_ROOT.z),
    )
}

/// Every planar face of `body` whose plane sits at station `y` — the
/// CHART GROUP, not a face: a full revolve cuts each wall at two seam
/// meridians, so the mouth is two half-discs sharing one plane, and
/// `shell_open` lifts a chart as one (`ShellError::OpenFaceChartPartial`
/// is what a half designation gets).
fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// The one closed latitude rim of `body` whose circle sits at station
/// `y` with radius `r` — the selection said BY DESCRIPTION, by hand,
/// because a directly revolved body has no selector (the register's
/// standing gap; `bud::rims_between` and `klein::corner_edges` are the
/// same scan).
fn rim_at(body: &Body<f64>, y: f64, r: f64) -> EdgeKey {
    let hits: Vec<EdgeKey> = body
        .edges()
        .filter(|(k, _)| {
            let Some(c) = body
                .get_curve_geom(body.get_edge(*k).expect("the edge").curve)
                .and_then(|g| g.certified())
            else {
                return false;
            };
            matches!(*c.carrier(), Curve3::Circle { center, radius, .. }
                if (center.y - y).abs() < 1e-12 && (radius - r).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "the description (station {y}, radius {r}) names exactly one rim"
    );
    hits[0]
}

// ---------------------------------------------------------------------
// The closed forms — derived HERE from the authored stations, never
// read back from the bodies they check.
// ---------------------------------------------------------------------

/// The volume of a stack of coaxial cylindrical segments `(radius,
/// height)`.
fn stack_volume(segments: &[(f64, f64)]) -> f64 {
    PI * segments.iter().map(|(r, h)| r * r * h).sum::<f64>()
}

/// A spherical zone's own two closed forms about a sphere of radius
/// `r` centred at station `c`, between stations `y0` and `y1`:
/// `(∫π x² dy, Archimedes' 2πr·Δy)`.
fn zone(r: f64, c: f64, y0: f64, y1: f64) -> (f64, f64) {
    let (a, b) = (y0 - c, y1 - c);
    (
        PI * (r * r * (b - a) - (b * b * b - a * a * a) / 3.0),
        2.0 * PI * r * (y1 - y0),
    )
}

/// A right cone frustum's volume between radii `r0` and `r1` over
/// height `h`.
fn frustum_volume(r0: f64, r1: f64, h: f64) -> f64 {
    PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
}

/// A right cone frustum's LATERAL area between radii `r0` and `r1`
/// over height `h`.
fn frustum_lateral(r0: f64, r1: f64, h: f64) -> f64 {
    PI * (r0 + r1) * ((r0 - r1) * (r0 - r1) + h * h).sqrt()
}

/// An annulus's area.
fn annulus(ro: f64, ri: f64) -> f64 {
    PI * (ro * ro - ri * ri)
}

/// A door's answer, as one line for the panel's note — the refusal's
/// own payload rather than a sentence about it (the wall-7 lesson: a
/// refusal's TEXT is not evidence of its cause; the payload and the
/// raising site are).
fn describe<T, E: core::fmt::Debug>(outcome: &Result<T, E>) -> String {
    match outcome {
        Ok(_) => "COMPOSED".to_string(),
        Err(e) => format!("{e:?}"),
    }
}

/// The genus of `body` by the Euler–Poincaré identity
/// `v − e + f − r = 2(s − g)`, summed over shells.
///
/// The identity's left side is EVEN on any body the identity applies
/// to, so an odd one is not a body with a surprising genus — it is a
/// census that does not satisfy Euler–Poincaré at all, and halving it
/// would turn that into a plausible number. Checked before the divide
/// rather than after, because after is too late.
///
/// Duplicated, deliberately, in `tests/verbs_teapot.rs`: a binary's
/// module cannot be imported by an integration test, and the two
/// copies are three lines of a published identity rather than a shared
/// invariant. The tie between them is that both are checked against
/// the same measured censuses.
/// **One of NINE copies of this helper across five crates (#1123).**
/// `demos/tour` is a separate workspace and an integration test cannot
/// import a binary's module, so no existing home covers them all; the
/// issue carries the list and the shared-test-support fix.
fn genus(body: &Body<f64>) -> i64 {
    let (v, e, f) = (
        body.vertices().count() as i64,
        body.edges().count() as i64,
        body.faces().count() as i64,
    );
    let r: i64 = body.faces().map(|(_, x)| x.rings.len() as i64).sum();
    let chi = v - e + f - r;
    assert!(
        chi % 2 == 0,
        "v - e + f - r = {chi} is ODD, so this census does not satisfy \
         Euler-Poincare and no genus follows from it"
    );
    body.shells().count() as i64 - chi / 2
}

/// The station where the foot cylinder meets the belly sphere, at
/// inward offset `d`: the sphere shrinks concentrically and the
/// cylinder shrinks radially, so their meeting slides ALONG the
/// meridian. This is the corner the simultaneous door solves, written
/// independently here — and it is what a transported corner gets
/// wrong.
fn pot_junction(d: f64) -> f64 {
    let (rr, rf) = (R_BELLY - d, R_FOOT - d);
    Y_BELLY_C - (rr * rr - rf * rf).sqrt()
}

/// The pot's boundary radius on the belly sphere at station `y` and
/// inward offset `d`.
fn belly_radius(y: f64, d: f64) -> f64 {
    let rr = R_BELLY - d;
    (rr * rr - (y - Y_BELLY_C) * (y - Y_BELLY_C)).sqrt()
}

/// The volume the pot's boundary encloses at inward offset `d`: the
/// foot's cylinder up to the junction, then the SPHERICAL ZONE from
/// there to the mouth. `π∫ρ²dy` in two pieces, with the second one the
/// zone integral `π[R²y − y³/3]` about the sphere's own centre.
fn pot_volume(d: f64) -> f64 {
    let (rf, rr) = (R_FOOT - d, R_BELLY - d);
    let (y0, y1) = (pot_junction(d), Y_MOUTH - d);
    let zone = |y: f64| {
        let u = y - Y_BELLY_C;
        rr * rr * u - u * u * u / 3.0
    };
    PI * rf * rf * (y0 - d) + PI * (zone(y1) - zone(y0))
}

/// The area of the pot's boundary at inward offset `d`: the base cap,
/// the foot's wall, the belly zone's lateral area (`2πRh`, Archimedes)
/// and the mouth cap.
fn pot_area(d: f64) -> f64 {
    let (rf, rr) = (R_FOOT - d, R_BELLY - d);
    let (y0, y1) = (pot_junction(d), Y_MOUTH - d);
    PI * rf * rf
        + 2.0 * PI * rf * (y0 - d)
        + 2.0 * PI * rr * (y1 - y0)
        + PI * belly_radius(y1, d) * belly_radius(y1, d)
}

// ---------------------------------------------------------------------
// The scene
// ---------------------------------------------------------------------

pub fn stops(tol: Tol) -> Vec<Stop> {
    // ---- the vessel, before the wall ----
    let bellied = revolved(vessel_meridian(tol), tol);
    assert_eq!(
        (
            bellied.vertices().count(),
            bellied.edges().count(),
            bellied.faces().count(),
        ),
        (8, 14, 8),
        "four revolved meridian segments (the fifth is the axis and sweeps nothing), \
         each cut at the two seam meridians into a pair of half-walls — one arc where \
         the squared pot spent three segments on shoulder, belly, shoulder"
    );

    // ---- the gates, MEASURED off the operand before the verb runs ----
    //
    // A plane's own reach is unbounded, so its per-face collapse margin
    // is vacuous and `wall_clearance` is what stands in for it: every
    // antiparallel non-adjacent planar pair must clear 2t. The OUTWARD
    // normal is the stored one turned by the face's sense bit, which is
    // what the gate reads and what a scan trusting the surface alone
    // would get wrong: a revolve stores its caps' planes with one
    // normal and lets the face's orientation say which way each looks,
    // so a sense-blind scan finds NO antiparallel pair on this pot at
    // all. The count that stored `+y` goes in the note, measured.
    let planes: Vec<(f64, f64)> = bellied
        .faces()
        .filter_map(|(_, f)| match bellied.get_surface(f.surface) {
            Some(Surface::Plane { origin, normal, .. }) => {
                Some((origin.y, if f.sense { normal.y } else { -normal.y }))
            }
            _ => None,
        })
        .collect();
    let stored_plus = planes.len()
        - bellied
            .faces()
            .filter(|(_, f)| {
                matches!(bellied.get_surface(f.surface),
                    Some(Surface::Plane { normal, .. }) if normal.y < 0.0)
            })
            .count();
    // Not merely reported: EVERY planar face of this pot stores `+y`,
    // and that is the whole content of the sense-bit warning below. A
    // number only printed is a number nothing checks — if a revolve
    // ever stores a cap's plane the other way round, this fails here
    // and the note's sentence gets re-derived rather than silently
    // becoming a different true statement.
    assert_eq!(
        stored_plus,
        planes.len(),
        "every planar face of this pot stores a +y normal; only the face's own sense \
         bit says which way each looks"
    );
    let mut clearance = f64::INFINITY;
    for (i, a) in planes.iter().enumerate() {
        for b in &planes[i + 1..] {
            if a.1 * b.1 < 0.0 {
                clearance = clearance.min((b.0 - a.0).abs());
            }
        }
    }
    assert!(
        clearance.is_finite(),
        "this pot HAS antiparallel planar pairs — a scan that found none read the \
         stored normal without its face's sense bit"
    );
    assert!(
        clearance > 2.0 * WALL,
        "the closest antiparallel planar pair clears {clearance} m and two walls need {}",
        2.0 * WALL
    );
    let reach = bellied
        .faces()
        .filter_map(|(_, f)| match bellied.get_surface(f.surface) {
            Some(Surface::Cylinder { radius, .. }) => Some(*radius - WALL),
            _ => None,
        })
        .fold(f64::INFINITY, f64::min);
    assert!(
        reach > 0.0,
        "every cylinder's realized inner radius is positive; the tightest has {reach} m left"
    );

    // ---- the sealed hollow: the body the scene SHIPS ----
    let pot = pncad::topo::shell(&bellied, WALL, FIT_TOL, tol)
        .unwrap_or_else(|e| panic!("the pot hollows, got {e}"));
    let (pv, pe, pf) = (
        pot.vertices().count(),
        pot.edges().count(),
        pot.faces().count(),
    );
    assert_eq!(
        (pv, pe, pf),
        (16, 28, 16),
        "the operand's 8/14/8, twice: the cavity is the same boundary offset inward, \
         inserted whole through the shared void door"
    );
    assert_eq!(
        pot.shells().count(),
        2,
        "the outer boundary and the cavity, in ONE solid"
    );
    assert_eq!(genus(&pot), 0, "two sphere-like shells, no handles");

    // The wall as a NUMBER, against the two stacks — and the cavity's
    // capacity asked for DIRECTLY rather than inferred from a
    // difference, which is the number a potter actually wants.
    let v_out = pot_volume(0.0);
    let v_cav = pot_volume(WALL);
    let v_want = v_out - v_cav;
    let a_want = pot_area(0.0) + pot_area(WALL);
    let props = pncad::topo::mass_properties(&pot, tol).expect("the pot's props");
    assert!(
        ((props.volume - v_want) / v_want).abs() < 1e-12,
        "pot V = {} vs the wall's own closed form {v_want}",
        props.volume
    );
    assert!(
        ((props.surface_area - a_want) / a_want).abs() < 1e-12,
        "pot A = {} vs the closed form {a_want}",
        props.surface_area
    );
    assert_eq!(props.volume_pad, 0.0, "closed forms need no pad");
    let classes = pncad::topo::classify_shells(&pot, tol).expect("per-shell classification");
    let voids: Vec<_> = classes
        .iter()
        .filter(|c| c.role == pncad::topo::ShellRole::Void)
        .collect();
    assert_eq!(
        voids.len(),
        1,
        "one cavity, one Void shell — DECIDED, not declared"
    );
    assert!(
        ((voids[0].volume + v_cav) / v_cav).abs() < 1e-12,
        "the pot holds {} m³ against the cavity stack's own {v_cav}, negated by the \
         orientation convention",
        voids[0].volume
    );
    let outers: Vec<_> = classes
        .iter()
        .filter(|c| c.role == pncad::topo::ShellRole::Outer)
        .collect();
    assert_eq!(outers.len(), 1, "one outer boundary");
    assert!(
        ((outers[0].volume - v_out) / v_out).abs() < 1e-12,
        "the outer shell encloses {} m³ against the outer stack's {v_out}",
        outers[0].volume
    );
    let capacity_l = v_cav * 1000.0;

    // ---- the opened hollow: the body the scene SHIPS ----
    //
    // The SEALED census above is the sealed arm's own evidence and
    // stays: it is where the two-shell insertion, the decided Void
    // role and the capacity are read. What the montage draws is the
    // OPENED pot, because a teapot has a mouth.
    let mouth = plane_chart_at(&bellied, Y_MOUTH);
    let cup = pncad::topo::shell_open(&bellied, WALL, &mouth, FIT_TOL, tol)
        .unwrap_or_else(|e| panic!("the pot opens at its mouth, got {e}"));

    // ---- the lid ----
    let plain_lid = revolved(lid_meridian(tol), tol);
    assert_eq!(
        (
            plain_lid.vertices().count(),
            plain_lid.edges().count(),
            plain_lid.faces().count(),
        ),
        (5, 10, 5),
        "an ANNULAR profile mints ONE full wall per segment — five walls, five closed \
         latitude rims and five seam meridians — where the pot's axis-touching profile \
         mints half-walls and open arcs"
    );
    let knob_rim = rim_at(&plain_lid, Y_TOP, R_KNOB);
    let rolled = fillet_edges(&plain_lid, &[knob_rim], ROLL, tol)
        .unwrap_or_else(|e| panic!("the knob's cylinder x plane top rim rolls, got {e:?}"));
    assert_eq!(
        (
            rolled.body.vertices().count(),
            rolled.body.edges().count(),
            rolled.body.faces().count(),
        ),
        (6, 12, 6),
        "the annulus band's own census delta: +1 vertex, +2 edges, +1 face"
    );
    assert_eq!(rolled.band_faces.len(), 1, "one rim, one band");
    let (band_major, band_minor) = {
        let f = rolled.band_faces[0];
        match rolled
            .body
            .get_surface(rolled.body.get_face(f).expect("the band face").surface)
        {
            Some(Surface::Torus {
                major_radius,
                minor_radius,
                center,
                ..
            }) => {
                assert!(
                    center.x.abs() < 1e-12
                        && center.z.abs() < 1e-12
                        && (center.y - (Y_TOP - ROLL)).abs() < 1e-12,
                    "the band's spine circle sits one roll below the knob's top, on the \
                     axis: got {center:?}"
                );
                (*major_radius, *minor_radius)
            }
            other => panic!("a coaxial cylinder x plane band is a torus, got {other:?}"),
        }
    };
    // The band re-derived from the two tangency conditions rather than
    // read back from the arm that minted it: on a cylinder of radius
    // R_KNOB the rolling ball's centre rides at R_KNOB - r; on a plane
    // normal to the axis it rides r below it. Both are lines in the
    // meridian half-plane, and they cross once.
    assert!(
        (band_minor - ROLL).abs() < 1e-12 && (band_major - (R_KNOB - ROLL)).abs() < 1e-12,
        "the band is the torus (R_KNOB - roll, roll) = ({}, {ROLL}); got ({band_major}, \
         {band_minor})",
        R_KNOB - ROLL
    );
    assert!(
        rolled
            .body
            .get_face(rolled.band_faces[0])
            .expect("the band face")
            .rings
            .is_empty(),
        "a curved band is ring-free: one cycle, two closed trim circles and a slit"
    );

    // The SHARP lid against its closed forms — the dome's area by
    // Archimedes, the rest by the stack — and the roll then bounded
    // against that twin rather than against a remembered number.
    let (v_dome, a_dome) = zone(DOME_R, DOME_C, LID_BASE, Y_KNOB);
    let v_lid = v_dome + stack_volume(&[(R_KNOB, Y_TOP - Y_KNOB)])
        - stack_volume(&[(R_VENT, Y_TOP - LID_BASE)]);
    let a_lid = a_dome
        + 2.0 * PI * R_KNOB * (Y_TOP - Y_KNOB)
        + annulus(R_KNOB, R_VENT)
        + annulus(R_NECK, R_VENT)
        + 2.0 * PI * R_VENT * (Y_TOP - LID_BASE);
    let sharp_lid_props = pncad::topo::mass_properties(&plain_lid, tol).expect("the lid's props");
    assert!(
        ((sharp_lid_props.volume - v_lid) / v_lid).abs() < 1e-12,
        "lid V = {} vs the closed form {v_lid}",
        sharp_lid_props.volume
    );
    assert!(
        ((sharp_lid_props.surface_area - a_lid) / a_lid).abs() < 1e-12,
        "lid A = {} vs the closed form {a_lid}",
        sharp_lid_props.surface_area
    );
    let lid_props = pncad::topo::mass_properties(&rolled.body, tol).expect("the rolled lid");
    let dv_lid = sharp_lid_props.volume - lid_props.volume;
    let pappus_cap = 2.0 * PI * R_KNOB * ROLL * ROLL;
    assert!(
        dv_lid > 0.0 && dv_lid < pappus_cap,
        "a convex rim's roll removes material, and less than the corner square swept \
         round the rim: ΔV = {dv_lid} against the bound {pappus_cap}"
    );

    // ---- the spout: built about its own axis, then placed ----
    let spout =
        pncad::topo::transform_rigid(&revolved(spout_meridian(tol), tol), &spout_placement(), tol)
            .expect("the spout is placed by a rigid map");
    let v_spout = frustum_volume(SPOUT_R0, SPOUT_R1, SPOUT_LEN)
        - frustum_volume(SPOUT_R0 - SPOUT_WALL, SPOUT_R1 - SPOUT_WALL, SPOUT_LEN);
    let a_spout = frustum_lateral(SPOUT_R0, SPOUT_R1, SPOUT_LEN)
        + frustum_lateral(SPOUT_R0 - SPOUT_WALL, SPOUT_R1 - SPOUT_WALL, SPOUT_LEN)
        + annulus(SPOUT_R0, SPOUT_R0 - SPOUT_WALL)
        + annulus(SPOUT_R1, SPOUT_R1 - SPOUT_WALL);
    let spout_props = pncad::topo::mass_properties(&spout, tol).expect("the spout's props");
    // Asserted AFTER the placement, which is the point: the closed
    // forms are stated in the spout's OWN frame, so the rigid map is
    // what has to have left them alone.
    assert!(
        ((spout_props.volume - v_spout) / v_spout).abs() < 1e-12,
        "spout V = {} vs the frustum difference {v_spout}",
        spout_props.volume
    );
    assert!(
        ((spout_props.surface_area - a_spout) / a_spout).abs() < 1e-12,
        "spout A = {} vs the closed form {a_spout}",
        spout_props.surface_area
    );

    // ---- the handle ----
    let sweep = 2.0 * (FRAC_PI_2 + HANDLE_OVER);
    let handle = tube_along_arc::<f64>(
        HANDLE_C,
        Vec3::unit_z(),
        Vec3::unit_x(),
        HANDLE_R,
        TubeWindow::Arc {
            t0: -(FRAC_PI_2 + HANDLE_OVER),
            t1: FRAC_PI_2 + HANDLE_OVER,
        },
        HANDLE_TUBE,
        tol,
    )
    .expect("the handle's arc tube builds")
    .body;
    let v_handle = sweep * HANDLE_R * PI * HANDLE_TUBE * HANDLE_TUBE;
    let a_handle = sweep * HANDLE_R * 2.0 * PI * HANDLE_TUBE + 2.0 * PI * HANDLE_TUBE * HANDLE_TUBE;
    let handle_props = pncad::topo::mass_properties(&handle, tol).expect("the handle's props");
    assert!(
        ((handle_props.volume - v_handle) / v_handle).abs() < 1e-12,
        "handle V = {} vs Pappus on the disc {v_handle}",
        handle_props.volume
    );
    assert!(
        ((handle_props.surface_area - a_handle) / a_handle).abs() < 1e-12,
        "handle A = {} vs the closed form {a_handle}",
        handle_props.surface_area
    );

    // ---- the walls ----
    //
    // Every one of these is ATTEMPTED here, on every pass, and pinned
    // by its own EXACT typed refusal: a different refusal, or a
    // success, fails the tour — so the findings list in the module
    // docs cannot rot behind a frontier that moved.

    // WALL 1 — RETIRED at #1081's PR-2b, and the retirement is the
    // pot above: the belly IS the arc now. What this wall pinned was
    // the sealed hollow of a sphere-zone meridian refusing
    // `ReanchorOffCarrier`, and it refused because `shell` moved one
    // chart at a time. The simultaneous door solves each corner
    // against every surface meeting it, so the arc ships and the
    // squared shoulders are gone from the scene entirely.
    //
    // WALL 1, RE-PLANTED and now RETIRED IN TURN. The re-planted wall
    // pushed the arc's centre OFF the axis, making the belly a TORUS —
    // a kind the meridian reduction did not know, so the body fell to
    // the per-chart loop and refused at the C5 table's plane×torus
    // pair. The reduction knows the kind now: a coaxial torus's
    // meridian is a circle centred `(R, h_c)` in the `(ρ, h)`
    // half-plane, which is the sphere's circle centred `(0, h_c)` with
    // one more number, so the corner solve takes it and never asks the
    // table. The pot below is ATTEMPTED live, exactly as the wall was,
    // and it hollows.
    let torus_belly = revolved(
        Open.at(Point2::new(0.0, 0.0))
            .line_to(Point2::new(R_FOOT, 0.0), tol)
            .expect("the base disc")
            .line_to(Point2::new(R_FOOT, Y_FOOT), tol)
            .expect("the foot")
            // The SAME two junction stations and the SAME 5/64 radius,
            // about the OTHER centre on their perpendicular bisector:
            // `(7/64, 5/64)`, which is off the axis. Both residuals are
            // still exactly zero (3-4-5 twice again) — the only thing
            // that changed is that the revolve now mints a TORUS.
            .arc_to(
                Center {
                    c: Point2::new(7.0 / 64.0, 5.0 / 64.0),
                    winding: ArcSweep::Cw,
                    p: Point2::new(R_NECK, Y_MOUTH),
                },
                tol,
            )
            .expect("a belly about a centre off the axis is a torus")
            .line_to(Point2::new(0.0, Y_MOUTH), tol)
            .expect("the mouth disc")
            .line_to(Start, tol)
            .expect("the axis closes the meridian")
            .into(),
        tol,
    );
    let torus_pot = pncad::topo::shell(&torus_belly, WALL, FIT_TOL, tol)
        .expect("a pot bellied about a centre off the axis hollows through the axial door");
    assert_eq!(
        pncad::topo::validate_geometric(&torus_pot, tol),
        Ok(()),
        "the torus-bellied pot: tier 3"
    );
    assert_eq!(
        torus_pot.shells().count(),
        2,
        "the torus-bellied pot's hollow is outer + cavity"
    );
    println!(
        "   wall 1 — RETIRED: the torus-bellied pot hollows ({} faces over two shells); \
         its junction corners are pinned to their closed forms in \
         crates/sweep/tests/torax_axial.rs",
        torus_pot.faces().count()
    );

    // THE MOUTH, OPENED — the scene's second finding, RETIRED. This
    // was wall 2: `shell_open` returned a body that passed tiers 1-3
    // while each designated half-disc carried its cavity counterpart's
    // own boundary as a ring, and the CDT refused it (#1082). It is
    // measured here rather than cited, because the pot the montage
    // ships is now this body.
    assert_eq!(
        mouth.len(),
        2,
        "the mouth is one PLANE worn by two half-disc faces — a full revolve's seam cut \
         — and the rim lift moves a chart as one"
    );
    assert_eq!(
        pncad::topo::validate_geometric(&cup, tol),
        Ok(()),
        "tier 3 on the cup, which now also refuses a ring standing on its outer loop"
    );
    let cup_rings: usize = cup.faces().map(|(_, f)| f.rings.len()).sum();
    assert_eq!(
        (cup_rings, genus(&cup), cup.shells().count()),
        (1, 0, 1),
        "ONE rim annulus carrying ONE ring, genus 0 as `topo::shell`'s docs promise a \
         cup is, and the cavity fused into the boundary"
    );
    assert_eq!(
        plane_chart_at(&cup, Y_MOUTH).len(),
        1,
        "the revolve's seam is retired before the glue, so the mouth plane is worn by \
         one rim face and not by two half-annuli"
    );
    let cup_props = pncad::topo::mass_properties(&cup, tol).expect("the cup's props");
    // The cup is the sealed wall LESS the disc of wall the mouth cap
    // was: opening lifts the cavity's cap from the mouth plane's own
    // station minus a wall, up to the plane.
    // The lifted mouth disc is NOT a cylinder any more: the cavity's
    // wall over that slab is the belly SPHERE, so the plug the lift
    // opens is the zone integral between the cavity's own top station
    // and the mouth plane. Reading it as a cylinder was right only
    // while the neck was one.
    let cup_want = v_want - {
        let rr = R_BELLY - WALL;
        let zone = |y: f64| {
            let u = y - Y_BELLY_C;
            rr * rr * u - u * u * u / 3.0
        };
        PI * (zone(Y_MOUTH) - zone(Y_MOUTH - WALL))
    };
    assert!(
        ((cup_props.volume - cup_want) / cup_want).abs() < 1e-12,
        "cup V = {} vs the wall's closed form less the lifted mouth disc {cup_want}",
        cup_props.volume
    );
    let cup_mesh = pncad::mesh::tessellate(&cup, DELTA, tol)
        .unwrap_or_else(|e| panic!("the opened pot must triangulate, got {e:?}"));
    let cup_triangles: usize = cup_mesh.patches.iter().map(|q| q.triangles.len()).sum();
    assert!(cup_triangles > 0, "a mesh with no triangles is not a mesh");

    // WALL 2 — the handle joined to the pot. A curved x curved pair at
    // the operand gate; the germ roster has no arm for it.
    //
    // Each union is run ONCE and its refusal carried into the panel's
    // note verbatim, so the payload the caption quotes and the payload
    // the probe pinned cannot be two different measurements.
    let handle_union = pncad::topo::union(&cup, &handle, tol);
    let handle_refusal = describe(&handle_union);
    crate::walls::wall(
        "teapot",
        2,
        "join the handle to the vessel (union; both roots driven 11.2 mm past the \
         belly's inner wall — a real overlap, not a tangency)",
        handle_union,
        |e| {
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::B,
                    kind: SurfaceKind::Torus,
                    // The pot's belly is a SPHERE now, not the squared
                    // pot's cylinder: same gate, and the pair it names
                    // is the pair the geometry actually has.
                    other_kind: SurfaceKind::Sphere,
                    ..
                }
            )
        },
        "make the teapot ONE solid: union the handle and the spout into the vessel, drop \
         walls 2 and 3, re-state the montage caption (which currently says four solids), \
         and RE-CUT THE HANDLE'S OVERSHOOT FIRST — at 0.5 rad its roots stand 11.2 mm \
         inside the cavity, which is fine for a refused request and wrong for a joined one",
    );

    // WALL 3 — the spout joined to the pot. A DIFFERENT pair, and it
    // is not the pair the model cares about: the gate is pair-scoped
    // and box-conservative, so the faces it names are the first whose
    // boxes MAY meet — here the spout's outer cone against a PLANE of
    // the pot, not the belly wall the spout actually pierces.
    let spout_union = pncad::topo::union(&cup, &spout, tol);
    let spout_refusal = describe(&spout_union);
    crate::walls::wall(
        "teapot",
        3,
        "join the spout to the vessel (union; the root disc wholly inside the belly)",
        spout_union,
        |e| {
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    op: None,
                    operand: Operand::B,
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Plane,
                    ..
                }
            )
        },
        "make the teapot ONE solid: union the handle and the spout into the vessel, drop \
         walls 2 and 3, re-state the montage caption (which currently says four solids), \
         and RE-CUT THE HANDLE'S OVERSHOOT FIRST — at 0.5 rad its roots stand 11.2 mm \
         inside the cavity, which is fine for a refused request and wrong for a joined one",
    );

    vec![Stop {
        name: "teapot",
        caption: "THE TEAPOT (FOUR solids: the opened pot, the lid lifted, and the \
                  spout and handle it will not join)"
            .to_string(),
        montage: true,
        story: "shell's designated demo. The pot is ONE revolved profile hollowed by \
                `shell` and OPENED at its mouth: a cavity the size of the tea, inside a \
                wall 7.8 mm thick, in one solid — drawn see-through, because a cavity \
                cannot be read from an opaque render at any camera. The lid is a SECOND solid, rendered lifted: \
                an exploded view, not a mate. The spout and the handle are two more, \
                and their unions with the pot are attempted on every pass and REFUSE — \
                so what the montage shows is four bodies sitting where a teapot's parts \
                sit, not a teapot",
        ops: "revolve(meridian, +y, Full) -> shell_open(pot, t = 7.8125 mm, the mouth's \
              chart) for the vessel; revolve + fillet_edges(the knob's top rim) for the \
              lid; revolve + transform_rigid for the spout; tube_along_arc for the \
              handle. Two walls pinned: both unions",
        delta: DELTA,
        note: Some(format!(
            "THE VESSEL, SEALED THEN OPENED. Sealed it is {pv} vertices, {pe} edges, \
             {pf} faces over TWO shells in one solid — the operand's 8/14/8 twice, \
             since the cavity is that same boundary offset inward and inserted whole \
             through the shared void door. Genus 0. V = {:.9} m³ of WALL against the \
             difference of two closed-form SPHERICAL ZONES on a foot, and A = {:.9} m² \
             against the two boundaries' own; zero enclosure pad. The capacity is asked for \
             DIRECTLY of THAT body rather than inferred: `classify_shells` gives the \
             cavity the Void role and its signed volume is {:.9} m³, the cavity's own closed form, \
             its {capacity_l:.4} LITRES negated by the orientation convention — a \
             reading the OPENED pot cannot give, because its cavity is no longer a \
             void. The gates, measured on the operand before the verb \
             ran: the closest antiparallel planar pair clears {clearance} m where two \
             walls need {:.7} m, so `wall_clearance` — which is what stands in for a \
             plane's vacuous reach margin — does not bind; the tightest cylinder has \
             {reach} m of realized inner radius left. Note the sense bit in that scan: \
             {stored_plus} of this pot's {} planar FACES store a +y normal and only the \
             face's own orientation says which way each looks, so a sense-blind scan \
             finds no antiparallel pair here at all. THE BELLY IS AN ARC, AND THAT IS THE \
             SCENE'S FIRST FINDING RETIRED. It read THE BELLY IS STILL SQUARED for \
             two waves: `shell` moved ONE chart at a time and re-anchored its \
             neighbours on carriers that had not moved, so a junction survived only \
             where the neighbouring surface was invariant under the moved face's own \
             offset — and the class was OBLIQUE junctions rather than curvature, a \
             triangular prism of all planes refusing exactly like a cone frustum. \
             #1081 made the offsets SIMULTANEOUS: every corner is solved against all \
             the surfaces meeting it at once, planar corners by a 3×3 solve (PR-2a) \
             and a body of revolution's in its own meridian half-plane (PR-2b), where \
             a plane is a line, a cylinder is a line, a cone is a line and a sphere is \
             a circle. So the shoulders are gone: foot cylinder, ONE spherical zone, \
             mouth. The frontier moved rather than vanished, and wall 1 has moved with \
             it — push the belly's arc centre OFF the axis and the wall is a TORUS, for \
             which the C5 table has no closed-form arm at all (`tests/verbs_teapot.rs` \
             carries the junction table, the tangency discriminator and the sweep's \
             blind spot). THE MOUTH IS \
             OPEN, AND THAT WAS THE SCENE'S SECOND FINDING BEFORE IT WAS FIXED: \
             `shell_open` used to return a body that passed tiers 1-3 while each \
             designated half-disc carried its own cavity counterpart's boundary as a \
             ring — sharing the axis apex the outer loop owned, running back along its \
             seam legs — so the CDT refused it and the pot shipped sealed (#1082). The \
             class was never \"a revolve's cap is two half-discs\": it was any \
             designated face whose cavity counterpart's boundary cannot become an \
             interior-disjoint RING of it, and what put it out of reach was the \
             REVOLVE's seam rather than the mouth. The seam is retired before the glue \
             now, through the Euler doors alone, and this pot's mouth comes back as ONE \
             annular rim carrying one ring, genus {}, V = {:.9} m³ against the wall's \
             closed form less the lifted mouth disc, {cup_triangles} triangles at δ = \
             {DELTA}. On an ANNULAR mouth the same class needs two disjoint annuli — a \
             face SPLIT — and that is built too; `tests/verbs_teapot.rs` carries both \
             shapes. THE OPENED POT NOW LEAVES AS STEP, which the sealed one could not: \
             the writer's outward/void classifier has closed forms for planar faces \
             only, so a CURVED solid of two or more shells refuses \
             CurvedShellClassification — and a cup is ONE shell. That gate keeps three \
             live probes (klein's wall 6, the `ring` scene's and `hollowtorus`'s); this \
             scene is no longer one of them, because the body it ships no longer \
             reaches it. THE LID. 5/10/5 sharp — an \
             ANNULAR profile mints one FULL wall per segment where the pot's \
             axis-touching profile mints half-walls — and 6/12/6 rolled, the annulus \
             band's own (+1, +2, +1). The band is the ring-free torus ({band_major}, \
             {band_minor}), which is ({}, {ROLL}) re-derived here from the two tangency \
             lines rather than read back from the arm that minted it, centred one roll \
             below the knob's top. ΔV = {dv_lid:.9} m³, inside the corner-square bound \
             {pappus_cap:.9}. The steam vent is what makes that rim a CLOSED edge at \
             all: bore the finial and the profile is annular; leave it solid and the rim \
             is two arcs over two half-discs, which the one-edge annulus band does not \
             carve. NO MATE IS AUTHORED — the lid renders {LIFT} m above the mouth and \
             the two bodies are strangers to the kernel; declared contact is M9's. THE \
             SPOUT AND THE HANDLE. Both build and both check against closed forms — the \
             spout as a difference of cone frusta, asserted AFTER `transform_rigid` \
             placed it, so the map's isometry is part of the receipt; the handle by \
             Pappus on its own disc. Neither JOINS. handle ∪ vessel: {handle_refusal}. \
             spout ∪ vessel: {spout_refusal}. Both are the pair-scoped operand gate \
             naming a germ PAIR with no wired arm, and note what the second one names — \
             the spout's outer CONE against a PLANE of the pot, not the belly wall the \
             spout actually pierces: box overlap is a MAY, and the gate reports the \
             first pair whose boxes may meet. The schedule is the banked germ-chord \
             lanes (DESIGN frontier (d)) and #1057's two C5 arms. A lofted or \
             canal-swept spout — the shape a potter would draw — is not authorable at \
             all; the register carries that as a note and this scene does not hack \
             around it",
            props.volume,
            props.surface_area,
            voids[0].volume,
            2.0 * WALL,
            planes.len(),
            genus(&cup),
            cup_props.volume,
            R_KNOB - ROLL,
        )),
        // The pot's axis is +y and its spout, handle and lid knob all
        // lie in the world z = 0 plane, so a camera near -z sees the
        // silhouette a teapot is recognised by. Ten degrees off it
        // toward the spout, and 22 up: enough elevation that the
        // shoulders and the lid's dome read as ellipses rather than
        // lines, and little enough that the lid's lift stays a visible
        // GAP rather than foreshortening onto the mouth.
        view: View {
            elev: 22.0,
            azim: -100.0,
            up: 'y',
        },
        bodies: vec![
            // See-through for the hollow ring's reason, on this verb's
            // own shape: the subject is a CAVITY, and no camera reads
            // one from an opaque render.
            SceneBody::plain("teapotvessel", [0.72, 0.70, 0.66], cup).transparent(45),
            SceneBody::plain("teapotlid", [0.58, 0.64, 0.72], rolled.body),
            SceneBody::plain("teapotspout", [0.72, 0.70, 0.66], spout),
            SceneBody::plain("teapothandle", [0.58, 0.64, 0.72], handle),
        ],
    }]
}
