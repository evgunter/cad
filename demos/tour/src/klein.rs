//! **The Klein bottle** — the tour's non-orientable stop, and the
//! kernel's densest wall list so far.
//!
//! # What is actually built
//!
//! A Klein bottle is a 2-manifold, and this kernel is manifold-first
//! and SOLID-first (D1): a zero-thickness sheet is not a body it can
//! hold. So the model is the honest 3-D stand-in Ev asked for — a
//! THIN 3-manifold with boundary whose midsurface is the classic
//! immersed Klein bottle. Every piece is a wall of thickness
//! [`WALL`] wrapped around a spine, and the spine is the bottle's own
//! tube:
//!
//! - the **bulb** — the neck, the flaring body wall, the wide bottom
//!   rim it turns back on, and the straight tube that comes back UP
//!   through that rim's hole — is ONE full `revolve` of ONE meridian
//!   band about the z-axis. Cylinder, torus, cone, torus, cylinder,
//!   two annular caps: seven surface runs, each appearing twice (once
//!   per wall face), all exact analytic surfaces.
//! - the **top loop** — the handle that carries the neck over the top
//!   and back down INTO the bulb — is ONE `sweep_body` of the annular
//!   cross-section along the loop's whole spine: a 270° arc over the
//!   top, then a 90° arc turning back onto the bottle's axis, joined
//!   tangent as four rational quarter arcs of one curve. Two planar
//!   caps and eight rational NURBS walls (finding 5).
//!
//! The two bodies MEET on two annular faces, each of the loop's caps
//! flat on the bulb's rim it meets (finding 11).
//! They are not joined, because nothing in the kernel can join them
//! (wall 3). The loop's descent passes THROUGH the bulb's cone wall
//! with no hole cut in it, because nothing in the kernel can cut it
//! (wall 4) — which is, by luck, exactly how the classic picture is
//! drawn, but it is a refusal and not a choice.
//!
//! # Why the construction has this shape
//!
//! Ev's sketch was: torus loop → fillet → cone → tangent → wide
//! torus → fillet → straight tube → back to the loop. Two of those
//! steps do not survive contact with the geometry, and both
//! substitutions are recorded here rather than hidden:
//!
//! **(a) "the top loop is just a torus" cannot close.** The loop must
//! leave the neck along the bottle's axis and arrive at the inner
//! tube along that same axis. A circle meets a line in at most two
//! points and is TANGENT to it at only one, so one circular arc
//! cannot be tangent to the axis at two different heights: no single
//! torus has both of the loop's ends. Two tangent arcs can, and that
//! is what the loop is. (This is geometry, not a kernel limit, and
//! it does not make the loop two bodies: one sweep carries the
//! annulus along both arcs.)
//!
//! **(b) "fillet that torus–cone junction".** Taken literally — blend
//! the loop's torus against the bulb's cone — the two supports share
//! no axis, so the rolling ball's spine is neither a line nor a
//! circle and the blend is a general canal surface: DESIGN frontier
//! (f), the hardest thing in the register, not a missing table row.
//! Taken as a modeller means it, the junction wants a tangent NECK
//! between the loop and the flare, and then the blend is a surface of
//! revolution about the bottle's own axis — a torus. That torus is in
//! the meridian band as an ARC, authored by `.fillet(r)` on the
//! profile, so the bulb's every blend is exact and free. The same
//! trick is what makes the wide rim and its two tangencies exact, and
//! it generalizes: **any** blend between two COAXIAL surfaces of
//! revolution is itself one, so it belongs in the meridian rather
//! than in a rolling ball afterwards (Ev, 2026-08-16 — the
//! substitution is an improvement on the sketch, not a workaround for
//! it). Asking `fillet_edges` for that same torus on the sharp band's
//! full revolve rolls it, and the rolled bulb has the authored one's
//! volume (findings entry 3); on a partial revolve it is wall 2.
//!
//! # The findings (each one a wall probe below, except where noted)
//!
//! 1. **There is no `shell`, and this model is nothing but shell.**
//!    Every wall here is authored as its own two offsets: the
//!    meridian band spells the tube radius twice (`R ± WALL/2`) and
//!    every blend radius twice (`R_f ∓ WALL/2`, and the sign depends
//!    on which side of the spine the centre of curvature is on), and
//!    [`meridian`] computes the offset tangent points the author
//!    would otherwise have to transcribe. Nothing about that is
//!    wrong; it is just the whole `shell / offset` row of
//!    `docs/KERNEL-VERBS.md` being paid for by hand, once per wall.
//!    NOT a probe: there is no verb to call, so there is no refusal
//!    to pin.
//! 2. **A closed rim now meters an honest lever arm** (the full
//!    revolve's roll and wall 2 — the pair is the evidence). A full revolve's latitude
//!    rims are CLOSED circles: start vertex == end vertex, so an
//!    endpoint chord collapses to ~0 there. The battery's lever arm
//!    is the maximum pairwise chord over the samples
//!    `{t0, mid, t1}`, which meters a full circular rim at ~its
//!    diameter — so the neck→flare corner on the FULL revolve DECIDES
//!    its 30° dihedral, as the SAME profile revolved PARTIALLY does.
//!    (This pair once split: the
//!    endpoint-chord lever read ~0 on the closed rim and the
//!    dihedral classifier decided Zero — a false `TangentialEdge` on
//!    a 30° corner, #554. Both forms are still probed back to back
//!    because agreement between them is exactly what #554 restored.)
//! 3. **`fillet_edges` rolls the blend the meridian draws** (not a
//!    wall; wall 2 is its open-rim twin). The cone×cylinder arm is
//!    coaxial, and the ball rolls outside both neck walls' cylinders
//!    and outside the flare's cone — the turn's centre lies away from
//!    the axis — so no support bends toward it and the curvature
//!    headroom sets no limit at `RF ± WALL/2`. The rolled full revolve
//!    is tier-3 valid with the authored bulb's volume. What bounds the
//!    inner corner's radius is the wall it stands in: past r ≈ 1.47 the
//!    band crosses the outer wall and the reach meter refuses it
//!    (`FaceClearance`). The meridian arc stays the authoring: it is
//!    exact and free, and the roll is the check that the two agree. On
//!    the partial revolve the open rim's ends meet the cut faces, and
//!    that corner does not classify (`UnsupportedCorner {
//!    Indeterminate }`, wall 2).
//! 4. **Neither join can start** (walls 3, 4). `union` refuses at the
//!    contact itself: the sweep reads the seam line of the bulb's outer
//!    neck cylinder, which ends on the rim at z = 3 that the loop's top
//!    cap meets, against one of the loop's four NURBS walls rising from
//!    that rim, and the Boolean has no crossing layer for a spline face
//!    (`CurvedBooleanUnsupported { operand: B, kind: Nurbs }`). The rims
//!    themselves are circles; no seam far from the contact is read
//!    first. `subtract` answers
//!    first at the revert roster with `{ op: Some(Subtract), kind:
//!    Nurbs, other_kind: Plane }` — a NURBS wall of the loop against a
//!    planar face of the bulb, a pair whose boxes MAY meet (box overlap
//!    over-approximates; the kernel cannot rule the meeting out, a
//!    weaker claim than that they do). So the bottle cannot be one
//!    body, and the self-intersection — the neck piercing the bulb,
//!    the one place a Klein bottle MUST cross itself in 3-space —
//!    cannot be trimmed.
//! 5. **The loop is ONE swept body** (walls 5 and 8, both retired).
//!    The sweep carries the annulus, two `circle`s, around the U-turn —
//!    its stacking statement is the loft's fold over adjacent station
//!    pairs, each decided against both its sections' normals (issue
//!    368). Its mesh was wall 8 (its semicircle walls' C0 knot, which
//!    the mesher refused until #5499 cut along it); its tier-3 sign
//!    was wall 5 (the sign walk stopped at the quadrature's reporting
//!    budget until the walk refined past it).
//!
//!    The walls are rational, so the loop's volume is a certified
//!    ENCLOSURE — a bracket at the default ε. [`stops`] holds it, and
//!    the mesh's volume, to Pappus's A·L within a stated discretization
//!    allowance.
//! 6. **`tube_along_arc` WAS solid-only — RETIRED by VERBS-TUBEWALL.**
//!    The torus door took a `minor_radius` and no wall, so a hollow
//!    tube had to be re-said as a revolve of an annulus and gave up
//!    the door's whole point: the caller's intent parameters stored
//!    bit-exactly. The door now has a hollow sibling,
//!    `tube_along_arc_hollow`, taking the outer minor radius plus a
//!    `wall` — the outer wall's radii and frame are still the caller's
//!    numbers bit for bit, and the inner wall's radius is
//!    `minor_radius - wall`, one IEEE subtraction the caller can
//!    repeat. Both window policies are on the tour:
//!    `tubewall::hollowelbow` (an open elbow of annular section, which
//!    asserts the storage contract on the scene body itself) and
//!    `tubewall::hollowtorus` (the full period, whose bore is a
//!    cavity). Never a probe here and none is owed: the gap was a
//!    MISSING PARAMETER, so there was nothing to refuse.
//! 7. **The one-call hollow ring BUILDS; its STEP export is the wall**
//!    (wall 6 — re-baselined by VERBS-RING). A full revolve of a
//!    holed profile no longer refuses: the annulus revolves into a
//!    two-shell solid (torus + toroidal cavity) through the shared
//!    void-insertion door, tier-valid, and the probe asserts that
//!    build every run. What the shape still cannot do is leave as a
//!    STEP file: the writer reads the cavity as the void it is and
//!    refuses it, `VoidShellUnsupported`, because it does not write
//!    `BREP_WITH_VOIDS` yet
//!    (`work/export/step-export-refuses-every-hollow-body.md`) —
//!    recorded here and never worked around.
//! 8. **Edge selection by adjacent surface kinds works at BOTH
//!    seats.** `select_where` + `GeomPred::AdjacentKinds` says "the
//!    cone×cylinder corners" over an `Evaluation` (the die scene says
//!    exactly that); a body built by calling `revolve` directly says
//!    the same thing through the kernel query seat —
//!    [`corner_edges`] below is `query::all_edges` filtered by
//!    `query::edge_adjacent_matches`, the one implementation the
//!    document door delegates to.
//! 9. **A valid body the tessellator refused, at ordinary
//!    proportions — CLOSED, and wall 7 retired with it.**
//!    `mesh::planar`'s module docs used to bank exactly one uncovered
//!    case: when a planar face's boundary points carry off-plane
//!    noise, the chart frame's FAR point has an *engineered*
//!    exact-zero v-coordinate whose float residue is ~ν², which for
//!    small ν is nonzero but below spade's coordinate floor
//!    (`MIN_ALLOWED_VALUE` = 2⁻¹⁴²), and the face refused
//!    `Triangulation`. That paragraph called the case "synthetic
//!    today (no corpus body hits it)"; **this bottle hit it**, which
//!    is what took the case off the bank. The bulb's annular top rim
//!    — a slit annulus, 0.05 m wide, at plain coordinates — refused
//!    at EVERY δ, its far point projecting to
//!    (0.4978884624952486, -2.19e-48), so no δ the caller could pick
//!    was an escape. Whether it refused was a roundoff lottery over
//!    parameters with nothing to do with the cap: sweeping the flare
//!    angle against the rim radius, the refusal appeared at
//!    (30°, 0.85 m) and (34°, 1.00 m) — and, re-swept on the tree that
//!    fixed it, at (24°, 0.85 m) and (26°, 0.75 m) as well, so the
//!    lottery was denser than the first sweep recorded. `mesh::planar`
//!    now WRITES that coordinate as the zero its own construction
//!    produced rather than reading the residue back off the dot
//!    product — not value snapping but its opposite, a refusal to
//!    invent a nonzero the frame never had; the module's prose carries
//!    that argument against its own no-snapping doctrine. Wall 7 no
//!    longer pins a refusal: it re-runs all four of those lattice
//!    points and requires them to mesh.
//! 10. **What went RIGHT, and is worth stating.** The bottle's hole
//!     is the tube's diameter — Ev's own constraint — so the neck
//!     wall and the inner tube wall are literally the same cylinder
//!     about the same axis, twice, in one profile loop. Everything
//!     downstream copes: the band validates, the full revolve builds,
//!     the body is tier-3 green and exports STEP. What it does NOT do
//!     is notice: the revolve's cosurface merge is a run-ADJACENCY
//!     decision, and these two runs are not adjacent, so the bulb
//!     carries 12 faces on 12 surface keys — four cylinder faces that
//!     are geometrically two cylinders, described four ways (four
//!     different chart origins; the two outer walls do carry the one
//!     authored radius, since the lattice stores the carriers it builds).
//!     Any predicate whose certified lane is "same `SurfaceKey`" —
//!     M9-2's chart-region rule is the live example — sees two charts
//!     where the model has one. [`stops`] pins the whole picture.
//! 11. **The loop's spine is written out by hand.** The spine is the
//!     two arcs as four rational quarter arcs, end tangents exactly
//!     +z and −z, so the caps lie flat on the rims they meet. No public
//!     door composes it: `NurbsCurve3::interpolate` takes no end
//!     derivatives, and nothing joins two exact arcs into one path, so
//!     the scene writes the rational control net itself
//!     ([`exact_spine`]). The sweep carries the start frame along the
//!     half turn station to station (`crates/sweep/README.md`
//!     `[sweep.frame]`), so the end tangent being the start's reversed
//!     asks nothing of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p2, v2, validated};
use pncad::geom::NurbsCurve3;
use pncad::geom_core::linalg::frame::path_start_frame;
use pncad::geom_core::{KnotVector, Point3, RigidMotion, Tol, Turns, UnitVec3};
use pncad::prelude::{ConstructedLoop, Open, Start, SurfaceKindSet, circle, query};
use pncad::prelude::{Convexity, SurfaceKind};
use pncad::profile::SketchPlane;
use pncad::sweep::blend::{BlendError, CornerConfig, fillet_edges};
use pncad::sweep::{LoftError, Revolution, RevolveAxis, revolve, sweep_body};
use pncad::topo::readback::euler_counts;
use pncad::topo::{AtRestBody, Body, BooleanError, BooleanOp, EdgeKey, Operand};

