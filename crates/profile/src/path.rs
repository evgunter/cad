//! The PATHS authoring algebra (LIB-U2 PR-1): typed profile-loop
//! authoring in which **accidental tangency is unrepresentable,
//! intended tangency is exact by construction, and every authored
//! point lies on the final path, authored once** — the ratified design
//! of `docs/PATHS-DESIGN.md` §§1–7, lowered to the existing v1 form
//! ([`ProfileLoop`]: segments + declared tangency flags).
//!
//! # The binding lattice
//!
//! A [`PartialPath<T, P, A>`]'s tip typestate is exactly which of
//! {position, angle} it has bound (PATHS-DESIGN §2):
//!
//! - **Open** = `PartialPath<T, NoPos, NoAng>` — every fillet's freshly
//!   opened arrival side (the *entry* Open is the [`Open`] token, whose
//!   binders mint the path).
//! - **Point** = `PartialPath<T, HasPos<F>, NoAng>` — two flavors in
//!   the position marker: [`Plain`] (no incoming carrier: `Open.at(p)`,
//!   a fillet arrival's `.at(p)`) and [`WithIncoming`] (a leg end —
//!   position plus the leg's incoming end tangent as read-only
//!   intrinsic data).
//! - **Angle** = `PartialPath<T, NoPos, HasAng>` — a fillet arrival
//!   bound angle-first (or the entry after `Open.angle(θ)`).
//! - **Directed** = `PartialPath<T, HasPos<F>, HasAng>` — the only
//!   state legs and [`fillet`](PartialPath::fillet) consume.
//!
//! The OUTGOING angle is a binding slot, set at most once per side
//! (a second director on a Directed tip is ill-typed); the INCOMING
//! direction is never a slot — it is intrinsic data on a leg end,
//! consultable by [`tangent`](PartialPath::tangent) /
//! [`turn`](PartialPath::turn) and the junction check, settable by
//! nothing.
//!
//! # Closure
//!
//! [`Start`] is a first-class directed-point value — the bound entry.
//! Using it is closing, structurally: `line_to(Start)`,
//! `arc_to(Start, b)`, `.tangent().tangent_arc_to(Start)`, and the
//! seam fillet `.angle(θ).fillet(r).to(Start)`. There is deliberately
//! no `close()` alias. The entry authors the first side; the seam is
//! authored once, at the back, by the verb that targets `Start` — a
//! leading `.fillet`/`.tangent()` is ill-typed (they need bits the
//! entry Open lacks).
//!
//! # Lowering
//!
//! Elaboration is strictly forward, single pass, seam last
//! (PATHS-DESIGN §5): each verb resolves its geometry from its own
//! arguments plus already-bound state, in closed form (no iteration
//! anywhere), and emits [`ProfileLoop`] vertices exactly as the
//! equivalent [`crate::LoopBuilder`] calls would — the line×line
//! fillet trim geometry is literally shared code
//! (`sugar::line_line_fillet_trims`), so the two doors emit
//! bit-identical fillet geometry. The #101 verify layer
//! ([`crate::Profile::validate`]) runs UNCHANGED on the lowered
//! output: every declared flag is re-verified at build —
//! verified-never-trusted; nothing is trusted because the algebra
//! produced it.
//!
//! One canonicalization is worth naming (it defines which coordinates
//! the authoring determines *exactly*, the differential tests' bit
//! contract): a fillet's trim geometry is computed against the two
//! sides' **anchors** (the incoming ray's origin and the arrival
//! side's anchor), so `line_line_fillet_trims(origin, corner, anchor,
//! r)` — matching a hand author's `LoopBuilder::fillet(corner, anchor,
//! r)` bit-for-bit whenever the ray origin is the chain head, which is
//! every case except a side squeezed between two fillets (where the
//! head is the previous trim point, mathematically on the same
//! carrier; exact inputs still agree exactly).
//!
//! # Refusals
//!
//! Compile-time, from the lattice: double director; legs/`fillet` from
//! non-Directed tips; `.tangent()` on a plain point; leading
//! `.fillet`/`.tangent()`; use after close (closing verbs consume the
//! path and return the loop). Typed runtime errors, from geometry —
//! the lattice guarantees the authoring, never the geometry: see
//! [`PathError`]. Never a panic (evaluation code is total; every
//! decision goes through the reified-predicate funnel).
//!
//! The run-global [`Tolerance`] (`Tolerance::get()`, D4 ¶1's one ε per
//! run) supplies ε_input for every junction classification — the
//! ratified surface has no per-call tolerance slot.
//!
//! # Vocabulary growth (LIB-G1)
//!
//! Five constructors added under the PROFILES-V2 VQ1(b) ruling — the
//! algebra grows until the persisted corpus authors fully — and
//! documented as the §2 addendum of `docs/PATHS-DESIGN.md`:
//!
//! - [`circle`] — the closed-carrier PROGRAM FORM. Not a chain: it
//!   authors no seam, so PQ4 is untouched, and the conventional split
//!   is its private lowering.
//! - [`arc_via`](PartialPath::arc_via) — the arc through a point.
//! - [`arc_center`](PartialPath::arc_center) — the arc about a centre,
//!   with a structural winding; equidistance checked, never repaired.
//! - `to(anchor)` on a bound arrival direction — the **far-end
//!   anchor**: the arrival side ends AT its authored anchor, with no
//!   synthetic mid-side point and no measured length.
//! - [`toward`](PartialPath::toward) — the direction-valued director:
//!   axis-aligned rays are exact, where `.angle(θ)` round-trips through
//!   `sin_cos`.
//!
//! Two exactness contracts hold across all five. **Authored points are
//! stored verbatim**: every point the author types is emitted as itself,
//! and every derived quantity (bulges, rays, corners, trims) is computed
//! at lowering — nothing computed is ever re-typed by the author, so the
//! algebra and a hand chain fed the same authored points agree bit for
//! bit. **Direction-exact rays**: a director spelled as components fixes
//! the ray, not an angle, so no trig round-trip stands between the
//! authoring and the geometry.
//!
//! # Not in this lowering (v1 scope)
//!
//! NURBS legs (`nurbs_in_place`, `nurbs(curve)` and variants,
//! `FilletCarrierUnsupported`) are specified by PATHS-DESIGN §2 but
//! have **no representation in the v1 lowering target** (a
//! [`ProfileLoop`] is a vertex+bulge chain; this crate deliberately
//! depends on `geom-core` only) — they arrive with the v2
//! profiles-as-programs representation (#104). Arc-arrival fillets are
//! explicitly out of scope (§7), which with straight-only fillet
//! carriers makes every v1 algebra fillet line×line. Mixed authoring
//! is OUT (§6): a loop is authored either here or as a raw chain,
//! never both; there is no path-concatenation operator — repeated
//! motifs are builder functions over the one chain.
//!
//! # Example: the all-rounded square (4 anchors + 4 directions)
//!
//! Every anchor mᵢ is a real on-path point (a side midpoint); the
//! corners are never authored — they exist only as carrier
//! intersections, and the seam fillet reads exactly like the interior
//! ones:
//!
//! ```
//! use geom_core::{Point2, Tolerance};
//! use profile::{Open, Profile, SketchPlane, Start};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let (east, north) = (0.0_f64, std::f64::consts::FRAC_PI_2);
//! let (west, south) = (std::f64::consts::PI, -north);
//! let r = 0.25;
//! let square = Open.at(Point2::new(0.0, -1.0)).angle(east)?
//!     .fillet(r)?.at(Point2::new(1.0, 0.0))?.angle(north)?
//!     .fillet(r)?.at(Point2::new(0.0, 1.0))?.angle(west)?
//!     .fillet(r)?.at(Point2::new(-1.0, 0.0))?.angle(south)?
//!     .fillet(r)?.to(Start)?;
//! assert_eq!(square.vertices.len(), 8);
//! assert_eq!(square.tangent_joints.len(), 8);
//! Profile::new(SketchPlane::xy(), vec![square]).validate(Tolerance::get())?;
//! # Ok(())
//! # }
//! ```
//!
//! # Off-lattice states are unreachable (compile-fail gallery)
//!
//! A second director on a Directed tip is ill-typed:
//!
//! ```compile_fail,E0599
//! use geom_core::Point2;
//! use profile::Open;
//! let p = Open.at(Point2::new(0.0, 0.0)).angle(0.0).unwrap().angle(1.0);
//! ```
//!
//! `.tangent()` on a plain point (no incoming direction to inherit):
//!
//! ```compile_fail,E0599
//! use geom_core::Point2;
//! use profile::Open;
//! let p = Open.at(Point2::new(0.0, 0.0)).tangent();
//! ```
//!
//! A leading `.fillet` / `.tangent()` (the seam's content cannot be
//! authored from the front):
//!
//! ```compile_fail,E0599
//! use profile::Open;
//! let p = Open.fillet(0.5);
//! ```
//!
//! Legs cannot depart a half-bound tip:
//!
//! ```compile_fail,E0599
//! use profile::Open;
//! let p = Open.angle(0.0_f64).line(1.0);
//! ```
//!
//! The widened director surface is the SAME slot, so `.toward` is a
//! second director on a Directed tip exactly as `.angle` is:
//!
//! ```compile_fail,E0599
//! use geom_core::Point2;
//! use profile::Open;
//! let p = Open.at(Point2::new(0.0, 0.0)).angle(0.0).unwrap().toward(1.0, 0.0);
//! ```
//!
//! ```compile_fail,E0599
//! use geom_core::Point2;
//! use profile::Open;
//! let p = Open.at(Point2::new(0.0_f64, 0.0)).toward(1.0, 0.0).unwrap().toward(0.0, 1.0);
//! ```
//!
//! The new arc modes are legs from a Point, so they are ill-typed on a
//! Directed tip (the departure is already bound):
//!
//! ```compile_fail,E0599
//! use geom_core::Point2;
//! use profile::Open;
//! let p = Open.at(Point2::new(0.0, 0.0)).angle(0.0).unwrap()
//!     .arc_via(Point2::new(1.0, 1.0), Point2::new(2.0, 0.0));
//! ```
//!
//! ```compile_fail,E0599
//! use geom_core::Point2;
//! use profile::{ArcSweep, Open};
//! let p = Open.at(Point2::new(0.0, 0.0)).angle(0.0).unwrap()
//!     .arc_center(Point2::new(1.0, 0.0), Point2::new(2.0, 0.0), ArcSweep::Ccw);
//! ```
//!
//! `circle` is a complete-loop PROGRAM FORM, not a chain: there is no
//! tip to continue from, so no chain verb exists on its result:
//!
//! ```compile_fail,E0599
//! use geom_core::Point2;
//! let loop_ = profile::circle(Point2::new(0.0, 0.0), 1.0).unwrap();
//! let more = loop_.line_to(Point2::new(1.0, 0.0));
//! ```
//!
//! The far-end anchor is an ARRIVAL-side form: it needs the position
//! slot empty and the angle slot bound, so it is ill-typed on a
//! Directed tip (position already bound) …
//!
//! ```compile_fail,E0308
//! use geom_core::Point2;
//! use profile::Open;
//! let p = Open.at(Point2::new(0.0, 0.0)).angle(0.0).unwrap()
//!     .to(Point2::new(1.0, 0.0));
//! ```
//!
//! … and its `Start` spelling is deliberately absent (the seam fillet
//! `.fillet(r).to(Start)` is the closing form, from the unbound Open):
//!
//! ```compile_fail,E0277
//! use geom_core::Point2;
//! use profile::{Open, Start};
//! let p = Open.at(Point2::new(0.0, 0.0)).angle(0.0).unwrap()
//!     .fillet(0.5).unwrap().angle(1.0).unwrap().to(Start);
//! ```
//!
//! Use after close (closing verbs consume the path):
//!
//! ```compile_fail,E0382
//! use geom_core::Point2;
//! use profile::{Open, Start};
//! let path = Open
//!     .at(Point2::new(0.0, 0.0))
//!     .line_to(Point2::new(1.0, 0.0))
//!     .unwrap()
//!     .line_to(Point2::new(0.5, 1.0))
//!     .unwrap();
//! let done = path.line_to(Start);
//! let again = path.line_to(Start);
//! ```

