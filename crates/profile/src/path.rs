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
//! equivalent raw vertex-and-bulge chain would — the line×line
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
//! r)` — matching a hand author's raw `fillet(corner, anchor,
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
//! - the `Via { q, p }` and `Center { c, winding, p }` modes of
//!   [`arc_to`](PartialPath::arc_to) — the arc through a point, and
//!   the arc about a centre with a structural winding (equidistance
//!   checked, never repaired). §2c unified them with `Bulge { p, b }`
//!   into the one sharp arc leg.
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
//! NURBS legs (`nurbs_in_place`, `nurbs(curve)` and variants) are
//! specified by PATHS-DESIGN §2 but have **no representation in the
//! v1 lowering target** (a [`ProfileLoop`] is a vertex+bulge chain;
//! this crate deliberately depends on `geom-core` only) — they arrive
//! with the v2 profiles-as-programs representation (#104). There is no
//! NURBS-adjacent fillet WALL waiting with them: bare `fillet(r)` is
//! the uniform ray extension (§2c round 10), and `nurbs_fillet` is an
//! absent verb. Mixed authoring is OUT (§6): a loop is authored
//! either here or as a raw chain, never both; there is no
//! path-concatenation operator — repeated motifs are builder
//! functions over the one chain.
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
//! assert_eq!(square.loop_.vertices().len(), 8);
//! assert_eq!(square.loop_.tangent_joints().len(), 8);
//! // The chain also RECORDED itself: the program replays to the same
//! // loop, bit for bit (profiles-as-programs v2 — see [`program`]).
//! assert_eq!(square.program.len(), 13);
//! Profile::new(SketchPlane::xy(), vec![square.loop_]).validate(Tolerance::get())?;
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
//! The endpoint-FULL modes are legs from a Point, so they are
//! ill-typed on a Directed tip (the departure is already bound, and
//! the mode would have to value-match it):
//!
//! ```compile_fail,E0277
//! use geom_core::Point2;
//! use profile::{Open, Via};
//! let p = Open.at(Point2::new(0.0, 0.0)).angle(0.0).unwrap()
//!     .arc_to(Via { q: Point2::new(1.0, 1.0), p: Point2::new(2.0, 0.0) });
//! ```
//!
//! ```compile_fail,E0277
//! use geom_core::Point2;
//! use profile::{ArcSweep, Center, Open};
//! let p = Open.at(Point2::new(0.0, 0.0)).angle(0.0).unwrap()
//!     .arc_to(Center { c: Point2::new(1.0, 0.0), winding: ArcSweep::Ccw, p: Point2::new(2.0, 0.0) });
//! ```
//!
//! and the endpoint-FREE pair is symmetrically ill-typed on a bare
//! point, which has no departure tangent to sweep about:
//!
//! ```compile_fail,E0277
//! use geom_core::Point2;
//! use profile::{ArcSide, Open, Sweep};
//! let p = Open.at(Point2::new(0.0_f64, 0.0))
//!     .arc_to(Sweep { r: 1.0, side: ArcSide::Left, angle: 1.0 });
//! ```
//!
//! The matrix is CLOSED, not extensible: [`PointLeg`] is sealed, so a
//! foreign spec type cannot mint a seventh row.
//!
//! ```compile_fail,E0277
//! use geom_core::Point2;
//! use profile::path::{Flavor, HasPos, NoAng, PartialPath, PointLeg};
//! struct ForeignSpec;
//! impl<F: Flavor> PointLeg<f64, F> for ForeignSpec {
//!     type Out = ();
//!     fn leg_from(_path: PartialPath<f64, HasPos<F>, NoAng>, _spec: Self) {}
//! }
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
//! **§2c**: the fused family's INADMISSIBLE (state, mode) pairs are
//! missing trait impls. `Bulge` is never an arrival (no chord exists
//! there) …
//!
//! ```compile_fail,E0277
//! use geom_core::Point2;
//! use profile::{Bulge, Open};
//! let p = Open.at(Point2::new(0.0, 0.0_f64)).angle(0.0).unwrap()
//!     .fillet_arc(0.25, Bulge { p: Point2::new(2.0, 2.0), b: 0.5 });
//! ```
//!
//! … an interior arc arrival EMITS its run at the verb and lands on an
//! ordinary directed point (§2c dissolution), so a SHARP continuation
//! after an arc arrival is spelled with an ordinary director — the
//! junction check guards the geometry there, not the lattice:
//!
//! ```
//! use geom_core::Point2;
//! use profile::{ArcSweep, Center, Open};
//! let sharp = Open.at(Point2::new(5.05, -1.6_f64)).toward(2.1, 0.8).unwrap()
//!     .fillet_arc(0.5, Center {
//!         c: Point2::new(7.0, 0.0),
//!         winding: ArcSweep::Ccw,
//!         p: Point2::new(8.5, 0.0),
//!     })
//!     .unwrap()
//!     // The arrival tip's tangent is +y; 2.6 rad is a genuine corner.
//!     .angle(2.6).unwrap()
//!     .line(0.5);
//! assert!(sharp.is_ok());
//! ```
//!
//! … and the arc-arrival CLOSE (`Center { p: Start }`) is a complete
//! loop, so nothing continues from its result:
//!
//! ```compile_fail,E0599
//! use geom_core::Point2;
//! use profile::{ArcSweep, Center, Open, Start};
//! let done = Open.at(Point2::new(0.0, 0.0_f64)).toward(1.0, 0.0).unwrap()
//!     .fillet_arc(0.3, Center {
//!         c: Point2::new(2.0, -2.0),
//!         winding: ArcSweep::Ccw,
//!         p: Start,
//!     })
//!     .unwrap();
//! let more = done.line_to(Point2::new(1.0, 1.0));
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

use crate::RawLoop;
use core::marker::PhantomData;

use geom_core::k_stats::decide;
use geom_core::{Tol, Band, Decide, Indeterminate, Margin, Point2, Real, Sign, Tolerance, Vec2};

use crate::path::program::{ClosedLoop, Step, Target};
use crate::sugar::{
    ArcSweep, LineFilletTrims, TrimRefusal, bulge_from_center, bulge_from_via,
    line_line_fillet_trims,
};
use crate::validate::{FilletLeg, FilletLegCarrier, NoCornerReason};
use crate::{ProfileLoop, ProfileVertex};

/// The arc-carrier fillet boundary — the algebra's derived-corner
/// resolution and the lifted S8 ladder (LIB-G2 §3b). It is a separate
/// module because it is the ONE place in `path` that reads a bracket:
/// see its docs for the ratified justification, and `path.rs` itself
/// stays `Bounds`-free.
pub(crate) mod arc_fillet;
mod family;
pub mod program;
pub(crate) mod verbs;