use crate::booleans::finished;
use crate::scalar::Scalar;
use crate::{SceneBody, TourStop, View};

// ---------------------------------------------------------------
// The bottle's dimensions (metres). Chosen, not measured: a stylized
// bottle the kernel can state exactly beats a literal one it must
// approximate (the lily's rule).
// ---------------------------------------------------------------

/// Spine radius of the tube — half Ev's "tube diameter", the one
/// number the whole bottle is keyed to: the top loop's tube, the
/// neck, the straight inner tube and the wide rim's HOLE are all this
/// size.
const R: f64 = 0.25;
/// Wall thickness of the thin manifold. There is no `shell`, so this
/// is spelled into both offsets of every run by hand (finding 1).
const WALL: f64 = 0.05;
/// Half-angle of the body's flare, from the axis.
const ALPHA: f64 = 30.0 * PI / 180.0;
/// Height of the neck's top rim, where the loop attaches.
pub const ZTOP: f64 = 3.0;
/// Height of the spine corner where the neck turns into the flare.
const ZNECK: f64 = 2.5;
/// Spine radius of the neck→flare blend (authored in the meridian;
/// `fillet_edges` rolls the same blend — findings entry 3).
pub const RF: f64 = 0.30;
/// Spine radius of the wide bottom rim — the "wide torus" the surface
/// turns back on. Its hole comes out at exactly `R` by construction.
const RRIM: f64 = 0.80;
/// Spine radius of each of the top loop's two arcs.
pub const RLOOP: f64 = 1.20;

