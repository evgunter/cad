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
//!   tangent and interpolated as one curve. Two planar caps and eight
//!   rational NURBS walls (finding 5).
//!
//! The two bodies MEET on two annular faces, each of the loop's caps
//! turned off the bulb's rim by its spine's end tangent (finding 11).
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
//! it). Asking `fillet_edges` for that same torus is walls 1 and 2 —
//! which now refuse on the ball's SIZE, not on a missing arm.
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
//! 2. **A closed rim now meters an honest lever arm** (walls 1 and 2 —
//!    the pair is the evidence). A full revolve's latitude
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
//! 3. **The cone×cylinder fillet arm EXISTS now; what both walls meet
//!    is the RADIUS** (walls 1, 2). The `constant-radius fillet on
//!    CURVED support pairs` row shipped its coaxial half, so this
//!    corner's spine is no longer the obstacle — predicate 1 is. The
//!    blend the meridian draws has spine radius `RF`, and a ball that
//!    big does not fit the neck wall's own curvature, on either rim.
//!    That is not a defect: it is the reason the substitution is an
//!    improvement rather than a workaround (Ev, 2026-08-16). An arc
//!    in the profile is a CONSTRUCTED part of the wall and answers to
//!    no rolling ball; a post-hoc roll of the same size cannot exist.
//! 4. **Neither join can start** (walls 3, 4). `union` refuses at the
//!    operand gate before any pair is looked at:
//!    `CurvedEdgeUnsupported { operand: B }` — the loop is a sweep, its
//!    longitudinal edges are NURBS carriers, and rung-3 edges are what
//!    the curved zip MINTS, not what it consumes. `subtract` answers
//!    first at the revert roster with `{ op: Some(Subtract), kind:
//!    Cone, other_kind: Plane }` — the flare against a planar cap of
//!    the loop, a pair whose boxes MAY meet (box overlap
//!    over-approximates; the kernel cannot rule the meeting out, a
//!    weaker claim than that they do). So the bottle cannot be one
//!    body, and the self-intersection — the neck piercing the bulb,
//!    the one place a Klein bottle MUST cross itself in 3-space —
//!    cannot be trimmed.
//! 5. **The loop is ONE swept body, and its section is not spelled
//!    the natural way.** `sweep_body` carries the annulus around the
//!    U-turn — the loft's stacking statement is a fold over adjacent
//!    section pairs, each decided against its own base section's
//!    normal (issue 368) — from the plane `path_start_frame` hands out
//!    (the interpolated spine's start tangent is 3.99e-4 rad off +z).
//!    Two findings shape it:
//!    - the walls are `circle_split(.., 4, ..)`, not `circle` ([`annulus`]
//!      carries the gap comment): a lofted `circle` has semicircle
//!      walls with a C0 knot the mesher refuses, and that same body
//!      also fails tier 3 at the loop's position
//!      (`work/quad/a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin`);
//!    - the station count and skin degree decide whether the quartered
//!      loop meshes at the stop's δ, so [`STATIONS`] / [`V_DEGREE`] sit
//!      inside a measured neighbourhood of settings that all pass
//!      (`work/tess/a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate`).
//!
//!    The walls are rational, so the loop's volume is a certified
//!    ENCLOSURE — a bracket at the default ε — and [`stops`] asserts it
//!    holds Pappus's A·L.
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
//!    STEP file: the writer's outward/void shell classifier has
//!    closed forms for planar faces only, so a multi-shell CURVED
//!    solid refuses `CurvedShellClassification` — the KNOWN standing
//!    gate of OFFSET-DESIGN O6's demo-gates list, recorded here and
//!    never worked around.
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
//! 10. **The lattice cannot say "tangent straight leg to THIS point",
//!     and the drift is measurable.** After a declared-tangent joint
//!     off an arc, the only straight continuation the PATHS lattice
//!     offers is `.line(len)` — `.to(anchor)` belongs to a fillet's
//!     arrival side. So the inner tube's wall is placed by LENGTH from
//!     the rim arc's end tangent instead of by the author's own
//!     coordinate, and the revolve reconstructs its cylinder radius
//!     from the swept endpoints: the two walls that are geometrically
//!     the same cylinder come out with radii differing by up to ~38
//!     ulps across the proportions swept for finding 9. This is the
//!     same drift class the `tube_along_arc` door was built to retire
//!     (see the `tube` scene's bit-exact `minor_radius`), met from the
//!     profile side. Pinned in [`stops`].
//! 11. **What went RIGHT, and is worth stating.** The bottle's hole
//!     is the tube's diameter — Ev's own constraint — so the neck
//!     wall and the inner tube wall are literally the same cylinder
//!     about the same axis, twice, in one profile loop. Everything
//!     downstream copes: the band validates, the full revolve builds,
//!     the body is tier-3 green and exports STEP. What it does NOT do
//!     is notice: the revolve's cosurface merge is a run-ADJACENCY
//!     decision, and these two runs are not adjacent, so the bulb
//!     carries 12 faces on 12 surface keys — four cylinder faces that
//!     are geometrically two cylinders, described four ways (four
//!     different chart origins, and the radius drift of finding 10).
//!     Any predicate whose certified lane is "same `SurfaceKey`" —
//!     M9-2's chart-region rule is the live example — sees two charts
//!     where the model has one. [`stops`] pins the whole picture.
//!     The loop meets the bulb at its two caps, and neither is
//!     coincident: each cap is the annulus in the plane normal to the
//!     interpolated spine's end tangent, which is 3.99e-4 rad off the
//!     axis, so a cap's vertices sit 2·r·sin(tilt/2) ≈ 1.1e-4 m off
//!     the rim they meet — the numbers a declared REST contact (C7)
//!     would be asked to accept.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p2, p3, v2, v3, validated};
use pncad::geom::NurbsCurve3;
use pncad::geom_core::linalg::frame::path_start_frame;
use pncad::geom_core::{Point3, Tol};
use pncad::prelude::SurfaceKind;
use pncad::prelude::{ConstructedLoop, Open, Start, SurfaceKindSet, circle, circle_split, query};
use pncad::sweep::blend::{BlendError, fillet_edges};
use pncad::sweep::{Revolution, RevolveAxis, revolve, sweep_body};
use pncad::topo::readback::euler_counts;
use pncad::topo::{Body, BooleanError, BooleanOp, EdgeKey, Operand};

