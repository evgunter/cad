//! **The Klein bottle** — the tour's non-orientable stop, and the
//! kernel's densest wall list so far.
//!
//! # What is actually built
//!
//! A Klein bottle is a 2-manifold, and this kernel is manifold-first
//! and SOLID-first (D1): a zero-thickness sheet is not a body it can
//! hold. So the model is the honest 3-D stand-in Evan asked for — a
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
//!   and back down INTO the bulb — is two thin-walled elbows, each a
//!   PARTIAL `revolve` of the annular cross-section about the elbow's
//!   own axis: 270° over the top, then 90° turning back onto the
//!   bottle's axis.
//!
//! The three bodies MEET, exactly, on three coincident annular faces.
//! They are not joined, because nothing in the kernel can join them
//! (wall 3). The descending elbow passes THROUGH the bulb's cone wall
//! with no hole cut in it, because nothing in the kernel can cut it
//! (wall 4) — which is, by luck, exactly how the classic picture is
//! drawn, but it is a refusal and not a choice.
//!
//! # Why the construction has this shape
//!
//! Evan's sketch was: torus loop → fillet → cone → tangent → wide
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
//! is what the loop is. (This is geometry, not a kernel limit — but
//! it is what makes the loop two BODIES, which is a kernel limit:
//! wall 6.)
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
//! than in a rolling ball afterwards (Evan, 2026-08-16 — the
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
//!    improvement rather than a workaround (Evan, 2026-08-16). An arc
//!    in the profile is a CONSTRUCTED part of the wall and answers to
//!    no rolling ball; a post-hoc roll of the same size cannot exist.
//! 4. **No boolean may touch a Cone or a Torus face** (walls 3, 4).
//!    The operand gate is per-FACE-KIND and it rejects the whole
//!    body: `union` refuses `CurvedBooleanUnsupported { kind: Torus }`
//!    and `subtract` refuses `CurvedOpUnsupported` before either
//!    reaches a pair. So the bottle cannot be one body, and the
//!    self-intersection — the neck piercing the bulb, the one place a
//!    Klein bottle MUST cross itself in 3-space — cannot be trimmed.
//! 5. **`sweep_body` cannot carry a section around a U-turn**
//!    (wall 5). The loop's whole spine is one path and would be ONE
//!    body, but the loft's stacking trilean compares only the LAST
//!    placement against the FIRST section's plane normal; a path that
//!    ends behind where it started refuses `ReversedStacking`
//!    wholesale, however well every consecutive pair stacks. That
//!    gate is what makes the loop two bodies rather than one.
//! 6. **`tube_along_arc` is solid-only** — the torus door takes a
//!    `minor_radius` and no wall, so a HOLLOW tube cannot use it and
//!    must be re-said as a partial revolve of an annulus. The door's
//!    whole point is that it stores the caller's intent parameters
//!    bit-exactly (`tube` scene); a thin tube gives that up. NOT a
//!    probe: the parameter does not exist, so there is nothing to
//!    refuse.
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
//! 8. **Edge selection by adjacent surface kinds is document-layer
//!    only.** `select_where` + `GeomPred::AdjacentKinds` is the
//!    ratified way to say "the cone×cylinder corners" (the die scene
//!    says exactly that), but it takes an `Evaluation`, so a body
//!    built by calling `revolve` directly has no selector at all:
//!    [`corner_edges`] below is a hand-rolled scan over
//!    `body.edges()` reading `get_surface(..)` through two
//!    back-pointers. NOT a probe: it is a gap in reach, not a
//!    refusal.
//! 9. **A valid body the tessellator refuses, at ordinary
//!    proportions** (wall 7). `mesh::planar`'s own module docs bank
//!    exactly one uncovered case: when a planar face's boundary
//!    points carry off-plane noise, the chart frame's FAR point has
//!    an *engineered* exact-zero v-coordinate whose float residue is
//!    ~ν², which for small ν is nonzero but below spade's coordinate
//!    floor (`MIN_ALLOWED_VALUE` = 2⁻¹⁴²), and the face refuses
//!    `Triangulation`. That paragraph calls the case "synthetic today
//!    (no corpus body hits it)". **This bottle hits it.** The bulb's
//!    annular top rim — a slit annulus, 0.05 m wide, at plain
//!    coordinates — refuses at EVERY δ with its far point projected to
//!    (0.4978884624952483, 3.94e-47). Whether it refuses is a
//!    roundoff lottery over parameters that have nothing to do with
//!    the cap: sweeping the flare angle and the rim radius, the
//!    refusal appears at (30°, 0.85 m) and (34°, 1.00 m) and not at
//!    their neighbours. The bottle ships at a rim radius of 0.80 m,
//!    which triangulates; wall 7 builds the SAME bottle at 0.85 m and
//!    pins the refusal, so this stops being synthetic.
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
//!     is the tube's diameter — Evan's own constraint — so the neck
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
//!     The three inter-body joints land coincident: the elbow↔elbow
//!     seam to 5e-16 m, the loop↔neck seam bit-exactly — the numbers a
//!     declared REST contact (C7) would be asked to accept.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p2, p3, v2, v3, validated};
use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Affine3, Band, Mat3, Point3, Tol};
use pncad::prelude::{Open, ProfileLoop, Start, circle};
use pncad::profile::SketchPlane;
use pncad::sweep::fillet::{FilletError, fillet_edges};
use pncad::sweep::{LoftError, Revolution, RevolveAxis, revolve};
use pncad::topo::{Body, BooleanError, EdgeKey, Operand};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};