/// The 270° arc, over the top.
const SWEEP_OVER: f64 = 1.5 * PI;
/// The 90° arc, turning back onto the bottle's axis.
const SWEEP_IN: f64 = 0.5 * PI;
/// Stations the loop's sweep places along its spine, and the skin's
/// degree through them. The volume does not choose them: tier 3
/// decides the loop's sign at every station count from 9 to 65, at
/// v-degree 2 and 3, with two or 4, 8 or 16 arcs a wall, at ε = 1e-6,
/// 1e-9 and 1e-12 (its tests' `loop_volume_grid` pins a grid of it). The mesher
/// does. Meshing at the stop's δ was measured on the loop's earlier
/// section, four quarter arcs a wall: every station count from 9 to 19
/// at v-degree 2 meshed, and so did 13 at v-degree 3; 21 at v-degree 2
/// refused `CertificateExceeded`
/// (`work/tess/a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate`).
/// The `circle` loop is meshed at this setting only.
const STATIONS: usize = 13;
const V_DEGREE: usize = 2;
/// The stop's mesh tolerance — the δ the loop's neighbourhood was
/// measured at.
const DELTA: f64 = 1e-2;

/// The meridian's derived geometry, in sketch coordinates
/// `(radius, height)`. Structure selection is f64 (C6): these are the
/// tangent points and centres the band is authored from, chosen once
/// and embedded into whatever scalar the scene runs at.
pub struct Meridian {
    /// Spine radius of the neck→flare blend (a parameter, not a
    /// constant, because wall 7 builds bottles at other proportions).
    rf: f64,
    /// Outer and inner offsets of the tube radius.
    pub ro: f64,
    pub ri: f64,
    /// The flare's direction, `(sin α, −cos α)`.
    dir: (f64, f64),
    /// Where the rim arc leaves the flare, on the outer offset…
    g_out: (f64, f64),
    /// …and on the inner one.
    g_in: (f64, f64),
    /// Where the rim arc that continues the INNER flare offset lands
    /// on the inner tube — at radius `R + WALL/2`, the tube's OUTER
    /// wall. The offsets swap sides at the rim, because that is what
    /// "the surface turns back on itself" means; keeping the swap
    /// straight is the shell operation's bookkeeping, done by hand
    /// (finding 1).
    h_from_in: (f64, f64),
    /// …and where the OUTER flare offset's rim arc lands: radius
    /// `R − WALL/2`, the tube's inner wall.
    h_from_out: (f64, f64),
    /// Height of the rim arc's centre (its radius is `R + RRIM`).
    rim_z: f64,
    /// Height of the inner tube's top rim, where the loop re-enters.
    z_tube: f64,
}

/// The meridian's closed form.
///
/// The rim is the constraint that fixes everything else: it is
/// tangent to the flare and comes back up VERTICALLY at radius `R`,
/// which is what "the wide torus's inner diameter is the tube
/// diameter" means. A circle tangent to a vertical line at radius `R`
/// on its far side has centre radius `R + RRIM`; walking the flare
/// line out to the radius where its perpendicular hits that centre
/// gives the tangent point, and the centre's height follows.
pub fn meridian() -> Meridian {
    meridian_at(ALPHA, RF, RRIM, RLOOP)
}

/// The meridian at any proportions. Only wall 7 asks for proportions
/// other than the bottle's own: it re-runs the flare-angle × rim-radius
/// lattice points that used to decide whether the cap meshed.
pub fn meridian_at(alpha: f64, rf: f64, rrim: f64, rloop: f64) -> Meridian {
    let half = WALL / 2.0;
    let (sa, ca) = alpha.sin_cos();
    // The flare line runs from the spine corner (R, ZNECK) along dir.
    let gx = R + rrim * (1.0 + ca);
    let gz = ZNECK - rrim * (1.0 + ca) * ca / sa;
    let rim_z = gz - rrim * sa;
    // The offsets: `(cos α, sin α)` is the flare's left normal (left
    // of travel is AWAY from the axis on the way down).
    Meridian {
        rf,
        ro: R + half,
        ri: R - half,
        dir: (sa, -ca),
        g_out: (gx + half * ca, gz + half * sa),
        g_in: (gx - half * ca, gz - half * sa),
        // The rim arc ends at the centre radius (R + RRIM) minus its
        // own (RRIM ∓ half), i.e. R ± half — the two walls of the
        // inner tube, with the sides swapped.
        h_from_in: (R + half, rim_z),
        h_from_out: (R - half, rim_z),
        rim_z,
        z_tube: ZTOP - 2.0 * rloop,
    }
}

/// The bulb's meridian band: one closed loop, walked once down the
/// OUTSIDE of the wall and once back up the inside.
///
/// Read it as the bottle's own surface, twice: neck down, blend,
/// flare out, rim around, inner tube up, across the tube's rim, and
/// back. The two blends are `.fillet(r)` corners and the two rim
/// tangencies are `.tangent()` declarations, so every G1 joint in the
/// bulb is a construction and not a coincidence.
///
/// Every radius appears twice, `∓ WALL/2`, and which sign goes with
/// which side depends on where the centre of curvature is: the
/// neck→flare blend curves AWAY from the axis (outer offset is the
/// smaller radius), the rim curves toward it. That bookkeeping is
/// finding 1 in its most concrete form.
pub fn band<S: Scalar>(m: &Meridian, tol: Tol) -> ConstructedLoop<S> {
    let half = S::from_f64(WALL / 2.0);
    Open.at(p2::<S>(m.ri, ZTOP))
        .toward(S::from_f64(0.0), S::from_f64(-1.0), tol)
        .expect("the neck runs down")
        .fillet(S::from_f64(m.rf) + half, tol)
        .expect("the inner neck→flare blend")
        .toward(S::from_f64(m.dir.0), S::from_f64(m.dir.1), tol)
        .expect("the flare runs down and out")
        .to(p2::<S>(m.g_in.0, m.g_in.1), tol)
        .expect("the inner flare ends where the rim takes over")
        .tangent()
        .tangent_arc_to(p2::<S>(m.h_from_in.0, m.h_from_in.1), tol)
        .expect("the rim arc, minor radius RRIM − WALL/2")
        .tangent()
        .line(S::from_f64(m.z_tube - m.rim_z), tol)
        .expect("the inner tube runs back up")
        .line_to(p2::<S>(m.ri, m.z_tube), tol)
        .expect("the inner tube's top rim")
        .line_to(p2::<S>(m.h_from_out.0, m.h_from_out.1), tol)
        .expect("and down its other wall")
        .tangent()
        .tangent_arc_to(p2::<S>(m.g_out.0, m.g_out.1), tol)
        .expect("the rim arc, minor radius RRIM + WALL/2")
        .tangent()
        .fillet(S::from_f64(m.rf) - half, tol)
        .expect("the outer flare→neck blend")
        .toward(S::from_f64(0.0), S::from_f64(1.0), tol)
        .expect("the neck runs up")
        .to(p2::<S>(m.ro, ZTOP), tol)
        .expect("the outer neck ends at the top rim")
        .line_to(Start, tol)
        .expect("the neck's top rim closes the band")
        .into()
}

