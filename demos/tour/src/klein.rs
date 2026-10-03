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
//!    the natural way** (walls 5 and 8). `sweep_body` carries the
//!    annulus around the U-turn — the loft's stacking statement is a
//!    fold over adjacent section pairs, each decided against its own
//!    base section's normal (issue 368). Two findings shape it:
//!    - the walls are `circle_split(.., 4, ..)`, not `circle` ([`annulus`]
//!      carries the gap comment). The `circle` loop is wall 5 (tier 3
//!      refuses `QuadratureBudget` at the default ε and finer,
//!      `work/quad/a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin`)
//!      and wall 8 (its semicircle walls' C0 knot, which the mesher
//!      refuses at every ε,
//!      `work/tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`);
//!    - the station count and skin degree decide whether the quartered
//!      loop meshes at the stop's δ, so [`STATIONS`] / [`V_DEGREE`] sit
//!      inside a measured neighbourhood of settings that all pass
//!      (`work/tess/a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate`).
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
//! 12. **The loop's caps are tilted, and the tilt is load-bearing**
//!     (wall 9). No public door pins a spine's end tangents:
//!     `NurbsCurve3::interpolate` takes no end derivatives, and
//!     nothing joins two exact arcs into one path. So the spine is an
//!     interpolant whose end tangents are 3.99e-4 rad off the axis, in
//!     mirror image, and each cap — the annulus in the plane normal to
//!     its end tangent — sits turned off the rim it meets, its edge up
//!     to 2·r·sin(tilt/2) ≈ 1.1e-4 m away: the numbers a declared REST
//!     contact (C7) would be asked to accept. The tilt cannot be
//!     authored away, and it is ALSO what lets the loop build:
//!     `sweep_places` carries every station by one minimal rotation
//!     from the BASE tangent, and the exact spine — end tangents
//!     exactly ±z, authored by hand as four rational quarter arcs —
//!     refuses `PathTangentReversal` at its last station (wall 9). The
//!     interpolant's mirrored tilts leave |u₀ × u₁| ≈ 3.1e-11 rather
//!     than 0, which is the C6 float knife edge
//!     `review_m5_pr10::review_half_turn_path_builds_on_the_float_knife_edge`
//!     pins as executed behaviour, not as intent. If the frame choice
//!     gains a real margin, this scene refuses. [`sweep_loop`] carries
//!     the gap comment.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p2, p3, v2, v3, validated};
use pncad::geom::NurbsCurve3;
use pncad::geom_brep::PropsError;
use pncad::geom_core::linalg::frame::path_start_frame;
use pncad::geom_core::{KnotVector, Point3, Tol};
use pncad::prelude::SurfaceKind;
use pncad::prelude::{ConstructedLoop, Open, Start, SurfaceKindSet, circle, circle_split, query};
use pncad::sweep::blend::{BlendError, fillet_edges};
use pncad::sweep::{LoftError, Revolution, RevolveAxis, SkinError, revolve, sweep_body};
use pncad::topo::readback::euler_counts;
use pncad::topo::{
    AtRestBody, Body, BooleanError, BooleanOp, EdgeKey, MassPropsError, Operand, ValidationError,
};

use crate::booleans::finished;
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
/// The stop's mesh tolerance — the δ the loop's neighbourhood was
/// measured at.
const DELTA: f64 = 1e-2;

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