// ---------------------------------------------------------------
// The bottle's dimensions (metres). Chosen, not measured: a stylized
// bottle the kernel can state exactly beats a literal one it must
// approximate (the lily's rule).
// ---------------------------------------------------------------

/// Spine radius of the tube — half Evan's "tube diameter", the one
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

/// The meridian's derived geometry, in sketch coordinates
/// `(radius, height)`. Structure selection is f64 (C6): these are the
/// tangent points and centres the band is authored from, chosen once
/// and embedded into whatever scalar the scene runs at.
struct Meridian {
    /// Spine radius of the neck→flare blend (a parameter, not a
    /// constant, because wall 7 needs a SECOND bottle).
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
/// other than the bottle's own — and the only thing it changes is the
/// rim radius.
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
fn band<S: Scalar>(m: &Meridian, tol: Tol) -> ProfileLoop<S> {
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
fn sharp_band<S: Scalar>(m: &Meridian, tol: Tol) -> ProfileLoop<S> {
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
fn bulb<S: Scalar>(loop_: ProfileLoop<S>, revolution: Revolution<S>, tol: Tol) -> Body<S> {
    let plane = SketchPlane::from_frame(
        p3::<S>(0.0, 0.0, 0.0),
        v3::<S>(1.0, 0.0, 0.0),
        v3::<S>(0.0, 0.0, 1.0),
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

/// One elbow of the top loop: the annular cross-section, revolved
/// about an axis one `RLOOP` to the +x side of the bottle's own.
///
/// The sketch plane is HORIZONTAL, at the elbow's own end: the
/// annulus sits on the bottle's axis and the elbow axis is the
/// sketch's +y line at x = `RLOOP`. That is the only frame in which
/// both the cross-section and the elbow axis are in one plane, which
/// is what a revolve needs — and it puts the start cap exactly on the
/// annular rim this elbow has to meet.
///
/// The angle is NEGATIVE because the placed axis is −ŷ (the revolve
/// meters its radial coordinate from `dir`, and the annulus must land
/// on the `r ≥ 0` side): the right-hand rule about −ŷ then carries
/// the section UP, which is where the loop goes.
fn elbow<S: Scalar>(z0: f64, sweep: f64, tol: Tol) -> Body<S> {
    let plane = SketchPlane::from_frame(
        p3::<S>(0.0, 0.0, z0),
        v3::<S>(1.0, 0.0, 0.0),
        v3::<S>(0.0, 1.0, 0.0),
    );
    let annulus = vec![
        circle(p2::<S>(0.0, 0.0), S::from_f64(R + WALL / 2.0), tol)
            .expect("the outer wall")
            .into(),
        circle(p2::<S>(0.0, 0.0), S::from_f64(R - WALL / 2.0), tol)
            .expect("the inner wall")
            .into(),
    ];
    revolve(
        &validated(plane, annulus, tol).expect("the annulus validates"),
        RevolveAxis {
            origin: p2::<S>(RLOOP, 0.0),
            dir: v2::<S>(0.0, -1.0),
        },
        Revolution::Partial(S::from_f64(-sweep)),
        tol,
    )
    .expect("the elbow revolves")
    .body
}

/// The three bodies of the bottle, in surface order: bulb, then the
/// loop's two arcs.
fn bottle<S: Scalar>(tol: Tol) -> [Body<S>; 3] {
    let m = meridian();
    [
        bulb(band::<S>(&m, tol), Revolution::Full, tol),
        elbow::<S>(ZTOP, SWEEP_OVER, tol),
        elbow::<S>(m.z_tube, SWEEP_IN, tol),
    ]
}

/// Every edge of `body` whose two faces are a cone and a cylinder.
///
/// Finding 8: this is what `select_where(&ev, node, &edges,
/// &[GeomPred::AdjacentKinds(cone, cylinder)], &params)` says in one
/// line at the document layer — and there is no kernel-level door for
/// it, so a directly-built body scans its own arena through two
/// back-pointers instead.
fn corner_edges<S: Scalar>(body: &Body<S>, a: SurfaceKind, b: SurfaceKind) -> Vec<EdgeKey> {
    let kind_at = |he| {
        let l = body.get_half_edge(he)?.parent_loop;
        let f = body.get_loop(l)?.face;
        body.get_surface(body.get_face(f)?.surface)
            .map(SurfaceKind::of)
    };
    body.edges()
        .filter(|(_, e)| {
            let (ka, kb) = (kind_at(e.he_plus), kind_at(e.he_minus));
            (ka, kb) == (Some(a), Some(b)) || (ka, kb) == (Some(b), Some(a))
        })
        .map(|(k, _)| k)
        .collect()
}

/// The lever arm every angular fillet predicate is metered against —
/// the maximum pairwise chord over the battery's own per-link sample
/// schedule (`sweep::fillet::battery::CHAIN_SAMPLES`). Findings
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
    let n = pncad::sweep::fillet::battery::CHAIN_SAMPLES;
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
    let [bulb, over, into] = bottle::<f64>(tol);

    // The elbows' volumes are the annulus times the spine length,
    // exactly (Pappus with the centroid ON the spine): a closed-form
    // oracle for a body whose walls are two tori.
    let ring = PI * ((R + WALL / 2.0).powi(2) - (R - WALL / 2.0).powi(2));
    for (name, body, sweep) in [("over", &over, SWEEP_OVER), ("into", &into, SWEEP_IN)] {
        let want = ring * sweep * RLOOP;
        let got = pncad::topo::mass_properties(body, tol)
            .expect("mass properties")
            .volume;
        assert!(
            (got - want).abs() < 1e-12,
            "elbow {name}: Pappus says {want}, the kernel says {got}"
        );
    }

    // Findings entry 11, executed: the same cylinder, said four ways.
    // Evan's constraint — the rim's hole is the tube's diameter —
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
    // The elbow↔elbow seam is the interesting one — the two annular
    // caps are the same annulus reached by rotating ∓270° and ∓90°
    // about the same axis, so the residual is trigonometry's, not the
    // construction's.
    let seam = |b: &Body<f64>, want_x: f64| -> Vec<Point3<f64>> {
        b.vertices()
            .map(|(_, v)| *b.get_point(v.point).expect("point"))
            .filter(|p| (p.x - want_x).abs() < 1e-9)
            .collect()
    };
    let (a, b) = (seam(&over, RLOOP), seam(&into, RLOOP));
    assert_eq!(
        (a.len(), b.len()),
        (4, 4),
        "each elbow ends on a 4-vertex annulus"
    );
    let residual = a
        .iter()
        .map(|p| {
            b.iter()
                .map(|q| (*p - *q).norm())
                .fold(f64::INFINITY, f64::min)
        })
        .fold(0.0, f64::max);
    assert!(
        residual < 1e-15,
        "the two elbows must meet inside ε; residual {residual:e}"
    );

    // The self-intersection is REAL and it is where a Klein bottle
    // has to have one: the descending elbow's spine starts outside
    // the bulb's flare and ends inside it. (The flare's spine radius
    // at height z is R + (ZNECK − z)·tan α.)
    let flare_at = |z: f64| R + (ZNECK - z) * ALPHA.tan();
    let spine_at = |psi: f64| (RLOOP + RLOOP * psi.cos(), m.z_tube + RLOOP * psi.sin());
    let (out_x, out_z) = spine_at(0.5 * PI);
    let (in_x, in_z) = spine_at(5.0 * PI / 6.0);
    assert!(
        out_x > flare_at(out_z) && in_x < flare_at(in_z),
        "the descending elbow must cross the flare: ({out_x}, {out_z}) vs \
         {}, ({in_x}, {in_z}) vs {}",
        flare_at(out_z),
        flare_at(in_z)
    );

    let story = "a Klein bottle — as a THIN 3-manifold, because a 2-manifold sheet is not a \
                 body this kernel holds. The bulb (neck, flare, the wide rim it turns back \
                 on, and the straight tube coming back up through that rim's hole) is ONE \
                 full revolve of ONE meridian band; the loop over the top is two thin \
                 elbows, partial revolves of the annulus. All three MEET on coincident \
                 annular faces and none of them can be joined: every boolean refuses a \
                 body carrying a cone or a torus. The neck passes through the flare \
                 uncut for the same reason";
    vec![Stop {
        name: "klein",
        caption: "Klein bottle — three bodies the kernel cannot join".to_string(),
        montage: true,
        story,
        ops: "revolve(Full) of a filleted meridian band + 2 x revolve(Partial) of an \
              annulus; NO boolean, NO fillet_edges, NO shell — see klein::wall_probes",
        delta: 1e-2,
        note: Some(format!(
            "tube diameter {:.2} m, wall {:.2} m; the wide rim's hole comes out at the \
             tube diameter by construction (centre radius R + RRIM = {:.3}, minor radii \
             {:.3}/{:.3}). Every blend in the bulb is an ARC IN THE MERIDIAN, exact and \
             free — and no rolling ball of that size fits the wall (walls 1-2). The elbows' \
             volumes are Pappus-exact: ring area {ring:.6} m^2 times spine length. The \
             two elbows meet with residual {residual:.1e} m and the loop meets the neck \
             bit-exactly — the numbers a declared REST contact would accept",
            2.0 * R,
            WALL,
            R + RRIM,
            RRIM + WALL / 2.0,
            RRIM - WALL / 2.0,
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
            SceneBody::plain("klein_loop_over", [0.78, 0.60, 0.30], over).transparent(45),
            // A colour of its own: this is the piece that runs INSIDE
            // the bulb, and seeing where it enters is the point.
            SceneBody::plain("klein_loop_into", [0.80, 0.34, 0.24], into).transparent(45),
        ],
    }]
}

/// The bottle's frontier, run live (the lily's rule): every shape
/// this model wanted and the kernel would not state, attempted for
/// real and pinned by its own typed refusal.
pub fn wall_probes<S: Scalar>(tol: Tol) {
    println!("\n-- the Klein bottle's walls: what a non-orientable surface asks for --");
    let m = meridian();
    let [bulb_body, over, into] = bottle::<S>(tol);
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
        fillet_edges(
            &sharp_full,
            &[full_edges[0]],
            band_radius,
            Band::linear(tol).expect("the run's band"),
            tol,
        ),
        |e| matches!(e, FilletError::RadiusHeadroom { .. }),
        "roll a ball as big as the blend the meridian draws for free",
    );
    crate::walls::wall(
        "bottle",
        2,
        "fillet the SAME corner on a partial revolve (open rim, honest lever)",
        fillet_edges(
            &sharp_part,
            &[part_edges[0]],
            band_radius,
            Band::linear(tol).expect("the run's band"),
            tol,
        ),
        // The SAME refusal as wall 1, and that is the pair's point: the
        // lever is honest on both rims, the dihedral decides on both,
        // and what stops both is the ball's own size against the neck
        // wall's curvature — not the closedness of the rim, and no
        // longer a missing arm.
        |e| matches!(e, FilletError::RadiusHeadroom { .. }),
        "roll a ball as big as the blend the meridian draws for free",
    );

    // Wall 3: the bottle is ONE surface. Its three bodies meet on
    // coincident annular faces — the declared REST mate — and the
    // union refuses before it ever looks at them.
    crate::walls::wall(
        "bottle",
        3,
        "join the loop to the bulb (coincident annular mate)",
        pncad::topo::union(&bulb_body, &over, tol),
        |e| {
            matches!(
                e,
                BooleanError::CurvedBooleanUnsupported {
                    operand: Operand::A,
                    kind: SurfaceKind::Torus,
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
        pncad::topo::subtract(&bulb_body, &into, tol),
        |e| matches!(e, BooleanError::CurvedOpUnsupported { .. }),
        "trim the self-intersection instead of letting the walls interpenetrate",
    );

    // Wall 5: the loop is one path. One sweep would be one body.
    let spine: Vec<Point3<f64>> = (0..=48)
        .map(|k| {
            let s = f64::from(k) / 48.0 * (SWEEP_OVER + SWEEP_IN);
            if s <= SWEEP_OVER {
                Point3::new(RLOOP * (1.0 - s.cos()), 0.0, ZTOP + RLOOP * s.sin())
            } else {
                let psi = 0.5 * PI + (SWEEP_OVER + SWEEP_IN - s);
                Point3::new(RLOOP + RLOOP * psi.cos(), 0.0, m.z_tube + RLOOP * psi.sin())
            }
        })
        .collect();
    let path =
        pncad::geom::NurbsCurve3::interpolate(&spine, 3).expect("the loop's spine interpolates");
    let annulus: Vec<ProfileLoop<f64>> = vec![
        circle(p2(0.0, 0.0), R + WALL / 2.0, tol)
            .expect("outer")
            .into(),
        circle(p2(0.0, 0.0), R - WALL / 2.0, tol)
            .expect("inner")
            .into(),
    ];
    crate::walls::wall(
        "bottle",
        5,
        "sweep the annulus along the loop's WHOLE spine, one body",
        pncad::sweep::sweep_body::<f64>(
            &annulus,
            Affine3::from_parts(
                Mat3::from_cols(v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(0.0, 0.0, 1.0)),
                v3(0.0, 0.0, ZTOP),
            ),
            &path,
            33,
            3,
            tol,
        ),
        |e| matches!(e, LoftError::ReversedStacking),
        "build the loop as ONE body and drop the two-elbow split",
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
    let ring_plane = SketchPlane::from_frame(
        p3::<S>(0.0, 0.0, 0.0),
        v3::<S>(1.0, 0.0, 0.0),
        v3::<S>(0.0, 0.0, 1.0),
    );
    let ring = validated(
        ring_plane,
        vec![
            circle(p2::<S>(RLOOP, 0.0), S::from_f64(R + WALL / 2.0), tol)
                .expect("outer")
                .into(),
            circle(p2::<S>(RLOOP, 0.0), S::from_f64(R - WALL / 2.0), tol)
                .expect("inner")
                .into(),
        ],
        tol,
    )
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
        SketchPlane::from_frame(
            p3::<f64>(0.0, 0.0, 0.0),
            v3::<f64>(1.0, 0.0, 0.0),
            v3::<f64>(0.0, 0.0, 1.0),
        ),
        vec![
            circle(p2::<f64>(RLOOP, 0.0), R + WALL / 2.0, tol)
                .expect("outer")
                .into(),
            circle(p2::<f64>(RLOOP, 0.0), R - WALL / 2.0, tol)
                .expect("inner")
                .into(),
        ],
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
         findings entry 7 (the O6 demo-gates list row), and retire the `ring` scene's \
         `step_at_frontier` declaration, which pins this same refusal on the rendered \
         ring — the two are one gate with two probes and retire together",
    );

    // Wall 7: a valid body the tessellator refuses. The ONLY change
    // from the bottle this scene ships is the rim's radius, 0.80 m →
    // 0.85 m; the face that refuses is the bulb's annular top rim,
    // whose own geometry does not depend on the rim radius at all.
    // f64: `mesh::tessellate` is the one door in this scene that is
    // not generic over the scalar — meshing is a rendering-side
    // operation and takes the run's own numbers.
    let wider = bulb::<f64>(
        band::<f64>(&meridian_at(ALPHA, RF, 0.85, RLOOP), tol),
        Revolution::Full,
        tol,
    );
    crate::walls::wall(
        "bottle",
        7,
        "tessellate the same bottle with a 5 cm wider bottom rim",
        pncad::mesh::tessellate(&wider, 1e-2, tol),
        |e| matches!(e, pncad::mesh::TessellateError::Triangulation { .. }),
        "check whether `mesh::planar`'s banked sub-floor case was CLOSED (the far \
         point's engineered exact-zero v-coordinate) — and if it was, say so in \
         findings entry 9 instead of deleting it",
    );

    let _ = into;
}