/// The same band with the two blends taken OUT: a hard corner where
/// the neck meets the flare. Built only to ask `fillet_edges` for the
/// blend the band authors for free — findings entry 3 and wall 2.
fn sharp_band<S: Scalar>(m: &Meridian, tol: Tol) -> ConstructedLoop<S> {
    // Where the flare's two offsets cross the neck's two walls.
    let corner = |g: (f64, f64), x: f64| {
        let s = (x - g.0) / m.dir.0;
        p2::<S>(x, g.1 + s * m.dir.1)
    };
    // The outer flare's straight run, from the rim's tangent point
    // back up to the sharp corner: the line parameter itself, since
    // `dir` is a unit vector.
    let flare_run = S::from_f64(((m.ro - m.g_out.0) / m.dir.0).abs());
    Open.at(p2::<S>(m.ri, ZTOP))
        .line_to(corner(m.g_in, m.ri), tol)
        .expect("the neck runs down to the sharp corner")
        .line_to(p2::<S>(m.g_in.0, m.g_in.1), tol)
        .expect("the inner flare")
        .tangent()
        .tangent_arc_to(p2::<S>(m.h_from_in.0, m.h_from_in.1), tol)
        .expect("the rim arc, minor radius RRIM − WALL/2")
        .tangent()
        .line(S::from_f64(m.z_tube - m.rim_z), tol)
        .expect("the inner tube runs back up")
        .line_to(p2::<S>(m.ri, m.z_tube), tol)
        .expect("the inner tube's top rim")
        .line_to(p2::<S>(m.h_from_out.0, m.h_from_out.1), tol)
        .expect("and down its other wall")
        .tangent()
        .tangent_arc_to(p2::<S>(m.g_out.0, m.g_out.1), tol)
        .expect("the rim arc, minor radius RRIM + WALL/2")
        .tangent()
        .line(flare_run, tol)
        .expect("the outer flare")
        .line_to(p2::<S>(m.ro, ZTOP), tol)
        .expect("the outer neck runs up to the top rim")
        .line_to(Start, tol)
        .expect("the neck's top rim closes the band")
        .into()
}

/// **The bottle's meridian plane**: the world xy plane turned a
/// quarter turn about x̂, so sketch `u` runs out along x̂ and `v` up
/// the bottle's axis ẑ.
fn meridian_plane<S: Scalar>() -> SketchPlane<S> {
    SketchPlane::from_frame(RigidMotion::turned(
        Point3::origin(),
        UnitVec3::exact_x(),
        Turns::quarters(1),
    ))
}

/// Revolves a meridian band about the bottle's axis. `Full` is the
/// bulb; the partial form exists only so wall 2 can ask the same
/// question of an OPEN rim (findings entry 2).
pub fn bulb<S: Scalar>(loop_: ConstructedLoop<S>, revolution: Revolution<S>, tol: Tol) -> Body<S> {
    let plane = meridian_plane::<S>();
    revolve(
        &validated(plane, vec![loop_], tol).expect("the meridian band validates"),
        RevolveAxis {
            origin: p2::<S>(0.0, 0.0),
            dir: v2::<S>(0.0, 1.0),
        },
        revolution,
        tol,
    )
    .expect("the meridian band revolves")
    .body
}

/// The spine's descent, at angle `psi` on the 90° arc centred one
/// `RLOOP` to the +x side of the inner tube's top rim: `psi = π/2` is
/// where it meets the 270° arc over the top, `psi = π` is the rim,
/// reached straight down.
fn into_arc(m: &Meridian, psi: f64) -> (f64, f64) {
    (RLOOP * (1.0 + psi.cos()), m.z_tube + RLOOP * psi.sin())
}

/// The top loop's whole spine: the two tangent arcs as four rational
/// quarter arcs (degree 2, weights 1, √½, 1), so its end tangents are
/// exactly +z and −z.
pub fn exact_spine(m: &Meridian) -> NurbsCurve3<f64> {
    let (top_c, tube_c) = ((RLOOP, ZTOP), (RLOOP, m.z_tube));
    // The quarter points, written as the numbers they are, so the end
    // tangents carry no trig residue.
    let on = [
        (0.0, ZTOP),
        (RLOOP, ZTOP + RLOOP),
        (2.0 * RLOOP, ZTOP),
        (RLOOP, ZTOP - RLOOP),
        (0.0, m.z_tube),
    ];
    // A quarter arc's middle control point is the corner of the square
    // its two ends and its centre span.
    let corner = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| (a.0 + b.0 - c.0, a.1 + b.1 - c.1);
    let centres = [top_c, top_c, top_c, tube_c];
    let mut ctrl = vec![on[0]];
    for k in 0..4 {
        ctrl.push(corner(on[k], on[k + 1], centres[k]));
        ctrl.push(on[k + 1]);
    }
    let w = core::f64::consts::FRAC_1_SQRT_2;
    NurbsCurve3::new(
        KnotVector::clamped(vec![0., 0., 0., 1., 1., 2., 2., 3., 3., 4., 4., 4.], 2)
            .expect("four quarter-arc spans"),
        ctrl.into_iter()
            .map(|(x, z)| Point3::new(x, 0.0, z))
            .collect(),
        vec![1., w, 1., w, 1., w, 1., w, 1.],
    )
    .expect("the exact spine is a valid NURBS")
}

/// The tube's annular cross-section, the meridian's two wall radii
/// about `(cx, 0)` in its sketch plane.
fn annulus<S: Scalar>(m: &Meridian, cx: f64, tol: Tol) -> Vec<ConstructedLoop<S>> {
    [m.ro, m.ri]
        .into_iter()
        .map(|r| {
            circle(p2::<S>(cx, 0.0), S::from_f64(r), tol)
                .expect("a wall")
                .into()
        })
        .collect()
}

/// `section` swept along `spine` by `sweep_body` at `stations`
/// stations and v-degree `v_degree`, from the start frame the kernel
/// hands out (`path_start_frame`). The section is built in `f64`, so it
/// enters `S` as the computed values it is; the body comes out at `S`.
pub fn swept_loop<S: Scalar>(
    spine: &NurbsCurve3<f64>,
    section: &[ConstructedLoop<f64>],
    stations: usize,
    v_degree: usize,
    tol: Tol,
) -> Result<Body<S>, LoftError> {
    let (t0, _) = spine.domain();
    let (start, tangent) = spine.ders1(t0);
    let place = path_start_frame(
        start.map(S::from_computed),
        tangent.map(S::from_computed),
        tol,
    )
    .expect("the spine's start tangent fixes a frame");
    let section: Vec<ConstructedLoop<S>> = section.iter().map(|lp| lp.lift_computed()).collect();
    sweep_body::<S>(
        &section,
        SketchPlane::from_frame(place),
        spine,
        stations,
        v_degree,
        tol,
    )
    .map(|l| l.body)
}

/// The bottle: the bulb and the loop, each finished, and the spine the
/// loop was swept along.
pub struct Bottle<S: Scalar> {
    pub bulb: AtRestBody<S>,
    pub top: AtRestBody<S>,
    pub spine: NurbsCurve3<f64>,
}