/// The loop's spine as a user would author it EXACTLY: the same two
/// arcs as four rational quarter arcs (degree 2, weights 1, √½, 1),
/// so its end tangents are exactly +z and −z. Built only for wall 9.
fn exact_spine(m: &Meridian) -> NurbsCurve3<f64> {
    let (top_c, tube_c) = ((RLOOP, ZTOP), (RLOOP, m.z_tube));
    // The quarter points, written as the numbers they are: sampling
    // `over_arc`/`into_arc` there would leave trig residue of ~1e-16
    // in the end tangents, which is enough to build (finding 12).
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
///
/// GAP (library finding,
/// `work/tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`):
/// a `lofted` section says each wall as FOUR quarter arcs
/// (`circle_split`) where the natural spelling is `circle`. A
/// `circle` is two semicircles, a semicircle is not one rational
/// Bézier, so every lofted wall would carry a C0 knot the mesher
/// refuses — after the body built and validated, naming a `FaceKey`
/// (wall 8). The circle is the same circle; only its seam count is
/// authored.
fn annulus<S: Scalar>(m: &Meridian, cx: f64, lofted: bool, tol: Tol) -> Vec<ConstructedLoop<S>> {
    [m.ro, m.ri]
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

/// `section` swept along `spine` as ONE body (the loft's stacking
/// statement is per-slab, issue 368), drawn in the plane normal to the
/// spine's start tangent, which the kernel hands out
/// (`path_start_frame`). The section and the spine are `f64` because
/// `sweep_body` takes them so; the body comes out at `S`.
///
/// GAP (library finding,
/// `work/carve/a-half-turn-spine-sweeps-only-off-its-exact-tangents`):
/// the scene's loop rides a float knife edge (finding 12).
/// `sweep_places` carries every station by ONE minimal rotation from
/// the base tangent, and the loop's spine turns exactly a half turn,
/// so its end tangents are anti-parallel in ℝ. The exact spine refuses
/// `PathTangentReversal` (wall 9); the interpolant builds only because
/// its end tangents tilt 3.99e-4 rad in mirror image, leaving
/// |u₀ × u₁| ≈ 3.1e-11 where the C6 test reads `sin > 0.0`
/// (`review_m5_pr10::review_half_turn_path_builds_on_the_float_knife_edge`,
/// pinned as executed behaviour, not as intent). Give the frame choice
/// a real margin and this scene refuses.
fn sweep_loop<S: Scalar>(
    spine: &NurbsCurve3<f64>,
    section: &[ConstructedLoop<f64>],
    tol: Tol,
) -> Result<Body<S>, LoftError> {
    let (t0, _) = spine.domain();
    let (start, tangent) = spine.ders1(t0);
    let place =
        path_start_frame(start, tangent, tol).expect("the spine's start tangent fixes a frame");
    sweep_body::<S>(section, place, spine, STATIONS, V_DEGREE, tol).map(|l| l.body)
}

/// The bottle: the bulb and the loop, each finished, and the spine the
/// loop was swept along.
struct Bottle<S: Scalar> {
    bulb: AtRestBody<S>,
    top: AtRestBody<S>,
    spine: NurbsCurve3<f64>,
}

fn bottle<S: Scalar>(tol: Tol) -> Bottle<S> {
    let m = meridian();
    let spine = loop_spine(&m);
    let top = sweep_loop::<S>(&spine, &annulus::<f64>(&m, 0.0, true, tol), tol)
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

/// Wall 3's pinned refusal: the loop's NURBS edges at the union's
/// operand gate. Shared with `verbs_gate_r1_probes`, so the two
/// cannot drift.
fn wall3_pinned(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::CurvedEdgeUnsupported {
            operand: Operand::B,
            ..
        }
    )
}

/// Wall 4's pinned refusal: the flare's cone against a planar cap of
/// the loop, at the ∖/∩ revert roster. Both kinds and the op are
/// pinned, so a pair that changed kind reds. The cone has no arm under
/// any op, and the roster has no covered rung whatever the pair; it
/// answers before the edge gate that stops wall 3.
fn wall4_pinned(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::CurvedPairUnsupported {
            op: Some(BooleanOp::Subtract),
            kind: SurfaceKind::Cone,
            other_kind: SurfaceKind::Plane,
            ..
        }
    )
}

/// The ε rows the tour runs at. A wall whose outcome differs between
/// them names the row; any other ε is a row nobody measured, and says
/// so rather than guessing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EpsRow {
    Coarse,
    Default,
    Fine,
}