use core::marker::PhantomData;

use geom_core::{Band, Decide, Indeterminate, Length, Point2, Real, Sign, Tolerance, Vec2};

use crate::k_stats::decide;
use crate::sugar::{
    ArcSweep, LineFilletTrims, TrimRefusal, bulge_from_center, bulge_from_via,
    line_line_fillet_trims,
};
use crate::validate::FilletLeg;
use crate::{ProfileLoop, ProfileVertex};

// ------------------------------------------------------------------
// Lattice markers (PATHS-DESIGN §5: one struct under type-level
// markers; the position marker carries the plain-vs-directed flavor).
// ------------------------------------------------------------------

mod sealed {
    /// Seals the lattice-marker and closure-target traits: the four
    /// states and the two target kinds are the whole lattice — foreign
    /// impls would mint off-lattice states.
    pub trait Sealed {}
}

/// Position slot empty (the Open and Angle states).
#[derive(Clone, Copy, Debug)]
pub struct NoPos;

/// Position slot bound, with flavor `F` ([`Plain`] or
/// [`WithIncoming`]) — the Point and Directed states.
#[derive(Clone, Copy, Debug)]
pub struct HasPos<F>(PhantomData<F>);

/// A plain point: position only, no incoming carrier (the entry
/// `Open.at(p)`, a fillet arrival's `.at(p)`).
#[derive(Clone, Copy, Debug)]
pub struct Plain;

/// A directed point: a leg end — position plus the leg's incoming end
/// tangent, carried as read-only intrinsic data. The only state
/// [`PartialPath::tangent`] / [`PartialPath::turn`] exist on.
#[derive(Clone, Copy, Debug)]
pub struct WithIncoming;

/// Angle slot empty.
#[derive(Clone, Copy, Debug)]
pub struct NoAng;

/// Angle slot bound (the outgoing departure direction).
#[derive(Clone, Copy, Debug)]
pub struct HasAng;

/// The position-marker family: [`NoPos`] or [`HasPos<F>`]. Sealed.
pub trait PosMarker: sealed::Sealed {}
/// The angle-marker family: [`NoAng`] or [`HasAng`]. Sealed.
pub trait AngMarker: sealed::Sealed {}
/// The point-flavor family: [`Plain`] or [`WithIncoming`]. Sealed.
pub trait Flavor: sealed::Sealed {}

impl sealed::Sealed for NoPos {}
impl<F: Flavor> sealed::Sealed for HasPos<F> {}
impl sealed::Sealed for Plain {}
impl sealed::Sealed for WithIncoming {}
impl sealed::Sealed for NoAng {}
impl sealed::Sealed for HasAng {}

impl PosMarker for NoPos {}
impl<F: Flavor> PosMarker for HasPos<F> {}
impl Flavor for Plain {}
impl Flavor for WithIncoming {}
impl AngMarker for NoAng {}
impl AngMarker for HasAng {}

// ------------------------------------------------------------------
// Tokens.
// ------------------------------------------------------------------

/// The entry token: `Open.at(p)` / `Open.angle(θ)` author the first
/// side (either binder order). The entry's own junction check happens
/// at the seam, when a verb targets [`Start`].
///
/// A leading `.fillet`/`.tangent()` is ill-typed here — the seam's
/// content cannot be authored from the front (PATHS-DESIGN §2's entry
/// rule): this token has no such methods, and the fillet-arrival Open
/// state ([`PartialPath<T, NoPos, NoAng>`]) has none either.
///
/// `.to(dp)` at the entry is omitted in this lowering: its only
/// arguments are curve-pose values (`c.start()`/`c.end()`), which
/// arrive with NURBS legs in v2 (see the module docs).
#[derive(Clone, Copy, Debug)]
pub struct Open;

/// The closure token: a first-class directed-point VALUE — the bound
/// entry (both bits by the time the loop returns), legal wherever a
/// directed-point/position argument goes. **Using it is closing,
/// structurally**: the endpoint IS the start point by reference,
/// authored once; closure never depends on re-typed coordinates
/// value-matching.
#[derive(Clone, Copy, Debug)]
pub struct Start;

impl sealed::Sealed for Start {}

// ------------------------------------------------------------------
// Typed refusals.
// ------------------------------------------------------------------

/// Why a fillet's virtual corner does not exist (PATHS-DESIGN §2's
/// DOF check: parallel/non-intersecting carriers, or an intersection
/// behind the ray start, refuse typed). Named `Path…` to keep it
/// distinct from the crate-root [`crate::NoCornerReason`] (the verify
/// layer's fillet-constructor vocabulary) — different doors, different
/// conditions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathNoCornerReason {
    /// The incoming ray and the arrival carrier are parallel at
    /// tolerance (which includes the tangent/anti-tangent corner —
    /// there is no corner to cut; PATHS routes the declaration through
    /// `.tangent()`/the seam fillet instead).
    CarriersParallel,
    /// The carrier intersection lies behind the incoming ray's start
    /// (or at it, at tolerance): the corner is not ahead of the side
    /// being authored.
    BehindIncomingRay,
    /// The carrier intersection does not lie behind the arrival side's
    /// anchor: the corner sits on the wrong side of (or at) the
    /// anchor, so the arrival ray never came from it.
    BehindArrivalAnchor,
}

/// Typed refusals of the authoring algebra — geometry the lattice
/// cannot rule out, refused loudly (PATHS-DESIGN §3 "Refusals" and §4).
/// The verify layer's own errors ([`ProfileError`]) still apply to the
/// lowered loop at [`crate::Profile::validate`], unchanged.
#[derive(Clone, Debug)]
pub enum PathError<T: Real> {
    /// §4 item 1: the authored departure is within ε_input of the
    /// incoming TANGENT direction — one refusal, one recourse, for any
    /// sub-ε_input margin: if the tangency is intended, author it
    /// structurally (`.tangent()`, or the tangent-arc / seam-fillet
    /// close at the seam), which makes it exact by construction;
    /// otherwise move the geometry (or lower the tolerance). The
    /// margin rides along as data; the message never forks on
    /// exactly-on vs in-band.
    JunctionTangent {
        /// The classified turn margin sin φ · arm, meters (scalar-typed
        /// payload — data, not a decision).
        margin: T,
        /// The lever arm (the incoming leg's extent, capped by its
        /// carrier radius), meters.
        arm: T,
    },
    /// §4 item 1, reverse class: the departure is within ε_input of
    /// the REVERSE of the incoming tangent — a cusp. No declaration
    /// door exists: the kernel's material-wedge invariant refuses cusp
    /// wedges in any solid built from such a profile; #131 is the
    /// tabled front door that does not exist yet.
    JunctionCusp {
        /// The classified turn margin sin φ · arm, meters.
        margin: T,
        /// The lever arm, meters.
        arm: T,
    },
    /// The overdetermined tangent LINE close (PATHS-DESIGN §2,
    /// closure): direction inherited AND through `Start` — refused
    /// ALWAYS, exact collinearity included (a ray hitting an
    /// independently-authored point is a value coincidence, and the
    /// ratified ladder never infers from values). The two structural
    /// spellings: close with the tangent ARC instead
    /// (`.tangent().tangent_arc_to(Start)`), or rotate the loop's
    /// authoring origin so the straight run is authored forward as
    /// side 1 and the arc becomes the closer.
    TangentLineClose {
        /// The offending collinearity/turn margin, meters.
        margin: T,
    },
    /// §4 item 4: the constructed junction joins two segments of the
    /// SAME carrier (collinear line onto line, cocircular arc onto
    /// arc) under a tangency declaration — carrier identity, not
    /// tangency; refused exactly as #101's `same_carrier` rule. (The
    /// post-fillet continuation is exempt by construction: it extends
    /// one leg rather than minting a collinear neighbor.)
    SameCarrierJunction {
        /// The classified identity margin (center distance + radius
        /// difference for circles; perpendicular offset for lines),
        /// meters.
        margin: T,
    },
    /// The fillet's virtual corner does not exist (see
    /// [`PathNoCornerReason`]).
    NoCornerForFillet {
        /// Which structural condition failed.
        reason: PathNoCornerReason,
        /// The requested radius, meters (diagnostic).
        radius: T,
    },
    /// A fillet trim would eat a side's anchoring on-path point (the
    /// authored anchor, the incoming ray's origin, or — under a seam
    /// fillet — the entry point): the #101 `TangentJointOutOfRange`
    /// fit-gating generalized (PATHS-DESIGN §3). This is also where a
    /// too-large radius lands: the setback exceeds the extent the
    /// anchor pins.
    AnchorOutsideTrimmedExtent {
        /// Which side's anchor the trim would eat.
        side: FilletLeg,
        /// The tangent setback from the corner, meters (diagnostic).
        setback: T,
        /// The anchored extent available to the trim, meters.
        available: T,
    },
    /// A seam fillet would land on a first side that is not a straight
    /// carrier (the first authored segment is an arc): arc-arrival
    /// fillets are explicitly out of scope in v1 (PATHS-DESIGN §7,
    /// "additive, with a use case").
    SeamFilletOntoArc,
    /// A fillet arrival cannot be bound with an arc leg (`arc_to` on
    /// an arrival point): arc-arrival fillets are out of scope in v1
    /// (PATHS-DESIGN §7).
    ArcArrivalFillet,
    /// A leg length that is not definitely positive: a negative length
    /// would run the side BACKWARD, detaching the tip's anchored
    /// on-path points from the final path (§4 item 3's invariant,
    /// broken silently — the verify layer cannot see intent); a
    /// sub-ε_input length is a degenerate segment. Classified through
    /// the funnel (`path_leg_length`).
    NonpositiveLeg {
        /// The refused length, meters (scalar-typed payload).
        length: T,
    },
    /// A fillet radius that is not definitely positive: r = 0
    /// degenerates the arc and a negative r mirrors the tangent points
    /// past the corner — either way the declared-tangent construction
    /// the fillet promises does not exist. Classified through the
    /// funnel (`path_fillet_radius`) at `.fillet(r)` itself, before an
    /// arrival can be authored against it.
    NonpositiveFilletRadius {
        /// The refused radius, meters (scalar-typed payload).
        radius: T,
    },
    /// A [`circle`] radius that is not definitely positive: r = 0 is a
    /// point and r < 0 names no circle. Classified through the funnel
    /// (`path_circle_radius`), consistent with the other sign gates.
    NonpositiveCircleRadius {
        /// The refused radius, meters (scalar-typed payload).
        radius: T,
    },
    /// A director spelled as components named no direction: the norm of
    /// `(dx, dy)` is within ε_input of zero
    /// ([`PartialPath::toward`]). Only the components' ratio is read,
    /// so the recourse is free — scale them up.
    ZeroDirection {
        /// The refused x component.
        dx: T,
        /// The refused y component.
        dy: T,
    },
    /// An `arc_via` through-point is within ε_input of the CHORD LINE:
    /// the three points name no arc. On the chord the construction
    /// degenerates to the straight segment; off the far end it
    /// degenerates to the same line traversed as a ±π turn (an
    /// astronomically large carrier). One refusal for the whole
    /// collinear class — the recourse is to move the through-point off
    /// the chord, or to author the straight segment as a line.
    ArcViaCollinear {
        /// The through-point's signed perpendicular offset from the
        /// chord line, meters.
        offset: T,
    },
    /// An arc leg's endpoints are within ε_input of each other: the
    /// chord is degenerate, so neither the via nor the centre form
    /// determines an arc (a full turn is a closed carrier, which is
    /// [`circle`]'s business, not a chain leg's — PQ4).
    DegenerateArcChord {
        /// The chord length, meters.
        chord: T,
    },
    /// An `arc_center` centre is not equidistant from the two
    /// endpoints: the authored data contradicts itself. Refused, never
    /// repaired — silently re-projecting the centre (or the endpoint)
    /// onto a fitted circle would move an AUTHORED point, which §4
    /// item 3 forbids. The recourse is to fix whichever of the three
    /// authored points is wrong.
    ArcCenterNotEquidistant {
        /// |tip − centre|, meters.
        tip_radius: T,
        /// |end − centre|, meters.
        end_radius: T,
    },
    /// An `arc_center` centre is within ε_input of an endpoint: the
    /// carrier has no radius, so the winding selects nothing.
    DegenerateArcCenter {
        /// The classified radius, meters.
        radius: T,
    },
    /// The far-end-anchor form (`.angle(θ).to(p)` / `.toward(..).to(p)`)
    /// was reached with no opened fillet — at the ENTRY, where the
    /// direction is bound but no side is waiting to be terminated. The
    /// form ends an ARRIVAL side at its own anchor; the entry authors
    /// the first side with `.at(p)` and the seam is authored at the
    /// back (PATHS-DESIGN §2's entry rule).
    FarEndAnchorWithoutFillet,
    /// A junction/corner classification could not be decided at this
    /// scalar (in-band margin or poisoned input) — the reified
    /// predicate escalation, typed (never a guess).
    Escalated {
        /// The escalation, naming the predicate.
        source: Indeterminate,
    },
    /// The run tolerance could not form a classification band.
    Band(geom_core::BandError),
    /// Elaborator backstop (PATHS-DESIGN §5): a leg reached emission
    /// without the bindings the surface guarantees. Expected
    /// unreachable from the typed surface; reaching it is a design
    /// finding, not a silent fix.
    UnderdeterminedLeg {
        /// The elaboration site.
        site: &'static str,
    },
    /// Elaborator backstop (PATHS-DESIGN §5): a junction resolution
    /// received constraints it cannot have (e.g. the shared fillet
    /// closed form refused in a shape the algebra's own gates should
    /// have caught first). Expected unreachable from the typed
    /// surface; reaching it is a design finding.
    OverdeterminedJunction {
        /// The elaboration site.
        site: &'static str,
    },
}