use crate::scalar::{Scalar, sketch_frame};
use crate::{SceneBody, Stop, View};

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
const ZTOP: f64 = 3.0;
/// Height of the spine corner where the neck turns into the flare.
const ZNECK: f64 = 2.5;
/// Spine radius of the neck→flare blend (finding: authored in the
/// meridian, and a rolling ball this big does not fit the neck wall's
/// curvature — walls 1, 2).
const RF: f64 = 0.30;
/// Spine radius of the wide bottom rim — the "wide torus" the surface
/// turns back on. Its hole comes out at exactly `R` by construction.
const RRIM: f64 = 0.80;
/// Spine radius of each of the top loop's two arcs.
const RLOOP: f64 = 1.20;

/// The 270° arc, over the top.
const SWEEP_OVER: f64 = 1.5 * PI;
/// The 90° arc, turning back onto the bottle's axis.
const SWEEP_IN: f64 = 0.5 * PI;
/// The loop's spine samples: one interval per 7.5° on each arc.
const SPINE_OVER: u32 = 36;
const SPINE_IN: u32 = 12;
/// Sections the loop's sweep places along its spine, and the skin's
/// degree through them. Chosen inside a measured neighbourhood rather
/// than at a lucky cell: at v-degree 2, every station count from 9 to
/// 19 passes tier 3 and meshes at the stop's δ at ε = 1e-6, 1e-9 and
/// 1e-12, and so does 13 stations at v-degree 3. The edges are
/// 21 stations at v-degree 2 (the mesher refuses `CertificateExceeded`)
/// and 17 at v-degree 3 (at ε = 1e-6 the volume continuation refuses
/// with no bracket) — `work/tess/a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate`.
const STATIONS: usize = 13;
const V_DEGREE: usize = 2;