pub fn bottle<S: Scalar>(tol: Tol) -> Bottle<S> {
    let m = meridian();
    let spine = exact_spine(&m);
    let top = swept_loop::<S>(
        &spine,
        &annulus::<f64>(&m, 0.0, tol),
        STATIONS,
        V_DEGREE,
        tol,
    )
    .expect("the annulus sweeps along the loop's whole spine");
    Bottle {
        bulb: finished(
            "the bulb",
            bulb(band::<S>(&m, tol), Revolution::Full, tol),
            tol,
        ),
        top: finished("the loop", top, tol),
        spine,
    }
}

/// Wall 3's pinned refusal: the bulb's cone seam, a line in the plane
/// `y = 0` slanting off the axis (the neck's seam, which ends on the
/// loop's rim, is cleared there by its departure), meets a NURBS wall of the loop on the joint of the wall's two
/// rational arcs, which lies in that plane. The joint is a knot run of
/// multiplicity the degree, a crease by its knots: the wall is tangent
/// continuous across it, but its parametric derivative jumps there, and
/// the spline-face arm reads no Jacobian across such a knot. Shared with
/// `verbs_gate_r1_probes`, so the two cannot drift.
pub fn wall3_pinned<S: Scalar>(
    e: &BooleanError,
    bulb: &AtRestBody<S>,
    top: &AtRestBody<S>,
) -> bool {
    let BooleanError::SplineCrossingUnsettled {
        operand: Operand::A,
        edge,
        face,
        face_of: Operand::B,
        why: pncad::geom_brep::roots::Unsettled::Crease,
    } = e
    else {
        return false;
    };
    let in_the_seam_plane = bulb
        .get_edge(*edge)
        .and_then(|e| bulb.get_curve_geom(e.curve))
        .is_some_and(|g| match g {
            pncad::topo::CurveGeom::Certified(c) => matches!(
                c.carrier(),
                pncad::geom::Curve3::Line { origin, dir }
                    if [origin.y, dir.y].iter().all(|&c| {
                        let (lo, hi) = (pncad::geom_core::Bounds::lo(c), pncad::geom_core::Bounds::hi(c));
                        lo.abs() < 1e-12 && hi.abs() < 1e-12
                    }) && pncad::geom_core::Bounds::lo(dir.x).abs() > 0.1
            ),
            _ => false,
        });
    let on_an_arc_joint = top
        .get_face(*face)
        .and_then(|f| top.get_surface(f.surface))
        .is_some_and(|s| match s {
            pncad::geom::Surface::Nurbs(n) => !n.knot_values_u().crease_values().is_empty(),
            _ => false,
        });
    in_the_seam_plane && on_an_arc_joint
}

/// Wall 4's pinned refusal: one of the loop's rational NURBS walls
/// against a planar face of the bulb, at the ∖/∩ revert roster. Both
/// kinds and the op are pinned, so a pair that changed kind reds. A
/// NURBS face has no seam lane under ∖ and ∩, and the roster has no
/// covered rung whatever the pair; it answers before the sweep that
/// stops wall 3.
pub fn wall4_pinned(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::CurvedPairUnsupported {
            op: Some(BooleanOp::Subtract),
            operand: Operand::B,
            kind: SurfaceKind::Nurbs,
            other_kind: SurfaceKind::Plane,
            ..
        }
    )
}

/// Every edge of `body` whose two faces are a cone and a cylinder
/// (finding 8): the kernel query seat's adjacent-kind predicate over
/// its materializer — what `select_where(&ev, node, &edges,
/// &[GeomPred::AdjacentKinds(cone, cylinder)], &params)` says at the
/// document layer, said in keys at the body layer through the same
/// one implementation.
fn corner_edges<S: Scalar>(body: &Body<S>, a: SurfaceKind, b: SurfaceKind) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&e| {
            query::edge_adjacent_matches(body, e, SurfaceKindSet::just(a), SurfaceKindSet::just(b))
        })
        .collect()
}

/// The sharp band's two neck→flare corners rolled by `fillet_edges`
/// at the blend radii the meridian authors: `RF + WALL/2` on the inner
/// wall's corner, `RF − WALL/2` on the outer's (findings entry 1's
/// bookkeeping — the centre of curvature lies away from the axis).
fn roll_the_meridian_blends<S: Scalar>(
    sharp: &AtRestBody<S>,
    corners: &[EdgeKey],
    m: &Meridian,
    tol: Tol,
) -> Body<S> {
    let (inner, outer) = corners_by_wall(sharp, corners);
    let half = WALL / 2.0;
    let once = fillet_edges(sharp, &[inner], S::from_f64(m.rf + half), tol)
        .unwrap_or_else(|e| panic!("the inner corner rolls at RF + WALL/2: {e:?}"))
        .body;
    // The outer corner's key survives the first roll: that blend's
    // surgery touches the inner wall's faces only.
    fillet_edges(
        &finished("once", once, tol),
        &[outer],
        S::from_f64(m.rf - half),
        tol,
    )
    .unwrap_or_else(|e| panic!("the outer corner rolls at RF − WALL/2: {e:?}"))
    .body
}

/// A sharp band's two neck→flare corners as `(inner wall's, outer
/// wall's)`, told apart by their rims' size: the inner wall's is the
/// smaller circle.
fn corners_by_wall<S: Scalar>(sharp: &Body<S>, corners: &[EdgeKey]) -> (EdgeKey, EdgeKey) {
    let [a, b] = corners else {
        panic!(
            "a sharp band has two neck→flare corners, found {}",
            corners.len()
        )
    };
    if lever_arm(sharp, *a) < lever_arm(sharp, *b) {
        (*a, *b)
    } else {
        (*b, *a)
    }
}

/// A body's exact volume, read out at `f64`.
fn volume<S: Scalar>(b: &Body<S>, tol: Tol) -> f64 {
    pncad::topo::mass_properties(&crate::booleans::at_rest(b, tol), tol, crate::printed())
        .expect("the volume integrates")
        .enclosure
        .volume
        .f()
}

/// The lever arm every angular fillet predicate is metered against —
/// the maximum pairwise chord over the battery's own per-link sample
/// schedule (`sweep::blend::battery::CHAIN_SAMPLES`). Findings
/// entry 2 is a statement about this number: it stays ~the rim's
/// diameter whether or not the rim closes.
fn lever_arm<S: Scalar>(body: &Body<S>, edge: EdgeKey) -> f64 {
    let e = body.get_edge(edge).expect("edge");
    let c = body
        .get_curve_geom(e.curve)
        .expect("curve")
        .certified()
        .expect("a revolved rim carries a certified carrier");
    let (t0, t1) = c.params();
    let carrier = c.carrier();
    let n = pncad::sweep::blend::battery::CHAIN_SAMPLES;
    let pts: Vec<_> = (0..n)
        .map(|i| {
            let f = S::from_f64(f64::from(i) / f64::from(n - 1));
            carrier.eval(t0 + (t1 - t0) * f)
        })
        .collect();
    let mut best = S::from_f64(0.0);
    for (i, a) in pts.iter().enumerate() {
        for b in &pts[(i + 1)..] {
            best = best.max((*b - *a).norm());
        }
    }
    best.f()
}