impl<T: Real> core::fmt::Display for PathError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::JunctionTangent { margin, arm } => write!(
                f,
                "this junction is tangent at any precision you could care about \
                 (turn margin {margin:?} m on a {arm:?} m arm) — if intended, author it \
                 structurally: .tangent() at an interior junction (exact by construction), or \
                 the tangent-arc / seam-fillet close at the seam; otherwise move the geometry \
                 (or lower the tolerance)"
            ),
            Self::JunctionCusp { margin, arm } => write!(
                f,
                "this junction reverses onto the incoming direction (turn margin {margin:?} m \
                 on a {arm:?} m arm): a cusp, which the material-wedge invariant refuses in \
                 any solid built from such a profile — there is no declaration door for cusps \
                 (#131 is the tabled front door that does not exist yet); move the geometry"
            ),
            Self::TangentLineClose { margin } => write!(
                f,
                "a tangent LINE close is overdetermined — direction inherited AND through Start \
                 (margin {margin:?} m) — and refuses always, exact collinearity included: \
                 close with the tangent ARC instead (.tangent().tangent_arc_to(Start)), or \
                 rotate the loop's authoring origin so the straight run is authored forward as \
                 side 1 and the arc becomes the closer"
            ),
            Self::SameCarrierJunction { margin } => write!(
                f,
                "this junction joins two pieces of the SAME carrier (identity margin \
                 {margin:?} m): carrier identity is not tangency (#101) — extend the leg \
                 instead of minting a collinear/cocircular neighbor"
            ),
            Self::NoCornerForFillet { reason, radius } => {
                let what = match reason {
                    PathNoCornerReason::CarriersParallel => {
                        "the incoming ray and the arrival carrier are parallel at tolerance — \
                         no corner exists (if they are meant to run tangentially, author the \
                         tangency: .tangent(), or the seam fillet at the seam)"
                    }
                    PathNoCornerReason::BehindIncomingRay => {
                        "the carrier intersection lies behind the incoming ray's start"
                    }
                    PathNoCornerReason::BehindArrivalAnchor => {
                        "the carrier intersection does not lie behind the arrival side's anchor"
                    }
                };
                write!(f, "no corner for a radius-{radius:?} m fillet: {what}")
            }
            Self::AnchorOutsideTrimmedExtent {
                side,
                setback,
                available,
            } => write!(
                f,
                "the fillet trim would eat the {side} side's anchoring on-path point: tangent \
                 setback {setback:?} m exceeds the {available:?} m the anchor pins — reduce \
                 the radius or move the anchor"
            ),
            Self::SeamFilletOntoArc => write!(
                f,
                "a seam fillet would land on a first side that is not straight: arc-arrival \
                 fillets are out of scope in v1 (PATHS-DESIGN §7) — author the arc side \
                 elsewhere in the cycle so the seam falls on a straight side"
            ),
            Self::ArcArrivalFillet => write!(
                f,
                "a fillet arrival cannot be an arc side in v1 (PATHS-DESIGN §7): bind the \
                 arrival with a direction (.angle) or a line (line_to), not arc_to"
            ),
            Self::NonpositiveLeg { length } => write!(
                f,
                "a leg must advance the tip by a definitely positive length (got {length:?} m): \
                 a negative length runs the side backward and detaches anchored points from \
                 the final path (every authored point lies on the final path, authored once); \
                 a sub-tolerance length is a degenerate segment"
            ),
            Self::NonpositiveFilletRadius { radius } => write!(
                f,
                "a fillet needs a definitely positive radius (got {radius:?} m): r = 0 \
                 degenerates the arc and a negative r mirrors the tangent points past the \
                 corner — no tangent construction exists to declare"
            ),
            Self::NonpositiveCircleRadius { radius } => write!(
                f,
                "a circle needs a definitely positive radius (got {radius:?} m): r = 0 is a \
                 point and r < 0 names no circle"
            ),
            Self::ZeroDirection { dx, dy } => write!(
                f,
                "a director spelled as components must name a direction (got \
                 ({dx:?}, {dy:?}), whose norm is within tolerance of zero): only the ratio \
                 of the components is read, so scaling them up costs nothing"
            ),
            Self::ArcViaCollinear { offset } => write!(
                f,
                "the through-point lies on the chord line (offset {offset:?} m, within \
                 tolerance of zero): three collinear points name no arc — move the \
                 through-point off the chord, or author the straight segment as a line"
            ),
            Self::DegenerateArcChord { chord } => write!(
                f,
                "an arc leg's endpoints are within tolerance of each other (chord {chord:?} \
                 m): a leg spans a chord, and a closed carrier is a circle primitive, not a \
                 chain leg (PATHS-DESIGN §6, PQ4)"
            ),
            Self::ArcCenterNotEquidistant {
                tip_radius,
                end_radius,
            } => write!(
                f,
                "the authored centre is not equidistant from the arc's endpoints \
                 (|tip - centre| = {tip_radius:?} m, |end - centre| = {end_radius:?} m): the \
                 three authored points contradict each other. Nothing is re-projected — an \
                 authored point is never moved to make a construction work; fix whichever \
                 of the three is wrong"
            ),
            Self::DegenerateArcCenter { radius } => write!(
                f,
                "the authored centre is within tolerance of an endpoint (radius {radius:?} \
                 m): the carrier has no radius, so the winding selects nothing"
            ),
            Self::FarEndAnchorWithoutFillet => write!(
                f,
                "the far-end-anchor form ends an ARRIVAL side at its own anchor, and no \
                 fillet is open here: the entry authors its first side with .at(p), and the \
                 seam is authored at the back by the verb that targets Start \
                 (PATHS-DESIGN §2's entry rule)"
            ),
            Self::Escalated { source } => write!(f, "path junction classification: {source}"),
            Self::Band(e) => write!(f, "path tolerance band: {e}"),
            Self::UnderdeterminedLeg { site } => write!(
                f,
                "elaborator backstop UnderdeterminedLeg at {site}: expected unreachable from \
                 the typed surface — a reachable case is a design finding (PATHS-DESIGN §5)"
            ),
            Self::OverdeterminedJunction { site } => write!(
                f,
                "elaborator backstop OverdeterminedJunction at {site}: expected unreachable \
                 from the typed surface — a reachable case is a design finding (PATHS-DESIGN §5)"
            ),
        }
    }
}

impl<T: Real> std::error::Error for PathError<T> {}

// ------------------------------------------------------------------
// Internal state. Fields are private throughout: binders are the only
// constructors, so off-lattice states are representable at runtime but
// unreachable through the surface (PATHS-DESIGN §5).
// ------------------------------------------------------------------

/// An emitted arc segment's carrier (center + radius), kept for the
/// §4 item 4 carrier-identity refusals at zero-fit knife edges.
#[derive(Clone, Copy, Debug)]
struct ArcData<T: Real> {
    center: Point2<T>,
    radius: T,
}

/// A tangent arc leg's derived geometry (the fields
/// `tangent_arc_geom` resolves in one pass, named rather than
/// returned as a wide tuple).
#[derive(Clone, Copy, Debug)]
struct TangentArcGeom<T: Real> {
    /// The arc's bulge, tan(Δ/2).
    bulge: T,
    /// The arc's end tangent (departure + 2Δ).
    end_ang: Dir<T>,
    /// The arc's carrier circle.
    carrier: ArcData<T>,
    /// The chord length, meters (the junction check's lever cap).
    chord: T,
}

/// A bound direction: the angle in radians **and** the unit vector the
/// rays are actually built from.
///
/// The two are carried together because they are not interchangeable at
/// the bit level (the G1 VQ4 exactness contract). A director spelled as
/// an ANGLE fixes θ and derives the ray by `sin_cos`, so an axis
/// direction picks up the quantization of π (`unit(PI).y = 1.22e-16`);
/// a director spelled as COMPONENTS ([`PartialPath::toward`]) fixes the
/// ray exactly — `(-1, 0)` normalizes to itself — and derives θ by
/// `atan2` only for the angle arithmetic (`.turn(δ)`, arc end tangents)
/// that genuinely needs a number. Every ray construction reads
/// [`Dir::unit`]; nothing rebuilds a ray from [`Dir::ang`], so
/// exactness survives every hop it can.
#[derive(Clone, Copy, Debug)]
struct Dir<T: Real> {
    /// The direction's angle, radians.
    ang: T,
    /// The unit ray — exact when the director authored components.
    unit: Vec2<T>,
}