pub use arc_fillet::ArcCarrierScalar;
#[doc(hidden)]
pub use family::FusedIncoming;
pub use family::{
    ArrivalSpec, LegEndIncoming, PointIncoming, PointLeg, RadiusArrival, RadiusArrivalAt,
    RadiusArrivalDir, TangentIncoming, ViaArrival, ViaArrivalStart,
};
/// The complete-loop program forms are declared as table rows (they
/// are `Entry → Closed` transitions), so they are defined in
/// [`program`]; this module is their public home.
#[doc(inline)]
pub use program::{circle, circle_split};
pub use verbs::{ArcLen, ArcSide, Bulge, Center, Radius, Sweep, Via};
#[doc(hidden)]
pub use verbs::{DirectedPoint, TangentArcLeg};

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
    /// **G2**: the two carriers do not meet at all — a ray that misses
    /// its circle, or circles that are disjoint, concentric, or one
    /// inside the other. Distinct from
    /// [`CarriersParallel`](Self::CarriersParallel), which is the
    /// tangency knife edge: there they touch, and there is still no
    /// corner to cut.
    CarriersDoNotMeet,
    /// **G2**: a derived corner exists, but the ratified S2
    /// construction finds no tangent circle of the requested radius
    /// there. The constructor door's own vocabulary is carried through
    /// rather than flattened, so "the radius is too large for this
    /// corner" and "every tangent circle touches a leg past the corner"
    /// stay distinguishable at the algebra door too.
    NoTangentCircle(NoCornerReason),
}