fn eps_row(tol: Tol) -> EpsRow {
    let at = |row: f64| (tol.eps() / row - 1.0).abs() < 1e-3;
    if at(1e-6) {
        EpsRow::Coarse
    } else if at(1e-9) {
        EpsRow::Default
    } else if at(1e-12) {
        EpsRow::Fine
    } else {
        panic!(
            "ε = {:e} is not a row the klein walls were measured at (1e-6, 1e-9, 1e-12)",
            tol.eps()
        )
    }
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
    // rational): a number with its half-width where the reporting
    // target is met, the narrowest bracket the certificate held where
    // it is not. The body's true volume is in it AND in the band, so
    // the two must meet.
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

    // Findings entry 12, measured on the body: each cap is a plane
    // through its spine end, normal to its spine end's tangent, holding
    // the meridian's two wall radii — so the cap is the bulb's rim
    // turned by that tangent's tilt, and its edge sits up to
    // 2·r·sin(tilt/2) off the rim it meets.
    let (t0, t1) = spine.domain();
    let ends = [spine.ders1(t0), spine.ders1(t1)];
    let tilts = ends.map(|(_, d)| (d.z.abs() / d.norm()).acos());
    let caps: Vec<(Point3<f64>, pncad::geom_core::Vec3<f64>)> = top
        .surfaces()
        .filter_map(|(_, sf)| match sf {
            pncad::topo::Surface::Plane { origin, normal, .. } => Some((*origin, *normal)),
            _ => None,
        })
        .collect();
    assert_eq!(caps.len(), 2, "the loop has two planar caps");
    for ((end, d), tilt) in ends.iter().zip(tilts) {
        let (origin, normal) = caps
            .iter()
            .copied()
            .find(|(o, _)| (*o - *end).norm() < 1e-12)
            .unwrap_or_else(|| panic!("no cap sits on the spine end {end:?}: {caps:?}"));
        let cap_tilt = (normal.z.abs() / normal.norm()).acos();
        assert!(
            (cap_tilt - tilt).abs() < 1e-12 && normal.cross(*d).norm() / d.norm() < 1e-12,
            "the cap at {origin:?} is tilted {cap_tilt:e} rad, not normal to its spine end \
             (tilt {tilt:e})"
        );
        for p in top
            .vertices()
            .map(|(_, v)| *top.get_point(v.point).expect("point"))
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
    let seams = tilts.map(|t| 2.0 * m.ro * (0.5 * t).sin());

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
        delta: DELTA,
        note: Some(format!(
            "tube diameter {:.2} m, wall {:.2} m; the wide rim's hole comes out at the \
             tube diameter by construction (centre radius R + RRIM = {:.3}, minor radii \
             {:.3}/{:.3}). Every blend in the bulb is an ARC IN THE MERIDIAN, exact and \
             free — and no rolling ball of that size fits the wall (walls 1-2). The loop's \
             certified volume [{v_lo:.6}, {v_hi:.6}] m^3 and mesh volume {v_mesh:.6} meet \
             Pappus's ring area {ring:.6} m^2 times spine length = {pappus:.6} within its \
             {:.2}% discretization allowance. Its caps meet the neck's and \
             the inner tube's rims {:.1e} / {:.1e} m off, each turned by its spine's end \
             tangent ({:.2e} / {:.2e} rad) — the numbers a declared REST contact would \
             be asked to accept",
            2.0 * R,
            WALL,
            R + RRIM,
            RRIM + WALL / 2.0,
            RRIM - WALL / 2.0,
            100.0 * allowance / pappus,
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
        wall3_pinned,
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

    // Wall 5: the loop with its section spelled the natural way, as
    // two `circle`s. It builds, tier-1 valid and closed, and tier 3 —
    // which every scene body passes — cannot decide the sign of its
    // volume on a rational swept wall at the loop's position. The
    // refusal is the flux enclosure's width (1.04e-5 m, the same at
    // every ε row) against a 1024·ε target, so it stands at the
    // default ε and finer, and at the coarse row the sign decides.
    // Measured across 9–65 stations at skin degrees 2 and 3, from the
    // path's start frame and from world axes: it refuses at every one.
    let round = sweep_loop::<S>(&loop_spine(&m), &annulus::<f64>(&m, 0.0, false, tol), tol)
        .expect("the `circle` loop builds");
    assert_eq!(
        pncad::topo::validate(&round),
        Ok(()),
        "the `circle` loop is tier-1 valid"
    );
    assert_eq!(pncad::topo::validate_closed(&round), Ok(()), "and closed");
    let round_tier3 = pncad::topo::validate_geometric(&round, tol);
    match eps_row(tol) {
        EpsRow::Coarse => {
            assert_eq!(
                round_tier3,
                Ok(()),
                "at ε = 1e-6 the `circle` loop's 1024·ε target is wider than its flux \
                 enclosure, so its sign decides"
            );
            println!("   wall 5 — not standing at ε = 1e-6: the `circle` loop passes tier 3 here");
        }
        EpsRow::Default | EpsRow::Fine => crate::walls::wall(
            "bottle",
            5,
            "validate the loop swept with `circle` sections at tier 3",
            round_tier3,
            |e| {
                matches!(
                    e[..],
                    [ValidationError::VolumeUncomputable {
                        source: MassPropsError::Face {
                            source: PropsError::QuadratureBudget { .. },
                            ..
                        },
                        ..
                    }]
                )
            },
            "and if wall 8 has retired too, spell [`annulus`]'s lofted walls as `circle`: \
             the scene's loop takes the natural section, and findings entry 5 and \
             `annulus`'s gap comment go",
        ),
    }

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
    let ring = validated(ring_plane, annulus::<S>(&m, RLOOP, false, tol), tol)
        .expect("the annulus validates");
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
        annulus::<f64>(&m, RLOOP, false, tol),
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

    // Wall 8: the `circle` loop's mesh. Each wall is two semicircles,
    // a semicircle is not one rational Bézier, so every lofted wall
    // carries an interior knot of multiplicity = degree — the C0
    // crease the mesher refuses, at every ε and every station count
    // and skin degree measured (9–65, 2 and 3).
    let round64 = sweep_loop::<f64>(&loop_spine(&m), &annulus::<f64>(&m, 0.0, false, tol), tol)
        .expect("the `circle` loop builds at f64");
    crate::walls::wall(
        "bottle",
        8,
        "tessellate the loop swept with `circle` sections",
        pncad::mesh::tessellate(&round64, DELTA, tol),
        |e| matches!(e, pncad::mesh::TessellateError::UnsupportedNurbsFace { .. }),
        "and if wall 5 has retired too, spell [`annulus`]'s lofted walls as `circle` \
         (work/tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late)",
    );

    // Wall 9: the loop along its EXACT spine — the two arcs as four
    // rational quarter arcs, end tangents exactly ±z, so the caps would
    // lie flat on the rims they meet. `sweep_places` carries every
    // station by one minimal rotation from the base tangent, and at an
    // exact half turn the last station's tangent is anti-parallel to it:
    // the frame refuses (finding 12).
    crate::walls::wall(
        "bottle",
        9,
        "sweep the loop along its exact two-arc spine",
        sweep_loop::<S>(&exact_spine(&m), &annulus::<f64>(&m, 0.0, true, tol), tol),
        |e| {
            matches!(
                e,
                LoftError::Skin(SkinError::PathTangentReversal { station })
                    if *station == STATIONS - 1
            )
        },
        "sweep the scene's loop along `exact_spine`, so its caps lie on the bulb's rims: \
         re-derive findings entry 12's seam measures (they become zero) and drop \
         `sweep_loop`'s gap comment",
    );
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
    //! Same pins as the walls themselves — one matcher each,
    //! `wall3_pinned` and `wall4_pinned`, so the two cannot drift.
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
        let Bottle {
            bulb: bulb_body,
            top,
            ..
        } = bottle::<f64>(tol);

        let joined = pncad::topo::union(&bulb_body, &top, tol)
            .expect_err("the bottle's pieces still cannot be joined");
        println!("klein wall 3: {joined:?}");
        assert!(
            wall3_pinned(&joined),
            "wall 3 must refuse the loop's NURBS edges at the operand gate: {joined:?}"
        );

        let trimmed = pncad::topo::subtract(&bulb_body, &top, tol)
            .expect_err("the self-intersection still cannot be trimmed");
        println!("klein wall 4: {trimmed:?}");
        assert!(
            wall4_pinned(&trimmed),
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