impl<T: Real> Dir<T> {
    /// A director spelled as an angle: the ray is the `sin_cos`
    /// round-trip (the historic `.angle(θ)` behaviour, bit-for-bit).
    fn from_angle(ang: T) -> Self {
        Self {
            ang,
            unit: unit(ang),
        }
    }

    /// A director spelled as an already-unit ray: the ray is stored
    /// verbatim and the angle derived from it.
    fn from_unit(u: Vec2<T>) -> Self {
        Self {
            ang: u.y.atan2(u.x),
            unit: u,
        }
    }
}

/// A directed point's intrinsic incoming data: the arriving leg's end
/// tangent, the leg's lever arm (extent capped by its carrier radius —
/// the junction check's meters lever), and its carrier kind.
#[derive(Clone, Copy, Debug)]
struct Incoming<T: Real> {
    ang: Dir<T>,
    arm: T,
    carrier: Option<ArcData<T>>,
}

/// The one-struct tip of PATHS-DESIGN §5: `pos: Option<PosData>`,
/// `ang: Option<…>`; the position data's optional incoming tangent is
/// what the junction check reads at runtime (one generic function).
/// G1 constructor 5 widens the angle slot's PAYLOAD to a [`Dir`]
/// (angle-or-direction — one slot, two spellings); the §5 shape is
/// unchanged: still one struct, still exactly two optional bits.
#[derive(Clone, Debug)]
struct Tip<T: Real> {
    pos: Option<PosData<T>>,
    ang: Option<Dir<T>>,
    /// Whether the bound angle was inherited by `.tangent()` (the
    /// declared-tangency continuations; drives the §4 item 4 checks).
    ang_by_tangent: bool,
}

/// The bound position bit: the point plus (directed flavor only) the
/// incoming intrinsic data.
#[derive(Clone, Debug)]
struct PosData<T: Real> {
    at: Point2<T>,
    incoming: Option<Incoming<T>>,
}

/// What kind of segment leaves the entry vertex — pinned at first
/// emission so the seam knows side 1's carrier kind structurally
/// (never by comparing a bulge to zero).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FirstSeg {
    NotYet,
    Line,
    Arc,
}

/// An opened fillet awaiting its arrival side (the `Open` state's
/// runtime content): the consumed departure ray, the radius, and the
/// ray-origin junction's book-keeping for the zero-fit knife edges.
#[derive(Clone, Debug)]
struct PendingFillet<T: Real> {
    origin: Point2<T>,
    ang: Dir<T>,
    radius: T,
    /// The ray was bound by `.tangent()` (its origin joint is already
    /// declared).
    by_tangent: bool,
    /// The ray origin's incoming carrier, if the origin was a leg end.
    origin_incoming: Option<Incoming<T>>,
}

/// The accumulated lowering state: the vertex chain emitted so far
/// (mirroring [`crate::LoopBuilder`]'s emission verb-for-verb), the
/// declared joints, the entry pose (the [`Start`] value), and the
/// pending fillet, if a side is open.
#[derive(Clone, Debug)]
struct Core<T: Real> {
    verts: Vec<ProfileVertex<T>>,
    tangent: Vec<usize>,
    start_pos: Option<Point2<T>>,
    start_ang: Option<Dir<T>>,
    first_seg: FirstSeg,
    pending: Option<PendingFillet<T>>,
    /// The carrier of the last emitted segment when it is an arc.
    last_arc: Option<ArcData<T>>,
}

impl<T: Real> Core<T> {
    fn empty() -> Self {
        Self {
            verts: Vec::new(),
            tangent: Vec::new(),
            start_pos: None,
            start_ang: None,
            first_seg: FirstSeg::NotYet,
            pending: None,
            last_arc: None,
        }
    }

    /// Seeds the entry vertex (the chain's provisional first vertex —
    /// a seam fillet may later retrim it to the seam arc's end).
    fn seed(&mut self, p: Point2<T>) {
        self.verts.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self.start_pos = Some(p);
    }

    /// Sets the bulge of the segment leaving the current last vertex —
    /// exactly [`crate::LoopBuilder`]'s `set_leaving_bulge`, plus the
    /// structural first-segment kind pin.
    fn set_leaving(&mut self, bulge: T, kind: FirstSeg) -> Result<(), PathError<T>> {
        if self.verts.len() == 1 && self.first_seg == FirstSeg::NotYet {
            self.first_seg = kind;
        }
        match self.verts.last_mut() {
            Some(v) => {
                v.bulge = bulge;
                Ok(())
            }
            None => Err(PathError::UnderdeterminedLeg {
                site: "set_leaving on an empty chain",
            }),
        }
    }

    /// Appends a straight segment to `p` (LoopBuilder `line_to`).
    fn push_line(&mut self, p: Point2<T>) -> Result<(), PathError<T>> {
        self.set_leaving(T::zero(), FirstSeg::Line)?;
        self.verts.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self.last_arc = None;
        Ok(())
    }

    /// Appends an arc segment to `p` with `bulge` (LoopBuilder
    /// `arc_to`), remembering the carrier for identity checks.
    fn push_arc(
        &mut self,
        p: Point2<T>,
        bulge: T,
        carrier: ArcData<T>,
    ) -> Result<(), PathError<T>> {
        self.set_leaving(bulge, FirstSeg::Arc)?;
        self.verts.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self.last_arc = Some(carrier);
        Ok(())
    }

    /// The current chain end.
    fn head(&self) -> Result<Point2<T>, PathError<T>> {
        self.verts
            .last()
            .map(|v| v.pos)
            .ok_or(PathError::UnderdeterminedLeg {
                site: "head of an empty chain",
            })
    }

    /// Declares the joint at the current last vertex tangent
    /// (LoopBuilder `declare_tangent`).
    fn declare_last(&mut self) {
        if let Some(last) = self.verts.len().checked_sub(1) {
            self.tangent.push(last);
        }
    }

    /// Whether the joint at the current last vertex is already
    /// declared (the zero-fit knife-edge bookkeeping).
    fn last_declared(&self) -> bool {
        match self.verts.len().checked_sub(1) {
            Some(last) => self.tangent.last() == Some(&last),
            None => false,
        }
    }

    /// Finishes the loop.
    fn build(self) -> ProfileLoop<T> {
        ProfileLoop {
            vertices: self.verts,
            tangent_joints: self.tangent,
        }
    }
}

// ------------------------------------------------------------------
// Shared classification helpers (every decision through the reified
// predicate funnel; margins in meters).
// ------------------------------------------------------------------

/// The run's linear classification band (ε_input, K·ε_input) from the
/// global [`Tolerance`].
fn linear_band<T: Real>() -> Result<Band, PathError<T>> {
    let tol = Tolerance::get();
    Band::new(tol.eps, tol.k * tol.eps).map_err(PathError::Band)
}

/// Unit direction of an angle.
fn unit<T: Real>(ang: T) -> Vec2<T> {
    let (s, c) = ang.sin_cos();
    Vec2::new(c, s)
}

/// The carrier circle of the arc from `a` to `b` with `bulge` (the
/// crate docs' closed forms: center = midpoint + n̂·apothem, radius =
/// L(1+b²)/(4|b|)). A zero bulge yields infinite/poison carrier data —
/// consumed only by identity classifications, where an infinite margin
/// is definitely-distinct (and an interval poison escalates honestly).
fn arc_carrier<T: Real>(a: Point2<T>, b: Point2<T>, bulge: T) -> ArcData<T> {
    let chord = b - a;
    let l = chord.norm_squared().sqrt();
    let four = T::from_f64(4.0);
    let half = T::from_f64(0.5);
    let mid = a + chord * half;
    let n_hat = Vec2::new(-chord.y, chord.x) * (T::one() / l);
    let apothem = l * (T::one() - bulge.powi(2)) / (four * bulge);
    ArcData {
        center: mid + n_hat * apothem,
        radius: l * (T::one() + bulge.powi(2)) / (four * bulge.abs()),
    }
}