/// Typed refusals of the authoring algebra — geometry the lattice
/// cannot rule out, refused loudly (PATHS-DESIGN §3 "Refusals" and §4).
/// The verify layer's own errors ([`crate::ProfileError`]) still apply to the
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
        /// **G2 §3c**: that side's CARRIER KIND, carrying the angular
        /// margin `(extent − setback)/R` for a circular side. A bare
        /// linear setback says nothing on a circle, so the payload is
        /// metered in the carrier's own currency (M5 S2's
        /// does-not-fit diagnostic shape, at the algebra door).
        carrier: FilletLegCarrier,
        /// The tangent setback from the corner, meters (diagnostic; an
        /// arc length `R·Δθ` on a circular side).
        setback: T,
        /// The anchored extent available to the trim, meters (same
        /// currency as `setback`).
        available: T,
    },
    /// **M8**: the derived corner and a tangent circle of the requested
    /// radius both exist, but the tangent point on one side cannot be
    /// certified — that side's offset radius ρ = R − σ·τ·r, the lever
    /// the tangent point is recovered over, is shorter than the least
    /// lever the run's band supports at the corner's scale.
    ///
    /// The derivation of the least lever lives on
    /// `sugar::ArcCarrier::offset_circles`. The
    /// situation is a fillet radius too close to that side's own
    /// carrier radius, so the recourse is to move one of the two.
    FilletOffsetLeverTooShort {
        /// The side whose offset lever is short.
        side: FilletLeg,
        /// That side's carrier radius R, meters (scalar-typed payload).
        carrier_radius: T,
        /// Its signed offset radius ρ = R − σ·τ·r, meters.
        offset_radius: T,
        /// The least |ρ| the band supports here, meters.
        least_lever: T,
        /// The classified margin |ρ| − `least_lever`, meters.
        margin: T,
    },
    /// A sharp arc LEG was reached while a fillet is still open (its
    /// arrival direction unbound). §2c binds an arc arrival by its own
    /// CARRIER, inside the fused verb — `fillet_arc(r, spec)` /
    /// `arc_fillet_arc(spec, r, spec2)` — so that the trim and the arc
    /// it is tangent to are ONE authoring act; an arc leg departing an
    /// already-positioned arrival point would instead claim the
    /// arrival direction a second time.
    ArcLegOnOpenFillet {
        /// What was reached, and what binds it instead.
        site: &'static str,
    },
    /// A `.to(Start)` seam fillet reached a chain whose FIRST side is
    /// an arc: the seam retrims the entry vertex, and retrimming the
    /// start of an arc would slide it off its own carrier (LB5: a
    /// mid-arc seam vertex is authored topology). Closing onto a
    /// carrier while KEEPING the entry vertex is the arc-arrival close,
    /// `fillet_arc(r, Center { c, winding, p: Start })`.
    SeamRetrimsArcFirstSide,
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
    /// **§2c**: a fused/endpoint-free arc spec whose authored datum
    /// names no arc — a zero bulge (the arc degenerates to its chord),
    /// or a sweep angle / arc length that is not definitely positive.
    /// Classified through the funnel (`path_arc_bulge` /
    /// `path_arc_sweep`).
    DegenerateArcSpec {
        /// The refused authored datum (bulge, angle, or length).
        value: T,
    },
    /// A [`circle_split`] subdivision count below 2: one vertex cannot
    /// carry a full turn (bulge = tan(θ/4) diverges at θ = 2π), so the
    /// smallest declared subdivision of a closed carrier is two arcs —
    /// which is [`circle`]'s own private lowering. A structural check,
    /// not a classified one: `n` is a count, never a measured value.
    CircleSplitCount {
        /// The refused subdivision count.
        n: usize,
    },
    /// An [`arc_continue`](PartialPath::arc_continue) reached with no
    /// incoming ARC carrier: the declared-subdivision step splits the
    /// carrier the chain is already running on, so a straight incoming
    /// leg (or a tip with no incoming leg data) has nothing to split —
    /// a collinear "subdivision" of a line is spelled as two `line_to`
    /// legs... which the same-carrier rule refuses, deliberately: the
    /// recorded need is arc subdivision (the half-disc's equator
    /// vertex); a line form would be new vocabulary with no use case.
    ArcContinueNeedsArcCarrier,
    /// An [`arc_continue`](PartialPath::arc_continue) target that does
    /// not lie on the incoming carrier (|target − centre| − r decided
    /// nonzero): the authored data contradicts itself — refused, never
    /// re-projected (an authored point never moves, §4 item 3).
    ArcContinueOffCarrier {
        /// The classified radial offset, meters.
        offset: T,
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
    /// A `Via` mode's through-point is within ε_input of the CHORD LINE:
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
    /// A `Center` mode's centre is not equidistant from the two
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
    /// A `Center` mode's centre is within ε_input of an endpoint: the
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
                 — move the geometry"
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
                 {margin:?} m): carrier identity is not tangency — extend the leg \
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
                    PathNoCornerReason::CarriersDoNotMeet => {
                        "the two carriers do not meet: a ray missing its circle, or circles \
                         disjoint, concentric, or one inside the other"
                    }
                    PathNoCornerReason::NoTangentCircle(reason) => match reason {
                        NoCornerReason::OffsetCarriersDisjoint => {
                            "a corner exists, but no circle of that radius is tangent to both \
                             carriers there — the radius is too large for the corner"
                        }
                        NoCornerReason::NoCornerSideCandidate => {
                            "a corner exists, but every tangent circle of that radius touches a \
                             side past the corner"
                        }
                    },
                };
                write!(f, "no corner for a radius-{radius:?} m fillet: {what}")
            }
            Self::AnchorOutsideTrimmedExtent {
                side,
                carrier,
                setback,
                available,
            } => write!(
                f,
                "the fillet trim would eat the {side} side's anchoring on-path point on its \
                 {carrier} carrier: tangent setback {setback:?} m exceeds the {available:?} m \
                 the anchor pins — reduce the radius or move the anchor"
            ),
            Self::FilletOffsetLeverTooShort {
                side,
                carrier_radius,
                offset_radius,
                least_lever,
                margin,
            } => write!(
                f,
                "the {side} side's offset lever rho {offset_radius:?} m (carrier radius \
                 {carrier_radius:?} m) is shorter than the {least_lever:?} m this corner's \
                 scale needs at the run's tolerance (margin {margin:?} m): the fillet's \
                 tangent point is recovered by projecting its centre back onto that \
                 carrier, and dividing by a lever that short cannot place the point within \
                 tolerance — move the fillet radius away from that side's carrier radius, \
                 or bring the corner's carriers closer together"
            ),
            Self::ArcLegOnOpenFillet { site } => write!(f, "{site}"),
            Self::DegenerateArcSpec { value } => write!(
                f,
                "this arc spec's authored datum ({value:?}) names no arc: a zero bulge \
                 degenerates to the chord (author a line), and a sweep angle or arc length \
                 must be definitely positive"
            ),
            Self::SeamRetrimsArcFirstSide => write!(
                f,
                "a seam fillet retrims the entry vertex, so it needs a straight first side; \
                 to close onto an arc carrier and KEEP the entry vertex, use \
                 fillet_arc(r, Center {{ c, winding, p: Start }})"
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
            Self::CircleSplitCount { n } => write!(
                f,
                "circle_split needs at least 2 arcs (got n = {n}): a single vertex cannot \
                 carry a full turn (bulge diverges), so the smallest subdivision of a \
                 closed carrier is two arcs"
            ),
            Self::ArcContinueNeedsArcCarrier => write!(
                f,
                "arc_continue subdivides the incoming ARC carrier; the incoming leg here is \
                 straight (or absent), so there is no carrier to split — author the geometry \
                 as its own legs instead"
            ),
            Self::ArcContinueOffCarrier { offset } => write!(
                f,
                "the arc_continue target does not lie on the incoming carrier (radial offset \
                 {offset:?} m): a subdivision vertex is ON the carrier by definition — fix the \
                 authored point rather than expecting a re-projection"
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
                 chain leg"
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
pub(crate) struct ArcData<T: Real> {
    center: Point2<T>,
    radius: T,
}

/// What the arrival side does once its fillet resolves — the one bit
/// that decides whether the fillet arc's OUTGOING joint is a declared
/// tangency or a free junction.
///
/// The arc is tangent to the arrival carrier at `t2` by construction.
/// Whether that makes the joint AT `t2` a tangency depends on what the
/// chain emits next, which only the calling verb knows:
///
/// - [`Continues`](ArrivalKind::Continues) — an `.at`/`.angle`/`line_to`
///   arrival: the tip stays Directed on the arrival side, so EVERY
///   continuation departs along the arrival ray and is tangent to the
///   arc by construction. The algebra declares what a hand author would
///   have to declare manually.
/// - [`EndsAtAnchor`](ArrivalKind::EndsAtAnchor) — the far-end anchor:
///   the side STOPS, so the tip is a directed point whose next
///   direction is free. With a Positive fit the straight run to the
///   anchor still rides the carrier and the joint at `t2` is a genuine
///   tangency; with an exact (Zero) fit there is no run at all, the
///   arc's end IS the side's end, and declaring it would claim a
///   tangency against an arbitrary next side — §4 item 2's
///   declaration-without-construction. Gated exactly as the hand door
///   gates its own outgoing declaration (`sugar.rs`, `fit_out`).
/// - [`Seam`](ArrivalKind::Seam) — the `.to(Start)` close.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArrivalKind {
    Continues,
    EndsAtAnchor,
    Seam,
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
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct Dir<T: Real> {
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

/// Chain-side bookkeeping for an opened fillet — the §4 item 4
/// zero-fit knife-edge data. This is NOT part of the kernel's
/// [`verbs::Pending`] state value: the kernel cannot name it, which is
/// what keeps the verbs pure (§2c round 12).
#[derive(Clone, Debug)]
struct PendingMeta<T: Real> {
    /// The ray was bound by `.tangent()` (or ray-extended off a leg
    /// end): its origin joint is already declared.
    by_tangent: bool,
    /// The ray origin's incoming carrier, if the origin was a leg end.
    origin_incoming: Option<Incoming<T>>,
    /// **Arc extension** (§2c): the fused arc incoming CONTINUES the
    /// origin leg's own carrier, so the incoming run's emission MOVES
    /// that leg's end vertex to the trim point (the §4 item 4
    /// exemption, exactly as ray extension) instead of pushing a
    /// co-carrier neighbour.
    extends_carrier: bool,
}

/// The accumulated lowering state: the vertex chain emitted so far
/// (mirroring the raw chain builder's emission verb-for-verb), the
/// declared joints, the entry pose (the [`Start`] value), and the
/// pending fillet, if a side is open.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Core<T: Real> {
    verts: Vec<ProfileVertex<T>>,
    tangent: Vec<usize>,
    start_pos: Option<Point2<T>>,
    start_ang: Option<Dir<T>>,
    first_seg: FirstSeg,
    pending: Option<verbs::Pending<T>>,
    /// Chain-side knife-edge bookkeeping for `pending` (same lifetime).
    pending_meta: Option<PendingMeta<T>>,
    /// The carrier of the last emitted segment when it is an arc.
    last_arc: Option<ArcData<T>>,
    /// **Profiles-as-programs (v2)**: the authoring verbs, recorded as
    /// they lower. Each binder pushes exactly its own step, so one
    /// chain yields both the lowered loop and its program.
    program: Vec<Step<T>>,
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
            pending_meta: None,
            last_arc: None,
            program: Vec::new(),
        }
    }

    /// Records one authoring verb (record-as-you-lower).
    fn record(&mut self, step: Step<T>) {
        self.program.push(step);
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
    /// exactly the raw builder's `set_leaving_bulge`, plus the
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

    /// Appends a straight segment to `p` (the raw `line_to`).
    fn push_line(&mut self, p: Point2<T>) -> Result<(), PathError<T>> {
        self.set_leaving(T::zero(), FirstSeg::Line)?;
        self.verts.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self.last_arc = None;
        Ok(())
    }

    /// Appends an arc segment to `p` with `bulge` (the raw
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
    /// (the raw `declare_tangent`).
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

    /// Finishes the loop, returning it PAIRED with the program that
    /// produced it (see [`ClosedLoop`]).
    fn build(self) -> ClosedLoop<T> {
        ClosedLoop {
            loop_: ProfileLoop {
                vertices: self.verts,
                tangent_joints: self.tangent,
            },
            program: self.program,
        }
    }
}

// ------------------------------------------------------------------
// Shared classification helpers (every decision through the reified
// predicate funnel; margins in meters).
// ------------------------------------------------------------------

/// The run's linear classification band (ε_input, K·ε_input) from the
/// global [`Tolerance`].
fn linear_band<T: Real>(tol: Tol) -> Result<Band, PathError<T>> {
    Band::new(tol.eps(), tol.k() * tol.eps()).map_err(PathError::Band)
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
    tol: Tol,
) -> Result<(), PathError<T>> {
    let band = linear_band(tol)?;
    let u_in = inc.ang.unit;
    let u_dep = dep.unit;
    let turn = u_in.perp_dot(u_dep);
    match decide("path_junction_turn", Margin::levered(turn, inc.arm), band) {
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
                Margin::levered(u_in.dot(u_dep), inc.arm),
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

/// **§2c arc extension's same-carrier decision**: whether the carrier a
/// fused `Radius` incoming derives from a directed point IS that
/// point's own incoming carrier — the same d + |Δr| identity margin as
/// [`refuse_identical_carriers`], read as a DECISION: `Zero` continues
/// the arriving leg (the vertex-move exemption), definite non-zero is
/// a new tangent carrier constructed at the tip. Both outcomes are
/// legal spellings, which is what deletes the old mismatched-r hole
/// structurally: every authored `r` names a sound construction.
fn carriers_are_identical<T: Decide>(a: &ArcData<T>, b: &ArcData<T>, tol: Tol) -> Result<bool, PathError<T>> {
    let band = linear_band(tol)?;
    let d = (a.center - b.center).norm_squared().sqrt();
    let margin = d + (a.radius - b.radius).abs();
    match decide("path_carrier_identity", Margin::of(margin), band) {
        Ok(Sign::Zero) => Ok(true),
        Ok(_) => Ok(false),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// §4 item 4: refuses a declared continuation whose constructed
/// carrier is the incoming carrier itself (cocircular arcs) — the
/// `carrier_circles_identity` margin d + |Δr| on the linear band.
fn refuse_identical_carriers<T: Decide>(
    a: &ArcData<T>,
    b: &ArcData<T>,
    tol: Tol,
) -> Result<(), PathError<T>> {
    let band = linear_band(tol)?;
    let d = (a.center - b.center).norm_squared().sqrt();
    let margin = d + (a.radius - b.radius).abs();
    match decide("path_carrier_identity", Margin::of(margin), band) {
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
            // The line×line seam has only straight sides by
            // construction, so its carrier kind is structural, not
            // measured — no bracket read enters `path.rs`.
            carrier: FilletLegCarrier::Line,
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
    /// Takes the opened fillet and its chain-side bookkeeping together.
    fn take_pending(
        &mut self,
        site: &'static str,
    ) -> Result<(verbs::Pending<T>, PendingMeta<T>), PathError<T>> {
        let pending = self
            .pending
            .take()
            .ok_or(PathError::OverdeterminedJunction { site })?;
        let meta = self.pending_meta.take().unwrap_or(PendingMeta {
            by_tangent: false,
            origin_incoming: None,
            extends_carrier: false,
        });
        Ok((pending, meta))
    }

    /// Resolves a FUSED-incoming fillet (`Pending::Arc`) against a
    /// STRAIGHT arrival (the generic binders' ray, the far-end anchor,
    /// or the seam): the boundary machinery derives the corner from the
    /// authored carrier × the arrival ray, and the chain applies the
    /// emissions — the trimmed incoming run along its own carrier, then
    /// the fillet arc (interior) or the closing retrim (seam).
    fn resolve_arc_pending_ray_arrival(
        &mut self,
        arc: verbs::PendingArc<T>,
        meta: PendingMeta<T>,
        arr_pos: Point2<T>,
        arr_ang: Dir<T>,
        kind: ArrivalKind,
        tol: Tol,
    ) -> Result<(ArcData<T>, Sign), PathError<T>> {
        // The seam gate runs BEFORE resolution, exactly as the straight
        // path's does: retrimming an arc first side is refused whatever
        // the corner would have been.
        if kind == ArrivalKind::Seam && self.first_seg != FirstSeg::Line {
            return Err(PathError::SeamRetrimsArcFirstSide);
        }
        let incoming = arc_fillet::FilletSide {
            anchor: arc.anchor,
            carrier: arc_fillet::SideCarrier::Circle {
                centre: arc.centre,
                winding: arc.winding,
            },
        };
        let arrival = arc_fillet::FilletSide {
            anchor: arr_pos,
            carrier: arc_fillet::SideCarrier::Ray(arr_ang.unit),
        };
        let trims = (arc.resolver)(incoming, arrival, arc.radius, tol)?;
        self.emit_fillet_in(&trims, meta.extends_carrier, tol)?;
        match kind {
            ArrivalKind::Seam => {
                // The fillet arc IS the closing segment; the entry
                // vertex retrims to its end and joint 0 is the
                // constructed seam tangency (the straight seam's rule).
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
            }
            ArrivalKind::Continues => self.emit_fillet_arc(&trims, true)?,
            ArrivalKind::EndsAtAnchor => {
                self.emit_fillet_arc(&trims, trims.fit_out == Sign::Positive)?;
            }
        }
        Ok((trims.arc, trims.fit_out))
    }

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
        kind: ArrivalKind,
        tol: Tol,
    ) -> Result<(ArcData<T>, Sign), PathError<T>> {
        let (pending, meta) = self.take_pending("fillet resolution without an opened fillet")?;
        let pending = match pending {
            verbs::Pending::Ray(ray) => ray,
            // §2c: a fused verb AUTHORED the incoming arc, so a straight
            // arrival completes it through the boundary machinery — the
            // old carrier-keyed refusal is gone with the register.
            verbs::Pending::Arc(arc) => {
                return self.resolve_arc_pending_ray_arrival(arc, meta, arr_pos, arr_ang, kind, tol);
            }
        };
        let band = linear_band(tol)?;
        let u1 = pending.dir.unit;
        let u2 = arr_ang.unit;
        let w = arr_pos - pending.origin;
        let wn = w.norm_squared().sqrt();
        // (1) parallel/tangent carriers admit no corner: the turn
        // margin sin φ levered by the anchor separation.
        let cross = u1.perp_dot(u2);
        match decide("path_corner_turn", Margin::levered(cross, wn), band) {
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
        match decide("path_corner_advance", Margin::of(t_ray), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(PathError::NoCornerForFillet {
                    reason: PathNoCornerReason::BehindIncomingRay,
                    radius: pending.radius,
                });
            }
            Err(source) => return Err(PathError::Escalated { source }),
        }
        match decide("path_corner_advance", Margin::of(-s_arr), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(PathError::NoCornerForFillet {
                    reason: PathNoCornerReason::BehindArrivalAnchor,
                    radius: pending.radius,
                });
            }
            Err(source) => return Err(PathError::Escalated { source }),
        }
        // (3) a `.to(Start)` seam RETRIMS the entry vertex, so the
        // segment leaving it must be the straight side 1 — retrimming
        // the start of an arc would slide that arc off its own carrier
        // (LB5: a mid-arc seam vertex is authored topology). Closing
        // onto a carrier and KEEPING the vertex is the arc-arrival
        // close, `fillet_arc(r, Center { c, winding, p: Start })`.
        if kind == ArrivalKind::Seam && self.first_seg != FirstSeg::Line {
            return Err(PathError::SeamRetrimsArcFirstSide);
        }
        let corner = pending.origin + u1 * t_ray;
        // (4) the shared line×line closed form, anchored: head = the
        // ray's origin, next = the arrival's anchor.
        let trims = line_line_fillet_trims(pending.origin, corner, arr_pos, pending.radius)
            .map_err(map_fillet_err)?;
        let arc = fillet_arc_carrier(&trims, u2, pending.radius);
        // (5) incoming side emission: Positive fit emits the straight
        // piece + declared joint (exactly the raw fillet's rule); Zero
        // fit springs the arc off the last vertex — if that joint
        // carries a declared flag (a `.tangent()` ray, or a previous
        // fillet's arc end), §4 item 4 refuses carrier identity there.
        if trims.fit_in == Sign::Positive {
            if meta.by_tangent
                && meta
                    .origin_incoming
                    .as_ref()
                    .is_some_and(|i| i.carrier.is_none())
            {
                // Ray extension of a STRAIGHT leg: extend the leg
                // itself (see `extend_leg_to`); the joint declared at
                // the leg's end is the leg→arc tangency.
                self.extend_leg_to(trims.t1)?;
            } else {
                self.push_line(trims.t1)?;
                self.declare_last();
            }
        } else if self.last_declared() {
            let adjacent = if meta.by_tangent {
                meta.origin_incoming.as_ref().and_then(|inc| inc.carrier)
            } else {
                self.last_arc
            };
            if let Some(adj) = adjacent {
                refuse_identical_carriers(&adj, &arc, tol)?;
            }
        }
        // (6) the arc. Interior: emitted, its outgoing joint declared
        // (Positive fit: as the raw fillet; Zero fit: the
        // continuation extends the arrival carrier tangentially by
        // construction, so the algebra declares what a hand author
        // must declare manually). Seam: the arc IS the closing
        // segment; the entry vertex retrims to its end and joint 0 is
        // the constructed seam tangency.
        if kind == ArrivalKind::Seam {
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
            // The outgoing joint is declared only when something
            // tangent actually follows it (see [`ArrivalKind`]): a
            // continuing arrival always rides the arrival ray, and a
            // far-end side does too WHILE it still has a straight run
            // left. On an exact fit the far-end side ends here, and the
            // next direction is free — declaring would be a claim, not
            // a construction.
            if kind == ArrivalKind::Continues || trims.fit_out == Sign::Positive {
                self.declare_last();
            }
        }
        Ok((arc, trims.fit_out))
    }

    /// **G2**: emits the trimmed INCOMING run of an arc-carrier fillet
    /// — along that side's own carrier when it has one, straight when
    /// it does not — and declares the joint it lands on.
    ///
    /// Bit-identity note: this is `fillet_corner`'s emission verbatim
    /// (`arc_to_center(t1, centre, sweep)` / `line_to(t1)`, then
    /// `declare_tangent()`), which is what lets a migrated site
    /// reproduce the hand-authored loop to the bit. A `Zero` fit emits
    /// nothing and springs the arc off the last vertex, where §4 item 4
    /// refuses carrier identity against an already-declared neighbour.
    fn emit_fillet_in(
        &mut self,
        t: &arc_fillet::ArcFilletTrims<T>,
        merge: bool,
        tol: Tol,
    ) -> Result<(), PathError<T>> {
        if t.fit_in == Sign::Positive {
            match t.in_arc {
                None if merge => self.extend_leg_to(t.t1)?,
                None => self.push_line(t.t1)?,
                Some((centre, sweep)) if merge => self.extend_arc_to(t.t1, centre, sweep)?,
                Some((centre, sweep)) => {
                    let head = self.head()?;
                    let bulge = bulge_from_center(head, t.t1, centre, sweep);
                    let radius = (t.t1 - centre).norm_squared().sqrt();
                    self.push_arc(
                        t.t1,
                        bulge,
                        ArcData {
                            center: centre,
                            radius,
                        },
                    )?;
                }
            }
            if !(t.in_arc.is_none() && merge) {
                self.declare_last();
            }
        } else if self.last_declared()
            && let Some(adj) = self.last_arc
        {
            refuse_identical_carriers(&adj, &t.arc, tol)?;
        }
        Ok(())
    }

    /// **§2c round 10 (ray extension after a STRAIGHT leg)**: the
    /// surviving ray piece and the leg it extends share one carrier, so
    /// emitting it as a separate segment would mint the collinear
    /// neighbor §4 item 4 forbids — instead the leg's own end vertex
    /// MOVES to the trim point (the §4 exemption, "extends one leg",
    /// applied at emission). The leg's authored end stays on the final
    /// path, interior to the extended segment, and the joint the
    /// extension declared becomes the leg→fillet-arc tangency — exactly
    /// what a hand author drawing the leg long would have written.
    fn extend_leg_to(&mut self, t1: Point2<T>) -> Result<(), PathError<T>> {
        match self.verts.last_mut() {
            Some(v) => {
                v.pos = t1;
                Ok(())
            }
            None => Err(PathError::UnderdeterminedLeg {
                site: "ray extension on an empty chain",
            }),
        }
    }

    /// **§2c (arc extension after an ARC leg)**: the surviving carrier
    /// run and the leg it continues share one circle, so emitting it as
    /// a separate segment would mint the co-carrier neighbour §4 item 4
    /// forbids — instead the leg's own end vertex MOVES forward along
    /// the carrier to the trim point (the §4 exemption, exactly as
    /// [`Self::extend_leg_to`]) and the segment's bulge is re-derived
    /// for the longer sweep. The leg's authored end stays on the final
    /// path, interior to the extended segment.
    fn extend_arc_to(
        &mut self,
        t1: Point2<T>,
        centre: Point2<T>,
        sweep: ArcSweep,
    ) -> Result<(), PathError<T>> {
        let n = self.verts.len();
        let from = n
            .checked_sub(2)
            .and_then(|i| self.verts.get(i))
            .map(|v| v.pos)
            .ok_or(PathError::UnderdeterminedLeg {
                site: "arc extension without an incoming segment",
            })?;
        let bulge = bulge_from_center(from, t1, centre, sweep);
        self.verts[n - 2].bulge = bulge;
        self.verts[n - 1].pos = t1;
        let radius = (t1 - centre).norm_squared().sqrt();
        self.last_arc = Some(ArcData {
            center: centre,
            radius,
        });
        Ok(())
    }

    /// **G2**: emits the fillet arc itself as a chain segment, declaring
    /// its outgoing joint when something tangent actually follows it
    /// (see [`ArrivalKind`]).
    fn emit_fillet_arc(
        &mut self,
        t: &arc_fillet::ArcFilletTrims<T>,
        declare: bool,
    ) -> Result<(), PathError<T>> {
        self.push_arc(t.t2, t.bulge, t.arc)?;
        if declare {
            self.declare_last();
        }
        Ok(())
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
    /// The kernel behind the table's `Open → Point` row: seeds the
    /// chain at `p` (recording is the row's, not the kernel's).
    fn at_kernel<T: Real>(self, p: Point2<T>) -> PartialPath<T, HasPos<Plain>, NoAng> {
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
fn unit_from_components<T: Decide>(dx: T, dy: T, tol: Tol) -> Result<Dir<T>, PathError<T>> {
    let band = linear_band(tol)?;
    // `powi(2)`, never `dx * dx`: a director's components straddle zero
    // by construction (every axis direction has a zero component), and
    // the plain product treats its factors as independent, so an
    // interval enclosure picks up a spurious negative lower bound and
    // poisons this `sqrt`. Gated by ci.yml's "interval-square powi(2)
    // allowlist".
    let norm = (dx.powi(2) + dy.powi(2)).sqrt();
    match decide("path_director_norm", Margin::of(norm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(PathError::ZeroDirection { dx, dy }),
        Err(source) => return Err(PathError::Escalated { source }),
    }
    Ok(Dir::from_unit(Vec2::new(dx / norm, dy / norm)))
}

/// The kernel behind the table's circle row: the lowered loop (the
/// row supplies the one-step program).
fn circle_kernel<T: Decide>(center: Point2<T>, radius: T, tol: Tol) -> Result<ProfileLoop<T>, PathError<T>> {
    let band = linear_band(tol)?;
    match decide("path_circle_radius", Margin::of(radius), band) {
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

/// The kernel behind the table's split-circle row: the lowered loop
/// (the row supplies the one-step program).
fn circle_split_kernel<T: Decide>(
    center: Point2<T>,
    radius: T,
    n: usize,
    phase: T,
    tol: Tol,
) -> Result<ProfileLoop<T>, PathError<T>> {
    let band = linear_band(tol)?;
    match decide("path_circle_radius", Margin::of(radius), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(PathError::NonpositiveCircleRadius { radius }),
        Err(source) => return Err(PathError::Escalated { source }),
    }
    if n < 2 {
        return Err(PathError::CircleSplitCount { n });
    }
    let n_t = T::from_f64(n as f64);
    let bulge = (T::pi() / (T::from_f64(2.0) * n_t)).tan();
    let vertices = (0..n)
        .map(|k| {
            let theta = phase + T::from_f64(2.0) * T::pi() * T::from_f64(k as f64) / n_t;
            let (s, c) = theta.sin_cos();
            ProfileVertex {
                pos: Point2::new(center.x + radius * c, center.y + radius * s),
                bulge,
            }
        })
        .collect();
    Ok(ProfileLoop::new(vertices))
}

impl<T: Decide, A: AngMarker> PartialPath<T, NoPos, A> {
    /// The kernel behind the table's position-binding rows: resolves a
    /// bound-angle fillet arrival, or seeds the chain (recording is the
    /// row's, not the kernel's).
    fn at_kernel(mut self, p: Point2<T>, tol: Tol) -> Result<PartialPath<T, HasPos<Plain>, A>, PathError<T>> {
        match (self.tip.ang, self.core.pending.is_some()) {
            (Some(theta), true) => {
                self.core.resolve_fillet(p, theta, ArrivalKind::Continues, tol)?;
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
    fn director(mut self, dir: Dir<T>, tol: Tol) -> Result<PartialPath<T, P, HasAng>, PathError<T>> {
        if let Some(pos) = &self.tip.pos {
            if let Some(inc) = &pos.incoming {
                junction_check(inc, dir, false, tol)?;
            }
            let at = pos.at;
            if self.core.pending.is_some() {
                self.core.resolve_fillet(at, dir, ArrivalKind::Continues, tol)?;
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
    /// The kernel behind the table's tangent-inheritance row (recording
    /// is the row's, not the kernel's).
    fn tangent_kernel(mut self) -> PartialPath<T, HasPos<WithIncoming>, HasAng> {
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

    /// The kernel behind the table's turn row (recording is the row's,
    /// not the kernel's).
    fn turn_kernel(
        mut self,
        delta: T,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, HasAng>, PathError<T>> {
        let inc = self.tip.pos.as_ref().and_then(|p| p.incoming).ok_or(
            PathError::UnderdeterminedLeg {
                site: "turn on a tip without incoming data",
            },
        )?;
        let theta = Dir::from_angle(inc.ang.ang + delta);
        junction_check(&inc, theta, false, tol)?;
        self.tip.ang = Some(theta);
        self.tip.ang_by_tangent = false;
        Ok(in_state(self.core, self.tip))
    }

    /// The kernel behind the table's declared-subdivision row (recording
    /// is the row's, not the kernel's).
    fn arc_continue_kernel(
        mut self,
        target: Point2<T>,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let pos = self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "arc_continue on a tip without a position",
        })?;
        let at = pos.at;
        let inc = pos.incoming.ok_or(PathError::UnderdeterminedLeg {
            site: "arc_continue on a tip without incoming data",
        })?;
        let carrier = inc.carrier.ok_or(PathError::ArcContinueNeedsArcCarrier)?;
        let band = linear_band(tol)?;
        // The target must LIE on the carrier: |target − c| − r decided
        // coincident (in-band Zero); a definite offset is contradictory
        // authored data.
        let offset = (target - carrier.center).norm_squared().sqrt() - carrier.radius;
        match decide("path_arc_continue_on_carrier", Margin::of(offset), band) {
            Ok(Sign::Zero) => {}
            Ok(_) => return Err(PathError::ArcContinueOffCarrier { offset }),
            Err(source) => return Err(PathError::Escalated { source }),
        }
        let chord_v = target - at;
        let chord = chord_v.norm_squared().sqrt();
        match decide("path_arc_chord", Margin::of(chord), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => return Err(PathError::DegenerateArcChord { chord }),
            Err(source) => return Err(PathError::Escalated { source }),
        }
        // The continuation departs ALONG the incoming tangent (same
        // carrier, same sense — that is what continuing means), so the
        // bulge is the tangent-chord relation, exactly
        // `tangent_arc_geom`'s derivation: δ = atan2(across, along),
        // b = tan(δ/2), end tangent = departure + 2δ. The travel sense
        // falls out of the signed δ — no sign is ever read or
        // classified here.
        let u = inc.ang.unit;
        let along = u.dot(chord_v);
        let across = u.perp_dot(chord_v);
        let delta = across.atan2(along);
        let bulge = (delta / T::from_f64(2.0)).tan();
        let end_ang = Dir::from_angle(inc.ang.ang + delta + delta);
        self.core.push_arc(target, bulge, carrier)?;
        let arm = carrier.radius.min(chord);
        Ok(in_state(
            self.core,
            leg_end_tip(target, end_ang, arm, Some(carrier)),
        ))
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

    /// The kernel behind the table's straight-leg row (recording is the
    /// row's, not the kernel's).
    fn line_kernel(
        mut self,
        len: T,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let (at, ang) = self.dep()?;
        let band = linear_band(tol)?;
        match decide("path_leg_length", Margin::of(len), band) {
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

    /// The kernel behind the table's corner-fillet row (recording is the
    /// row's, not the kernel's).
    fn fillet_kernel(mut self, radius: T, tol: Tol) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        let (at, ang) = self.dep()?;
        let band = linear_band(tol)?;
        match decide("path_fillet_radius", Margin::of(radius), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => return Err(PathError::NonpositiveFilletRadius { radius }),
            Err(source) => return Err(PathError::Escalated { source }),
        }
        self.core.pending = Some(verbs::Pending::Ray(verbs::PendingRay {
            origin: at,
            dir: ang,
            radius,
        }));
        self.core.pending_meta = Some(PendingMeta {
            by_tangent: self.tip.ang_by_tangent,
            origin_incoming: self.tip.pos.as_ref().and_then(|p| p.incoming),
            extends_carrier: false,
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
        tol: Tol,
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
                    let band = linear_band(tol)?;
                    match decide("path_collinear_target", Margin::of(across), band) {
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
                Some(prev) => refuse_identical_carriers(prev, &carrier, tol)?,
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
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let g = self.tangent_arc_geom(p, false, tol)?;
        self.core.push_arc(p, g.bulge, g.carrier)?;
        let arm = g.carrier.radius.min(g.chord);
        Ok(in_state(
            self.core,
            leg_end_tip(p, g.end_ang, arm, Some(g.carrier)),
        ))
    }

    fn tangent_arc_to_start(mut self, tol: Tol) -> Result<ClosedLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let start_ang = self.core.start_ang.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry direction is bound",
        })?;
        let g = self.tangent_arc_geom(start_pos, true, tol)?;
        let arm = g.carrier.radius.min(g.chord);
        junction_check(
            &Incoming {
                ang: g.end_ang,
                arm,
                carrier: Some(g.carrier),
            },
            start_ang,
            false,
            tol,
        )?;
        self.core.set_leaving(g.bulge, FirstSeg::Arc)?;
        Ok(self.core.build())
    }
}

// ------------------------------------------------------------------
// Point-state verbs (sugar tier: one call each, expands to core).
// ------------------------------------------------------------------

impl<T: Decide, F: Flavor> PartialPath<T, HasPos<F>, NoAng> {
    fn tip_pos(&self) -> Result<&PosData<T>, PathError<T>> {
        self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "point tip without a position",
        })
    }

    fn line_to_point(
        mut self,
        p: Point2<T>,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let pos = self.tip_pos()?;
        let at = pos.at;
        let d = p - at;
        let gamma = Dir::from_angle(d.y.atan2(d.x));
        if self.core.pending.is_some() {
            self.core
                .resolve_fillet(at, gamma, ArrivalKind::Continues, tol)?;
        } else {
            if let Some(inc) = &self.tip_pos()?.incoming {
                junction_check(inc, gamma, false, tol)?;
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

    fn line_to_start(mut self, tol: Tol) -> Result<ClosedLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let at = self.tip_pos()?.at;
        let d = start_pos - at;
        let gamma = Dir::from_angle(d.y.atan2(d.x));
        if self.core.pending.is_some() {
            self.core
                .resolve_fillet(at, gamma, ArrivalKind::Continues, tol)?;
        } else if let Some(inc) = &self.tip_pos()?.incoming {
            junction_check(inc, gamma, true, tol)?;
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
            tol,
        )?;
        self.core.set_leaving(T::zero(), FirstSeg::Line)?;
        Ok(self.core.build())
    }

    /// The chord length, gated definitely positive: every arc leg spans
    /// a chord, and a closed carrier is [`circle`]'s business (PQ4).
    fn arc_chord(&self, end: Point2<T>, tol: Tol) -> Result<T, PathError<T>> {
        let at = self.tip_pos()?.at;
        let band = linear_band(tol)?;
        let chord = (end - at).norm_squared().sqrt();
        match decide("path_arc_chord", Margin::of(chord), band) {
            Ok(Sign::Positive) => Ok(chord),
            Ok(_) => Err(PathError::DegenerateArcChord { chord }),
            Err(source) => Err(PathError::Escalated { source }),
        }
    }

    /// The `Via` mode's derived bulge: the collinear gate
    /// (the through-point's signed perpendicular offset from the chord
    /// LINE, meters — zero for on-chord and beyond-the-end alike, which
    /// is why one refusal covers the class), then the existing closed
    /// form on the three authored points.
    fn arc_via_bulge(&self, via: Point2<T>, end: Point2<T>, tol: Tol) -> Result<T, PathError<T>> {
        let at = self.tip_pos()?.at;
        let chord_len = self.arc_chord(end, tol)?;
        let band = linear_band(tol)?;
        let offset = (end - at).perp_dot(via - at) / chord_len;
        match decide("path_arc_via_offset", Margin::of(offset), band) {
            Ok(Sign::Zero) => return Err(PathError::ArcViaCollinear { offset }),
            Ok(_) => {}
            Err(source) => return Err(PathError::Escalated { source }),
        }
        Ok(bulge_from_via(at, via, end))
    }

    /// The `Center` mode's derived bulge: both radii
    /// gated definitely positive, then equidistance gated definitely
    /// ZERO (a definite mismatch refuses; an undecidable one escalates —
    /// neither is repaired), then the existing closed form.
    fn arc_center_bulge(
        &self,
        center: Point2<T>,
        end: Point2<T>,
        winding: ArcSweep,
        tol: Tol,
    ) -> Result<T, PathError<T>> {
        let at = self.tip_pos()?.at;
        let band = linear_band(tol)?;
        let r_tip = (at - center).norm_squared().sqrt();
        let r_end = (end - center).norm_squared().sqrt();
        for radius in [r_tip, r_end] {
            match decide("path_arc_center_radius", Margin::of(radius), band) {
                Ok(Sign::Positive) => {}
                Ok(_) => return Err(PathError::DegenerateArcCenter { radius }),
                Err(source) => return Err(PathError::Escalated { source }),
            }
        }
        match decide(
            "path_arc_center_equidistant",
            Margin::of(r_tip - r_end),
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
        self.arc_chord(end, tol)?;
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
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        if self.core.pending.is_some() {
            return Err(PathError::ArcLegOnOpenFillet {
                site: "an arc arrival is authored WITH the fillet that trims it — \
                       fillet_arc(r, spec) / arc_fillet_arc(spec, r, spec2) — not by an arc \
                       LEG from an already-positioned arrival point",
            });
        }
        let pos = self.tip_pos()?;
        let at = pos.at;
        let (start_t, end_t, chord) = Self::arc_angles(at, p, bulge);
        if let Some(inc) = &pos.incoming {
            junction_check(inc, start_t, false, tol)?;
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

    fn arc_to_start(mut self, bulge: T, tol: Tol) -> Result<ClosedLoop<T>, PathError<T>> {
        if self.core.pending.is_some() {
            return Err(PathError::ArcLegOnOpenFillet {
                site: "an arc arrival that CLOSES is authored with the fillet that trims it — \
                       fillet_arc(r, Center { c, winding, p: Start }) — not by an arc LEG from \
                       an already-positioned arrival point",
            });
        }
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let pos = self.tip_pos()?;
        let at = pos.at;
        let (start_t, end_t, chord) = Self::arc_angles(at, start_pos, bulge);
        if let Some(inc) = &pos.incoming {
            junction_check(inc, start_t, false, tol)?;
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
            tol,
        )?;
        self.core.set_leaving(bulge, FirstSeg::Arc)?;
        Ok(self.core.build())
    }
}

impl<T: Decide> PartialPath<T, NoPos, HasAng> {
    /// The kernel behind the table's far-end-anchor row (recording is
    /// the row's, not the kernel's).
    fn end_side_at(
        mut self,
        anchor: Point2<T>,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let dir = self.tip.ang.ok_or(PathError::UnderdeterminedLeg {
            site: "far-end anchor on a tip without a bound direction",
        })?;
        if self.core.pending.is_none() {
            return Err(PathError::FarEndAnchorWithoutFillet);
        }
        let (arc, fit_out) = self
            .core
            .resolve_fillet(anchor, dir, ArrivalKind::EndsAtAnchor, tol)?;
        if fit_out == Sign::Positive {
            let head = self.core.head()?;
            let arm = (anchor - head).norm_squared().sqrt();
            self.core.push_line(anchor)?;
            Ok(in_state(self.core, leg_end_tip(anchor, dir, arm, None)))
        } else {
            // Exact fit: the trim reached the anchor, so the arc IS the
            // whole side. Two consequences, both handled rather than
            // inherited. (1) No straight piece is emitted — a
            // zero-length segment is the degeneracy the fit gate exists
            // to avoid. (2) The arc's outgoing joint is NOT declared
            // (`ArrivalKind::EndsAtAnchor` suppressed it): the side ends
            // here, so the next direction is free and a declaration
            // would claim a tangency nobody constructed. (3) The side's
            // last vertex is the tangent point `t2`, which the fit gate
            // has just classified as coincident with `anchor` — the
            // authored anchor is ABSORBED into it rather than emitted
            // twice, exactly as the hand door absorbs its `next`.
            let head = self.core.head()?;
            Ok(in_state(
                self.core,
                leg_end_tip(head, dir, arc.radius, Some(arc)),
            ))
        }
    }
}

impl<T: Decide> PartialPath<T, NoPos, NoAng> {
    /// The kernel behind the table's seam-close row (recording is the
    /// row's, not the kernel's).
    fn close_at_seam(mut self, tol: Tol) -> Result<ClosedLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let start_ang = self.core.start_ang.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry direction is bound",
        })?;
        self.core
            .resolve_fillet(start_pos, start_ang, ArrivalKind::Seam, tol)?;
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
    fn line_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self, tol: Tol) -> Self::Out;
}

impl<T: Decide, F: Flavor> LineTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn line_from(mut path: PartialPath<T, HasPos<F>, NoAng>, target: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::LineTo(Target::Point(target)));
        path.line_to_point(target, tol)
    }
}

impl<T: Decide, F: Flavor> LineTarget<T, F> for Start {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn line_from(mut path: PartialPath<T, HasPos<F>, NoAng>, _target: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::LineTo(Target::Start));
        path.line_to_start(tol)
    }
}

/// A [`PartialPath::tangent_arc_to`] target: an authored absolute
/// point, or [`Start`] (the tangent-seam close). Sealed.
pub trait TangentArcTarget<T: Decide, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn tangent_arc_from(path: PartialPath<T, HasPos<F>, HasAng>, target: Self, tol: Tol) -> Self::Out;
}

impl<T: Decide, F: Flavor> TangentArcTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn tangent_arc_from(mut path: PartialPath<T, HasPos<F>, HasAng>, target: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::TangentArcTo(Target::Point(target)));
        path.tangent_arc_to_point(target, tol)
    }
}

impl<T: Decide, F: Flavor> TangentArcTarget<T, F> for Start {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn tangent_arc_from(mut path: PartialPath<T, HasPos<F>, HasAng>, _target: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::TangentArcTo(Target::Start));
        path.tangent_arc_to_start(tol)
    }
}