/// The bottle stop.
pub fn stops(tol: Tol) -> Vec<TourStop> {
    let m = meridian();
    let Bottle { bulb, top, spine } = bottle::<f64>(tol);

    // The loop's volume in the continuum is the annulus times the
    // spine length (Pappus: a planar spine, the section's centroid ON
    // it and the section symmetric about its plane), A·L with
    // L = RLOOP·2π. The swept body is not that continuum: the skin
    // interpolates STATIONS sections, evenly spaced Δ = 2π/(STATIONS−1)
    // around the spine, so its volume differs from A·L by the
    // discretization. The ALLOWANCE is the chordal deficit of the
    // station polygon, 1 − sin(Δ/2)/(Δ/2) — the length the coarsest
    // approximation through those stations would lose, which the skin
    // improves on — applied on both sides, since an interpolant may
    // also overshoot. Measured inside it: every setting of the
    // neighbourhood at STATIONS sits 0.13–0.32 % under A·L. Outside
    // it: a loop whose over arc is bulged 0.15 m mid-span reads +1.97 %.
    let ring = PI * (m.ro.powi(2) - m.ri.powi(2));
    let pappus = ring * RLOOP * (SWEEP_OVER + SWEEP_IN);
    let half_step = PI / (STATIONS as f64 - 1.0);
    let allowance = pappus * (1.0 - half_step.sin() / half_step);
    let (band_lo, band_hi) = (pappus - allowance, pappus + allowance);
    // The kernel's reading is a certified ENCLOSURE (the walls are
    // rational): a number with its half-width where the printed
    // precision is met, the narrowest bracket it reached where it is
    // not. The body's true volume is in it AND in the band, so the two
    // must meet.
    let e = pncad::topo::validate_geometric_certificate(&top, tol)
        .unwrap_or_else(|e| panic!("the loop's tier 3 refused: {e:?}"))
        .measure(crate::printed())
        .unwrap_or_else(|refusal| panic!("the loop's mass properties: {refusal}"))
        .enclosure
        .enclosure();
    let (v_lo, v_hi) = (e.volume_lo, e.volume_hi);
    assert!(
        v_lo <= band_hi && band_lo <= v_hi,
        "the loop's certified enclosure [{v_lo}, {v_hi}] misses Pappus's A·L = {pappus} \
         ± its discretization allowance {allowance:e}"
    );
    // The enclosure is wide at the default ε, so the band's teeth are
    // the mesh: inscribed, so it reads at most the body's volume, and a
    // loop that is not the authored one leaves the band.
    let v_mesh = pncad::mesh::validate::signed_volume(
        &pncad::mesh::tessellate(&top, DELTA, tol).expect("the loop meshes at the stop's δ"),
    );
    assert!(
        band_lo <= v_mesh && v_mesh <= band_hi,
        "the loop's mesh volume {v_mesh} is outside Pappus's A·L = {pappus} ± {allowance:e}: \
         the swept body is not the authored loop"
    );
    // The census: two annular caps and two semicircular walls per
    // side — a hollow tube, so one handle.
    let census = euler_counts(&top);
    assert_eq!(
        (
            census.v,
            census.e,
            census.f,
            census.r,
            census.s,
            census.genus()
        ),
        (8, 12, 6, 2, 1, Ok(1)),
        "the loop is one genus-1 shell of 6 faces; got {census:?}"
    );

    // Findings entry 10, executed: the same cylinder, said four ways.
    // Ev's constraint — the rim's hole is the tube's diameter —
    // makes the neck wall and the inner tube wall THE SAME cylinder
    // about THE SAME axis. The revolve's cosurface merge is a
    // run-ADJACENCY decision inside one loop, and these two runs are
    // not adjacent, so each gets its own surface key: 12 faces on 12
    // keys, four of them cylinders that are geometrically two.
    let (faces, surfaces) = (bulb.faces().count(), bulb.surfaces().count());
    assert_eq!(
        (faces, surfaces),
        (12, 12),
        "got {faces} faces on {surfaces} surfaces"
    );
    let cylinders: Vec<(Point3<f64>, f64)> = bulb
        .surfaces()
        .filter_map(|(_, s)| match s {
            pncad::topo::Surface::Cylinder { origin, radius, .. } => Some((*origin, *radius)),
            _ => None,
        })
        .collect();
    assert_eq!(cylinders.len(), 4, "neck + inner tube, two walls each");
    // Not one description: the chart origin is anchored at each run's
    // OWN start point, so the four differ in `origin.z` even where
    // they are the same surface.
    let origins: Vec<f64> = cylinders.iter().map(|(o, _)| o.z).collect();
    let mut sorted = origins.clone();
    sorted.sort_by(f64::total_cmp);
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        4,
        "each run anchors its own chart origin; got {origins:?}"
    );
    // Findings entry 11, measured on the body: each cap is a plane
    // through its spine end, normal to the axis the end tangent lies
    // on, holding the meridian's two wall radii — so the cap is flat on
    // the bulb's rim it meets.
    let (t0, t1) = spine.domain();
    let ends = [spine.ders1(t0), spine.ders1(t1)];
    let caps: Vec<(Point3<f64>, pncad::geom_core::Vec3<f64>)> = top
        .surfaces()
        .filter_map(|(_, sf)| match sf {
            pncad::topo::Surface::Plane { origin, normal, .. } => Some((*origin, *normal)),
            _ => None,
        })
        .collect();
    assert_eq!(caps.len(), 2, "the loop has two planar caps");
    for (end, d) in &ends {
        let (origin, normal) = caps
            .iter()
            .copied()
            .find(|(o, _)| (*o - *end).norm() < 1e-12)
            .unwrap_or_else(|| panic!("no cap sits on the spine end {end:?}: {caps:?}"));
        let cap_tilt = (normal.x.hypot(normal.y) / normal.norm()).asin();
        assert!(
            cap_tilt < 1e-12 && d.x == 0.0 && d.y == 0.0,
            "the cap at {origin:?} is tilted {cap_tilt:e} rad off the rim it meets, at a \
             spine end whose tangent is {d:?}"
        );
        for p in top
            .vertex_points()
            .map(|(_, p)| p)
            .filter(|p| (*p - origin).dot(normal).abs() < 1e-12)
        {
            let r = (p - origin).norm();
            assert!(
                (r - m.ro).abs() < 1e-12 || (r - m.ri).abs() < 1e-12,
                "a cap vertex at radius {r} is on neither wall ({} / {})",
                m.ro,
                m.ri
            );
        }
    }

    // The self-intersection is REAL and it is where a Klein bottle
    // has to have one: the loop's descent starts outside the bulb's
    // flare and ends inside it. (The flare's spine radius at height z
    // is R + (ZNECK − z)·tan α.)
    let flare_at = |z: f64| R + (ZNECK - z) * ALPHA.tan();
    let (out_x, out_z) = into_arc(&m, 0.5 * PI);
    let (in_x, in_z) = into_arc(&m, 5.0 * PI / 6.0);
    assert!(
        out_x > flare_at(out_z) && in_x < flare_at(in_z),
        "the loop's descent must cross the flare: ({out_x}, {out_z}) vs \
         {}, ({in_x}, {in_z}) vs {}",
        flare_at(out_z),
        flare_at(in_z)
    );

    let story = "a Klein bottle — as a THIN 3-manifold, because a 2-manifold sheet is not a \
                 body this kernel holds. The bulb (neck, flare, the wide rim it turns back \
                 on, and the straight tube coming back up through that rim's hole) is ONE \
                 full revolve of ONE meridian band; the loop over the top is ONE sweep \
                 of the annulus along its whole U-turn spine. The two MEET on annular \
                 faces, each cap flat on the bulb's rim, \
                 and they cannot be joined: union refuses a NURBS wall of the loop where \
                 it meets the bulb's neck at the mate, and subtract refuses the loop's NURBS walls against a \
                 planar face at the revert roster. The neck passes through the flare \
                 uncut for that second reason";
    vec![TourStop {
        name: "klein",
        caption: "Klein bottle — two bodies the kernel cannot join".to_string(),
        montage: true,
        story,
        ops: "revolve(Full) of a filleted meridian band + sweep_body of an annulus \
              (two circles) along a U-turn spine of four rational quarter arcs, \
              from path_start_frame; \
              NO boolean, NO shell — see klein::wall_probes",
        delta: DELTA,
        note: Some(format!(
            "tube diameter {:.2} m, wall {:.2} m; the wide rim's hole comes out at the \
             tube diameter by construction (centre radius R + RRIM = {:.3}, minor radii \
             {:.3}/{:.3}). Every blend in the bulb is an ARC IN THE MERIDIAN, exact and \
             free, and `fillet_edges` rolls the same blend on the sharp band. The loop's \
             certified volume [{v_lo:.6}, {v_hi:.6}] m^3 and mesh volume {v_mesh:.6} meet \
             Pappus's ring area {ring:.6} m^2 times spine length = {pappus:.6} within its \
             {:.2}% discretization allowance. Its caps lie flat on the neck's and \
             the inner tube's rims: the spine's end tangents are exactly +z and −z",
            2.0 * R,
            WALL,
            R + RRIM,
            RRIM + WALL / 2.0,
            RRIM - WALL / 2.0,
            100.0 * allowance / pappus,
        )),
        // The whole model is symmetric about the world xz-plane, so
        // that plane is the ONE camera to avoid: from it the loop
        // reads as a flat ring and the place where the neck enters
        // the flare — the crossing that makes this a Klein bottle —
        // is seen edge-on and hides behind the bulb's silhouette. 40°
        // out of it puts the loop in three-quarter view and the
        // crossing in the open, and the raised elevation keeps the
        // annular rims reading as rims rather than as lines.
        view: View {
            elev: 22.0,
            azim: -40.0,
            up: 'z',
        },
        bodies: vec![
            // See-through, and not for looks: the bottle's SUBJECT is
            // what happens inside the bulb — the neck crossing the
            // body wall and running down to the rim's hole. An opaque
            // render hides the half of the shape that makes it a
            // Klein bottle at any camera.
            SceneBody::plain("klein_bulb", [0.38, 0.62, 0.72], bulb.into_body()).transparent(55),
            // A colour of its own: the loop is the piece that runs
            // INSIDE the bulb, and seeing where it enters is the point.
            SceneBody::plain("klein_loop", [0.80, 0.34, 0.24], top.into_body()).transparent(45),
        ],
    }]
}