/// §4 item 1, one generic function: classifies the departure `dep`
/// against the incoming tangent and its reverse on the incoming leg's
/// lever arm. `line_close` selects the tangent-line-close refusal
/// flavor (a Start-targeting straight closer).
fn junction_check<T: Decide>(
    inc: &Incoming<T>,
    dep: Dir<T>,
    line_close: bool,
) -> Result<(), PathError<T>> {
    let band = linear_band()?;
    let u_in = inc.ang.unit;
    let u_dep = dep.unit;
    let turn = u_in.perp_dot(u_dep);
    match decide("path_junction_turn", Length::levered(turn, inc.arm), band) {
        Ok(Sign::Zero) => {
            let margin = turn * inc.arm;
            // Which refusal class — tangent (dep ≈ incoming) or cusp
            // (dep ≈ reverse)? A decision, so it goes through the
            // funnel: the alignment cos φ levered by the same arm. A
            // Zero here means the arm itself is degenerate (both
            // components sub-ε) — refused as the tangent class, the
            // recourse that names moving the geometry.
            let side = decide(
                "path_junction_side",
                Length::levered(u_in.dot(u_dep), inc.arm),
                band,
            );
            match side {
                Ok(Sign::Negative) => Err(PathError::JunctionCusp {
                    margin,
                    arm: inc.arm,
                }),
                Ok(_) if line_close => Err(PathError::TangentLineClose { margin }),
                Ok(_) => Err(PathError::JunctionTangent {
                    margin,
                    arm: inc.arm,
                }),
                Err(source) => Err(PathError::Escalated { source }),
            }
        }
        Ok(_) => Ok(()),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// §4 item 4: refuses a declared continuation whose constructed
/// carrier is the incoming carrier itself (cocircular arcs) — the
/// `carrier_circles_identity` margin d + |Δr| on the linear band.
fn refuse_identical_carriers<T: Decide>(
    a: &ArcData<T>,
    b: &ArcData<T>,
) -> Result<(), PathError<T>> {
    let band = linear_band()?;
    let d = (a.center - b.center).norm_squared().sqrt();
    let margin = d + (a.radius - b.radius).abs();
    match decide("path_carrier_identity", Length::of(margin), band) {
        Ok(Sign::Zero) => Err(PathError::SameCarrierJunction { margin }),
        Ok(_) => Ok(()),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// Maps the shared fillet closed form's refusals into the algebra's
/// vocabulary: a Negative leg fit here IS the anchor-fit refusal (the
/// helper is fed the two sides' anchoring extents); an escalation
/// stays an escalation.
fn map_fillet_err<T: Real>(refusal: TrimRefusal<T>) -> PathError<T> {
    match refusal {
        TrimRefusal::DoesNotFit {
            leg,
            setback,
            leg_length,
        } => PathError::AnchorOutsideTrimmedExtent {
            side: leg,
            setback,
            available: leg_length,
        },
        TrimRefusal::Escalated(source) => PathError::Escalated { source },
        TrimRefusal::Band(b) => PathError::Band(b),
    }
}

/// The fillet arc's own carrier: tangent to the arrival carrier at
/// `t2`, center r to the turn side σ = sign(tan(φ/2)).
fn fillet_arc_carrier<T: Real>(trims: &LineFilletTrims<T>, u2: Vec2<T>, radius: T) -> ArcData<T> {
    let sgn = T::one().copysign(trims.half_tan);
    let n_hat = Vec2::new(-u2.y, u2.x);
    ArcData {
        center: trims.t2 + n_hat * (sgn * radius),
        radius,
    }
}

impl<T: Decide> Core<T> {
    /// Resolves an opened fillet the moment its arrival side is
    /// Directed (PATHS-DESIGN §2: the r-arc tangent to both carriers is
    /// inserted at their implicit virtual corner, trimming both).
    ///
    /// The corner is the incoming ray × arrival carrier intersection
    /// (never authored); every side is anchored by a real on-path point
    /// (the ray's origin; the arrival's anchor — for the seam, the
    /// entry point), and the shared `line_line_fillet_trims` closed
    /// form is fed exactly those anchors, so its `fillet_leg_fit`
    /// gates ARE the anchor-fit checks (`AnchorOutsideTrimmedExtent`).
    ///
    /// `seam = true` is the `.to(Start)` resolution: the arc becomes
    /// the closing segment, the entry vertex is retrimmed to the arc's
    /// end (the entry anchor stays interior — its own fit check is the
    /// arrival-side gate), and joint 0 is declared.
    fn resolve_fillet(
        &mut self,
        arr_pos: Point2<T>,
        arr_ang: Dir<T>,
        seam: bool,
    ) -> Result<(ArcData<T>, Sign), PathError<T>> {
        let pending = self
            .pending
            .take()
            .ok_or(PathError::OverdeterminedJunction {
                site: "fillet resolution without an opened fillet",
            })?;
        let band = linear_band()?;
        let u1 = pending.ang.unit;
        let u2 = arr_ang.unit;
        let w = arr_pos - pending.origin;
        let wn = w.norm_squared().sqrt();
        // (1) parallel/tangent carriers admit no corner: the turn
        // margin sin φ levered by the anchor separation.
        let cross = u1.perp_dot(u2);
        match decide("path_corner_turn", Length::levered(cross, wn), band) {
            Ok(Sign::Zero) => {
                return Err(PathError::NoCornerForFillet {
                    reason: PathNoCornerReason::CarriersParallel,
                    radius: pending.radius,
                });
            }
            Ok(_) => {}
            Err(source) => return Err(PathError::Escalated { source }),
        }
        // (2) the corner must lie ahead of the incoming ray's origin
        // and behind the arrival side's anchor (ray parameters, meters).
        let t_ray = w.perp_dot(u2) / cross;
        let s_arr = w.perp_dot(u1) / cross;
        match decide("path_corner_advance", Length::of(t_ray), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(PathError::NoCornerForFillet {
                    reason: PathNoCornerReason::BehindIncomingRay,
                    radius: pending.radius,
                });
            }
            Err(source) => return Err(PathError::Escalated { source }),
        }
        match decide("path_corner_advance", Length::of(-s_arr), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(PathError::NoCornerForFillet {
                    reason: PathNoCornerReason::BehindArrivalAnchor,
                    radius: pending.radius,
                });
            }
            Err(source) => return Err(PathError::Escalated { source }),
        }
        // (3) a seam fillet lands on side 1, which must be a straight
        // carrier (arc-arrival fillets are out of scope, §7).
        if seam && self.first_seg != FirstSeg::Line {
            return Err(PathError::SeamFilletOntoArc);
        }
        let corner = pending.origin + u1 * t_ray;
        // (4) the shared line×line closed form, anchored: head = the
        // ray's origin, next = the arrival's anchor.
        let trims = line_line_fillet_trims(pending.origin, corner, arr_pos, pending.radius)
            .map_err(map_fillet_err)?;
        let arc = fillet_arc_carrier(&trims, u2, pending.radius);
        // (5) incoming side emission: Positive fit emits the straight
        // piece + declared joint (exactly LoopBuilder::fillet); Zero
        // fit springs the arc off the last vertex — if that joint
        // carries a declared flag (a `.tangent()` ray, or a previous
        // fillet's arc end), §4 item 4 refuses carrier identity there.
        if trims.fit_in == Sign::Positive {
            self.push_line(trims.t1)?;
            self.declare_last();
        } else if self.last_declared() {
            let adjacent = if pending.by_tangent {
                pending.origin_incoming.as_ref().and_then(|inc| inc.carrier)
            } else {
                self.last_arc
            };
            if let Some(adj) = adjacent {
                refuse_identical_carriers(&adj, &arc)?;
            }
        }
        // (6) the arc. Interior: emitted, its outgoing joint declared
        // (Positive fit: as LoopBuilder::fillet; Zero fit: the
        // continuation extends the arrival carrier tangentially by
        // construction, so the algebra declares what a hand author
        // must declare manually). Seam: the arc IS the closing
        // segment; the entry vertex retrims to its end and joint 0 is
        // the constructed seam tangency.
        if seam {
            self.set_leaving(trims.bulge, FirstSeg::Arc)?;
            match self.verts.first_mut() {
                Some(v0) => v0.pos = trims.t2,
                None => {
                    return Err(PathError::UnderdeterminedLeg {
                        site: "seam fillet on an empty chain",
                    });
                }
            }
            self.tangent.push(0);
        } else {
            self.push_arc(trims.t2, trims.bulge, arc)?;
            self.declare_last();
        }
        Ok((arc, trims.fit_out))
    }
}

// ------------------------------------------------------------------
// The path value and its binders.
// ------------------------------------------------------------------

/// A partially authored profile loop: the algebra's chain value, under
/// the lattice markers `P` (position slot) and `A` (angle slot) — see
/// the module docs for the four states and the surface vocabulary.
///
/// Values are moved through every verb (the chain is linear); closing
/// verbs consume the path and return the lowered [`ProfileLoop`], so
/// use-after-close is ill-typed. `Clone` FORKS the chain: both forks
/// are ordinary on-lattice values sharing the authored prefix, each
/// continuable and closable independently (motif exploration); every
/// lowered result still passes through the verify layer on its own —
/// forking mints no new closure door and no unverified state.
/// Repeated motifs are builder functions over the one chain
/// (`fn motif(p: PartialPath<f64, HasPos<Plain>, HasAng>) -> …`) —
/// there is no concatenation operator and no second path value.
#[derive(Clone, Debug)]
pub struct PartialPath<T: Real, P, A> {
    core: Core<T>,
    tip: Tip<T>,
    _state: PhantomData<(P, A)>,
}

/// Re-wraps runtime state under new lattice markers (private: binders
/// are the only constructors).
fn in_state<T: Real, P, A>(core: Core<T>, tip: Tip<T>) -> PartialPath<T, P, A> {
    PartialPath {
        core,
        tip,
        _state: PhantomData,
    }
}

/// A directed-point tip minted by a leg end.
fn leg_end_tip<T: Real>(at: Point2<T>, ang: Dir<T>, arm: T, carrier: Option<ArcData<T>>) -> Tip<T> {
    Tip {
        pos: Some(PosData {
            at,
            incoming: Some(Incoming { ang, arm, carrier }),
        }),
        ang: None,
        ang_by_tangent: false,
    }
}

impl Open {
    /// Binds the entry position: `Open → Point` (plain flavor — the
    /// entry has no incoming carrier; its junction check happens at
    /// the seam).
    pub fn at<T: Real>(self, p: Point2<T>) -> PartialPath<T, HasPos<Plain>, NoAng> {
        let mut core = Core::empty();
        core.seed(p);
        in_state(
            core,
            Tip {
                pos: Some(PosData {
                    at: p,
                    incoming: None,
                }),
                ang: None,
                ang_by_tangent: false,
            },
        )
    }

    /// Binds the entry direction first: `Open → Angle` (radians, in
    /// the sketch plane; position pending).
    pub fn angle<T: Real>(self, theta: T) -> PartialPath<T, NoPos, HasAng> {
        self.director(Dir::from_angle(theta))
    }

    /// Binds the entry direction first as exact COMPONENTS
    /// (`Open → Angle`): the direction-valued alternative to
    /// [`angle`](Self::angle) — see [`PartialPath::toward`] for the
    /// exactness contract and the refusal.
    pub fn toward<T: Decide>(
        self,
        dx: T,
        dy: T,
    ) -> Result<PartialPath<T, NoPos, HasAng>, PathError<T>> {
        Ok(self.director(unit_from_components(dx, dy)?))
    }

    fn director<T: Real>(self, dir: Dir<T>) -> PartialPath<T, NoPos, HasAng> {
        in_state(
            Core::empty(),
            Tip {
                pos: None,
                ang: Some(dir),
                ang_by_tangent: false,
            },
        )
    }
}

/// The shared director-from-components construction (G1 constructor 5):
/// normalizes `(dx, dy)` and stores the unit ray VERBATIM — no trig
/// round-trip, so an axis-aligned or Pythagorean direction is exact
/// (`(-1, 0)` → `(-1, 0)`; `(3, 4)` → `(0.6, 0.8)`).
///
/// The norm is classified through the funnel on the linear band and
/// must be definitely positive: `(0, 0)` names no direction at all, and
/// a norm within ε_input of zero cannot be normalized without
/// amplifying its own noise into the ray. Only the RATIO of the
/// components carries meaning, so the recourse is free — scale them up.
fn unit_from_components<T: Decide>(dx: T, dy: T) -> Result<Dir<T>, PathError<T>> {
    let band = linear_band()?;
    // `powi(2)`, never `dx * dx`: a director's components straddle zero
    // by construction (every axis direction has a zero component), and
    // the plain product treats its factors as independent, so an
    // interval enclosure picks up a spurious negative lower bound and
    // poisons this `sqrt` (memories/interval-square-poison.md).
    let norm = (dx.powi(2) + dy.powi(2)).sqrt();
    match decide("path_director_norm", Length::of(norm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(PathError::ZeroDirection { dx, dy }),
        Err(source) => return Err(PathError::Escalated { source }),
    }
    Ok(Dir::from_unit(Vec2::new(dx / norm, dy / norm)))
}

/// The circle primitive (G1 constructor 1): a **one-step complete-loop
/// program form**, not a chain — `circle(center, r)` IS the whole loop,
/// so it returns the lowered [`ProfileLoop`] directly and there is
/// nothing to continue, close, or bind.
///
/// **It authors no seam.** That is the whole point, and it is what
/// keeps PQ4 (PATHS-DESIGN §6: a chain's seam sits at a junction or
/// fillet, never mid-carrier) untouched: a chain still cannot close
/// mid-carrier, because the split this primitive uses is not authored
/// at all. The conventional split — two semicircles at the ±x poles,
/// counterclockwise — is the primitive's PRIVATE lowering, exactly the
/// M2 closed-carrier precedent: a detail of how a closed carrier
/// reaches a vertex+bulge document, not a junction anyone said. The two
/// joints are same-carrier identities, so nothing is declared tangent
/// (there is no tangency to declare — it is one circle).
///
/// `radius` must classify definitely positive
/// ([`PathError::NonpositiveCircleRadius`]), through the same funnel as
/// the other sign gates. A circle is one loop among others: profiles
/// mix circle loops and chain loops freely (per-loop wholesale, which
/// is the mixed-authoring rule of §6 read at loop granularity).
pub fn circle<T: Decide>(center: Point2<T>, radius: T) -> Result<ProfileLoop<T>, PathError<T>> {
    let band = linear_band()?;
    match decide("path_circle_radius", Length::of(radius), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(PathError::NonpositiveCircleRadius { radius }),
        Err(source) => return Err(PathError::Escalated { source }),
    }
    Ok(ProfileLoop::new(vec![
        ProfileVertex {
            pos: Point2::new(center.x + radius, center.y),
            bulge: T::one(),
        },
        ProfileVertex {
            pos: Point2::new(center.x - radius, center.y),
            bulge: T::one(),
        },
    ]))
}

impl<T: Decide, A: AngMarker> PartialPath<T, NoPos, A> {
    /// Adds the position bit (`Open → Point`, `Angle → Directed`) —
    /// written once, generic over the angle slot it does not touch.
    ///
    /// On a fillet arrival whose angle is already bound, completing
    /// the position resolves the fillet (both carriers fixed): the
    /// corner construction and anchor-fit gates run here — see
    /// [`PathError`]. On the angle-first entry, this seeds the chain.
    /// `p` is absolute (profile frame), a real on-path point (the
    /// side's anchor).
    pub fn at(mut self, p: Point2<T>) -> Result<PartialPath<T, HasPos<Plain>, A>, PathError<T>> {
        match (self.tip.ang, self.core.pending.is_some()) {
            (Some(theta), true) => {
                self.core.resolve_fillet(p, theta, false)?;
            }
            (Some(theta), false) => {
                self.core.seed(p);
                if self.core.start_ang.is_none() {
                    self.core.start_ang = Some(theta);
                }
            }
            (None, _) => {}
        }
        Ok(in_state(
            self.core,
            Tip {
                pos: Some(PosData {
                    at: p,
                    incoming: None,
                }),
                ang: self.tip.ang,
                ang_by_tangent: self.tip.ang_by_tangent,
            },
        ))
    }
}

impl<T: Decide, P: PosMarker> PartialPath<T, P, NoAng> {
    /// Adds the angle bit wherever it is missing (`Point → Directed`,
    /// `Open → Angle`) — one generic function; the junction check
    /// reads the flavor's optional incoming tangent at runtime.
    ///
    /// On a directed point this classifies `theta` against the
    /// incoming tangent and its reverse (PATHS-DESIGN §4 item 1):
    /// definitely-sharp proceeds; within ε_input of tangent refuses
    /// [`PathError::JunctionTangent`] (one refusal, one recourse:
    /// `.tangent()` makes intended tangency exact by construction);
    /// within ε_input of the reverse refuses
    /// [`PathError::JunctionCusp`] (no declaration door — #131). On a
    /// plain point there is nothing to check (an arrival side meets
    /// its fillet arc tangentially by construction; the entry's check
    /// happens at the seam). On a fillet arrival whose position is
    /// already bound, completing the direction resolves the fillet.
    pub fn angle(self, theta: T) -> Result<PartialPath<T, P, HasAng>, PathError<T>> {
        self.director(Dir::from_angle(theta))
    }

    /// The direction-valued director (G1 constructor 5): binds the same
    /// angular DOF as [`angle`](Self::angle) — the same lattice slot,
    /// set at most once per side — from exact COMPONENTS instead of an
    /// angle. `(dx, dy)` is normalized and the unit ray stored verbatim,
    /// so the departure never makes a trig round-trip: `.toward(-1, 0)`
    /// gives the ray `(-1, 0)` exactly, where `.angle(PI)` gives
    /// `(-1, 1.2246e-16)` and carries that ulp into every corner and
    /// trim point downstream. Only the components' RATIO is read
    /// (magnitude is not a length and binds nothing).
    ///
    /// `(0, 0)` — and any norm within ε_input of zero — refuses
    /// [`PathError::ZeroDirection`]: it names no direction, and the
    /// recourse is free, since scaling the components changes nothing
    /// else. Junction/fillet semantics are otherwise identical to
    /// [`angle`](Self::angle), including the §4 item 1 check on a
    /// directed point and the fillet resolution on a bound arrival.
    pub fn toward(self, dx: T, dy: T) -> Result<PartialPath<T, P, HasAng>, PathError<T>> {
        self.director(unit_from_components(dx, dy)?)
    }

    fn director(mut self, dir: Dir<T>) -> Result<PartialPath<T, P, HasAng>, PathError<T>> {
        if let Some(pos) = &self.tip.pos {
            if let Some(inc) = &pos.incoming {
                junction_check(inc, dir, false)?;
            }
            let at = pos.at;
            if self.core.pending.is_some() {
                self.core.resolve_fillet(at, dir, false)?;
            } else if self.core.start_ang.is_none() {
                self.core.start_ang = Some(dir);
            }
        }
        self.tip.ang = Some(dir);
        self.tip.ang_by_tangent = false;
        Ok(in_state(self.core, self.tip))
    }
}

impl<T: Decide> PartialPath<T, HasPos<WithIncoming>, NoAng> {
    /// Consumes a **directed point only**: re-uses the incoming end
    /// tangent as the departure — exact by construction, nothing for
    /// verification to contradict — and emits the DECLARED flag on
    /// lowering. Ill-typed on plain points (no direction to inherit),
    /// which is what makes "fillets sit between defined geometry"
    /// structural rather than a rule.
    pub fn tangent(mut self) -> PartialPath<T, HasPos<WithIncoming>, HasAng> {
        self.tip.ang = self
            .tip
            .pos
            .as_ref()
            .and_then(|p| p.incoming.as_ref())
            .map(|inc| inc.ang);
        self.tip.ang_by_tangent = true;
        self.core.declare_last();
        in_state(self.core, self.tip)
    }

    /// `.angle(incoming + δ)` sugar on a directed point: turns by `δ`
    /// radians from the incoming tangent. `turn(0)` lands in the
    /// tangent band and refuses (use [`tangent`](Self::tangent));
    /// `turn(±π)` lands in the reverse band and refuses as a cusp.
    pub fn turn(
        mut self,
        delta: T,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, HasAng>, PathError<T>> {
        let inc = self.tip.pos.as_ref().and_then(|p| p.incoming).ok_or(
            PathError::UnderdeterminedLeg {
                site: "turn on a tip without incoming data",
            },
        )?;
        let theta = Dir::from_angle(inc.ang.ang + delta);
        junction_check(&inc, theta, false)?;
        self.tip.ang = Some(theta);
        self.tip.ang_by_tangent = false;
        Ok(in_state(self.core, self.tip))
    }
}

// ------------------------------------------------------------------
// Legs (direction-consuming, from Directed) and the fillet.
// ------------------------------------------------------------------

impl<T: Decide, F: Flavor> PartialPath<T, HasPos<F>, HasAng> {
    /// The bound tip pose (backstopped: unreachable-missing data is a
    /// typed error, never a panic).
    fn dep(&self) -> Result<(Point2<T>, Dir<T>), PathError<T>> {
        let pos = self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "directed tip without a position",
        })?;
        let ang = self.tip.ang.ok_or(PathError::UnderdeterminedLeg {
            site: "directed tip without an angle",
        })?;
        Ok((pos.at, ang))
    }

    /// A straight leg of length `len` along the bound departure,
    /// terminating at a directed point. After a fillet this extends
    /// the arrival side's one leg past its anchor (no collinear
    /// neighbor is minted — §4 item 4's by-construction exemption).
    ///
    /// A declared straight continuation of a straight leg
    /// (`.tangent().line(len)` after a line) IS the same carrier and
    /// refuses [`PathError::SameCarrierJunction`] — extend the
    /// original leg instead.
    ///
    /// `len` must classify definitely positive
    /// ([`PathError::NonpositiveLeg`] otherwise): a negative length
    /// would run the side backward, silently detaching the tip's
    /// anchored points from the final path — the §4 item 3 invariant
    /// is gated here, at the one verb that takes a signed length.
    pub fn line(
        mut self,
        len: T,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let (at, ang) = self.dep()?;
        let band = linear_band()?;
        match decide("path_leg_length", Length::of(len), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => return Err(PathError::NonpositiveLeg { length: len }),
            Err(source) => return Err(PathError::Escalated { source }),
        }
        if self.tip.ang_by_tangent
            && let Some(inc) = self.tip.pos.as_ref().and_then(|p| p.incoming.as_ref())
            && inc.carrier.is_none()
        {
            return Err(PathError::SameCarrierJunction { margin: T::zero() });
        }
        let end = at + ang.unit * len;
        let head = self.core.head()?;
        self.core.push_line(end)?;
        let arm = (end - head).norm_squared().sqrt();
        Ok(in_state(self.core, leg_end_tip(end, ang, arm, None)))
    }

    /// Opens a corner fillet of radius `radius`: consumes the incoming
    /// Directed (the departure ray) and opens the arrival side Open,
    /// bound in either order (`.at(dd).angle(θ)`, `.angle(θ).at(dd)`,
    /// or `.to(Start)` for the seam). Once the arrival is Directed the
    /// r-arc tangent to both carriers is inserted at their implicit
    /// virtual corner, trimming both — the corner is never authored
    /// (it exists only as the carrier intersection), and authoring a
    /// point then filleting it away is unrepresentable.
    ///
    /// `radius` must classify definitely positive
    /// ([`PathError::NonpositiveFilletRadius`] otherwise), gated here —
    /// before an arrival can be authored against a fillet that has no
    /// tangent construction to offer.
    pub fn fillet(mut self, radius: T) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        let (at, ang) = self.dep()?;
        let band = linear_band()?;
        match decide("path_fillet_radius", Length::of(radius), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => return Err(PathError::NonpositiveFilletRadius { radius }),
            Err(source) => return Err(PathError::Escalated { source }),
        }
        self.core.pending = Some(PendingFillet {
            origin: at,
            ang,
            radius,
            by_tangent: self.tip.ang_by_tangent,
            origin_incoming: self.tip.pos.as_ref().and_then(|p| p.incoming),
        });
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// The unique arc tangent to the bound departure through the
    /// target: `tangent_arc_to(p)` continues to a directed point;
    /// `tangent_arc_to(Start)` is the tangent-seam close (the seam's
    /// junction check runs at `Start` with both directions known).
    pub fn tangent_arc_to<Tgt: TangentArcTarget<T, F>>(self, target: Tgt) -> Tgt::Out {
        Tgt::tangent_arc_from(self, target)
    }

    /// The tangent arc's geometry toward `p`: the tangent-chord angle
    /// Δ in the departure frame, bulge tan(Δ/2), end tangent
    /// departure + 2Δ, and the §4 item 4 refusals under an inherited
    /// (declared) departure: a collinear target degenerates the arc
    /// onto a straight incoming carrier (same line), a cocircular
    /// carrier is the incoming circle itself.
    fn tangent_arc_geom(
        &self,
        p: Point2<T>,
        closing: bool,
    ) -> Result<TangentArcGeom<T>, PathError<T>> {
        let (at, ang) = self.dep()?;
        let d = p - at;
        let u = ang.unit;
        let along = u.dot(d);
        let across = u.perp_dot(d);
        let delta = across.atan2(along);
        let bulge = (delta / T::from_f64(2.0)).tan();
        let carrier = arc_carrier(at, p, bulge);
        if self.tip.ang_by_tangent
            && let Some(inc) = self.tip.pos.as_ref().and_then(|pd| pd.incoming.as_ref())
        {
            match &inc.carrier {
                None => {
                    let band = linear_band()?;
                    match decide("path_collinear_target", Length::of(across), band) {
                        Ok(Sign::Zero) => {
                            return Err(if closing {
                                PathError::TangentLineClose { margin: across }
                            } else {
                                PathError::SameCarrierJunction { margin: across }
                            });
                        }
                        Ok(_) => {}
                        Err(source) => return Err(PathError::Escalated { source }),
                    }
                }
                Some(prev) => refuse_identical_carriers(prev, &carrier)?,
            }
        }
        let end_ang = Dir::from_angle(ang.ang + delta + delta);
        let chord = d.norm_squared().sqrt();
        Ok(TangentArcGeom {
            bulge,
            end_ang,
            carrier,
            chord,
        })
    }

    fn tangent_arc_to_point(
        mut self,
        p: Point2<T>,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let g = self.tangent_arc_geom(p, false)?;
        self.core.push_arc(p, g.bulge, g.carrier)?;
        let arm = g.carrier.radius.min(g.chord);
        Ok(in_state(
            self.core,
            leg_end_tip(p, g.end_ang, arm, Some(g.carrier)),
        ))
    }

    fn tangent_arc_to_start(mut self) -> Result<ProfileLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let start_ang = self.core.start_ang.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry direction is bound",
        })?;
        let g = self.tangent_arc_geom(start_pos, true)?;
        let arm = g.carrier.radius.min(g.chord);
        junction_check(
            &Incoming {
                ang: g.end_ang,
                arm,
                carrier: Some(g.carrier),
            },
            start_ang,
            false,
        )?;
        self.core.set_leaving(g.bulge, FirstSeg::Arc)?;
        Ok(self.core.build())
    }
}