/// The meridian's derived geometry, in sketch coordinates
/// `(radius, height)`. Structure selection is f64 (C6): these are the
/// tangent points and centres the band is authored from, chosen once
/// and embedded into whatever scalar the scene runs at.
struct Meridian {
    /// Spine radius of the neck→flare blend (a parameter, not a
    /// constant, because wall 7 builds bottles at other proportions).
    rf: f64,
    /// Outer and inner offsets of the tube radius.
    ro: f64,
    ri: f64,
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
fn meridian() -> Meridian {
    meridian_at(ALPHA, RF, RRIM, RLOOP)
}

/// The meridian at any proportions. Only wall 7 asks for proportions
/// other than the bottle's own: it re-runs the flare-angle × rim-radius
/// lattice points that used to decide whether the cap meshed.
fn meridian_at(alpha: f64, rf: f64, rrim: f64, rloop: f64) -> Meridian {
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
fn band<S: Scalar>(m: &Meridian, tol: Tol) -> ConstructedLoop<S> {
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
/// blend the band authors for free — walls 1 and 2.
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

/// Revolves a meridian band about the bottle's axis. `Full` is the
/// bulb; the partial form exists only so wall 2 can ask the same
/// question of an OPEN rim (findings entry 2).
fn bulb<S: Scalar>(loop_: ConstructedLoop<S>, revolution: Revolution<S>, tol: Tol) -> Body<S> {
    let plane = sketch_frame(
        p3::<S>(0.0, 0.0, 0.0),
        v3::<S>(1.0, 0.0, 0.0),
        v3::<S>(0.0, 0.0, 1.0),
        tol,
    );
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

/// The top loop's spine over the top, at angle `th` along it, in the
/// world xz-plane: the 270° arc centred one `RLOOP` to the +x side of
/// the neck's top rim, leaving the rim straight up.
fn over_arc(th: f64) -> (f64, f64) {
    (RLOOP * (1.0 - th.cos()), ZTOP + RLOOP * th.sin())
}

/// The spine's descent, at angle `psi` on the 90° arc centred one
/// `RLOOP` to the +x side of the inner tube's top rim: `psi = π/2` is
/// where it meets [`over_arc`]'s end, `psi = π` is the rim, reached
/// straight down.
fn into_arc(m: &Meridian, psi: f64) -> (f64, f64) {
    (RLOOP * (1.0 + psi.cos()), m.z_tube + RLOOP * psi.sin())
}

/// The top loop's whole spine: the two tangent arcs, sampled at exact
/// points ([`SPINE_OVER`] + [`SPINE_IN`] intervals) and interpolated
/// at degree 3 — `sweep_body` takes any `NurbsCurve3`, so the U-turn
/// is ONE path.
fn loop_spine(m: &Meridian) -> NurbsCurve3<f64> {
    let over =
        (0..=SPINE_OVER).map(|k| over_arc(SWEEP_OVER * f64::from(k) / f64::from(SPINE_OVER)));
    let into = (1..=SPINE_IN)
        .map(|k| into_arc(m, 0.5 * PI + SWEEP_IN * f64::from(k) / f64::from(SPINE_IN)));
    let points: Vec<Point3<f64>> = over
        .chain(into)
        .map(|(x, z)| Point3::new(x, 0.0, z))
        .collect();
    NurbsCurve3::interpolate(&points, 3).expect("the loop's spine interpolates")
}

/// The tube's annular cross-section, radii `R ± WALL/2` about
/// `(cx, 0)` in its sketch plane.
///
/// GAP (library finding,
/// `work/tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`):
/// a `lofted` section says each wall as FOUR quarter arcs
/// (`circle_split`) where the natural spelling is `circle`. A
/// `circle` is two semicircles, a semicircle is not one rational
/// Bézier, so every lofted wall would carry a C0 knot the mesher
/// refuses — after the body built and validated, naming a `FaceKey`.
/// The circle is the same circle; only its seam count is authored.
fn annulus<S: Scalar>(cx: f64, lofted: bool, tol: Tol) -> Vec<ConstructedLoop<S>> {
    [R + WALL / 2.0, R - WALL / 2.0]
        .into_iter()
        .map(|r| {
            let (centre, radius) = (p2::<S>(cx, 0.0), S::from_f64(r));
            if lofted {
                circle_split(centre, radius, 4, S::from_f64(0.0), tol)
                    .expect("a wall of four quarter arcs")
                    .into()
            } else {
                circle(centre, radius, tol).expect("a wall").into()
            }
        })
        .collect()
}

/// The top loop: the annulus swept along the whole U-turn spine as ONE
/// body (the loft's stacking statement is per-slab, issue 368).
///
/// The section is drawn in the plane normal to the path's start
/// tangent, which the kernel hands out (`path_start_frame`): the
/// interpolant's start tangent is 3.99e-4 rad off +z, so a world-axis
/// placement would tilt the section off the plane the sweep carries,
/// at every station. The section and the path are `f64` because
/// `sweep_body` takes them so; the body comes out at `S`.
fn top_loop<S: Scalar>(m: &Meridian, tol: Tol) -> Body<S> {
    let path = loop_spine(m);
    let (t0, _) = path.domain();
    let (start, tangent) = path.ders1(t0);
    let place =
        path_start_frame(start, tangent, tol).expect("the spine's start tangent fixes a frame");
    sweep_body::<S>(
        &annulus::<f64>(0.0, true, tol),
        place,
        &path,
        STATIONS,
        V_DEGREE,
        tol,
    )
    .expect("the annulus sweeps along the loop's whole spine")
    .body
}

/// The bottle's two bodies: the bulb, then the loop.
fn bottle<S: Scalar>(tol: Tol) -> [Body<S>; 2] {
    let m = meridian();
    [
        bulb(band::<S>(&m, tol), Revolution::Full, tol),
        top_loop::<S>(&m, tol),
    ]
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
pub fn stops(tol: Tol) -> Vec<Stop> {
    let m = meridian();
    let [bulb, top] = bottle::<f64>(tol);

    // The loop's volume in the continuum is the annulus times the
    // spine length (Pappus: a planar spine, the section's centroid ON
    // it and the section symmetric about its plane), A·RLOOP·2π. The
    // sweep reaches that through two discretizations — the spine
    // interpolant and the skin through STATIONS sections — and its
    // walls are rational, so the kernel's reading is a certified
    // ENCLOSURE: a number with its half-width where the reporting
    // target is met, the narrowest bracket the certificate held where
    // it is not. The oracle is that the enclosure holds A·L, which
    // stays exactly as strong as the certificate at every ε.
    let ring = PI * ((R + WALL / 2.0).powi(2) - (R - WALL / 2.0).powi(2));
    let pappus = ring * RLOOP * (SWEEP_OVER + SWEEP_IN);
    let (v_lo, v_hi) = match pncad::topo::validate_geometric_certificate(&top, tol)
        .unwrap_or_else(|e| panic!("the loop's tier 3 refused: {e:?}"))
        .measure()
    {
        Ok(p) => (p.volume - p.volume_pad, p.volume + p.volume_pad),
        Err(pncad::topo::TargetUnreached {
            bracket: Some(b), ..
        }) => (b.volume_lo, b.volume_hi),
        Err(unreached) => panic!("the loop's mass properties: {unreached}"),
    };
    assert!(
        v_lo <= pappus && pappus <= v_hi,
        "Pappus's A·L = {pappus} lies OUTSIDE the loop's certified enclosure [{v_lo}, {v_hi}]"
    );
    // The census: two annular caps and four quarter walls per side —
    // a hollow tube, so one handle.
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
        (16, 24, 10, 2, 1, Ok(1)),
        "the loop is one genus-1 shell of 10 faces; got {census:?}"
    );

    // Findings entry 11, executed: the same cylinder, said four ways.
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
    // Findings entry 10, PINNED. The outer pair's radii should both
    // be exactly R + WALL/2: it is an authored number. One of them is
    // not, because its run's position is DERIVED — the only straight
    // continuation the lattice offers after a declared-tangent joint
    // is `.line(len)`, so the tube wall's radius comes out of the rim
    // arc's end tangent instead of out of the author's hand.
    let mut outer: Vec<f64> = cylinders
        .iter()
        .map(|(_, r)| *r)
        .filter(|r| *r > R)
        .collect();
    outer.sort_by(f64::total_cmp);
    assert_eq!(outer.len(), 2, "two outer-wall cylinders");
    let drift = outer[1] - outer[0];
    assert!(
        drift > 0.0,
        "the two outer-wall cylinders now carry the SAME radius. If the PATHS \
         lattice grew a tangent-straight-leg-to-an-anchor (`.tangent().to(p)`), or \
         the revolve stopped reconstructing a wall radius from swept endpoints, \
         findings entry 10 has retired — delete it and this pin."
    );
    assert!(
        drift < 1e-14,
        "the derived wall radius drifted {drift:e} m from the authored one — far \
         beyond the tens of ulps findings entry 10 records"
    );

    // Findings entry 11, second half: how exactly do the pieces meet?
    // Each cap of the loop is the annulus in the plane normal to the
    // spine's end tangent, and the interpolant's end tangents are
    // `tilt` off the axis, so each cap is the bulb's rim turned by
    // that angle about a diameter: a cap vertex sits at most
    // 2·r·sin(tilt/2) from the rim circle it meets.
    let path = loop_spine(&m);
    let (t0, t1) = path.domain();
    let tilts = [path.ders1(t0).1, -path.ders1(t1).1].map(|d| (d.z / d.norm()).acos());
    let mut seams = [0.0_f64; 2];
    for (seam, (h, tilt)) in seams
        .iter_mut()
        .zip([ZTOP, m.z_tube].into_iter().zip(tilts))
    {
        let cap: Vec<Point3<f64>> = top
            .vertices()
            .map(|(_, v)| *top.get_point(v.point).expect("point"))
            .filter(|p| (p.z - h).abs() < WALL)
            .collect();
        assert_eq!(cap.len(), 8, "each cap is an annulus of 4 + 4 vertices");
        *seam = cap
            .iter()
            .map(|p| {
                let rho = p.x.hypot(p.y);
                let r = if rho > R {
                    R + WALL / 2.0
                } else {
                    R - WALL / 2.0
                };
                (rho - r).hypot(p.z - h)
            })
            .fold(0.0, f64::max);
        let bound = 2.0 * (R + WALL / 2.0) * (0.5 * tilt).sin();
        assert!(
            (*seam - bound).abs() < 1e-12,
            "a cap rim at height {h} sits {seam:e} m off the bulb's, where its tilt of \
             {tilt:e} rad puts it {bound:e}"
        );
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
                 faces, each cap tilted off the bulb's rim by its spine's end tangent, \
                 and they cannot be joined: union refuses the loop's NURBS edges at the \
                 operand gate, and subtract refuses the bulb's cone flare against a \
                 planar cap at the revert roster. The neck passes through the flare \
                 uncut for that second reason";
    vec![Stop {
        name: "klein",
        caption: "Klein bottle — two bodies the kernel cannot join".to_string(),
        montage: true,
        story,
        ops: "revolve(Full) of a filleted meridian band + sweep_body of an annulus \
              (circle_split x 4) along an interpolated U-turn spine from path_start_frame; \
              NO boolean, NO fillet_edges, NO shell — see klein::wall_probes",
        delta: 1e-2,
        note: Some(format!(
            "tube diameter {:.2} m, wall {:.2} m; the wide rim's hole comes out at the \
             tube diameter by construction (centre radius R + RRIM = {:.3}, minor radii \
             {:.3}/{:.3}). Every blend in the bulb is an ARC IN THE MERIDIAN, exact and \
             free — and no rolling ball of that size fits the wall (walls 1-2). The loop's \
             certified volume [{v_lo:.6}, {v_hi:.6}] m^3 holds Pappus's ring area \
             {ring:.6} m^2 times spine length = {pappus:.6}. Its caps meet the neck's and \
             the inner tube's rims {:.1e} / {:.1e} m off, each turned by its spine's end \
             tangent ({:.2e} / {:.2e} rad) — the numbers a declared REST contact would \
             be asked to accept",
            2.0 * R,
            WALL,
            R + RRIM,
            RRIM + WALL / 2.0,
            RRIM - WALL / 2.0,
            seams[0],
            seams[1],
            tilts[0],
            tilts[1],
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
            SceneBody::plain("klein_bulb", [0.38, 0.62, 0.72], bulb).transparent(55),
            // A colour of its own: the loop is the piece that runs
            // INSIDE the bulb, and seeing where it enters is the point.
            SceneBody::plain("klein_loop", [0.80, 0.34, 0.24], top).transparent(45),
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
    let [bulb_body, top] = bottle::<S>(tol);
    let band_radius = S::from_f64(RF);

    // Walls 1 and 2 are ONE question asked of two bodies, and the
    // pair is the finding: the same corner, on a full and a partial
    // revolve of the SAME band.
    let sharp_full = bulb::<S>(sharp_band::<S>(&m, tol), Revolution::Full, tol);
    let sharp_part = bulb::<S>(
        sharp_band::<S>(&m, tol),
        Revolution::Partial(S::from_f64(5.0)),
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
    // The REASON moved, and the move is the news. The cone×cylinder arm
    // exists now, so "the analytic arm is missing" is no longer what
    // stops this: what stops it is the bulb's own blend radius, which is
    // larger than the neck wall's curvature allows a rolling ball to be.
    // That is predicate 1, and it is the same fact the meridian
    // authoring exploits — an arc in the profile is a CONSTRUCTED part
    // of the wall and answers to no rolling ball.
    crate::walls::wall(
        "bottle",
        1,
        "fillet the neck→flare corner on the FULL revolve at the radius the band \
         authors by hand",
        fillet_edges(&sharp_full, &[full_edges[0]], band_radius, tol),
        |e| matches!(e.error, BlendError::RadiusHeadroom { .. }),
        "roll a ball as big as the blend the meridian draws for free",
    );
    crate::walls::wall(
        "bottle",
        2,
        "fillet the SAME corner on a partial revolve (open rim, honest lever)",
        fillet_edges(&sharp_part, &[part_edges[0]], band_radius, tol),
        // The SAME refusal as wall 1, and that is the pair's point: the
        // lever is honest on both rims, the dihedral decides on both,
        // and what stops both is the ball's own size against the neck
        // wall's curvature — not the closedness of the rim, and no
        // longer a missing arm.
        |e| matches!(e.error, BlendError::RadiusHeadroom { .. }),
        "roll a ball as big as the blend the meridian draws for free",
    );

    // Wall 3: the bottle is ONE surface. Its two bodies meet on
    // annular faces — the REST mate — and the union refuses before
    // any pair is looked at: the loop is a sweep, its longitudinal
    // edges are NURBS carriers, and the operand gate admits a rung-3
    // edge in no INPUT operand (rung-3 is what the curved zip mints,
    // not what it consumes) —
    // `work/cleave/boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule`.
    crate::walls::wall(
        "bottle",
        3,
        "join the loop to the bulb (annular mate)",
        pncad::topo::union(&bulb_body, &top, tol),
        |e| {
            matches!(
                e,
                BooleanError::CurvedEdgeUnsupported {
                    operand: Operand::B,
                    ..
                }
            )
        },
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
        |e| {
            // The matcher pins BOTH kinds as well as the op, so a pair
            // that changed kind reds here.
            //
            // The pair is (Cone, Plane) — the flare against a planar
            // cap of the loop. The refusal is the ∖/∩
            // revert roster's: the cone has no arm under any op, and
            // the roster has no covered rung whatever the pair; it
            // answers before the edge gate that stops wall 3.
            matches!(
                e,
                BooleanError::CurvedPairUnsupported {
                    op: Some(BooleanOp::Subtract),
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Plane,
                    ..
                }
            )
        },
        "trim the self-intersection instead of letting the walls interpenetrate",
    );

    // Wall 6 (RE-BASELINED by VERBS-RING): the one-call hollow ring —
    // what the loop would be if it closed on itself instead of
    // entering the bulb — now BUILDS: the holed full revolve executes
    // as revolve(outer) − revolve(hole-as-outer) through the shared
    // void-insertion door (the degenerate no-crossing arm), so the
    // probe asserts the two-shell tier-valid build it used to pin as
    // a refusal. The wall that REMAINS for this shape is its STEP
    // export: the writer's outward/void shell classifier has closed
    // forms for planar faces only, so a multi-shell CURVED solid
    // refuses typed — OFFSET-DESIGN O6's known standing demo gate,
    // recorded here and never worked around.
    let ring_plane = sketch_frame(
        p3::<S>(0.0, 0.0, 0.0),
        v3::<S>(1.0, 0.0, 0.0),
        v3::<S>(0.0, 0.0, 1.0),
        tol,
    );
    let ring =
        validated(ring_plane, annulus::<S>(RLOOP, false, tol), tol).expect("the annulus validates");
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
    let ring_f64 = validated(
        sketch_frame(
            p3::<f64>(0.0, 0.0, 0.0),
            v3::<f64>(1.0, 0.0, 0.0),
            v3::<f64>(0.0, 0.0, 1.0),
            tol,
        ),
        annulus::<f64>(RLOOP, false, tol),
        tol,
    )
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
            &hollow64.body,
            &pncad::step_export::StepOptions {
                product_name: "hollow_ring".into(),
                ..Default::default()
            },
            tol,
        ),
        |e| {
            matches!(
                e,
                pncad::step_export::StepExportError::CurvedShellClassification { .. }
            )
        },
        "record that the writer's outward/void classifier grew a curved arm, update \
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
}

#[cfg(test)]
mod verbs_gate_r1_probes {
    //! The bottle's two boolean walls, as a TEST rather than only as
    //! a run-the-whole-tour probe.
    //!
    //! `wall_probes` reaches these two through a full render pass, so
    //! a change to the boxes under the operand gate could only be
    //! re-measured by rebuilding and re-running the tour. That is a
    //! twenty-minute answer to a one-second question, and the walls
    //! are exactly what an operand-gate change moves.
    //!
    //! Same pins as the walls themselves, so the two cannot drift.
    //!
    //! **Why the operand gate's covered-pair rung cannot reach wall 4.**
    //! The gate does not refuse a pair the caller's declarations speak
    //! for — but "the caller's declarations" is the
    //! `BooleanDeclarations` value handed to the op, and both walls
    //! below go through `pncad::topo::union` and `pncad::topo::subtract`,
    //! the doors that take none. (Wall 3 refuses earlier still, on the
    //! loop's NURBS edges, which no declaration speaks for.)
    //!
    //! Wall 4 is doubly out of reach and the second reason is the more
    //! durable one: it is a SUBTRACT, and the revert roster it refuses
    //! at, on the flare's cone, has no covered rung at all. A
    //! declaration supplies the verdict a germ arm would have; it
    //! cannot supply a seam lane to revert through, so declaring the
    //! cone's pairs would not move wall 4. (The torus is on that roster:
    //! a declared torus pair passes it, and `mate7a_torus_rest` pins
    //! where it stops.)

    use super::*;

    #[test]
    fn the_bottles_two_boolean_walls_name_the_pairs_the_scene_claims() {
        let tol = Tol::witness();
        let [bulb_body, top] = bottle::<f64>(tol);

        let joined = pncad::topo::union(&bulb_body, &top, tol)
            .expect_err("the bottle's pieces still cannot be joined");
        println!("klein wall 3: {joined:?}");
        assert!(
            matches!(
                joined,
                BooleanError::CurvedEdgeUnsupported {
                    operand: Operand::B,
                    ..
                }
            ),
            "wall 3 must refuse the loop's NURBS edges at the operand gate: {joined:?}"
        );

        let trimmed = pncad::topo::subtract(&bulb_body, &top, tol)
            .expect_err("the self-intersection still cannot be trimmed");
        println!("klein wall 4: {trimmed:?}");
        assert!(
            matches!(
                trimmed,
                BooleanError::CurvedPairUnsupported {
                    op: Some(BooleanOp::Subtract),
                    kind: SurfaceKind::Cone,
                    other_kind: SurfaceKind::Plane,
                    ..
                }
            ),
            "wall 4 must name the flare against a planar cap of the loop: {trimmed:?}"
        );
    }
}

#[cfg(test)]
mod r1_mesh2_review_probes {
    //! R1 review probes for MESH-2 (PR #1421, issue 555): the
    //! flare-angle × rim-radius lottery cells, exercised through the
    //! public `mesh::tessellate` door on the real Klein bulb. The
    //! probe PRINTS each cell's outcome rather than asserting, so the
    //! identical file runs on the merge base (where the four cells
    //! should refuse) and on the head (where they should mesh).

    use super::*;

    #[test]
    fn r1_lottery_cells_outcomes() {
        let tol = Tol::witness();
        for (alpha_deg, rim) in [
            (24.0_f64, 0.85_f64), // PR re-sweep cell
            (26.0, 0.75),         // PR re-sweep cell
            (30.0, 0.85),         // issue 555 cell
            (34.0, 1.00),         // issue 555 cell
            (30.0, 0.80),         // the shipped bottle: control, always meshed
            (28.0, 0.85),         // an "ok" neighbour in both sweeps
        ] {
            let body = bulb::<f64>(
                band::<f64>(&meridian_at(alpha_deg * PI / 180.0, RF, rim, RLOOP), tol),
                Revolution::Full,
                tol,
            );
            match pncad::mesh::tessellate(&body, 1e-2, tol) {
                Ok(m) => println!(
                    "R1PROBE cell ({alpha_deg}, {rim}): MESHED {} triangles",
                    pncad::mesh::validate::triangle_count(&m)
                ),
                Err(e) => println!("R1PROBE cell ({alpha_deg}, {rim}): REFUSED {e:?}"),
            }
        }
    }
}