/// The bottle's frontier, run live (the lily's rule): every shape
/// this model wanted and the kernel would not state, attempted for
/// real and pinned by its own typed refusal. Wall 6's retirement is
/// asserted through `validate_geometric`, whose +V invariant reads a
/// certified volume enclosure — [`Scalar`] certifies, so the bound is
/// already the one that assertion needs.
pub fn wall_probes<S: Scalar>(tol: Tol) {
    println!("\n-- the Klein bottle's walls: what a non-orientable surface asks for --");
    let m = meridian();
    let Bottle {
        bulb: bulb_body,
        top,
        ..
    } = bottle::<S>(tol);

    // ONE question asked of two bodies, and the pair is the finding:
    // the same corner, on a full and a partial revolve of the SAME
    // band (the full revolve rolls; the partial is wall 2).
    let sharp_full = finished(
        "sharp_full",
        bulb::<S>(sharp_band::<S>(&m, tol), Revolution::Full, tol),
        tol,
    );
    let sharp_part = finished(
        "sharp_part",
        bulb::<S>(
            sharp_band::<S>(&m, tol),
            Revolution::Partial(S::from_f64(5.0)),
            tol,
        ),
        tol,
    );
    let full_edges = corner_edges(&sharp_full, SurfaceKind::Cone, SurfaceKind::Cylinder);
    let part_edges = corner_edges(&sharp_part, SurfaceKind::Cone, SurfaceKind::Cylinder);
    assert_eq!(
        (full_edges.len(), part_edges.len()),
        (2, 2),
        "each sharp band has two cone×cylinder corners, one per wall"
    );
    let (lever_full, lever_part) = (
        lever_arm(&sharp_full, full_edges[0]),
        lever_arm(&sharp_part, part_edges[0]),
    );
    println!(
        "   the fillet battery's lever arm on this corner: {lever_full:.3e} m when the \
         rim is CLOSED (full revolve), {lever_part:.3e} m when it is open (partial). \
         Same corner, same 30° dihedral, honest levers both."
    );
    // `fillet_edges` rolls the very blends the meridian authors. The ball rolls OUTSIDE both neck
    // walls' cylinders and outside the flare's cone — the turn's centre
    // lies away from the axis — so none of the three supports bends
    // toward it and the curvature headroom sets no limit. Each corner
    // takes its own wall's offset of the blend (`RF ± WALL/2`), and the
    // rolled body IS the authored bulb: the same volume.
    let rolled = roll_the_meridian_blends(&sharp_full, &full_edges, &m, tol);
    pncad::topo::validate_geometric(&rolled, tol)
        .unwrap_or_else(|e| panic!("the rolled bulb is tier-3 valid: {e:?}"));
    let (got, want) = (volume(&rolled, tol), volume(&bulb_body, tol));
    assert!(
        (got - want).abs() <= 1e-9 * want,
        "the full revolve rolled at RF ± WALL/2 is the authored bulb: volume {got} vs {want}"
    );
    println!(
        "   the full revolve's neck→flare corners, rolled by `fillet_edges` at RF ± WALL/2: \
         volume {got:.9} m^3, the authored bulb's {want:.9}"
    );
    // Nothing bends toward the ball, so what bounds the inner corner's
    // radius is the wall it stands in: past r ≈ 1.47 the band's
    // deepest point, on the corner's bisector, crosses the outer wall,
    // and the reach meter refuses it against that face.
    let (inner, _) = corners_by_wall(&sharp_full, &full_edges);
    let near = fillet_edges(&sharp_full, &[inner], S::from_f64(1.4), tol)
        .unwrap_or_else(|e| panic!("the inner corner rolls at r = 1.4, inside the wall: {e:?}"));
    pncad::topo::validate_geometric(&near.body, tol)
        .unwrap_or_else(|e| panic!("r = 1.4: tier 3, got {e:?}"));
    for r in [1.5, 1.6] {
        match fillet_edges(&sharp_full, &[inner], S::from_f64(r), tol) {
            Err(e)
                if matches!(
                    e.error,
                    BlendError::FaceClearance {
                        chain: Convexity::Convex,
                        ..
                    }
                ) => {}
            other => panic!(
                "r = {r}: the band leaves through the outer wall, so the reach meter \
                 refuses it (FaceClearance, Convex); got {:?}",
                other.map(|_| ()).map_err(|e| e.error)
            ),
        }
    }
    let far = fillet_edges(&sharp_full, &[inner], S::from_f64(2.0), tol);
    assert!(
        matches!(&far, Err(e) if matches!(e.error, BlendError::FaceClearanceUncertified { .. })),
        "r = 2.0 refuses on clearance, got {:?}",
        far.map(|_| ()).map_err(|e| e.error)
    );
    // Wall 2: the SAME corner on a partial revolve. Its rim is open, so
    // the chain ends where the rim meets the revolve's cut faces, and
    // the corner there does not classify.
    let (inner, _) = corners_by_wall(&sharp_part, &part_edges);
    crate::walls::wall(
        "bottle",
        2,
        "fillet the neck→flare corner on a partial revolve (open rim) at the radius the \
         band authors for that wall",
        fillet_edges(&sharp_part, &[inner], S::from_f64(m.rf + WALL / 2.0), tol),
        |e| {
            matches!(
                e.error,
                BlendError::UnsupportedCorner {
                    corner: CornerConfig::Indeterminate,
                    ..
                }
            )
        },
        "roll the partial revolve's corners as the full revolve's are rolled",
    );

    // Wall 3: the bottle is ONE surface. Its two bodies meet on
    // annular faces — the REST mate — and the union refuses past them:
    // the bulb's cone seam meets a NURBS wall of the loop on the joint of
    // the wall's two rational arcs. The wall is tangent continuous there,
    // but the joint is a crease of its knots, where its parametric
    // derivative jumps and the spline-face arm reads no Jacobian.
    crate::walls::wall(
        "bottle",
        3,
        "join the loop to the bulb (annular mate)",
        pncad::topo::union(&bulb_body, &top, tol),
        |e| wall3_pinned(e, &bulb_body, &top),
        "make the bottle a single body",
    );

    // Wall 4: the self-intersection. A Klein bottle immersed in
    // 3-space MUST cross itself once; the honest picture cuts the
    // flare where the neck goes through.
    crate::walls::wall(
        "bottle",
        4,
        "cut the flare where the descending neck passes through it",
        pncad::topo::subtract(&bulb_body, &top, tol),
        wall4_pinned,
        "trim the self-intersection instead of letting the walls interpenetrate",
    );

    // Wall 5 — RETIRED. The scene's loop is swept with `circle`
    //      sections, and the tour's gate holds it to tier 3 at each ε
    //      row the tour runs, so no probe repeats it.

    // Wall 6 (RE-BASELINED by VERBS-RING): the one-call hollow ring —
    // what the loop would be if it closed on itself instead of
    // entering the bulb — now BUILDS: the holed full revolve executes
    // as revolve(outer) − revolve(hole-as-outer) through the shared
    // void-insertion door (the degenerate no-crossing arm), so the
    // probe asserts the two-shell tier-valid build it used to pin as
    // a refusal. The wall that REMAINS for this shape is its STEP
    // export: the writer refuses its cavity as a void (no
    // BREP_WITH_VOIDS) — OFFSET-DESIGN O6's standing demo gate,
    // recorded here and never worked around.
    let ring_plane = meridian_plane::<S>();
    let ring =
        validated(ring_plane, annulus::<S>(&m, RLOOP, tol), tol).expect("the annulus validates");
    let hollow = revolve::<S>(
        &ring,
        RevolveAxis {
            origin: p2::<S>(0.0, 0.0),
            dir: v2::<S>(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the hollow ring builds in one call (VERBS-RING)");
    assert_eq!(
        hollow.body.shells().count(),
        2,
        "the ring is a two-shell solid: outer torus + toroidal cavity"
    );
    assert_eq!(hollow.cavities.len(), 1);
    assert_eq!(pncad::topo::validate(&hollow.body), Ok(()));
    assert_eq!(pncad::topo::validate_closed(&hollow.body), Ok(()));
    assert_eq!(
        pncad::topo::validate_geometric(&hollow.body, tol),
        Ok(()),
        "the hollow ring is tier-3 valid"
    );
    println!(
        "   wall 6 — RETIRED as a refusal: the annulus revolves to a two-shell \
         hollow ring in one call (VERBS-RING); the shape's remaining wall is \
         its STEP export, probed next"
    );
    // f64: `step_export` is a rendering/interchange-side door and
    // takes the run's own numbers (the wall-7 posture).
    let ring_f64 = validated(meridian_plane::<f64>(), annulus::<f64>(&m, RLOOP, tol), tol)
        .expect("the annulus validates at f64");
    let hollow64 = revolve::<f64>(
        &ring_f64,
        RevolveAxis {
            origin: p2::<f64>(0.0, 0.0),
            dir: v2::<f64>(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the hollow ring builds at f64");
    crate::walls::wall(
        "bottle",
        6,
        "export the hollow ring as STEP (a multi-shell curved solid)",
        pncad::step_export::step_string(
            &pncad::topo::AtRestBody::validate(hollow64.body, tol)
                .expect("the hollow ring is at rest"),
            &pncad::step_export::StepOptions {
                product_name: "hollow_ring".into(),
                ..Default::default()
            },
            tol,
        ),
        |e| {
            matches!(
                e,
                pncad::step_export::StepExportError::VoidShellUnsupported { .. }
            )
        },
        "record that the writer writes BREP_WITH_VOIDS, update \
         findings entry 7 (the O6 demo-gates list row), and retire the `ring`, \
         `tubewall::hollowtorus` and `torusvessel` scenes' `step_at_frontier` \
         declarations, which pin this same refusal on the rendered hollow ring, on the \
         parameter door's hollow torus and on the shelled torus-walled vessel — one \
         gate, four probes, retiring together",
    );

    // Wall 7 is RETIRED, per its own instruction: `mesh::planar`'s
    // banked sub-floor case is CLOSED. It pinned a valid body the
    // tessellator refused — the bulb's annular top rim, at a bottom
    // rim of 0.85 m rather than the 0.80 m this scene ships, the face
    // refusing on a parameter its own geometry does not depend on.
    // The chart frame's far point had an engineered exact-zero
    // v-coordinate whose float residue landed below spade's
    // coordinate floor; the projection now writes that coordinate as
    // the zero it is, and the rim meshes.
    //
    // Kept LIVE rather than turned into a comment, for the reason
    // walls are live at all (`crate::walls`): a closure is a claim
    // about the kernel, and a claim the tour stops attempting is a
    // claim nobody re-checks. The lattice points swept when the
    // refusal was characterised are the ones re-run here — the two
    // this scene's findings entry named, plus two more the same sweep
    // turned up — so a regression reappears where it was first seen
    // rather than somewhere a fresh probe happened to look.
    //
    // f64: `mesh::tessellate` is the one door in this scene that is
    // not generic over the scalar — meshing is a rendering-side
    // operation and takes the run's own numbers.
    for (alpha_deg, rim) in [
        (24.0_f64, 0.85_f64),
        (26.0, 0.75),
        (30.0, 0.85),
        (34.0, 1.00),
    ] {
        let body = bulb::<f64>(
            band::<f64>(&meridian_at(alpha_deg * PI / 180.0, RF, rim, RLOOP), tol),
            Revolution::Full,
            tol,
        );
        let m = pncad::mesh::tessellate(&body, 1e-2, tol).unwrap_or_else(|e| {
            panic!(
                "the bottle at (flare {alpha_deg}°, rim {rim} m) refuses {e:?} — \
                 `mesh::planar`'s sub-floor case was closed and findings entry 9 \
                 says so; re-derive BOTH before trusting either"
            )
        });
        println!(
            "   wall 7 RETIRED — the bottle at (flare {alpha_deg}°, rim {rim} m) \
             tessellates: {} triangles",
            pncad::mesh::validate::triangle_count(&m)
        );
    }

    // Wall 8, RETIRED by #5499: the `circle` loop's mesh. Each wall is
    // two semicircles, a semicircle is not one rational Bézier, so
    // every lofted wall carries an interior knot of multiplicity =
    // degree — a C0 crease the mesher refused by name until the
    // trimmed lane learned to cut along it
    // (`work/tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`).
    // The scene's own loop is that loop, and its stop meshes it at the
    // stop's δ, so a regression reappears there.

    // Wall 9, RETIRED: the loop sweeping along its exact spine. The
    // scene's loop is that sweep (`bottle`), so a regression reappears
    // there.
}