// ------------------------------------------------------------------
// Point-state verbs (sugar tier: one call each, expands to core).
// ------------------------------------------------------------------

impl<T: Decide, F: Flavor> PartialPath<T, HasPos<F>, NoAng> {
    /// `.angle(toward target).line(distance)` in one call
    /// (`Point → Point`, also from arrivals): on a directed point the
    /// junction check runs on the computed direction; on a fillet
    /// arrival this binds the arrival direction toward the target,
    /// resolves the fillet, and ends the side at the target.
    /// `line_to(Start)` is the sharp straight seam (both seam-side
    /// junction checks run; a within-band-tangent straight closer is
    /// the overdetermined tangent line close and refuses always).
    pub fn line_to<Tgt: LineTarget<T, F>>(self, target: Tgt) -> Tgt::Out {
        Tgt::line_from(self, target)
    }

    /// The arc with the given `bulge` to the target (`Point → Point`;
    /// direction from chord + bulge, the M2 convention: b = tan(θ/4),
    /// start tangent = chord − θ/2). On a directed point the junction
    /// check runs on the arc's start tangent. `arc_to(Start, b)` is
    /// the sharp arc seam. On a fillet arrival this refuses typed —
    /// arc-arrival fillets are out of scope in v1 (PATHS-DESIGN §7).
    pub fn arc_to<Tgt: ArcTarget<T, F>>(self, target: Tgt, bulge: T) -> Tgt::Out {
        Tgt::arc_from(self, target, bulge)
    }

    /// The arc THROUGH a point (G1 constructor 2): the unique arc
    /// through (current tip, `via`, target). A free arc — the junction
    /// semantics are `arc_to`'s exactly: on a directed point the §4
    /// item 1 check runs on the arc's start tangent; `arc_via(v, Start)`
    /// is the sharp arc seam; on a fillet arrival it refuses
    /// [`PathError::ArcArrivalFillet`] (§7).
    ///
    /// All three points are AUTHORED and stored verbatim — the two
    /// endpoints as chain vertices, the through-point only as the
    /// bulge's input. The bulge is derived at lowering by the existing
    /// closed form [`crate::bulge_from_via`] (inscribed angle,
    /// tan(Δ/2)), never re-typed by the author; a hand chain's
    /// `LoopBuilder::arc_to_via` feeds that same function the same three
    /// points, so the two doors emit the same bits.
    ///
    /// Refusals beyond `arc_to`'s: a through-point within ε_input of
    /// the chord LINE ([`PathError::ArcViaCollinear`] — the whole
    /// collinear class, on-chord and beyond-the-end alike), and
    /// coincident endpoints ([`PathError::DegenerateArcChord`]).
    pub fn arc_via<Tgt: ArcViaTarget<T, F>>(self, via: Point2<T>, target: Tgt) -> Tgt::Out {
        Tgt::arc_via_from(self, via, target)
    }

    /// The arc ABOUT a centre (G1 constructor 3): from the current tip
    /// about `center` to the target, with `winding` selecting which of
    /// the two arcs. The winding is a STRUCTURAL argument
    /// ([`ArcSweep::Ccw`] / [`ArcSweep::Cw`]), not a number to get the
    /// sign of — the choice is discrete, so it is spelled discretely.
    /// This is the centre-intent spelling: a lantern's belly is *the
    /// sphere's own arc about the globe centre*, and authoring it this
    /// way says so, rather than fitting an arc and hoping the carrier
    /// lands on the sphere.
    ///
    /// **Equidistance is checked, never repaired**: |tip − centre| and
    /// |end − centre| go through the funnel, and a definite mismatch
    /// refuses [`PathError::ArcCenterNotEquidistant`]. Silently
    /// re-projecting the centre onto the endpoints' bisector (or an
    /// endpoint onto the circle) would move an authored point, which §4
    /// item 3 forbids — three points that contradict each other are a
    /// bug in the authoring, and the refusal says which two disagree.
    ///
    /// The bulge is derived at lowering by [`crate::bulge_from_center`],
    /// bit-for-bit as `LoopBuilder::arc_to_center` derives it. Junction
    /// semantics are `arc_to`'s; `arc_center(c, Start, w)` is the sharp
    /// arc seam. A centre within ε_input of an endpoint refuses
    /// [`PathError::DegenerateArcCenter`].
    pub fn arc_center<Tgt: ArcCenterTarget<T, F>>(
        self,
        center: Point2<T>,
        target: Tgt,
        winding: ArcSweep,
    ) -> Tgt::Out {
        Tgt::arc_center_from(self, center, target, winding)
    }

    fn tip_pos(&self) -> Result<&PosData<T>, PathError<T>> {
        self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "point tip without a position",
        })
    }

    fn line_to_point(
        mut self,
        p: Point2<T>,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let pos = self.tip_pos()?;
        let at = pos.at;
        let d = p - at;
        let gamma = Dir::from_angle(d.y.atan2(d.x));
        if self.core.pending.is_some() {
            self.core.resolve_fillet(at, gamma, false)?;
        } else {
            if let Some(inc) = &self.tip_pos()?.incoming {
                junction_check(inc, gamma, false)?;
            }
            if self.core.start_ang.is_none() {
                self.core.start_ang = Some(gamma);
            }
        }
        let head = self.core.head()?;
        self.core.push_line(p)?;
        let arm = (p - head).norm_squared().sqrt();
        Ok(in_state(self.core, leg_end_tip(p, gamma, arm, None)))
    }

    fn line_to_start(mut self) -> Result<ProfileLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let at = self.tip_pos()?.at;
        let d = start_pos - at;
        let gamma = Dir::from_angle(d.y.atan2(d.x));
        if self.core.pending.is_some() {
            self.core.resolve_fillet(at, gamma, false)?;
        } else if let Some(inc) = &self.tip_pos()?.incoming {
            junction_check(inc, gamma, true)?;
        }
        let start_ang = *self.core.start_ang.get_or_insert(gamma);
        let head = self.core.head()?;
        let arm = (start_pos - head).norm_squared().sqrt();
        junction_check(
            &Incoming {
                ang: gamma,
                arm,
                carrier: None,
            },
            start_ang,
            true,
        )?;
        self.core.set_leaving(T::zero(), FirstSeg::Line)?;
        Ok(self.core.build())
    }

    /// The chord length, gated definitely positive: every arc leg spans
    /// a chord, and a closed carrier is [`circle`]'s business (PQ4).
    fn arc_chord(&self, end: Point2<T>) -> Result<T, PathError<T>> {
        let at = self.tip_pos()?.at;
        let band = linear_band()?;
        let chord = (end - at).norm_squared().sqrt();
        match decide("path_arc_chord", Length::of(chord), band) {
            Ok(Sign::Positive) => Ok(chord),
            Ok(_) => Err(PathError::DegenerateArcChord { chord }),
            Err(source) => Err(PathError::Escalated { source }),
        }
    }

    /// [`arc_via`](Self::arc_via)'s derived bulge: the collinear gate
    /// (the through-point's signed perpendicular offset from the chord
    /// LINE, meters — zero for on-chord and beyond-the-end alike, which
    /// is why one refusal covers the class), then the existing closed
    /// form on the three authored points.
    fn arc_via_bulge(&self, via: Point2<T>, end: Point2<T>) -> Result<T, PathError<T>> {
        let at = self.tip_pos()?.at;
        let chord_len = self.arc_chord(end)?;
        let band = linear_band()?;
        let offset = (end - at).perp_dot(via - at) / chord_len;
        match decide("path_arc_via_offset", Length::of(offset), band) {
            Ok(Sign::Zero) => return Err(PathError::ArcViaCollinear { offset }),
            Ok(_) => {}
            Err(source) => return Err(PathError::Escalated { source }),
        }
        Ok(bulge_from_via(at, via, end))
    }

    /// [`arc_center`](Self::arc_center)'s derived bulge: both radii
    /// gated definitely positive, then equidistance gated definitely
    /// ZERO (a definite mismatch refuses; an undecidable one escalates —
    /// neither is repaired), then the existing closed form.
    fn arc_center_bulge(
        &self,
        center: Point2<T>,
        end: Point2<T>,
        winding: ArcSweep,
    ) -> Result<T, PathError<T>> {
        let at = self.tip_pos()?.at;
        let band = linear_band()?;
        let r_tip = (at - center).norm_squared().sqrt();
        let r_end = (end - center).norm_squared().sqrt();
        for radius in [r_tip, r_end] {
            match decide("path_arc_center_radius", Length::of(radius), band) {
                Ok(Sign::Positive) => {}
                Ok(_) => return Err(PathError::DegenerateArcCenter { radius }),
                Err(source) => return Err(PathError::Escalated { source }),
            }
        }
        match decide(
            "path_arc_center_equidistant",
            Length::of(r_tip - r_end),
            band,
        ) {
            Ok(Sign::Zero) => {}
            Ok(_) => {
                return Err(PathError::ArcCenterNotEquidistant {
                    tip_radius: r_tip,
                    end_radius: r_end,
                });
            }
            Err(source) => return Err(PathError::Escalated { source }),
        }
        self.arc_chord(end)?;
        Ok(bulge_from_center(at, end, center, winding))
    }

    /// The target point a closing verb resolves to (the entry pose's
    /// position — [`Start`] by reference, never re-typed).
    fn start_target(&self) -> Result<Point2<T>, PathError<T>> {
        self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })
    }

    /// The arc's derived angles: chord direction γ, included angle
    /// θ = 4·atan(b), start tangent γ − θ/2, end tangent γ + θ/2.
    fn arc_angles(at: Point2<T>, target: Point2<T>, bulge: T) -> (Dir<T>, Dir<T>, T) {
        let d = target - at;
        let gamma = d.y.atan2(d.x);
        let theta = bulge.atan() * T::from_f64(4.0);
        let half = theta / T::from_f64(2.0);
        (
            Dir::from_angle(gamma - half),
            Dir::from_angle(gamma + half),
            d.norm_squared().sqrt(),
        )
    }

    fn arc_to_point(
        mut self,
        p: Point2<T>,
        bulge: T,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        if self.core.pending.is_some() {
            return Err(PathError::ArcArrivalFillet);
        }
        let pos = self.tip_pos()?;
        let at = pos.at;
        let (start_t, end_t, chord) = Self::arc_angles(at, p, bulge);
        if let Some(inc) = &pos.incoming {
            junction_check(inc, start_t, false)?;
        }
        if self.core.start_ang.is_none() {
            self.core.start_ang = Some(start_t);
        }
        let carrier = arc_carrier(at, p, bulge);
        self.core.push_arc(p, bulge, carrier)?;
        let arm = carrier.radius.min(chord);
        Ok(in_state(
            self.core,
            leg_end_tip(p, end_t, arm, Some(carrier)),
        ))
    }

    fn arc_to_start(mut self, bulge: T) -> Result<ProfileLoop<T>, PathError<T>> {
        if self.core.pending.is_some() {
            return Err(PathError::ArcArrivalFillet);
        }
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let pos = self.tip_pos()?;
        let at = pos.at;
        let (start_t, end_t, chord) = Self::arc_angles(at, start_pos, bulge);
        if let Some(inc) = &pos.incoming {
            junction_check(inc, start_t, false)?;
        }
        let start_ang = *self.core.start_ang.get_or_insert(start_t);
        let carrier = arc_carrier(at, start_pos, bulge);
        let arm = carrier.radius.min(chord);
        junction_check(
            &Incoming {
                ang: end_t,
                arm,
                carrier: Some(carrier),
            },
            start_ang,
            false,
        )?;
        self.core.set_leaving(bulge, FirstSeg::Arc)?;
        Ok(self.core.build())
    }
}

impl<T: Decide> PartialPath<T, NoPos, HasAng> {
    /// **The far-end anchor** (G1 constructor 4, the W5 wall): binds an
    /// arrival side's position bit to `anchor` AND ends the side there —
    /// the `to`-family's combined step, read on the arrival side.
    ///
    /// PATHS-DESIGN §3 already says every side is anchored by a real
    /// on-path point plus a direction, and `.angle(θ).at(p)` binds
    /// exactly that pair. What was missing was only the ability for the
    /// side to STOP at its anchor: `.at(p)` leaves the tip Directed at
    /// `p`, and the only continuations run PAST it, so a side whose
    /// natural end is its far vertex had to be authored as a synthetic
    /// mid-side anchor plus a length — a point that is not a vertex, and
    /// a number nobody measured. `.to(p)` says the natural thing: this
    /// side ends at `p`.
    ///
    /// It adds no geometry and no new determination — `.angle(θ).to(p)`
    /// fixes exactly what `.angle(θ).at(p)` fixes (the arrival carrier
    /// is the line through `p` in direction θ; the corner is still the
    /// carrier intersection, never authored). The difference is where
    /// the leg terminates, so the fillet resolution, its corner gates,
    /// and the anchor-fit checks are all `.at(p)`'s, unchanged; `p` is
    /// on the final path either way, authored once. The result is a
    /// directed point (incoming tangent θ), so the next verb's junction
    /// check runs exactly as after any leg.
    ///
    /// The direction must be bound FIRST (`.angle(θ).to(p)` /
    /// `.toward(dx, dy).to(p)`): with the anchor as the terminus, the
    /// side's carrier is what the director supplies. An exact trim fit —
    /// the fillet arc reaching `anchor` with no straight run left — is
    /// not an error: the side simply IS the arc, no degenerate segment
    /// is emitted, and the tip carries the arc as its incoming carrier.
    ///
    /// At the ENTRY (direction bound, no fillet open) there is no
    /// arrival side to end, and this refuses
    /// [`PathError::FarEndAnchorWithoutFillet`] — the entry authors its
    /// first side with `.at(p)`, and the seam is authored at the back
    /// (§2's entry rule). Targeting [`Start`] with the far-end form is
    /// deliberately NOT in this surface; see the module docs.
    pub fn to(
        mut self,
        anchor: Point2<T>,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let dir = self.tip.ang.ok_or(PathError::UnderdeterminedLeg {
            site: "far-end anchor on a tip without a bound direction",
        })?;
        if self.core.pending.is_none() {
            return Err(PathError::FarEndAnchorWithoutFillet);
        }
        let (arc, fit_out) = self.core.resolve_fillet(anchor, dir, false)?;
        if fit_out == Sign::Positive {
            let head = self.core.head()?;
            let arm = (anchor - head).norm_squared().sqrt();
            self.core.push_line(anchor)?;
            Ok(in_state(self.core, leg_end_tip(anchor, dir, arm, None)))
        } else {
            // Exact fit: the trim reached the anchor, so the arc IS the
            // whole side. Emitting a zero-length straight piece here
            // would mint the degenerate segment the fit gate exists to
            // avoid — the incoming side's `Zero` fit is suppressed the
            // same way inside `resolve_fillet`.
            let head = self.core.head()?;
            Ok(in_state(
                self.core,
                leg_end_tip(head, dir, arc.radius, Some(arc)),
            ))
        }
    }
}

impl<T: Decide> PartialPath<T, NoPos, NoAng> {
    /// The combined binder consuming a directed-point VALUE
    /// (`Open → Directed` in one step). [`Start`] is its canonical
    /// argument, and using it is closing: `.angle(θ).fillet(r)
    /// .to(Start)` is the seam fillet — both carriers bound, nothing
    /// pending, loop closed. (Curve-pose arguments — `c.start()` /
    /// `c.end()` — arrive with NURBS legs in v2; see the module docs.)
    pub fn to(mut self, target: Start) -> Result<ProfileLoop<T>, PathError<T>> {
        let Start = target;
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let start_ang = self.core.start_ang.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry direction is bound",
        })?;
        self.core.resolve_fillet(start_pos, start_ang, true)?;
        Ok(self.core.build())
    }
}

// ------------------------------------------------------------------
// Closure targets: ordinary verbs target either an authored point or
// Start — targeting Start IS closing, structurally.
// ------------------------------------------------------------------

impl<T: Real> sealed::Sealed for Point2<T> {}

/// A [`PartialPath::line_to`] target: an authored absolute point, or
/// [`Start`] (the sharp straight seam). Sealed.
pub trait LineTarget<T: Decide, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn line_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self) -> Self::Out;
}

impl<T: Decide, F: Flavor> LineTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn line_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self) -> Self::Out {
        path.line_to_point(target)
    }
}

impl<T: Decide, F: Flavor> LineTarget<T, F> for Start {
    type Out = Result<ProfileLoop<T>, PathError<T>>;
    fn line_from(path: PartialPath<T, HasPos<F>, NoAng>, _target: Self) -> Self::Out {
        path.line_to_start()
    }
}

/// A [`PartialPath::arc_to`] target: an authored absolute point, or
/// [`Start`] (the sharp arc seam). Sealed.
pub trait ArcTarget<T: Decide, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn arc_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self, bulge: T) -> Self::Out;
}

impl<T: Decide, F: Flavor> ArcTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn arc_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self, bulge: T) -> Self::Out {
        path.arc_to_point(target, bulge)
    }
}

impl<T: Decide, F: Flavor> ArcTarget<T, F> for Start {
    type Out = Result<ProfileLoop<T>, PathError<T>>;
    fn arc_from(path: PartialPath<T, HasPos<F>, NoAng>, _target: Self, bulge: T) -> Self::Out {
        path.arc_to_start(bulge)
    }
}

/// A [`PartialPath::arc_via`] target: an authored absolute point, or
/// [`Start`] (the sharp arc seam through the via-point). Sealed.
pub trait ArcViaTarget<T: Decide, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn arc_via_from(
        path: PartialPath<T, HasPos<F>, NoAng>,
        via: Point2<T>,
        target: Self,
    ) -> Self::Out;
}

impl<T: Decide, F: Flavor> ArcViaTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn arc_via_from(
        path: PartialPath<T, HasPos<F>, NoAng>,
        via: Point2<T>,
        target: Self,
    ) -> Self::Out {
        let bulge = path.arc_via_bulge(via, target)?;
        path.arc_to_point(target, bulge)
    }
}

impl<T: Decide, F: Flavor> ArcViaTarget<T, F> for Start {
    type Out = Result<ProfileLoop<T>, PathError<T>>;
    fn arc_via_from(
        path: PartialPath<T, HasPos<F>, NoAng>,
        via: Point2<T>,
        _target: Self,
    ) -> Self::Out {
        let bulge = path.arc_via_bulge(via, path.start_target()?)?;
        path.arc_to_start(bulge)
    }
}

/// A [`PartialPath::arc_center`] target: an authored absolute point, or
/// [`Start`] (the sharp arc seam about the centre). Sealed.
pub trait ArcCenterTarget<T: Decide, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn arc_center_from(
        path: PartialPath<T, HasPos<F>, NoAng>,
        center: Point2<T>,
        target: Self,
        winding: ArcSweep,
    ) -> Self::Out;
}

impl<T: Decide, F: Flavor> ArcCenterTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn arc_center_from(
        path: PartialPath<T, HasPos<F>, NoAng>,
        center: Point2<T>,
        target: Self,
        winding: ArcSweep,
    ) -> Self::Out {
        let bulge = path.arc_center_bulge(center, target, winding)?;
        path.arc_to_point(target, bulge)
    }
}

impl<T: Decide, F: Flavor> ArcCenterTarget<T, F> for Start {
    type Out = Result<ProfileLoop<T>, PathError<T>>;
    fn arc_center_from(
        path: PartialPath<T, HasPos<F>, NoAng>,
        center: Point2<T>,
        _target: Self,
        winding: ArcSweep,
    ) -> Self::Out {
        let bulge = path.arc_center_bulge(center, path.start_target()?, winding)?;
        path.arc_to_start(bulge)
    }
}

/// A [`PartialPath::tangent_arc_to`] target: an authored absolute
/// point, or [`Start`] (the tangent-seam close). Sealed.
pub trait TangentArcTarget<T: Decide, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn tangent_arc_from(path: PartialPath<T, HasPos<F>, HasAng>, target: Self) -> Self::Out;
}

impl<T: Decide, F: Flavor> TangentArcTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn tangent_arc_from(path: PartialPath<T, HasPos<F>, HasAng>, target: Self) -> Self::Out {
        path.tangent_arc_to_point(target)
    }
}

impl<T: Decide, F: Flavor> TangentArcTarget<T, F> for Start {
    type Out = Result<ProfileLoop<T>, PathError<T>>;
    fn tangent_arc_from(path: PartialPath<T, HasPos<F>, HasAng>, _target: Self) -> Self::Out {
        path.tangent_arc_to_start()
    }
}
