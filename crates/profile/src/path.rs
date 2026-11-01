//! The PATHS authoring algebra (LIB-U2 PR-1): typed profile-loop
//! authoring in which **accidental tangency is unrepresentable,
//! intended tangency is exact by construction, and every authored
//! point lies on the final path, authored once** — the ratified design
//! of `docs/PATHS-DESIGN.md` §§1–7, lowered to the existing v1 form
//! ([`ProfileLoop`]: vertices and segments), carried with the joints it
//! constructed tangent as a [`ConstructedLoop`].
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
//! - **Directed** = `PartialPath<T, HasPos<F>, HasAng>` — the state
//!   [`fillet`](PartialPath::fillet) and every DIRECTED leg consume.
//!   One leg form does not need it: `line(len)` also runs off a
//!   DIRECTED POINT, departing along that point's own tangent (the
//!   straight continuation — no authored direction, no junction).
//!
//! The OUTGOING angle is a binding slot, set at most once per side
//! (a second director on a Directed tip is ill-typed); the INCOMING
//! direction is never a slot — it is intrinsic data on a leg end,
//! consultable by [`tangent`](PartialPath::tangent) /
//! [`turn`](PartialPath::turn), the junction check, the straight
//! continuation and `lands_on_ray` (the declared continuation's on-ray
//! reading, PATHS-DESIGN §2c), settable by nothing.
//!
//! # Closure
//!
//! [`Start`] is a first-class directed-point value — the bound entry.
//! Using it is closing, structurally: `line_to(Start)`,
//! `arc_to(Bulge { p: Start, b })`, `.tangent().tangent_arc_to(Start)`,
//! `arc_onto(Start).toward(dx, dy)`, and the seam fillet
//! `.angle(θ).fillet(r).to(Start)`. There is deliberately no `close()`
//! alias. The entry authors the first side; the seam is authored once,
//! at the back, by the verb that targets `Start` — a leading
//! `.fillet`/`.tangent()` is ill-typed (they need bits the entry Open
//! lacks).
//!
//! The seam's own junction is the one declaration that cannot ride a
//! departing leg, because the arriving leg is authored LAST. It rides
//! the TARGET, and it classifies the JOINT rather than the leg:
//! [`Start::arrives_tangent`] declares that the seam's joint is a
//! TANGENT joint, and EVERY closing verb takes it. It is CHECKED where
//! the arriving leg's data fix its end direction, and SUPPLIES that
//! direction where they leave it free (`arc_onto`, the `Via` arrival).
//! The check reads
//! the arriving direction and `Start`'s own direction and nothing else
//! — never whether the two carriers are the same, which is the ruling
//! of 2026-09-02: every zero-turn joint is a tangent joint (D1).
//! Not constructed, a tangent seam refuses [`PathError::SeamTangent`]
//! from every closing verb.
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
//! bit-identical fillet geometry. Nothing is trusted because the
//! algebra produced it: the lowered loop's shape and joints are
//! validated as a table's are ([`crate::ConstructedProfile`]), and its
//! arcs' consistency, which validation does not decide again for a
//! constructed loop, holds over the reals by its construction and is
//! checked at the scalar it was built in, by an exact scalar's
//! registered identities or by this door reading a point scalar's
//! stored arc back against what was authored before it stores it.
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
//! Compile-time, from the lattice: double director; `fillet` and the
//! DIRECTED legs from non-Directed tips (`line(len)` is the exception
//! that proves the slot: it also has a directed-POINT row, the
//! straight continuation); `.tangent()` on a plain point; leading
//! `.fillet`/`.tangent()`; use after close (closing verbs consume the
//! path and return the loop). Typed runtime errors, from geometry —
//! the lattice guarantees the authoring, never the geometry: see
//! [`PathError`]. Never a panic (evaluation code is total; every
//! decision goes through the reified-predicate funnel).
//!
//! The run-global tolerance (D4 ¶1's one ε per run) supplies ε_input
//! for every junction classification, reached through the [`Tol`]
//! witness the caller passes in — the ratified surface has no per-call
//! tolerance VALUE slot, and the witness is not one: it names the
//! dependence without being able to carry a different ε.
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
//! v1 lowering target** (a [`ProfileLoop`] is a chain of vertices and
//! line or circular-arc segments;
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
//! use geom_core::{Point2, Tol};
//! use profile::{ConstructedProfile, Open, SketchPlane, Start};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let tol = Tol::witness();
//! let (east, north) = (0.0_f64, std::f64::consts::FRAC_PI_2);
//! let (west, south) = (std::f64::consts::PI, -north);
//! let r = 0.25;
//! let square = Open.at(Point2::new(0.0, -1.0)).angle(east, tol)?
//!     .fillet(r, tol)?.at(Point2::new(1.0, 0.0), tol)?.angle(north, tol)?
//!     .fillet(r, tol)?.at(Point2::new(0.0, 1.0), tol)?.angle(west, tol)?
//!     .fillet(r, tol)?.at(Point2::new(-1.0, 0.0), tol)?.angle(south, tol)?
//!     .fillet(r, tol)?.to(Start, tol)?;
//! assert_eq!(square.loop_.vertices().len(), 8);
//! assert_eq!(square.loop_.constructed_joints().len(), 8);
//! // The chain also RECORDED itself: the program replays to the same
//! // loop, bit for bit (profiles-as-programs v2 — see [`program`]).
//! assert_eq!(square.program.len(), 13);
//! ConstructedProfile::new(SketchPlane::xy(), vec![square.loop_]).validate(tol)?;
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
//! use geom_core::Tol;
//! use geom_core::Point2;
//! use profile::{ArcSweep, Center, Open};
//! let tol = Tol::witness();
//! let sharp = Open.at(Point2::new(5.05, -1.6_f64)).toward(2.1, 0.8, tol).unwrap()
//!     .fillet_arc(0.5, Center {
//!         c: Point2::new(7.0, 0.0),
//!         winding: ArcSweep::Ccw,
//!         p: Point2::new(8.5, 0.0),
//!     }, tol)
//!     .unwrap()
//!     // The arrival tip's tangent is +y; 2.6 rad is a genuine corner.
//!     .angle(2.6, tol).unwrap()
//!     .line(0.5, tol);
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

use core::marker::PhantomData;

use geom_core::k_stats::decide;
use geom_core::tolerance::DEFAULT_EPS;
use geom_core::{
    Arc2, Band, Decide, Indeterminate, KERNEL_DEFECT_ENDING, Margin, Point2, Real, Sign, Tol, Vec2,
    is_finite_length,
};

use crate::path::program::{ClosedLoop, ConstructedLoop, Step, Target};
use crate::seg;
use crate::sugar::{ArcSweep, LineFilletTrims, TrimRefusal, line_line_fillet_trims};
use crate::validate::{
    ARC_SCENE_RESOLUTION_RECOURSE, FILLET_FLATTENED_RECOURSE, FILLET_SCENE_RESOLUTION_RECOURSE,
    FILLET_STORED_FORM_INBAND_RECOURSE, FilletLeg, FilletLegCarrier, JUNCTION_TANGENT_LEVER,
    NoCornerReason, fillet_recourse_for,
};
use crate::{BuiltArc, ProfileLoop, Segment};

/// The arc-carrier fillet boundary — the algebra's derived-corner
/// resolution and the lifted S8 ladder (LIB-G2 §3b). It is a separate
/// module because it is the ONE place in `path` that reads a bracket:
/// see its docs for the ratified justification, and `path.rs` itself
/// stays `Bounds`-free.
pub(crate) mod arc_fillet;
mod family;
mod onto;
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
pub use onto::{ArcOnto, ArcOntoStart, OntoTarget};
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

impl Start {
    /// The entry, targeted with a declaration that the seam's joint is
    /// a **TANGENT joint** — the ONE arrival declaration there is.
    ///
    /// The declaration is the target, because the seam is the one
    /// junction whose arriving leg is the later-authored one — every
    /// declaration that rides the DEPARTING leg elsewhere
    /// (`line(len)`/[`continue_to`](PartialPath::continue_to),
    /// [`tangent`](PartialPath::tangent)) has no departing leg to ride
    /// here: the entry's first side is already authored and cannot
    /// carry the seam's content from the front (§2's entry rule).
    ///
    /// **Every zero-turn joint is a tangent joint** (Ev, in-chat,
    /// 2026-09-02; D1), so there is nothing else to construct and no
    /// sibling token. Every closing verb takes it — `line_to`,
    /// `continue_to`, `tangent_arc_to`, `arc_onto`, `arc_to` in every
    /// endpoint-full mode, and the closing `Center` and `Via` arrivals —
    /// because what it classifies is the JOINT, not the shape of the
    /// leg reaching it. The lowered loop constructs joint 0, which
    /// validation verifies.
    ///
    /// Where the leg's data fix its end direction, the kernel CHECKS
    /// the arriving direction against `Start`'s own direction, banded
    /// through the funnel and levered by the arriving leg's arm, and
    /// asks NOTHING about the carriers: whether the two sides ride one
    /// carrier or two, a declared zero-turn joint is a tangent joint. A
    /// seam past the band refuses [`PathError::SeamArrivalOffDirection`];
    /// undeclared, it keeps refusing [`PathError::SeamTangent`]. Where
    /// they leave it free (`arc_onto`, the `Via` arrival), the token
    /// SUPPLIES `Start`'s direction and the close completes at the verb.
    #[must_use]
    pub fn arrives_tangent(self) -> ArrivesTangent {
        ArrivesTangent
    }
}

/// [`Start`] with the seam's tangent joint declared — the ONE arrival
/// declaration, and a target of every closing verb. A straight leg
/// declares it exactly as a closing arc does, because what the token
/// classifies is the JOINT and not the leg. Built by
/// [`Start::arrives_tangent`].
#[derive(Clone, Copy, Debug)]
pub struct ArrivesTangent;

impl sealed::Sealed for ArrivesTangent {}

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
    /// **G2**: the two carriers do not meet at all — a ray that misses
    /// its circle, or circles that are disjoint, concentric, or one
    /// inside the other. Distinct from
    /// [`CarriersParallel`](Self::CarriersParallel), which is the
    /// tangency knife edge: there they touch, and there is still no
    /// corner to cut.
    CarriersDoNotMeet,
}

/// Which of the two anchor windows a derived corner falls outside.
///
/// The gates are stated on the sides, not on the pair: the incoming
/// side must ADVANCE to the corner and the arrival side must REACH
/// back to it, so a corner outside either window is a fact about that
/// corner and rides [`CornerReason::OutsideAnchors`] with the corner
/// point beside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CornerWindow {
    /// The corner lies behind the incoming ray's start (or at it, at
    /// tolerance): it is not ahead of the side being authored.
    BehindIncomingRay,
    /// The corner does not lie behind the arrival side's anchor: it
    /// sits on the wrong side of (or at) the anchor, so the arrival
    /// ray never came from it.
    BehindArrivalAnchor,
}

/// Why ONE derived corner of a carrier pair takes no fillet of the
/// requested radius.
///
/// Each arm is a statement about the corner [`CornerRefusal::at`]
/// names — the deixis is "this corner", and it is true, because the
/// point rides beside the reason.
#[derive(Clone, Debug)]
pub enum CornerReason<T: Real> {
    /// The corner's own gates: it falls outside one of the two anchor
    /// windows.
    OutsideAnchors(CornerWindow),
    /// A corner exists here, but the ratified S2 construction finds no
    /// tangent circle of the requested radius at it. The constructor
    /// door's own vocabulary is carried through rather than flattened,
    /// so "the radius is too large for this corner" and "every tangent
    /// circle touches a leg past the corner" stay distinguishable at
    /// the algebra door too.
    NoTangentCircle(NoCornerReason),
    /// A fillet trim at this corner would eat a side's anchoring
    /// on-path point (the authored anchor, the incoming ray's origin,
    /// or — under a seam fillet — the entry point): the #101
    /// `TangentJointOutOfRange` fit-gating generalized (PATHS-DESIGN
    /// §3). This is also where a too-large radius lands: the setback
    /// exceeds the extent the anchor pins.
    ///
    /// **Whose numbers.** At an arc-carrier corner the offset carriers
    /// can admit two candidate circles that both round the corner and
    /// both overrun a leg; the payload is then the candidate nearest to
    /// fitting IN THE SETBACK METRIC (`fillet_select::nearest_candidate`
    /// over the candidates' setback pairs), on that candidate's worse
    /// leg — the leg whose setback outruns its extent by more, ties to
    /// the incoming leg. `setback − available` is that leg's overrun, a
    /// setback-currency number: it is NOT the radius reduction that
    /// would make the corner fit (a setback does not scale 1:1 with
    /// the radius on either leg kind), and the sentence's recourse is
    /// deliberately un-metered.
    AnchorOutsideTrimmedExtent {
        /// Which side's anchor the trim would eat: the reported
        /// candidate's worse leg.
        side: FilletLeg,
        /// **G2 §3c**: that side's CARRIER KIND, carrying the angular
        /// margin `(extent − setback)/R` for a circular side. A bare
        /// linear setback says nothing on a circle, so the payload is
        /// metered in the carrier's own currency (M5 S2's
        /// does-not-fit diagnostic shape, at the algebra door).
        carrier: FilletLegCarrier,
        /// The reported candidate's tangent setback from the corner on
        /// that side, meters (diagnostic; an arc length `R·Δθ` on a
        /// circular side).
        setback: T,
        /// The anchored extent available to the trim, meters (same
        /// currency as `setback`).
        available: T,
    },
    /// The requested radius demands the **enclosing** tangency at this
    /// corner: on the named side the signed offset radius ρ = R − σ·τ·r
    /// is negative (σ·τ = +1 with r > R), so every circle of that radius
    /// tangent to that side's carrier with this corner's turn sense
    /// contains the carrier whole — and the corner with it, the corner
    /// being a point of that carrier. An arc that cannot touch the
    /// corner is not a fillet OF that corner, so no fillet of this
    /// corner exists at this radius, and none ever will: the class is
    /// permanently out of reach by design
    /// (`crates/profile/README.md`), which is why it stays
    /// distinguishable from the "no tangent circle" reasons — those
    /// would send the author looking for a corner that is right there.
    /// The bound is the named side's carrier radius; the recourse is a
    /// smaller radius.
    EnclosesLegCarrier {
        /// The side whose carrier the radius would swallow, or `None`
        /// when it swallows both — the ordinary case, since a swallowed
        /// carrier forces its partner to be swallowed too unless the
        /// corner is degenerate.
        side: Option<FilletLeg>,
        /// The tightest CLASS bound, meters: the smallest swallowed
        /// carrier radius. Necessary, never sufficient — see
        /// `largest_tangent_radius`.
        carrier_radius: T,
        /// The matching signed offset radius ρ = R − σ·τ·r, meters
        /// (negative).
        offset_radius: T,
        /// The EXISTENCE bound, meters, when the corner's two circular
        /// carriers define one: the largest radius that can be tangent
        /// to both of them here, (R₁ + R₂ − d)/2. This is the quantity
        /// the message endorses, because it is the one below which a
        /// tangent circle actually exists; the class bound alone would
        /// send an author to radii that refuse again for a different
        /// reason. `None` on the degenerate corners where the quantity
        /// is not defined at the gate (a straight partner, or a partner
        /// whose own ρ is positive), and there the message endorses no
        /// number.
        largest_tangent_radius: Option<T>,
    },
}

/// One derived corner of a carrier pair, and why no fillet of the
/// requested radius rounds it.
#[derive(Clone, Debug)]
pub struct CornerRefusal<T: Real> {
    /// The derived corner itself, in the sketch plane's chart.
    pub at: Point2<T>,
    /// Why this corner refused.
    pub reason: CornerReason<T>,
}

impl<T: Real> core::fmt::Display for CornerRefusal<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // A derived ordinate that lands on negative zero renders as
        // "-0" here — "at corner (0, -0)" on a corner on the
        // x axis. That is issue 1282's class (the Display float
        // rendering, which `num` owns) and not this site's to repair:
        // the sign is the arithmetic's, the payload keeps it exactly,
        // and a repair belongs at `num` where every arm gets it.
        write!(
            f,
            "at corner ({x}, {y}): {reason}",
            x = num(&self.at.x),
            y = num(&self.at.y),
            reason = self.reason
        )
    }
}

impl<T: Real> core::fmt::Display for CornerReason<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutsideAnchors(window) => match window {
                CornerWindow::BehindIncomingRay => {
                    write!(f, "this corner lies behind the incoming ray's start")
                }
                CornerWindow::BehindArrivalAnchor => write!(
                    f,
                    "this corner does not lie behind the arrival side's anchor"
                ),
            },
            Self::NoTangentCircle(reason) => match reason {
                NoCornerReason::OffsetCarriersDisjoint => write!(
                    f,
                    "no circle of that radius touches both carriers here — the radius is \
                     too large"
                ),
                NoCornerReason::NoCornerSideCandidate => write!(
                    f,
                    "every tangent circle of that radius touches a side past this corner"
                ),
            },
            // The carrier rides the payload; the sentence names the
            // anchor and the two lengths that decide it. The recourse is
            // the pair's, stated once after every corner
            // ([`CornerReason::recourse`]).
            Self::AnchorOutsideTrimmedExtent {
                side,
                setback,
                available,
                ..
            } => write!(
                f,
                "the fillet trim would eat the {side}'s anchor (setback {setback} m, \
                 {available} m available)",
                setback = num(setback),
                available = num(available)
            ),
            // The offset radius rho that decides the swallow rides the
            // payload; the sentence names the carrier's own radius.
            Self::EnclosesLegCarrier {
                side,
                carrier_radius,
                largest_tangent_radius,
                ..
            } => {
                let whose = match side {
                    Some(side) => &format!("the {side}'s carrier"),
                    None => "both legs' carriers",
                };
                write!(
                    f,
                    "it would SWALLOW {whose} (radius {carrier_radius} m)",
                    carrier_radius = num(carrier_radius),
                )?;
                match largest_tangent_radius {
                    // The endorsable number: a circle of this radius IS
                    // tangent to both carriers at the corner. Anchored
                    // extents can still require less, and refuse in their
                    // own words when they do.
                    Some(bound) => write!(
                        f,
                        "; the largest circle tangent to both has radius {bound} m",
                        bound = num(bound)
                    ),
                    // Nothing endorsable at this site: the class bound is
                    // necessary and not sufficient, so the sentence states
                    // what it rules OUT — every radius from the carrier's
                    // own up swallows it — and names no radius below it,
                    // which would be a promise this gate cannot keep; the
                    // pair's sentence adds that none may fit at all.
                    None => write!(
                        f,
                        "; no radius from {carrier_radius} m up fits",
                        carrier_radius = num(carrier_radius)
                    ),
                }
            }
        }
    }
}

impl<T: Real> CornerReason<T> {
    /// What to change about a request this corner refused, or `None`
    /// where the corner names no lever (it lies outside the anchors'
    /// window, which no radius moves). A carrier pair's refusal lists
    /// every corner and states ONE recourse after them all: the widest
    /// of its corners', so a list of two reads as one message with one
    /// recourse rather than two.
    #[must_use]
    pub fn recourse(&self) -> Option<&'static str> {
        match self {
            Self::AnchorOutsideTrimmedExtent { .. } => Some("reduce the radius or move the anchor"),
            Self::EnclosesLegCarrier { .. }
            | Self::NoTangentCircle(NoCornerReason::OffsetCarriersDisjoint) => {
                Some("reduce the radius")
            }
            Self::NoTangentCircle(NoCornerReason::NoCornerSideCandidate)
            | Self::OutsideAnchors(_) => None,
        }
    }
}

/// Typed refusals of the authoring algebra — geometry the lattice
/// cannot rule out, refused loudly (PATHS-DESIGN §3 "Refusals" and §4).
/// The verify layer's own errors ([`crate::ProfileError`]) still apply to the
/// lowered loop at [`crate::Profile::validate`], unchanged.
#[derive(Clone, Debug)]
pub enum PathError<T: Real> {
    /// §4 item 1: the AUTHORED departure is within ε_input of the
    /// incoming TANGENT direction — one refusal, one recourse, for any
    /// sub-ε_input margin: if the tangency is intended onto a new
    /// carrier, author it structurally (`.tangent()` —
    /// `.tangent().tangent_arc_to(p)` for an arc — or the tangent-arc /
    /// seam-fillet close at the seam), which makes it
    /// exact by construction; if a straight continuation of the same
    /// line is intended, spell it `line(len)` off the directed point,
    /// where no junction exists to classify; otherwise move the
    /// geometry (or lower the tolerance). The margin rides along as
    /// data; the message never forks on exactly-on vs in-band.
    JunctionTangent {
        /// The classified turn margin sin φ · arm, meters (scalar-typed
        /// payload — data, not a decision).
        margin: T,
        /// The lever arm (the incoming leg's extent, capped by its
        /// carrier radius), meters.
        arm: T,
    },
    /// §4 item 1, reverse class: the departure is within ε_input of
    /// the REVERSE of the incoming tangent — a cusp. One refusal, one
    /// recourse, and it is now the same SHAPE as the tangent class's:
    /// if the cusp is intended, author it structurally with
    /// `.cusp()`, which reverses the incoming ray exactly and DECLARES
    /// the joint; otherwise move the geometry. The declaration is where
    /// a cusp's intent lives — D1 declares it where the tangency is
    /// created and never infers it from a margin — which is why an
    /// authored near-reverse still refuses here.
    JunctionCusp {
        /// The classified turn margin sin φ · arm, meters.
        margin: T,
        /// The lever arm, meters.
        arm: T,
    },
    /// **The SEAM's own junction is tangent and nothing declared it**:
    /// the arriving direction is within ε_input of the entry's outgoing
    /// direction. Start-only, because only a closing verb classifies
    /// the seam at all, and raised by EVERY closing verb — the two arc
    /// closers included, since a seam arrival has a recourse no
    /// departure has.
    ///
    /// PQ4 (§6) no longer refuses the seam outright: since the
    /// fifth-round ruling a DECLARED subdivision point and a DECLARED
    /// G1 joint are both admissible seams, and what this variant names
    /// is the UNDECLARED case — the one the ladder still refuses,
    /// because reading "the author meant a subdivision" off two
    /// directions that happen to agree is the inference nothing here
    /// makes.
    ///
    /// The recourse is to DECLARE the joint, or to move the seam. The
    /// arriving leg is the later-authored one, so the declaration rides
    /// the target and says which JOINT this is:
    /// [`Start::arrives_tangent`], legal on any closing verb and
    /// CHECKED, not inferred.
    /// Undeclared, the seam stays refused however the closing leg is
    /// spelled — `continue_to(Start)` does not reach it either, because
    /// the junction in band is the entry's, not the closer's.
    ///
    /// **This variant used to carry a `site` payload**, because a
    /// closing verb classifies two junctions and the older lattice
    /// refused both under one name. The departure half is gone: a
    /// tangent DEPARTURE on a closing leg is geometrically identical to
    /// one mid-chain, and since the declared closer landed the recourse
    /// is identical too (spell it structurally), so it now refuses
    /// [`JunctionTangent`](Self::JunctionTangent) exactly as any other
    /// departure does. What is left is the case that genuinely IS
    /// special, and the name says which.
    SeamTangent {
        /// The offending collinearity/turn margin, meters.
        margin: T,
    },
    /// **The declared seam arrival's consistency refusal**: the closing
    /// leg targeted [`Start::arrives_tangent`] — declaring that the
    /// seam's joint is a tangent one, so that the arriving direction
    /// continues the entry's outgoing direction — and it definitely
    /// does not.
    ///
    /// Authored data contradicting itself, the arc verbs' consistency
    /// class and the exact mirror of
    /// [`ContinuationTargetOffRay`](Self::ContinuationTargetOffRay):
    /// there the intent is declared and the TARGET checked, here the
    /// intent is declared and the arriving DIRECTION checked. Not the
    /// value inference the ladder refuses — nothing reads intent off a
    /// coincidence, because the intent is what the target said. So the
    /// comparison is banded: an arrival the funnel cannot call
    /// continuous refuses here, one it cannot decide escalates, and one
    /// it calls continuous closes the loop.
    ///
    /// The datum is an ANGLE — the sine of the turn between the
    /// arriving direction and the entry's outgoing one — which means
    /// nothing until an arm says what it displaces, so it is LEVERED by
    /// the arriving leg's own arm (§4 item 1's precedent, and the
    /// mirror of the point-target check's decision to lever NOTHING:
    /// there the datum was already a length). `margin` is that product,
    /// in meters: the lateral distance the misalignment opens over the
    /// arriving leg. That is the point deviation the tolerance is
    /// defined about, so the threshold does not drift with leg length —
    /// the same physical miss at the seam gets the same verdict however
    /// long the closing leg is.
    ///
    /// An arrival that is REVERSED rather than merely off — the leg
    /// arriving anti-parallel to the entry's outgoing direction — is a
    /// cusp, and refuses [`JunctionCusp`](Self::JunctionCusp) as it
    /// does at any other junction: one fact, one refusal.
    SeamArrivalOffDirection {
        /// The declared arrival's miss, LEVERED to meters: the turn's
        /// sine times the arriving leg's arm.
        margin: T,
        /// The lever: the arriving leg's arm (its own length for a
        /// straight closer, `radius.min(chord)` for an arc one).
        arm: T,
    },
    /// **A declared seam arrival with no lever to measure it against.**
    /// The arriving leg's arm — its own length, or `radius.min(chord)`
    /// for an arc — is not definitely positive, so the levered turn and
    /// the levered alignment both read Zero and ANY arriving direction
    /// satisfies the declaration inside the band.
    ///
    /// [`junction_check`] meets the same degeneracy and refuses it (its
    /// side decision reading Zero is exactly "the arm itself is
    /// degenerate"); the declared twin owes the same refusal, because a
    /// declaration cannot rescue a junction nothing can measure. The
    /// conditioning shape is
    /// [`FilletOffsetLeverTooShort`](Self::FilletOffsetLeverTooShort)'s
    /// — a DERIVED lever too short to carry the question — rather than
    /// [`NonpositiveLeg`](Self::NonpositiveLeg)'s, which is about an
    /// extent the author wrote.
    SeamArrivalLeverTooShort {
        /// The degenerate lever, meters.
        arm: T,
    },
    /// **The declared point-target continuation's consistency
    /// refusal**: [`continue_to(target)`](PartialPath::continue_to)
    /// declares the leg to be the straight continuation of the run
    /// LANDING on `target`, and the target does not lie on the run's
    /// line: the line bound where the run's direction was bound, which
    /// the departing point's `lands_on_ray` reads (PATHS-DESIGN §2c).
    ///
    /// This is authored data contradicting itself — the arc verbs'
    /// consistency class ([`ArcCenterNotEquidistant`](Self::ArcCenterNotEquidistant)
    /// is the same shape) — and NOT the value inference the ladder
    /// refuses: nothing here reads intent off a coincidence, because
    /// the intent is what the verb said. So the comparison is banded,
    /// as every comparison in this kernel is; a target the funnel
    /// cannot call coincident with the ray refuses here, and one it
    /// cannot decide escalates.
    ///
    /// The miss is metered as the target's own LATERAL displacement
    /// from the ray, in meters — the distance the authored point would
    /// have to move to be on it. No lever converts it: the datum is a
    /// point, so the deviation it implies is the point deviation
    /// itself. (§4 item 1's turn margin needs a lever because ITS
    /// datum is an angle, which means nothing until an arm gives it a
    /// length.)
    ContinuationTargetOffRay {
        /// The lateral miss (û ⟂ component of `target − at`), meters —
        /// the classified margin, signed to the ray's left.
        across: T,
        /// How far along the ray the target's foot lies, meters — the
        /// leg length the declaration asks for. Data, not a decision.
        along: T,
    },
    /// The fillet's virtual corner does not exist (see
    /// [`PathNoCornerReason`]).
    NoCornerForFillet {
        /// Which structural condition failed.
        reason: PathNoCornerReason,
        /// The requested radius, meters (diagnostic).
        radius: T,
    },
    /// **No corner of the carrier pair takes a fillet of the requested
    /// radius**: every corner that refused at the stage the answer
    /// comes from, each with its own reason and its own point.
    ///
    /// One envelope for the whole pair, because a refusal about a pair
    /// is about the corners of it. Where the two crossings refuse for
    /// different reasons — the common case — both sentences reach the
    /// author, and the deixis of each is "this corner", which is true
    /// because [`CornerRefusal::at`] is beside it.
    ///
    /// **The stage** is one of the arc-carrier resolve's two channels:
    /// a corner that passed the anchor windows and then failed to admit
    /// a tangent circle is the real answer and answers alone, so the
    /// windows' entries appear only when NO corner reached the
    /// construction. The two lists are never merged because
    /// `docs/FILLET-ATTR-SPEC.md`'s acceptance row asks for a ONE-entry
    /// envelope where only one crossing sits in the windows: a corner
    /// the author did not bracket, listed beside the answer about the
    /// one they did, is noise rather than attribution. So the entry
    /// list is NOT every derived corner — a pair derives up to two, and
    /// on most refusals only one of them is an entry.
    ///
    /// The refusals that name no corner at all outrank this envelope
    /// and are reported instead of it: the pair-level conditions
    /// ([`PathNoCornerReason`]) as
    /// [`NoCornerForFillet`](Self::NoCornerForFillet), and the M8
    /// conditioning gate as
    /// [`FilletOffsetLeverTooShort`](Self::FilletOffsetLeverTooShort).
    /// A fact about the pair answers before a fact about one of its
    /// corners does. The straight carrier pair derives one corner and
    /// so carries a one-entry envelope of exactly this shape.
    ///
    /// **Order is presentation, not truth**: entries are sorted by the
    /// sum of the distances from the corner to the two bracketing
    /// anchors, ascending, ties in enumeration order — the first
    /// sentence is the corner the author most plausibly meant. Nothing
    /// in the kernel branches on the order, and no entry outranks
    /// another.
    NoCornerOfPair {
        /// The requested radius, meters (diagnostic).
        radius: T,
        /// The corners that REFUSED at the answering stage, nearest the
        /// bracketing anchors first. Never empty, and not necessarily
        /// every corner the pair derives.
        corners: Vec<CornerRefusal<T>>,
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
    // Deliberately NOT merged with `FilletOffsetLeverTooShort`, whose
    // payload it nearly parrots (a side, a carrier radius, a signed ρ, a
    // bound): the two answer different questions and offer different
    // recourses. That one says a corner and a tangent circle both exist
    // and this ε cannot place the tangent point — move the radius EITHER
    // way, or lower ε, and it is a conditioning fact about the run. This
    // one says no such circle exists at all, at any ε, forever — move the
    // radius DOWN, past a bound the geometry fixes. One variant carrying
    // both would have to render one sentence for two situations, which is
    // exactly what D4 ¶1's addendum forbids.
    /// **The fillet arc is too shallow to be STORED as an arc.** A
    /// profile holds an arc as its chord and a carrier, and the loop's
    /// reader classifies that pair back through the same predicates
    /// validation runs: a fillet whose sagitta `r(1 − cos(θ/2))` sits
    /// at or below the run's ε is read as a *line*, and the carrier the
    /// door computed is then absent from what the profile holds — the
    /// tangency the fillet declares has nothing left to be about.
    ///
    /// So the door asks the reader's own question of the loop it is
    /// about to emit — `seg::build_seg` on the fillet arc and
    /// `seg::joint_tangency` on each joint the fillet declares — and
    /// refuses here rather than minting a declaration validation would
    /// contradict. `predicate` and `margin` are that classification's,
    /// not a second expression for it.
    ///
    /// **Its sibling is
    /// [`FilletCarrierBelowSceneResolution`](Self::FilletCarrierBelowSceneResolution),
    /// and the two are deliberately NOT one variant.** They are the two
    /// ways a stored fillet loses its carrier, they are told apart by a
    /// fact the door has in hand (whether the stored segment reads as an
    /// arc), and their levers differ: a larger radius fixes this one,
    /// while that one's lever is the scene's magnitude, which a radius
    /// raises only once it outgrows the geometry's distance from the
    /// origin. D4 ¶1's addendum
    /// asks one message and one recourse per user situation, and
    /// `PathErrorKind` is the branchable discriminant a caller gets —
    /// merging them would put the only distinction into prose a caller
    /// would have to parse.
    FilletArcFlattenedInStorage {
        /// The fillet arc's stored sweep, radians (diagnostic).
        turn: T,
        /// The requested fillet radius, meters (diagnostic).
        radius: T,
        /// The arc's length, `radius · |turn|` meters (diagnostic).
        arc_length: T,
        /// The predicate whose classification refused the stored form.
        predicate: &'static str,
        /// The margin it classified, meters.
        margin: T,
    },
    /// **The fillet's stored arc IS an arc, and the scene cannot
    /// resolve its carrier.** The sagitta is metres; what fails is the
    /// classification. A joint's turn is read off headings taken from
    /// DIFFERENCES of coordinates at the scene's own magnitude — a
    /// vertex less its arc's centre, a chord's two ends — and such a
    /// difference cancels first-order. Each leg's heading is that
    /// difference over the length it is taken across (an arc's radius
    /// `ρ`, a line's length), so levered by the joint's arm — the
    /// shorter leg's, never longer than the fillet's own `min(ρ, chord)`
    /// — the turn resolves only to about `scale · 2⁻⁵² · arm/ρ`, at most
    /// about `scale · 2⁻⁵²` (`resolution`). Past the distance from the
    /// origin where that floor is coarser than ε the joint cannot be
    /// classified whatever the turn is, and the lever is the magnitude,
    /// not the turn: see
    /// [`FilletArcFlattenedInStorage`](Self::FilletArcFlattenedInStorage)
    /// for why the two are separate variants.
    FilletCarrierBelowSceneResolution {
        /// The fillet arc's stored sweep, radians (diagnostic).
        turn: T,
        /// The requested fillet radius, meters (diagnostic).
        radius: T,
        /// The magnitude the refusing reading was a difference at,
        /// meters — the joint's two legs' vertices, carrier radii and
        /// centres, or the arc's own vertices where its carrier
        /// disagrees with them.
        scale: T,
        /// About the finest that difference could have been read:
        /// `scale · 2^-52`, meters.
        resolution: T,
        /// The predicate whose classification refused the stored form.
        predicate: &'static str,
        /// The margin it classified, meters.
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
    /// past the corner — either way the tangent construction
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
    /// **An endpoint-free arc leg not definitely short of a full
    /// turn**: a `Sweep` angle or an `ArcLen` length whose arc length
    /// short of the full circle, `r(2π − θ)`, is not decided positive
    /// (`path_arc_sweep_full`) — a turn of 2π or more, and one within
    /// the band of 2π (escalated, or refused where decided Zero). D1
    /// admits a one-segment full turn, but a leg leaves its own vertex
    /// for another: at a full turn its chord is zero, and the closed
    /// carrier is `circle` / `circle_split`'s, never a chained leg's.
    ArcSweepNotShortOfFullTurn {
        /// The refused swept angle, radians.
        angle: T,
    },
    /// A [`circle_split`] subdivision count outside `2..=u32::MAX`.
    /// One vertex cannot carry a full turn (bulge = tan(θ/4) diverges
    /// at θ = 2π), so the smallest declared subdivision of a closed
    /// carrier is two arcs — which is [`circle`]'s own private
    /// lowering; and each arc is named by a `u32` piece index
    /// ([`crate::PieceRole::Piece`]), so a count past that bound would
    /// spell two arcs alike. A structural check, not a classified one:
    /// `n` is a count, never a measured value.
    CircleSplitCount {
        /// The refused subdivision count.
        n: usize,
    },
    /// A polygon authored from fewer than three vertices: a closed
    /// chain of straight legs needs three corners before it bounds
    /// anything, and two or fewer name a segment, a point or nothing
    /// at all. The recourse is to author the missing vertices. A
    /// structural check, not a classified one: `given` is a count,
    /// never a measured value.
    ///
    /// The count precondition belongs to a whole-table polygon door
    /// (`pncad::authoring::polygon`), which spells the table through
    /// this lattice; the lattice's own verbs take one vertex at a
    /// time and have no count to gate.
    PolygonTooFewVertices {
        /// The number of vertices given.
        given: usize,
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
    /// A vector a direction is derived from has **no finite length**:
    /// its components overflow the norm (past ~1e154), or one of them
    /// is not a number. Two doors raise it — the components director
    /// ([`PartialPath::toward`]) and the arc carrier's tangent, whose
    /// vector is the anchor's displacement from the centre.
    ///
    /// Distinct from [`PathError::ZeroDirection`] on purpose: the
    /// direction is **not** zero, and that arm's recourse — scale the
    /// components UP — is exactly backwards here.
    ///
    /// **The recourse is the ratio, not the geometry, wherever the
    /// caller holds the numbers.** Both doors read only the ratio of
    /// the components, so dividing the pair through by a common
    /// factor is free and always sufficient. At
    /// [`PartialPath::toward`] the caller spells the components and
    /// can do exactly that; at the arc carrier's tangent they are a
    /// displacement the caller does not hold directly, and moving the
    /// authored geometry into range is the way to reach the same
    /// division. Naming only the second would send a `toward` caller
    /// to move geometry that does not need moving — the
    /// wrong-recourse twin of the wrong-cause defect in
    /// `memories/refusal-text-is-not-cause.md`.
    NonFiniteDirection {
        /// The refused x component.
        dx: T,
        /// The refused y component.
        dy: T,
    },
    /// A vector a direction is derived from has a length that
    /// **underflowed out of the format**: its components are small
    /// enough (below ~1e-162 at `f64`) that their squares are not
    /// representable, so `norm_squared` flushes to zero and the norm
    /// measures exactly zero for a vector that names a direction
    /// perfectly well.
    ///
    /// The underflow end of [`PathError::NonFiniteDirection`]'s
    /// question, and distinct from [`PathError::ZeroDirection`] for the
    /// reason its overflow sibling is: this vector is not zero, so a
    /// refusal that says its norm is "within tolerance of zero" names
    /// the wrong cause, and no tolerance lever reaches it — the squared
    /// norm is zero at every ε.
    ///
    /// **One door raises it today**: the arc carrier's tangent, whose
    /// vector is the anchor's displacement from the centre.
    /// [`PartialPath::toward`]'s components director deliberately does
    /// NOT — a pair spelled `(1e-200, 0)` is refused
    /// [`PathError::ZeroDirection`], whose sentence ("within tolerance
    /// of zero") is true of it and whose recourse ("scaling them up
    /// costs nothing") is already the one that works, because the
    /// caller holds the numbers. So the recourse named below leads with
    /// the free one and names the geometry only where the components
    /// are derived rather than spelled.
    UnderflowedDirection {
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
    /// [`circle`]'s business, not a chain leg's — PQ4). A tangent arc
    /// (`tangent_arc_to`, `arc_onto`) with one end on the line tangent at
    /// the other, on the side only a full turn reaches, refuses here too:
    /// the only arc tangent there is that full turn, and its chord is
    /// whatever the ends span.
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
    /// **An arc the path built is off what was authored**, read back
    /// at a point scalar (`seg::read_back_arc`): its start off its
    /// carrier, its landing off its end vertex, a `Via` arc's carrier off
    /// its through-point, or the other arc between its ends than the
    /// turn it was authored to. The miss stands clear of the read's own
    /// rounding by the band, on a scene the authored data's magnitude
    /// resolves. Every construction's arc meets all four over the reals,
    /// so this is its arithmetic losing digits: a kernel defect, not the
    /// input.
    ArcMissesAuthoredPoint {
        /// The segment the arc is: its step and role.
        piece: crate::structure::Piece,
        /// The verb that step called.
        verb: program::Verb,
        /// The read: `path_arc_start_on_carrier`, `path_arc_landing`,
        /// `path_arc_through_point` or `path_arc_branch`.
        predicate: &'static str,
        /// The authored point the read is about, meters: the start or
        /// end vertex, or a `Via` arc's `q`.
        point: Point2<T>,
        /// How far the stored arc is off what was authored, meters (for
        /// the branch, the length of the arc it built instead).
        miss: T,
        /// The magnitude the read was taken at, meters.
        scale: T,
        /// The rounding the read carries at that magnitude, meters.
        resolution: T,
        /// The stored arc's turn, radians.
        turn: T,
        /// The turn sense the construction authored, where it authors
        /// one apart from its sweep.
        authored: Option<ArcSweep>,
    },
    /// **The same miss, where the scene cannot call it a defect**: the
    /// authored data's magnitude does not resolve the read, the miss is
    /// within the read's own rounding at its magnitude, or the read is
    /// not finite. The miss may be the input's conditioning or the
    /// construction's, which a point scalar cannot tell apart. Refused,
    /// not passed: a stored arc off what was authored is unusable either
    /// way, and the report names the step, the read and the miss so a
    /// construction that lost digits can still be told from a scene at
    /// its resolution.
    ArcMissBelowSceneResolution {
        /// The segment the arc is: its step and role.
        piece: crate::structure::Piece,
        /// The verb that step called.
        verb: program::Verb,
        /// The read: `path_arc_start_on_carrier`, `path_arc_landing`,
        /// `path_arc_through_point` or `path_arc_branch`.
        predicate: &'static str,
        /// The authored point the read is about, meters: the start or
        /// end vertex, or a `Via` arc's `q`.
        point: Point2<T>,
        /// How far the stored arc is off what was authored, meters (for
        /// the branch, the length of the arc it built instead).
        miss: T,
        /// The magnitude the read was taken at, meters.
        scale: T,
        /// The rounding the read carries at that magnitude, meters.
        resolution: T,
        /// The stored arc's turn, radians.
        turn: T,
        /// The turn sense the construction authored, where it authors
        /// one apart from its sweep.
        authored: Option<ArcSweep>,
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
    /// A GUIDED elaboration could not reproduce, at this scalar, a
    /// discrete decision the structure record hands it: the deciding
    /// predicate is indeterminate here, or it comes out definitely
    /// otherwise. Unreachable for an unguided pass, which has no
    /// record to disagree with.
    Structure(crate::structure::StructureRefusal),
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

/// Which arm of [`PathError`] refused — the discriminant alone, with no
/// payload.
///
/// [`PathError`] is generic in the evaluation scalar and its arms carry
/// scalar payloads, so it can never be `Eq` and a derived `PartialEq`
/// would not be worth having: `Real` deliberately omits comparison
/// (`geom_core::real`'s module docs), so the impl would hold only where
/// the scalar supplies equality itself — `f64`, and neither `Interval`
/// (an enclosure compared for equality is not a geometric question) nor
/// `Dual` — and even at `f64` it is float `==`, which is not reflexive
/// at the poison value `Real`'s totality contract promises. This
/// projection drops exactly the part that cannot compare, so the CLASS
/// of a refusal rides anywhere the error itself cannot: into a
/// `PartialEq` error enum, a hash key, an FFI tag map.
///
/// One variant per [`PathError`] arm, and [`PathError::kind`] matches
/// exhaustively — a new arm stops every consumer compiling rather than
/// falling into a wildcard.
///
/// The exhaustiveness runs one way only. A variant here with no arm
/// behind it is a PHANTOM: nothing constructs it, so no test can reach
/// it, and the only build it reds is a downstream map's — `pncad-py`'s
/// `path_error_tag`. The fix at that red is to delete the phantom, not
/// to give it a tag; a tag minted for a phantom publishes an FFI name
/// no refusal can ever carry.
///
/// Deliberately NOT `Ord`. The declaration order mirrors [`PathError`]'s
/// for reading, but nothing depends on it and an order derived on a
/// public enum is a promise about a sequence that means nothing here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathErrorKind {
    /// [`PathError::JunctionTangent`].
    JunctionTangent,
    /// [`PathError::JunctionCusp`].
    JunctionCusp,
    /// [`PathError::SeamTangent`].
    SeamTangent,
    /// [`PathError::SeamArrivalOffDirection`].
    SeamArrivalOffDirection,
    /// [`PathError::SeamArrivalLeverTooShort`].
    SeamArrivalLeverTooShort,
    /// [`PathError::ContinuationTargetOffRay`].
    ContinuationTargetOffRay,
    /// [`PathError::NoCornerForFillet`].
    NoCornerForFillet,
    /// [`PathError::NoCornerOfPair`].
    NoCornerOfPair,
    /// [`PathError::FilletOffsetLeverTooShort`].
    FilletOffsetLeverTooShort,
    /// [`PathError::FilletArcFlattenedInStorage`].
    FilletArcFlattenedInStorage,
    /// [`PathError::FilletCarrierBelowSceneResolution`].
    FilletCarrierBelowSceneResolution,
    /// [`PathError::ArcLegOnOpenFillet`].
    ArcLegOnOpenFillet,
    /// [`PathError::SeamRetrimsArcFirstSide`].
    SeamRetrimsArcFirstSide,
    /// [`PathError::NonpositiveLeg`].
    NonpositiveLeg,
    /// [`PathError::NonpositiveFilletRadius`].
    NonpositiveFilletRadius,
    /// [`PathError::NonpositiveCircleRadius`].
    NonpositiveCircleRadius,
    /// [`PathError::DegenerateArcSpec`].
    DegenerateArcSpec,
    /// [`PathError::ArcSweepNotShortOfFullTurn`].
    ArcSweepNotShortOfFullTurn,
    /// [`PathError::CircleSplitCount`].
    CircleSplitCount,
    /// [`PathError::PolygonTooFewVertices`].
    PolygonTooFewVertices,
    /// [`PathError::ZeroDirection`].
    ZeroDirection,
    /// [`PathError::NonFiniteDirection`].
    NonFiniteDirection,
    /// [`PathError::UnderflowedDirection`].
    UnderflowedDirection,
    /// [`PathError::ArcViaCollinear`].
    ArcViaCollinear,
    /// [`PathError::DegenerateArcChord`].
    DegenerateArcChord,
    /// [`PathError::ArcCenterNotEquidistant`].
    ArcCenterNotEquidistant,
    /// [`PathError::DegenerateArcCenter`].
    DegenerateArcCenter,
    /// [`PathError::ArcMissesAuthoredPoint`].
    ArcMissesAuthoredPoint,
    /// [`PathError::ArcMissBelowSceneResolution`].
    ArcMissBelowSceneResolution,
    /// [`PathError::FarEndAnchorWithoutFillet`].
    FarEndAnchorWithoutFillet,
    /// [`PathError::Escalated`].
    Escalated,
    /// [`PathError::Band`].
    Band,
    /// [`PathError::Structure`].
    Structure,
    /// [`PathError::UnderdeterminedLeg`].
    UnderdeterminedLeg,
    /// [`PathError::OverdeterminedJunction`].
    OverdeterminedJunction,
}

impl<T: Real> PathError<T> {
    /// Which arm refused, without the payload.
    ///
    /// Exhaustive over [`PathError`]: adding an arm there is a compile
    /// error here and in every consumer that maps this enum.
    pub fn kind(&self) -> PathErrorKind {
        match self {
            Self::JunctionTangent { .. } => PathErrorKind::JunctionTangent,
            Self::JunctionCusp { .. } => PathErrorKind::JunctionCusp,
            Self::SeamTangent { .. } => PathErrorKind::SeamTangent,
            Self::SeamArrivalOffDirection { .. } => PathErrorKind::SeamArrivalOffDirection,
            Self::SeamArrivalLeverTooShort { .. } => PathErrorKind::SeamArrivalLeverTooShort,
            Self::ContinuationTargetOffRay { .. } => PathErrorKind::ContinuationTargetOffRay,
            Self::NoCornerForFillet { .. } => PathErrorKind::NoCornerForFillet,
            Self::NoCornerOfPair { .. } => PathErrorKind::NoCornerOfPair,
            Self::FilletOffsetLeverTooShort { .. } => PathErrorKind::FilletOffsetLeverTooShort,
            Self::FilletArcFlattenedInStorage { .. } => PathErrorKind::FilletArcFlattenedInStorage,
            Self::FilletCarrierBelowSceneResolution { .. } => {
                PathErrorKind::FilletCarrierBelowSceneResolution
            }
            Self::ArcLegOnOpenFillet { .. } => PathErrorKind::ArcLegOnOpenFillet,
            Self::SeamRetrimsArcFirstSide => PathErrorKind::SeamRetrimsArcFirstSide,
            Self::NonpositiveLeg { .. } => PathErrorKind::NonpositiveLeg,
            Self::NonpositiveFilletRadius { .. } => PathErrorKind::NonpositiveFilletRadius,
            Self::NonpositiveCircleRadius { .. } => PathErrorKind::NonpositiveCircleRadius,
            Self::DegenerateArcSpec { .. } => PathErrorKind::DegenerateArcSpec,
            Self::ArcSweepNotShortOfFullTurn { .. } => PathErrorKind::ArcSweepNotShortOfFullTurn,
            Self::CircleSplitCount { .. } => PathErrorKind::CircleSplitCount,
            Self::PolygonTooFewVertices { .. } => PathErrorKind::PolygonTooFewVertices,
            Self::ZeroDirection { .. } => PathErrorKind::ZeroDirection,
            Self::NonFiniteDirection { .. } => PathErrorKind::NonFiniteDirection,
            Self::UnderflowedDirection { .. } => PathErrorKind::UnderflowedDirection,
            Self::ArcViaCollinear { .. } => PathErrorKind::ArcViaCollinear,
            Self::DegenerateArcChord { .. } => PathErrorKind::DegenerateArcChord,
            Self::ArcCenterNotEquidistant { .. } => PathErrorKind::ArcCenterNotEquidistant,
            Self::DegenerateArcCenter { .. } => PathErrorKind::DegenerateArcCenter,
            Self::ArcMissesAuthoredPoint { .. } => PathErrorKind::ArcMissesAuthoredPoint,
            Self::ArcMissBelowSceneResolution { .. } => PathErrorKind::ArcMissBelowSceneResolution,
            Self::FarEndAnchorWithoutFillet => PathErrorKind::FarEndAnchorWithoutFillet,
            Self::Escalated { .. } => PathErrorKind::Escalated,
            Self::Band(_) => PathErrorKind::Band,
            Self::Structure(_) => PathErrorKind::Structure,
            Self::UnderdeterminedLeg { .. } => PathErrorKind::UnderdeterminedLeg,
            Self::OverdeterminedJunction { .. } => PathErrorKind::OverdeterminedJunction,
        }
    }
}

/// Render a scalar payload inside a refusal sentence.
///
/// [`Real`] carries `Debug` and no `Display`, so an arm below can only
/// reach its scalars through `{:?}` — the shortest round-tripping form
/// of an `f64`, which puts an 8 mm radius that arithmetic produced into
/// a human sentence as `0.008000000000000002 m`. Where the `Debug` form
/// parses back as an `f64` this renders the shortest spelling that still
/// names the same number **to the finer of a relative 1e-9 and an
/// absolute ε/10**; anything else (an interval, a dual) passes through
/// untouched. A DISPLAY choice only — the payload keeps the exact
/// scalar, and every claim a caller branches on reads the field, never
/// this string.
///
/// **The two arms are load-bearing at opposite ends of the range, and
/// the tolerance is their `min` because each is the coarser one
/// somewhere.** ε is a LENGTH ([`DEFAULT_EPS`], D4 ¶1), so a purely
/// relative 1e-9 crosses it at one metre and is coarser above: at a
/// kilometre it is a micron, and two lengths the kernel certifies as
/// different then render as one number — a refusal reading *"margin
/// 1234.5 m exceeds the 1234.5 m the anchor pins"*. The absolute arm
/// caps the grid one decade below ε, so a difference the kernel can
/// decide is always a difference the sentence spells. Below a decimetre
/// the relative arm is the finer of the two and governs alone, and the
/// shortening is there the same proposition at every magnitude: a
/// picometre margin is rounded to nine significant figures of a
/// picometre, never to the nearest nanometre and never to `0`. A FLOOR
/// under the tolerance is the mirror defect and is precisely what a
/// `min` cannot become — it would round every margin below the floor to
/// the floor's own zero, erasing the sub-nanometre values these messages
/// exist to report.
///
/// The cap is the compile-time [`DEFAULT_EPS`] and never the run's live
/// `Tolerance::eps()`. A live ε would make every rendered refusal a
/// function of process configuration, spelling one payload three ways
/// across the ε rows CI gates; the grid is a display choice stated once
/// against the ratified default.
///
/// `0.0` has no significant figures to round to and only its exact
/// spelling round-trips, which is `0`; `-0.0`'s is `-0`, a sign this
/// helper reports because the payload carries it, not because a
/// magnitude was erased.
///
/// Notation follows the `Debug` form's own choice — fixed where `{:?}`
/// is fixed, exponential where `{:?}` is exponential — so this helper
/// only ever shortens the mantissa of the spelling Rust already picked,
/// and never turns one into the other. A metre-scale number keeps its
/// decimal point and a far-from-unity one keeps its exponent, which is
/// what makes both of them readable: `1e-12` rather than eleven zeros,
/// `1e300` rather than 301 digits.
///
/// A plain `f64` field reaches the same defect by the other door: its
/// own `Display` is that shortest round-tripping spelling too, so a
/// scalar payload typed concretely is no more shortened than one behind
/// [`Real`], and both belong here.
///
/// Every scalar-bearing refusal rendering in this crate goes through
/// here — the arms below, and [`crate::FilletLegCarrier`]'s `Display`
/// in `validate`, whose sentence is interpolated into
/// [`CornerReason::AnchorOutsideTrimmedExtent`]'s. One grid for the
/// crate's refusals, not one per module. Non-scalar payloads — a side,
/// a carrier, an index, a `&'static str` site — are not this helper's
/// business and reach the sentence through their own `Display`, which
/// renders any scalars of its own through here in turn.
pub(crate) fn num<T: core::fmt::Debug>(v: &T) -> String {
    let raw = format!("{v:?}");
    let Ok(x) = raw.parse::<f64>() else {
        return raw;
    };
    let tol = (DEFAULT_EPS * 0.1).min(x.abs() * 1e-9);
    let exponential = raw.contains('e');
    for prec in 0..=17 {
        let short = if exponential {
            format!("{x:.prec$e}")
        } else {
            format!("{x:.prec$}")
        };
        if short
            .parse::<f64>()
            .is_ok_and(|back| (back - x).abs() <= tol)
        {
            if exponential {
                // The first precision that round-trips cannot carry a
                // trailing zero in its mantissa: dropping one would
                // name the same `f64`, so the shorter precision would
                // have been accepted first.
                return short;
            }
            let trimmed = if short.contains('.') {
                short.trim_end_matches('0').trim_end_matches('.')
            } else {
                &short
            };
            return trimmed.to_string();
        }
    }
    // No 18-significant-figure spelling in the chosen notation names
    // the value: a non-finite payload, whose comparisons are all false.
    raw
}

impl<T: Real> core::fmt::Display for PathError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::JunctionTangent { margin, arm } => write!(
                f,
                "this junction is tangent at any precision you could care about \
                 (turn margin {margin} m on a {arm} m arm). Recourse: \
                 {JUNCTION_TANGENT_LEVER} (or lower the tolerance)",
                margin = num(margin),
                arm = num(arm)
            ),
            Self::JunctionCusp { margin, arm } => write!(
                f,
                "this junction reverses onto the incoming direction at any precision you \
                 could care about (turn margin {margin} m on a {arm} m arm): an undeclared \
                 cusp. Recourse: if intended, author .cusp() at an interior junction; \
                 otherwise move the geometry. At the seam the closing leg arrives REVERSED: \
                 rotate the loop\'s authoring origin, or cut the seam at a corner",
                margin = num(margin),
                arm = num(arm)
            ),
            Self::SeamTangent { margin } => write!(
                f,
                "the SEAM arrives tangent to the entry\'s first side (margin {margin} m) and \
                 nothing declared it. Recourse: DECLARE the joint on the target with \
                 Start.arrives_tangent(), which every closing verb takes, or cut the loop at \
                 a CORNER instead (author the seam where the outline actually turns)",
                margin = num(margin)
            ),
            Self::SeamArrivalOffDirection { margin, arm } => write!(
                f,
                "the declared seam arrival (Start.arrives_tangent()) does not continue the \
                 entry\'s outgoing direction: it misses by {margin} m over the arriving \
                 leg\'s {arm} m arm. Recourse: move the geometry so the leg arrives along \
                 that side, or drop the declaration and author the seam at a CORNER; for a \
                 closing ARC tangent at BOTH ends, use the seam FILLET \
                 (.angle(theta).fillet(r).to(Start))",
                margin = num(margin),
                arm = num(arm)
            ),
            Self::SeamArrivalLeverTooShort { arm } => write!(
                f,
                "the declared seam arrival has no lever: the closing leg\'s arm is {arm} m, \
                 not definitely positive, so its direction cannot be checked. Recourse: \
                 lengthen the closing leg or move the entry off it, or drop the vertex if it \
                 was not wanted",
                arm = num(arm)
            ),
            Self::ContinuationTargetOffRay { across, along } => write!(
                f,
                "the declared straight continuation\'s target is not on its run\'s line (the \
                 line bound where the run\'s direction was bound): it misses by {across} m \
                 across the line, {along} m along it from the departing point. Recourse: move \
                 the target onto the run\'s line; if a TURN was \
                 meant, author the direction (.turn(delta)/.angle(theta)) and use line_to",
                across = num(across),
                along = num(along)
            ),
            Self::NoCornerForFillet { reason, radius } => {
                let what = match reason {
                    PathNoCornerReason::CarriersParallel => {
                        "the incoming ray and the arrival carrier are parallel at tolerance — \
                         no corner exists (if they are meant to run tangentially, author the \
                         tangency: .tangent(), or the seam fillet at the seam)"
                    }
                    PathNoCornerReason::CarriersDoNotMeet => {
                        "the two carriers do not meet: a ray missing its circle, or circles \
                         disjoint, concentric, or one inside the other"
                    }
                };
                write!(
                    f,
                    "no corner for a radius-{radius} m fillet: {what}",
                    radius = num(radius)
                )
            }
            Self::NoCornerOfPair { radius, corners } => {
                // The count is of ENTRIES — the corners that refused at
                // the answering stage — and the sentence says so. A
                // carrier pair derives up to two corners and most
                // refusals list only one of them, so "n derived
                // corners" would be false about the pair on the
                // majority of refusals, and a false fact in a refusal
                // is worse than no fact.
                write!(
                    f,
                    "no corner of these carriers takes a radius-{radius} m fillet",
                    radius = num(radius),
                )?;
                for corner in corners {
                    write!(f, "; {corner}")?;
                }
                // The unbounded swallow's hedge is about the PAIR — its
                // class bound is necessary and not sufficient — so it
                // is said once, after every corner, not per corner.
                if corners.iter().any(|c| {
                    matches!(
                        c.reason,
                        CornerReason::EnclosesLegCarrier {
                            largest_tangent_radius: None,
                            ..
                        }
                    )
                }) {
                    write!(f, "; these carriers may admit no fillet at all")?;
                }
                // One recourse for the pair: moving the anchor is the
                // wider menu, and it is offered whenever any corner's
                // anchor refused.
                let recourses: Vec<&str> =
                    corners.iter().filter_map(|c| c.reason.recourse()).collect();
                let recourse = recourses
                    .iter()
                    .find(|r| r.contains("anchor"))
                    .or(recourses.first());
                match recourse {
                    Some(r) => write!(f, ". Recourse: {r}"),
                    None => Ok(()),
                }
            }
            Self::FilletOffsetLeverTooShort {
                side,
                carrier_radius,
                offset_radius,
                least_lever,
                margin,
            } => write!(
                f,
                "the {side} side's offset lever rho {offset_radius} m (carrier radius \
                 {carrier_radius} m) is shorter than the {least_lever} m this corner needs \
                 at the run's tolerance (margin {margin} m), so the fillet's tangent point \
                 cannot be placed. Recourse: move the fillet radius away from that side's \
                 carrier radius, or bring the corner's carriers closer together",
                offset_radius = num(offset_radius),
                carrier_radius = num(carrier_radius),
                least_lever = num(least_lever),
                margin = num(margin)
            ),
            Self::FilletArcFlattenedInStorage {
                turn,
                radius,
                arc_length,
                predicate,
                margin,
            } => write!(
                f,
                "a radius-{radius} m fillet through a turn of {turn} rad is a {arc_length} m \
                 arc, too shallow to store as a chord and a carrier ('{predicate}' classifies \
                 it at {margin} m). Recourse: {FILLET_FLATTENED_RECOURSE}",
                radius = num(radius),
                turn = num(turn),
                arc_length = num(arc_length),
                margin = num(margin)
            ),
            Self::FilletCarrierBelowSceneResolution {
                turn,
                radius,
                scale,
                resolution,
                predicate,
                margin,
            } => write!(
                f,
                "a radius-{radius} m fillet through a turn of {turn} rad needs a tangency \
                 the scene cannot resolve: lengths of about {scale} m resolve only to about \
                 {resolution} m ('{predicate}' classifies it at {margin} m). Recourse: \
                 {FILLET_SCENE_RESOLUTION_RECOURSE}",
                radius = num(radius),
                turn = num(turn),
                scale = num(scale),
                resolution = num(resolution),
                margin = num(margin)
            ),
            Self::ArcLegOnOpenFillet { site } => write!(f, "{site}"),
            Self::DegenerateArcSpec { value } => write!(
                f,
                "this arc spec's authored datum ({value}) names no arc: a zero bulge \
                 degenerates to the chord (author a line), and a sweep angle or arc length \
                 must be definitely positive",
                value = num(value)
            ),
            Self::ArcSweepNotShortOfFullTurn { angle } => write!(
                f,
                "this arc leg turns {angle} rad, which is not short of a full turn: a leg \
                 sweeps less than 2π; a whole circle is circle(c, r), or circle_split(c, r, \
                 n, phase) to place its seams",
                angle = num(angle)
            ),
            Self::SeamRetrimsArcFirstSide => write!(
                f,
                "a seam fillet retrims the entry vertex, so it needs a straight first side; \
                 to close onto an arc carrier and KEEP the entry vertex, use \
                 fillet_arc(r, Center {{ c, winding, p: Start }})"
            ),
            Self::NonpositiveLeg { length } => write!(
                f,
                "a leg must advance the tip by a definitely positive length (got {length} m): \
                 a negative length runs the side backward and detaches anchored points from \
                 the final path (every authored point lies on the final path, authored once); \
                 a sub-tolerance length is a degenerate segment",
                length = num(length)
            ),
            Self::NonpositiveFilletRadius { radius } => write!(
                f,
                "a fillet needs a definitely positive radius (got {radius} m): r = 0 \
                 degenerates the arc and a negative r mirrors the tangent points past the \
                 corner — no tangent construction exists to declare",
                radius = num(radius)
            ),
            Self::NonpositiveCircleRadius { radius } => write!(
                f,
                "a circle needs a definitely positive radius (got {radius} m): r = 0 is a \
                 point and r < 0 names no circle",
                radius = num(radius)
            ),
            Self::CircleSplitCount { n } => write!(
                f,
                "circle_split needs between 2 and {max} arcs (got n = {n}): a single vertex \
                 cannot carry a full turn (bulge diverges), and each arc is named by a u32 \
                 piece index",
                max = u32::MAX
            ),
            Self::PolygonTooFewVertices { given } => write!(
                f,
                "a polygon needs at least 3 vertices (got {given}): a closed chain of \
                 straight legs bounds nothing with fewer corners — author the missing \
                 vertices"
            ),
            Self::ZeroDirection { dx, dy } => write!(
                f,
                "a director spelled as components must name a direction (got \
                 ({dx}, {dy}), whose norm is within tolerance of zero): only the ratio \
                 of the components is read, so scaling them up costs nothing",
                dx = num(dx),
                dy = num(dy)
            ),
            Self::NonFiniteDirection { dx, dy } => write!(
                f,
                "a direction derived from ({dx}, {dy}) has no finite length (a component \
                 overflows the norm or is not a number). Recourse: only the ratio of the \
                 components is read, so divide them through by a common factor; if they \
                 come from authored geometry, {RANGE_RECOURSE}",
                dx = num(dx),
                dy = num(dy),
                RANGE_RECOURSE = geom_core::RANGE_RECOURSE
            ),
            Self::UnderflowedDirection { dx, dy } => write!(
                f,
                "a direction derived from ({dx}, {dy}) has a length that underflowed out of \
                 the format, though the direction is good. Recourse: only the ratio of the \
                 components is read, so multiply them through by a common factor; if they \
                 come from authored geometry, {RANGE_RECOURSE}",
                dx = num(dx),
                dy = num(dy),
                RANGE_RECOURSE = geom_core::RANGE_RECOURSE
            ),
            Self::ArcViaCollinear { offset } => write!(
                f,
                "the through-point lies on the chord line (offset {offset} m, within \
                 tolerance of zero): three collinear points name no arc — move the \
                 through-point off the chord, or author the straight segment as a line",
                offset = num(offset)
            ),
            Self::DegenerateArcChord { chord } => write!(
                f,
                "an arc leg spans no arc short of a full turn (chord {chord} m): its ends \
                 coincide within tolerance, or a tangent arc's far end lies on its tangent \
                 line where only a full turn reaches it. Recourse: move the ends apart, or that \
                 end off the tangent line; a closed carrier is a circle",
                chord = num(chord)
            ),
            Self::ArcCenterNotEquidistant {
                tip_radius,
                end_radius,
            } => write!(
                f,
                "the authored centre is not equidistant from the arc's endpoints \
                 (|tip - centre| = {tip_radius} m, |end - centre| = {end_radius} m): the \
                 three authored points contradict each other. Nothing is re-projected — an \
                 authored point is never moved to make a construction work; fix whichever \
                 of the three is wrong",
                tip_radius = num(tip_radius),
                end_radius = num(end_radius)
            ),
            Self::DegenerateArcCenter { radius } => write!(
                f,
                "the authored centre is within tolerance of an endpoint (radius {radius} \
                 m): the carrier has no radius, so the winding selects nothing",
                radius = num(radius)
            ),
            Self::ArcMissesAuthoredPoint {
                piece,
                verb,
                predicate,
                point,
                miss,
                scale,
                resolution,
                turn,
                authored,
            } => {
                read_back_sentence(f, piece, verb, predicate, point, miss, turn, *authored)?;
                write!(
                    f,
                    ", clear of the {resolution} m its read rounds to at {scale} m \
                     ('{predicate}'). {KERNEL_DEFECT_ENDING}",
                    resolution = num(resolution),
                    scale = num(scale)
                )
            }
            Self::ArcMissBelowSceneResolution {
                piece,
                verb,
                predicate,
                point,
                miss,
                scale,
                resolution,
                turn,
                authored,
            } => {
                read_back_sentence(f, piece, verb, predicate, point, miss, turn, *authored)?;
                write!(
                    f,
                    "; at {scale} m its read rounds to {resolution} m, too coarse to call \
                     this a defect ('{predicate}'). Recourse: {ARC_SCENE_RESOLUTION_RECOURSE}",
                    resolution = num(resolution),
                    scale = num(scale)
                )
            }
            Self::FarEndAnchorWithoutFillet => write!(
                f,
                "the far-end-anchor form ends an ARRIVAL side at its own anchor, and no \
                 fillet is open here. Recourse: author the entry's first side with .at(p), \
                 and the seam with the verb that targets Start"
            ),
            // A recourse is routed by the escalated predicate's NAME.
            // A name in two layers gets the EARLIER layer's sentence, so
            // a `fillet_*` name written into a later arm is dead code
            // rather than a silent override of its own sentence; the
            // layers' name sets are disjoint today, and
            // `recourse_roster::the_dispatch_order_owns_each_name_in_exactly_one_layer`
            // is what measures that rather than assuming it.
            //
            // A name no layer claims ends by its row in
            // `validate::DECISION_ENDINGS`; a name with no row names the
            // hole (`geom_core::MissingRecourse`).
            Self::Escalated { source } => {
                if let Some(predicate) = source.predicate
                    && let Some(recourse) = fillet_recourse_for(predicate)
                {
                    // The fillet constructor's own gates. The caller
                    // asked for a fillet and the door could not
                    // classify one of the construction's decisions, so
                    // the site is the corner being resolved and the
                    // recourse is the gate's own — never the shared
                    // coincidence one, which advises declaring a
                    // coincidence at a joint this caller never
                    // authored.
                    return source
                        .undecided(
                            "the fillet at this corner",
                            geom_core::lever_recourse(recourse, None),
                        )
                        .fmt(f);
                }
                match source.predicate {
                    Some("path_continuation_target_offset") => write!(
                        f,
                        "the declared straight continuation's target is neither on its \
                     run's line nor definitely off it: {payload}. Recourse: move the target \
                     onto the run's line",
                        payload = source.payload()
                    ),
                    Some("path_seam_arrival_turn" | "path_seam_arrival_side") => write!(
                        f,
                        "the declared seam arrival is neither continuing the entry\'s outgoing \
                     direction nor definitely off it: {payload}; the declaration is the target. \
                     Recourse: move the geometry so the two directions agree",
                        payload = source.payload()
                    ),
                    Some("path_leg_length") => write!(
                        f,
                        "an authored leg extent could not be told from zero: {payload}. \
                     Recourse: author a longer leg",
                        payload = source.payload()
                    ),
                    // The STORED-FORM read's own classifications
                    // (`Core::fillets_carry_their_tangency` re-runs
                    // validation's segment and joint predicates on the loop
                    // the door is about to emit). This is not a junction:
                    // the door minted the joint itself, so "declare the
                    // coincidence" is advice about a declaration the caller
                    // never wrote, and the levers are the two the stored
                    // form actually has. The read's joint turn is
                    // `path_fillet_joint_turn`: validation's turn
                    // (`seg::joint_tangency`) under the stored form's own
                    // name, so its in-band refusal names these levers
                    // rather than the junction's `.tangent()`. The
                    // `carrier_line_circle` and `carrier_circles_*` names
                    // but the identity are the pair pass's, which the
                    // read does not run; they are listed so that the
                    // near-tangency names render alike here and there.
                    // Four of these names —
                    // `carrier_line_circle` and every `carrier_circles_*`
                    // but the identity — also carry a note at
                    // `ProfileError::Escalated`, and the two sentences
                    // differ on purpose: THERE the segment pair is the
                    // caller's own, so the declare lever is theirs and
                    // the note says how to pull it; HERE the loop is one
                    // this door is about to store. The class this arm
                    // leaves is
                    // `work/blend/every-escalation-carries-the-coincidence-recourse-first.md`
                    // — an escalation that renders the shared recourse
                    // before its own site's — and this is one instance
                    // repaired at the site.
                    //
                    // A `fillet_*` name added to THIS list is dead: the
                    // fillet arm above is asked first and answers every
                    // name `validate::fillet_recourse_for` knows, so the
                    // name never reaches here. Verified by mutation, not
                    // asserted from the shape of the code.
                    Some(
                        "vertex_separation"
                        | "segment_straightness"
                        | "arc_diameter_clearance"
                        | "chord_side"
                        | "path_fillet_joint_turn"
                        | "carrier_line_circle"
                        | "carrier_circles_identity"
                        | "carrier_circles_external"
                        | "carrier_circles_internal"
                        | "carrier_circles_chord",
                    ) => source
                        .undecided(
                            "the fillet arc about to be stored",
                            geom_core::lever_recourse(FILLET_STORED_FORM_INBAND_RECOURSE, None),
                        )
                        .fmt(f),
                    // A name no arm above claims ends as validation ends
                    // it (`validate::EscalationEnding`), by its row in
                    // `validate::DECISION_ENDINGS`, opening with what the
                    // decision was deciding.
                    _ => source
                        .undecided(
                            source
                                .predicate
                                .and_then(crate::validate::decision_subject)
                                .unwrap_or(crate::validate::UNNAMED_DECISION),
                            crate::validate::EscalationEnding(source),
                        )
                        .fmt(f),
                }
            }
            Self::Band(e) => write!(f, "the path's tolerance could not form a band: {e}"),
            Self::Structure(r) => write!(f, "{r}"),
            Self::UnderdeterminedLeg { site } => write!(
                f,
                "the sketch leg at {site} is underdetermined, which the authoring surface \
                 should rule out (a kernel bug)"
            ),
            Self::OverdeterminedJunction { site } => write!(
                f,
                "the sketch junction at {site} is overdetermined, which the authoring \
                 surface should rule out (a kernel bug)"
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
    /// The leg, as [`tangent_arc`] built it.
    arc: Option<BuiltArc<T>>,
    /// The arc's end tangent: the departure reflected across the chord.
    end_ang: Dir<T>,
    /// The leg's carrier circle (none for a line).
    carrier: Option<ArcData<T>>,
    /// The chord length, meters (the junction check's lever cap).
    chord: T,
}

/// A bound direction: the unit ray, and nothing else.
///
/// How a director is SPELLED fixes its bits (the G1 VQ4 exactness
/// contract). One spelled as an ANGLE derives the ray by `sin_cos`, so
/// an axis direction picks up the quantization of π
/// (`unit(PI).y = 1.22e-16`); one spelled as COMPONENTS
/// ([`PartialPath::toward`], a leg's chord) fixes the ray exactly —
/// `(-1, 0)` normalizes to itself. Every later direction is the ray
/// itself, its negation ([`Dir::reversed`]), its reflection
/// ([`reflected_across`]) or its rotation ([`turned`]), never a ray
/// rebuilt from an angle read back off it, so exactness survives every
/// hop it can, and no direction's enclosure is ever an angle's read
/// across `atan2`'s branch cut.
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct Dir<T: Real> {
    /// The unit ray — exact when the director authored components.
    unit: Vec2<T>,
}

impl<T: Real> Dir<T> {
    /// A director spelled as an angle: the ray is the `sin_cos`
    /// round-trip (the historic `.angle(θ)` behaviour, bit-for-bit).
    fn from_angle(ang: T) -> Self {
        Self { unit: unit(ang) }
    }

    /// A director spelled as an already-unit ray: the ray is stored
    /// verbatim.
    ///
    /// **The ray is DECIDED before it gets here, at the door that
    /// built it, and this constructor deliberately does not decide it
    /// again.** Every caller either normalized against the funnel on
    /// its own band — `unit_from_components` under
    /// `path_director_norm`, [`arc_fillet::carrier_tangent`] under
    /// `path_arc_center_radius` — or normalized a chord whose length
    /// other gates decide ([`chord_director`]), or holds a ray that is
    /// unit by construction: [`Dir::reversed`]'s exact negation, and
    /// [`reflected_across`]'s reflection and [`turned`]'s rotation,
    /// each of a unit ray, unit over the reals and to its rounding in
    /// the floats. A
    /// second decision here would either re-ask a question already
    /// answered (a second name in the K census for one length) or,
    /// worse, re-derive the ray and lose the exactness this type
    /// exists to keep: the VQ4 contract is that a director spelled as
    /// COMPONENTS fixes the ray, so `(-1, 0)` stays `(-1, 0)` and a
    /// declared cusp is a structural fact rather than a value
    /// coincidence.
    ///
    /// So the invariant is the callers', and it is measured rather
    /// than assumed — the row at the bottom of this file walks each
    /// producing door. What NEITHER 2-D door asks, unlike the 3-D
    /// direction body in `topo::query`, is whether the length is a
    /// FINITE number before deciding its sign; that residue is filed
    /// rather than fixed here
    /// (`work/seat/two-d-director-doors-skip-the-finiteness-question`).
    fn from_unit(u: Vec2<T>) -> Self {
        Self { unit: u }
    }

    /// The exact REVERSE of this director — the cusp door's departure.
    ///
    /// The ray is NEGATED, never re-derived as `θ + π`: negation is
    /// exact in every backend, so a reverse-tangent junction authored
    /// through this is exactly reverse-tangent and there is nothing
    /// for verification to contradict. That is the same guarantee
    /// `.tangent()` gets by inheriting the incoming ray verbatim, and
    /// it is why a DECLARED cusp is a structural fact rather than a
    /// value coincidence.
    fn reversed(self) -> Self {
        Self::from_unit(-self.unit)
    }
}

use directed::{Bound, Incoming, Run};

/// **The directed point's intrinsic data, behind its one API** (PATHS-DESIGN
/// §2c): its tangent, and `lands_on_ray`, the on-ray reading of a point
/// target. The point's position is its tip's ([`PosData::at`]).
///
/// The representations are a family no verb can see. One is the bare
/// (position, tangent); the other is the line of the straight run the
/// point ends ([`Run`]), handed on by every straight continuation, so
/// `lands_on_ray` on a run of continuations reads the line bound where
/// the run's direction was bound, never re-anchored at an accepted
/// vertex. The run is a private field and [`Run`] has no accessor: no
/// verb can branch on, refuse on, or depend on which representation a
/// point holds, and the module's privacy is what says so.
mod directed {
    use super::{ArcData, Dir, PathError, Point2, Real, Tol, linear_band};
    use geom_core::k_stats::decide;
    use geom_core::{Decide, Margin, Sign};

    /// The line a straight run rides: its first vertex and the
    /// direction bound there. Opaque: what reads it reads its two
    /// displacements ([`Run::across`], [`Run::along`]), never its
    /// fields, so no reader can tell which vertex anchors it.
    #[derive(Clone, Copy, Debug)]
    pub(super) struct Run<T: Real> {
        anchor: Point2<T>,
        dir: Dir<T>,
    }

    impl<T: Real> Run<T> {
        /// `p`'s lateral displacement from this line, in metres: the
        /// one spelling of the anchored-ray reading that the declared
        /// continuation (`lands_on_ray`) and the fillet run-out
        /// ([`super::PendingRunOut::rides`]) each decide under a key of
        /// their own.
        pub(super) fn across(&self, p: Point2<T>) -> T {
            self.dir.unit.perp_dot(p - self.anchor)
        }

        /// How far along this line's direction `p` stands from `from`,
        /// in metres.
        pub(super) fn along(&self, from: Point2<T>, p: Point2<T>) -> T {
            self.dir.unit.dot(p - from)
        }

        /// The line bound at `anchor` along `dir`: a straight leg whose
        /// direction was bound where it starts begins a run there.
        pub(super) fn bound(anchor: Point2<T>, dir: Dir<T>) -> Self {
            Self { anchor, dir }
        }

        /// The same line run the other way: the reverse-tangent
        /// continuation's (`.cusp()`), which negates the direction and
        /// keeps the line.
        pub(super) fn reversed(self) -> Self {
            Self {
                anchor: self.anchor,
                dir: self.dir.reversed(),
            }
        }
    }

    /// **A bound direction**, the tip's angle slot: the direction, and
    /// the run it rides where it was inherited bitwise (`.tangent()`,
    /// `.cusp()`), so a straight leg off it continues that run
    /// (PATHS-DESIGN §2c). The run is private: it is held only beside
    /// its direction, read by [`Bound::run_at`] alone, and `None`
    /// wherever the direction was bound afresh.
    #[derive(Clone, Copy, Debug)]
    pub(super) struct Bound<T: Real> {
        pub(super) dir: Dir<T>,
        run: Option<Run<T>>,
    }

    impl<T: Real> Bound<T> {
        /// A direction bound where it is authored.
        pub(super) fn fresh(dir: Dir<T>) -> Self {
            Self { dir, run: None }
        }

        /// A direction inherited bitwise, riding `run`.
        pub(super) fn inherited(dir: Dir<T>, run: Run<T>) -> Self {
            Self {
                dir,
                run: Some(run),
            }
        }

        /// The run a straight leg from `at` along this direction rides:
        /// the inherited one, or one begun at `at`, where the direction
        /// was bound.
        pub(super) fn run_at(&self, at: Point2<T>) -> Run<T> {
            self.run.unwrap_or(Run::bound(at, self.dir))
        }
    }

    /// A directed point's intrinsic incoming data: the arriving leg's
    /// end tangent, the leg's lever arm (extent capped by its carrier
    /// radius — the junction check's meters lever), its carrier kind,
    /// and the run its straight continuations ride (private: read only
    /// by [`Incoming::lands_on_ray`] and handed on by
    /// [`Incoming::run_from`]).
    #[derive(Clone, Copy, Debug)]
    pub(super) struct Incoming<T: Real> {
        pub(super) ang: Dir<T>,
        pub(super) arm: T,
        pub(super) carrier: Option<ArcData<T>>,
        run: Option<Run<T>>,
    }

    impl<T: Real> Incoming<T> {
        /// A point whose direction was bound where it ends: the bare
        /// representation, its run starting at its own position.
        pub(super) fn fresh(ang: Dir<T>, arm: T, carrier: Option<ArcData<T>>) -> Self {
            Self {
                ang,
                arm,
                carrier,
                run: None,
            }
        }

        /// The end of a straight continuation along `run`: the
        /// representation it was itself checked against.
        pub(super) fn continuing(run: Run<T>, ang: Dir<T>, arm: T) -> Self {
            Self {
                ang,
                arm,
                carrier: None,
                run: Some(run),
            }
        }

        /// The run a straight continuation from this point at `at`
        /// rides: the run it ends, or one starting here along its
        /// tangent.
        pub(super) fn run_from(&self, at: Point2<T>) -> Run<T> {
            self.run.unwrap_or(Run {
                anchor: at,
                dir: self.ang,
            })
        }
    }

    impl<T: Decide> Incoming<T> {
        /// **The declared point-target continuation's one decision**: is
        /// the authored `target` ON this point's ray? Returns how far
        /// ALONG it from `at` (the point's position) the target sits,
        /// the leg length the declaration implies, or refuses.
        ///
        /// The ray is the run's ([`Incoming::run_from`]): on a run of
        /// continuations, the line bound at the run's first vertex. So
        /// every vertex of a declared straight run lies within the band
        /// of that one line, and n accepted misses are n misses of it,
        /// never a sum (PATHS-DESIGN §4).
        ///
        /// Two facts are gated, and they are separate facts, so they get
        /// separate refusals. The LATERAL miss decides whether the
        /// target is on the run's LINE, and it is classified on the same
        /// linear band every other decision in this kernel uses, with
        /// the same reading: below ε_precision the target and the line
        /// are the same place at the precision anything here
        /// represents, so the declaration is consistent; above ε_input
        /// (= K·ε) they are definitely different places, and the
        /// authored data contradicts itself; between them nothing is
        /// decidable, so the band escalates rather than guesses. The
        /// refusal edge IS ε_input, which is the band the input-quality
        /// role names — this is a question about authored input, not
        /// about what the kernel can build — and it is reached through
        /// the funnel rather than by comparing against K·ε directly,
        /// because a bare comparison would swallow the escalation band
        /// and decide where the numbers cannot.
        ///
        /// The margin is metered with NO lever: `across` is already the
        /// target's own displacement from the line in meters — the
        /// distance the authored point would have to move — so
        /// [`Margin::of`](geom_core::Margin::of) is the honest door.
        /// (§4 item 1 levers its turn margin because ITS datum is an
        /// angle; an angle is a pure number until an arm says what it
        /// displaces. Levering here would mean dividing this length by
        /// the leg to get an angle and multiplying it back, which can
        /// only lose bits and would make the threshold depend on how far
        /// away the author put the point.)
        ///
        /// The ALONG component is the leg's half-line-ness, measured
        /// from the departing position, and it is the same fact
        /// `line(len)` gates on its authored length: a target behind the
        /// departure, or on top of it, is a leg of non-positive length
        /// ([`PathError::NonpositiveLeg`]), not a target that misses.
        ///
        /// **Sibling measurement, cross-declared** (R1 S1): this,
        /// [`super::tangent_arc_geom`] and [`super::PendingRunOut::rides`]'s
        /// ray arm compute the same kind of displacement — `across =
        /// û⊥·(target − anchor)` (and `along = û·d` here and in the
        /// tangent arc; `rides` measures its advance from the chain head
        /// instead), then a banded decision — under three different
        /// predicate keys. They are kept separate deliberately rather
        /// than shared, because the keys are the point: this site
        /// classifies `across` as a declared target's MISS
        /// (`path_continuation_target_offset`, an authored-data
        /// disagreement), the tangent-arc site classifies the same
        /// number as a degenerate-arc condition, and the run-out site as
        /// which step an emitted segment is named for
        /// (`path_run_out_carrier`). The funnel key is what tells a
        /// margin telemetry reader which question was being answered, so
        /// collapsing them would lose the distinction that makes the
        /// funnel worth having.
        pub(super) fn lands_on_ray(
            &self,
            at: Point2<T>,
            target: Point2<T>,
            tol: Tol,
        ) -> Result<T, PathError<T>> {
            let run = self.run_from(at);
            let across = run.across(target);
            let along = run.along(at, target);
            let band = linear_band(tol)?;
            match decide("path_continuation_target_offset", Margin::of(across), band) {
                Ok(Sign::Zero) => {}
                Ok(_) => return Err(PathError::ContinuationTargetOffRay { across, along }),
                Err(source) => return Err(PathError::Escalated { source }),
            }
            match decide("path_leg_length", Margin::of(along), band) {
                Ok(Sign::Positive) => Ok(along),
                Ok(_) => Err(PathError::NonpositiveLeg { length: along }),
                Err(source) => Err(PathError::Escalated { source }),
            }
        }
    }
}

/// A junction bound but not yet read: the arriving leg's data, the
/// departure, and the tolerance it is read at. [`Core::set_leaving`]
/// reads it once the departing leg is stored, so the junction is read
/// at both legs' arms ([`junction_check`]).
#[derive(Clone, Copy, Debug)]
struct PendingJunction<T: Real> {
    inc: Incoming<T>,
    dep: Dir<T>,
    tol: Tol,
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
    ang: Option<Bound<T>>,
    /// Whether the bound angle was inherited by `.tangent()` (the
    /// constructed-tangency continuations; drives the §4 item 4 checks).
    ang_by_tangent: bool,
}

/// The bound position bit: the point plus (directed flavor only) the
/// incoming intrinsic data.
#[derive(Clone, Debug)]
struct PosData<T: Real> {
    at: Point2<T>,
    incoming: Option<Incoming<T>>,
}

/// **What follows a fillet arc on its arrival side.**
///
/// A straight arrival's run out is drawn by whichever later step
/// continues along the arrival ray, so it waits as a
/// [`PendingRunOut`] and [`PendingRunOut::rides`] decides each
/// candidate emission from its geometry (N1's "two pieces on one
/// carrier"). A fused verb's authored arrival arc IS its run out, and
/// the site that emits it claims it so ([`Core::claim`]) — a later
/// binder step can be the one lowering it, and a geometric reading
/// would stand between the arc and its name.
#[derive(Clone, Copy, Debug)]
enum ArrivalCarrier<T: Real> {
    /// A straight arrival: the line through the arrival anchor along
    /// the arrival direction — the tip's own departing ray, so a
    /// straight continuation's miss is measured from the same point
    /// along the same direction here as where it was accepted.
    Ray { anchor: Point2<T>, dir: Dir<T> },
    /// A fused verb's authored arrival arc, claimed at its emission.
    AuthoredArc,
}

/// A fillet whose run out may be the next emission: the step that
/// authored its radius, the arrival ray the run rides, and the
/// tolerance witness the riding is decided under.
#[derive(Clone, Copy, Debug)]
struct PendingRunOut<T: Real> {
    /// The step the fillet's pieces are credited to.
    step: usize,
    /// The arrival ray: the line through the arrival anchor along the
    /// arrival direction, read through the one anchored-ray reading
    /// ([`Run`]).
    ray: Run<T>,
    /// The witness of the resolution that emitted the fillet arc.
    tol: Tol,
}

impl<T: Decide> PendingRunOut<T> {
    /// Whether the segment of `kind` from `from` to `to` lies on this
    /// run's arrival ray, continuing it from the chain head `from`: a
    /// straight segment whose end is on the ray's line (the lateral
    /// miss from the fillet's arrival anchor, the same reading
    /// `lands_on_ray` takes from its run's anchor, [`Run::across`]) and
    /// whose advance from the head is definitely positive. An arc never
    /// rides a ray.
    ///
    /// Every margin is decided under `path_run_out_carrier` and an
    /// undecidable one escalates: a name is a durable locator, so a
    /// guess would silently move it to another step's segment.
    fn rides(&self, kind: FirstSeg, from: Point2<T>, to: Point2<T>) -> Result<bool, PathError<T>> {
        if kind != FirstSeg::Line {
            return Ok(false);
        }
        let band = linear_band(self.tol)?;
        let on = |margin: Margin<T>| match decide("path_run_out_carrier", margin, band) {
            Ok(sign) => Ok(sign),
            Err(source) => Err(PathError::Escalated { source }),
        };
        if on(Margin::of(self.ray.across(to)))? != Sign::Zero {
            return Ok(false);
        }
        Ok(on(Margin::of(self.ray.along(from, to)))? == Sign::Positive)
    }
}

/// What kind of segment leaves the entry vertex — pinned at first
/// emission from the emitting verb's kind, so the seam knows side 1's
/// kind structurally (an arc verb whose lowering stores a line, a zero
/// bulge, is an arc side all the same).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FirstSeg {
    NotYet,
    Line,
    Arc,
}

/// Chain-side bookkeeping for an opened fillet: the authored ADDRESS
/// its radius arguments are recorded at (`bound_at`,
/// `incoming_radius`) and the §4 item 4 zero-fit knife-edge data
/// (`by_tangent`, `origin_incoming`, `extends_carrier`). This is NOT
/// part of the kernel's [`verbs::Pending`] state value: the kernel
/// cannot name it, which is what keeps the verbs pure (§2c round 12).
/// It is held WITH the pending side it describes ([`Core::pending`]),
/// so the two cannot be one without the other.
#[derive(Clone, Debug)]
struct PendingMeta<T: Real> {
    /// **The authored step this fillet was BOUND at**, in program
    /// order. The radius is that step's argument wherever the arc is
    /// finally emitted: a `fillet(r)` binds on one step and its arc is
    /// emitted by the arrival step, and the emission record names the
    /// binder, because that is where the radius a reader would edit
    /// lives.
    bound_at: usize,
    /// Whether the INCOMING side's carrier was authored by a radius
    /// argument (a `Radius`, `Sweep` or `ArcLen` spec). A bulge, a
    /// through-point or a centre authors the same carrier from no
    /// radius at all, so the arc it emits has no address to record.
    incoming_radius: bool,
    /// The ray was bound by `.tangent()` (or ray-extended off a leg
    /// end): its origin joint is already constructed.
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
/// constructed joints, the entry pose (the [`Start`] value), and the
/// pending fillet, if a side is open.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Core<T: Real> {
    /// Each vertex with the arc its construction built for the segment
    /// leaving it, `None` for a line (and for the chain's last vertex
    /// until its segment is set).
    verts: Vec<(Point2<T>, Option<BuiltArc<T>>)>,
    /// The joints the chain's constructors made tangent
    /// ([`ConstructedLoop::constructed_joints`]).
    tangent: Vec<usize>,
    start_pos: Option<Point2<T>>,
    start_ang: Option<Dir<T>>,
    first_seg: FirstSeg,
    /// The opened fillet side and its chain-side bookkeeping. ONE
    /// field because they are one fact with one lifetime: every door
    /// that opens a fillet writes both, every resolution takes both,
    /// and a side without its bookkeeping — or bookkeeping without its
    /// side — is a state no door can leave the chain in because it is
    /// unrepresentable, rather than one [`Core::take_pending`] has to
    /// refuse.
    pending: Option<(verbs::Pending<T>, PendingMeta<T>)>,
    /// Every fillet arc emitted into the chain, as the index of the
    /// vertex it LEAVES paired with the radius that was asked for —
    /// what [`Core::fillets_carry_their_tangency`] re-reads at the
    /// close, and the radius its sentence names.
    ///
    /// Not `radii` below in another spelling, and deliberately not
    /// folded into it: this one is a CLOSE-TIME CHECK keyed by vertex
    /// and carrying the scalar radius the sentence prints, that one is
    /// the authored ADDRESS of every radius-drawn arc — fillet and
    /// carrier alike — and carries no scalar at all.
    fillet_arcs: Vec<(usize, T)>,
    /// Which segment each authored radius drew, in emission order —
    /// recorded as the pass emits, and reported beside the spans
    /// ([`Core::finish`]). See [`crate::structure::RadiusEmission`].
    radii: Vec<crate::structure::RadiusEmission>,
    /// **Profiles-as-programs (v2)**: the authoring verbs, recorded as
    /// they lower. Each binder pushes exactly its own step, so one
    /// chain yields both the lowered loop and its program.
    program: Vec<Step<T>>,
    /// The chain length each recorded step found, parallel to
    /// `program`: `step_starts[j]` is `verts.len()` at the moment step
    /// `j` was recorded, which is BEFORE its own emission — every row
    /// records, then constructs, the ENTRY rows included
    /// ([`Core::record`]). Two consecutive entries bracket the vertices
    /// one step pushed, and so the segments it produced —
    /// [`Core::step_spans`] does that arithmetic once, at the close.
    step_starts: Vec<usize>,
    /// **The piece each emitted segment is**, indexed by the vertex the
    /// segment leaves — written when the segment is set
    /// ([`Core::set_leaving`], every emission's one door) and reported
    /// beside the spans ([`Core::finish`]).
    pieces: Vec<Option<crate::structure::Piece>>,
    /// The role the NEXT emission draws, where a fillet names it: its
    /// run in, its arc, or a fused verb's authored arrival arc as its
    /// run out. `None` is the default, the current step's leg.
    claim: Option<crate::structure::Piece>,
    /// **A fillet whose run out may be the next segment**: set when a
    /// fillet arc is emitted with a straight arrival following it, and
    /// taken by the next emission, which is that run exactly when it
    /// lies on the fillet's arrival ray ([`PendingRunOut::rides`]). An
    /// emission off that ray is not the run out, whatever its kind, and
    /// is the piece of the step that draws it.
    ///
    /// The lattice decides which emissions can follow; the naming reads
    /// each candidate's geometry, so a verb the lattice admits later
    /// names its segment correctly without this field changing.
    run_out: Option<PendingRunOut<T>>,
    /// A junction whose departure is bound and whose departing leg is
    /// not stored yet: its turn is read when that leg is, at the
    /// shorter of the two legs' arms ([`junction_check`]).
    junction: Option<PendingJunction<T>>,
    /// How this lowering treats the discrete decisions inside it:
    /// selecting freely and recording what it selected, or consuming a
    /// prior elaboration's selections and re-verifying each at this
    /// scalar. A chain carries one guide from its entry verb to its
    /// close, so every fillet resolution along it reaches the same one.
    guide: crate::structure::Guide<T>,
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
            fillet_arcs: Vec::new(),
            radii: Vec::new(),
            program: Vec::new(),
            step_starts: Vec::new(),
            pieces: Vec::new(),
            claim: None,
            run_out: None,
            junction: None,
            guide: crate::structure::Guide::recording(),
        }
    }

    /// Installs the guide this lowering runs under, replacing the
    /// fresh recording one [`Core::empty`] mints.
    pub(crate) fn adopt(&mut self, guide: crate::structure::Guide<T>) {
        self.guide = guide;
    }

    /// The structure this lowering selected — taken at the close,
    /// where the chain's core is consumed.
    pub(crate) fn take_structure(&mut self) -> crate::structure::Guide<T> {
        core::mem::replace(&mut self.guide, crate::structure::Guide::recording())
    }

    /// The guide, for the resolution machinery that reads and writes
    /// it.
    pub(crate) fn guide_mut(&mut self) -> &mut crate::structure::Guide<T> {
        &mut self.guide
    }

    /// Records one authoring verb (record-as-you-lower), with the
    /// chain length it starts from — the left end of the span it is
    /// about to produce.
    ///
    /// **Every row records BEFORE it constructs, the entry rows
    /// included.** An entry verb has no core to record into until its
    /// kernel mints one, so the kernel takes the step and records it
    /// there ([`Open::at_kernel`], [`Open::director`]) rather than the
    /// row recording into the path it just built. The uniformity is
    /// what makes `step_starts[j]` mean one thing for every `j`.
    fn record(&mut self, step: Step<T>) {
        self.program.push(step);
        self.step_starts.push(self.verts.len());
    }

    /// Which segments each recorded step produced, in program order.
    ///
    /// Segment `k` leaves vertex `k`, so a chain of `a` vertices has
    /// completed `a - 1` of them — none at all while it is empty or
    /// holds only its seed, which is what `completed` below says and
    /// why it saturates. A step that ran while the chain grew from `a`
    /// to `b` vertices therefore produced `completed(a)..completed(b)`.
    /// The CLOSING step produces one more — the seam segment, which
    /// leaves the chain's last vertex and needs no vertex of its own —
    /// so the final span ends at the vertex count itself.
    ///
    /// `completed` is monotone and `step_starts` is non-decreasing (a
    /// chain only grows), so `start <= end` holds for every span by
    /// construction; [`StepSpan::new`] asserts it rather than this
    /// clamping it back into range.
    fn step_spans(&self) -> Vec<crate::structure::StepSpan> {
        let completed = |verts: usize| verts.saturating_sub(1);
        let closed = self.verts.len();
        self.step_starts
            .iter()
            .enumerate()
            .map(|(j, &a)| {
                let end = match self.step_starts.get(j + 1) {
                    Some(&b) => completed(b),
                    None => closed,
                };
                crate::structure::StepSpan::new(completed(a), end)
            })
            .collect()
    }

    /// Seeds the entry vertex (the chain's provisional first vertex —
    /// a seam fillet may later retrim it to the seam arc's end).
    fn seed(&mut self, p: Point2<T>) {
        self.verts.push((p, None));
        self.start_pos = Some(p);
    }

    /// Claims the next emission as `role` of the fillet bound at
    /// `step`.
    fn claim(&mut self, step: usize, role: crate::structure::PieceRole) {
        self.claim = Some(crate::structure::Piece { step, role });
    }

    /// The chain's entry vertex — where a closing segment ends.
    fn entry(&self) -> Result<Point2<T>, PathError<T>> {
        self.verts
            .first()
            .map(|&(pos, _)| pos)
            .ok_or(PathError::UnderdeterminedLeg {
                site: "entry of an empty chain",
            })
    }

    /// The first leg's arm: the departing leg at the seam junction.
    fn first_leg_arm(&self) -> Result<T, PathError<T>> {
        match self.verts.as_slice() {
            [(a, arc), (b, _), ..] => {
                Ok(leg_arm(carrier_of(*arc), (*b - *a).norm_squared().sqrt()))
            }
            _ => Err(PathError::UnderdeterminedLeg {
                site: "a seam junction on a chain with no first leg",
            }),
        }
    }
}

impl<T: Decide> Core<T> {
    /// Reads the junction at the chain's head, the departure `dep` off
    /// the arriving leg `inc`, at the arriving leg's arm, and binds it
    /// until the departing leg is stored ([`PendingJunction`]), where it
    /// is read again at the shorter of the two arms: that second reading
    /// is the junction's verdict, the one validation reads. The first
    /// refuses at the verb that authored the direction what the second
    /// would refuse anyway — a turn Zero or in band at the arriving arm
    /// is Zero or in band at any shorter one.
    fn pend_junction(
        &mut self,
        inc: &Incoming<T>,
        dep: Dir<T>,
        tol: Tol,
    ) -> Result<(), PathError<T>> {
        junction_check(inc, dep, inc.arm, false, tol)?;
        self.junction = Some(PendingJunction {
            inc: *inc,
            dep,
            tol,
        });
        Ok(())
    }

    /// Sets the segment leaving the current last vertex for `to` — a
    /// line, or the arc its construction built — plus the structural
    /// first-segment kind pin, the emitting verb's `kind`. `to` is
    /// where the segment ends: the vertex a push is about to append, or
    /// the entry vertex a close returns to.
    fn set_leaving(
        &mut self,
        arc: Option<BuiltArc<T>>,
        kind: FirstSeg,
        to: Point2<T>,
    ) -> Result<(), PathError<T>> {
        if let Some(pending) = self.junction.take() {
            let from = self.head()?;
            let leaving_arm = leg_arm(carrier_of(arc), (to - from).norm_squared().sqrt());
            junction_check(&pending.inc, pending.dep, leaving_arm, false, pending.tol)?;
        }
        if self.verts.len() == 1 && self.first_seg == FirstSeg::NotYet {
            self.first_seg = kind;
        }
        let from = match self.verts.last_mut() {
            Some((from, leaving)) => {
                *leaving = arc;
                *from
            }
            None => {
                return Err(PathError::UnderdeterminedLeg {
                    site: "set_leaving on an empty chain",
                });
            }
        };
        self.attribute(self.verts.len() - 1, kind, from, to)
    }

    /// Sets the CLOSING segment, the one leaving the last vertex for
    /// the entry vertex.
    fn close_leaving(
        &mut self,
        arc: Option<BuiltArc<T>>,
        kind: FirstSeg,
    ) -> Result<(), PathError<T>> {
        let to = self.entry()?;
        self.set_leaving(arc, kind, to)
    }

    /// **Names the segment leaving vertex `segment`** as the piece it
    /// is (`crate::structure::Piece`): the fillet role a site claimed
    /// for it, or else the current step's leg — and a pending run out
    /// competes for it when the segment rides the fillet's arrival
    /// ray, since that segment is the fillet's run out whichever step
    /// draws it. A segment named before (a closing segment re-set)
    /// keeps its earlier name where that one outranks.
    fn attribute(
        &mut self,
        segment: usize,
        kind: FirstSeg,
        from: Point2<T>,
        to: Point2<T>,
    ) -> Result<(), PathError<T>> {
        use crate::structure::{Piece, PieceRole};
        let claimed = self.claim.take();
        let mut best = match claimed {
            Some(c) => c,
            None => Piece {
                step: self.current_step()?,
                role: PieceRole::Leg,
            },
        };
        let run_out = self.run_out.take();
        if best.role != PieceRole::Arc
            && let Some(run) = run_out
            && run.rides(kind, from, to)?
        {
            let candidate = Piece {
                step: run.step,
                role: PieceRole::RunOut,
            };
            if candidate.outranks(&best) {
                best = candidate;
            }
        }
        if self.pieces.len() <= segment {
            self.pieces.resize(segment + 1, None);
        }
        if let Some(Some(earlier)) = self.pieces.get(segment)
            && earlier.outranks(&best)
        {
            best = *earlier;
        }
        self.pieces[segment] = Some(best);
        Ok(())
    }

    /// Appends a straight segment to `p` (the raw `line_to`).
    fn push_line(&mut self, p: Point2<T>) -> Result<(), PathError<T>> {
        self.set_leaving(None, FirstSeg::Line, p)?;
        self.verts.push((p, None));
        Ok(())
    }

    /// Appends the arc segment its construction built, ending at `p`.
    fn push_arc(&mut self, p: Point2<T>, arc: BuiltArc<T>) -> Result<(), PathError<T>> {
        self.push_leg(p, Some(arc))
    }

    /// Appends an arc verb's leg ending at `p`: the arc its
    /// construction built, or a line where the mode's lowering rule
    /// stores one. Its first-segment kind is the verb's, an arc.
    fn push_leg(&mut self, p: Point2<T>, leg: Option<BuiltArc<T>>) -> Result<(), PathError<T>> {
        self.set_leaving(leg, FirstSeg::Arc, p)?;
        self.verts.push((p, None));
        Ok(())
    }
}

impl<T: Real> Core<T> {
    /// The current chain end.
    fn head(&self) -> Result<Point2<T>, PathError<T>> {
        self.verts
            .last()
            .map(|&(pos, _)| pos)
            .ok_or(PathError::UnderdeterminedLeg {
                site: "head of an empty chain",
            })
    }

    /// Constructs the joint at the current last vertex tangent.
    fn construct_last(&mut self) {
        if let Some(last) = self.verts.len().checked_sub(1) {
            self.tangent.push(last);
        }
    }

    /// Records that the radius argument at `(step, role)` drew the arc
    /// about to be set, and hands back the segment it
    /// recorded — the one leaving the chain's last vertex, which is the
    /// one spelling for every emission: the interior arc that pushes
    /// its end vertex, the seam arc that retrims the entry vertex, and
    /// the closing arc that does neither.
    ///
    /// **The one place a radius emission is addressed.** Called
    /// immediately BEFORE the emission, so the index it reads is the
    /// segment the arc becomes; an arc no radius argument authored
    /// never comes here, because there is no address to record.
    ///
    /// An arc leaves a vertex, so a chain with no vertices cannot be
    /// about to emit one; the door says so rather than recording
    /// nothing and letting a reader see a shorter list than the loop
    /// has arcs.
    fn record_radius(
        &mut self,
        step: usize,
        role: crate::structure::RadiusRole,
    ) -> Result<usize, PathError<T>> {
        let segment = self
            .verts
            .len()
            .checked_sub(1)
            .ok_or(PathError::UnderdeterminedLeg {
                site: "an arc emitted onto an empty chain",
            })?;
        self.radii.push(crate::structure::RadiusEmission {
            step,
            role,
            segment,
        });
        Ok(segment)
    }

    /// Records the fillet arc of `radius` about to be emitted: its
    /// authored address, through [`Core::record_radius`] like every
    /// other radius-drawn arc, AND the close-time tangency re-read the
    /// address does not carry. Hands back the vertex index both were
    /// recorded at, so the caller emits onto the segment it just
    /// claimed.
    ///
    /// **Every fillet arc in the chain goes through here**, which is
    /// what makes [`Core::fillets_carry_their_tangency`] a reading of
    /// all of them rather than of the ones a site remembered to push.
    fn record_fillet_arc(&mut self, radius: T, bound_at: usize) -> Result<usize, PathError<T>> {
        let leaving = self.record_radius(bound_at, crate::structure::RadiusRole::Fillet)?;
        self.fillet_arcs.push((leaving, radius));
        self.claim(bound_at, crate::structure::PieceRole::Arc);
        Ok(leaving)
    }

    /// The step being lowered: the last one recorded, since every row
    /// records before it constructs ([`Core::record`]).
    ///
    /// A chain with no recorded step has nothing under construction, so
    /// an emission claiming one is an internal invariant break rather
    /// than a step 0 to assume.
    fn current_step(&self) -> Result<usize, PathError<T>> {
        self.program
            .len()
            .checked_sub(1)
            .ok_or(PathError::UnderdeterminedLeg {
                site: "an arc emitted before any step was recorded",
            })
    }

    /// Constructs the SEAM joint — joint 0, the entry vertex — tangent,
    /// which validation verifies. EVERY tangent arrival lands here:
    /// every zero-turn joint is a tangent joint (Ev, in-chat,
    /// 2026-09-02; D1), so there is no second kind of construction to
    /// sort.
    ///
    /// The seam fillet does the same push inline when its arc IS the
    /// closing segment.
    fn construct_seam(&mut self) {
        if !self.tangent.contains(&0) {
            self.tangent.push(0);
        }
    }

    /// Finishes the loop, returning it PAIRED with the program that
    /// produced it (see [`ClosedLoop`]): each vertex with the segment
    /// its construction stored, every arc's proved endpoint facts
    /// registered at the run's `tol` on the vertices the chain closed
    /// with ([`BuiltArc::register`]).
    fn finish(mut self, tol: Tol) -> ClosedLoop<T> {
        let spans = self.step_spans();
        let radii = core::mem::take(&mut self.radii);
        let pieces = self.segment_pieces();
        let structure = self.take_structure();
        let structure = structure.into_record(spans, radii, pieces);
        structure.check_role_lists(&self.program);
        let mut joints = core::mem::take(&mut self.tangent);
        joints.sort_unstable();
        joints.dedup();
        ClosedLoop {
            loop_: ConstructedLoop {
                loop_: ProfileLoop::from_chain(self.stored_chain(Some(tol))),
                joints,
            },
            program: self.program,
            structure,
        }
    }

    /// The chain as (vertex, stored segment) pairs, each arc's proved
    /// endpoint facts registered at `tol` when there is one.
    fn stored_chain(&self, tol: Option<Tol>) -> Vec<(Point2<T>, Segment<T>)> {
        let n = self.verts.len();
        (0..n).map(|k| self.stored_segment(k, tol)).collect()
    }

    /// Vertex `k` and the segment leaving it, as stored.
    fn stored_segment(&self, k: usize, tol: Option<Tol>) -> (Point2<T>, Segment<T>) {
        let n = self.verts.len();
        let (start, leaving) = self.verts[k];
        let segment = match leaving {
            None => Segment::Line,
            Some(built) => {
                if let Some(tol) = tol {
                    built.register(start, self.verts[(k + 1) % n].0, tol);
                }
                Segment::Arc(built.arc)
            }
        };
        (start, segment)
    }

    /// The piece every segment of the closed chain is, in segment
    /// order. Every segment of a closed chain was set, and so was named
    /// ([`Core::set_leaving`]); a segment that was not would
    /// be an emission that bypassed the door.
    fn segment_pieces(&self) -> Vec<crate::structure::Piece> {
        (0..self.verts.len())
            .map(|k| {
                self.pieces.get(k).copied().flatten().unwrap_or_else(|| {
                    unreachable!(
                        "every segment of a closed chain is emitted through set_leaving, which \
                         names it; segment {k} was not"
                    )
                })
            })
            .collect()
    }
}

// ------------------------------------------------------------------
// Shared classification helpers (every decision through the reified
// predicate funnel; margins in meters).
// ------------------------------------------------------------------

/// The run's linear classification band **(ε, K·ε = ε_input)**, read
/// through the caller's [`Tol`] witness — the zero edge at the
/// precision tolerance, the escalate edge at the input tolerance.
///
/// Under D4's two-tolerance principle ε_input is a ROLE, not a third
/// dial: it IS K·ε, the escalating edge. So the band's two edges are ε
/// and ε_input, which is what lets a reader check this body against
/// the doc.
///
/// This is the module's one home for that band paired with
/// [`PathError::Band`]; every path decision needing it comes here
/// rather than re-spelling the mapping.
fn linear_band<T: Real>(tol: Tol) -> Result<Band, PathError<T>> {
    Band::linear(tol).map_err(PathError::Band)
}

/// What [`Core::record_fillet_arc`]'s answer means, checked beside
/// every emission that takes it: the recorded index is the vertex the
/// arc leaves, which an interior emission leaves two from the end (it
/// pushed the arc's end vertex) and a seam emission leaves last (it
/// retrimmed the entry vertex instead). The pairing is what lets the
/// close find the arc again, and a debug assertion is where it is held
/// rather than in a comment.
const PAIRED: &str = "a recorded fillet arc must be the vertex its emission leaves";

/// Unit direction of an angle.
fn unit<T: Real>(ang: T) -> Vec2<T> {
    let (s, c) = ang.sin_cos();
    Vec2::new(c, s)
}

/// **The arc about a known centre**: Δθ from `a` to `b` about `centre`
/// on the carrier of `radius`, turning in the `winding` sense, spelled
/// `4·atan(X)` with X the [`crate::sugar::quarter_tan_about`]
/// [`crate::bulge_from_center`] reads too — accurate from a short arc
/// to a near full turn.
fn center_sweep<T: Real>(
    a: Point2<T>,
    b: Point2<T>,
    centre: Point2<T>,
    radius: T,
    winding: ArcSweep,
) -> T {
    T::from_f64(4.0)
        * crate::sugar::quarter_tan_about(a, b, centre, radius, winding.sign::<T>()).atan()
}

/// **A fillet side's carrier run**: the arc from `a` to `b` about the
/// side's `centre`, on the side's circle of `radius` (the mode's own
/// spelling of it, [`verbs::PendingArc::circle_radius`]), in the side's
/// `winding` — the sweep [`center_sweep`] — with the `facts` its caller
/// proved.
///
/// **Its endpoint facts are theorems** ([`crate::Facts::Registered`])
/// wherever both ends lie on that circle by construction: the fillet
/// resolution puts its tangent point on the side's circle, and the
/// other end is the anchor itself or a point the incoming mode's
/// algebra put on the same circle; the quarter-tangent [`center_sweep`]
/// reads is then tan(θ/4) exactly. A caller with an end on it only to a decision passes
/// [`crate::Facts::Decided`].
fn side_arc<T: Real>(
    a: Point2<T>,
    b: Point2<T>,
    centre: Point2<T>,
    radius: T,
    winding: ArcSweep,
    facts: crate::Facts,
) -> BuiltArc<T> {
    BuiltArc::new(
        Arc2 {
            centre,
            radius,
            sweep: center_sweep(a, b, centre, radius, winding),
        },
        facts,
    )
    .sensed(winding)
}

/// **The `Center` mode's one conversion**: the arc from the tip `a`
/// to `b` about the AUTHORED `centre`, on the circle through `a` —
/// the radius `‖a − centre‖` (the rim the door gated), the sweep
/// [`center_sweep`]. `b` lies on that circle only to the door's own
/// `path_arc_center_equidistant` decision, so nothing is registered
/// ([`crate::Facts::Decided`]).
fn center_arc<T: Real>(
    a: Point2<T>,
    b: Point2<T>,
    centre: Point2<T>,
    winding: ArcSweep,
) -> BuiltArc<T> {
    side_arc(
        a,
        b,
        centre,
        (a - centre).norm(),
        winding,
        crate::Facts::Decided,
    )
}

/// **The `Via` mode's one conversion**: the arc from `a` through `q` to
/// `b`, lowered from its chord ([`crate::bulge_leg`]) with X =
/// [`crate::bulge_from_via`], tan(θ/4) of the turn at `q` from `q − a`
/// to `b − q` (by the inscribed angle, half the included angle). The
/// turn reaches ±π only with `q` on the chord's line outside it, which
/// the door's collinear gate refuses first.
fn via_arc<T: Real>(a: Point2<T>, q: Point2<T>, b: Point2<T>) -> Option<BuiltArc<T>> {
    crate::bulge_leg(a, b, crate::bulge_from_via(a, q, b)).map(|built| built.through(Some(q)))
}

/// **The tangent arc's one conversion**: the arc leaving `a` along the
/// unit `u` and ending at `b`, off the departure line (the line is
/// [`tangent_arc_geom`]'s decision, not this function's). Its carrier is
/// the circle tangent to `u` at `a` through `b`: the centre `a + n̂·ρ`
/// (n̂ the left normal of `u`) at the signed offset
/// `ρ = |d|² / (2·across)`, the radius `|ρ|`, with `d = b − a` resolved
/// along and across `u` — no root. Δθ = 4·atan(X) with X the tangent of
/// half the angle δ from `u` to the chord (the arc turns 2δ, so X is
/// tan(θ/4)), [`crate::sugar::split_half_tan`]`(across, along, |d|)`.
///
/// **Its endpoint facts are theorems** ([`crate::Facts::Registered`]):
/// `‖a − c‖ = |ρ|` because n̂ is a unit normal, and
/// `‖b − c‖² = |d|² − 2ρ·across + ρ² = ρ²` by the choice of ρ; the
/// centre lies on the chord's perpendicular bisector, so the included
/// angle about it is 2δ, whose quarter-tangent X is.
fn tangent_arc<T: Real>(a: Point2<T>, u: Vec2<T>, b: Point2<T>) -> BuiltArc<T> {
    let d = b - a;
    let (along, across) = (u.dot(d), u.perp_dot(d));
    let x = crate::sugar::split_half_tan(across, along, d.norm());
    let rho = d.norm_squared() / (T::from_f64(2.0) * across);
    BuiltArc::new(
        Arc2 {
            centre: a + Vec2::new(-u.y, u.x) * rho,
            radius: rho.abs(),
            sweep: T::from_f64(4.0) * x.atan(),
        },
        crate::Facts::Registered,
    )
}

/// **The tangent arc's geometry**: the leg from `at` leaving along
/// `dep` to `p` ([`tangent_arc`]), its end tangent (the departure
/// reflected across the chord), and the degeneracies only the declared
/// departure can name. A `p` DECIDED on the departure line is the
/// straight segment ahead of `at`, and no arc behind it
/// ([`PathError::DegenerateArcChord`], as is `p` at `at`); a `p` the
/// band cannot place on or off the line escalates. `tangent_arc_to`
/// runs it from its tip; `arc_onto` runs it from its target, backwards
/// ([`onto_arc_geom`]).
///
/// **Sibling measurement, cross-declared** (R1 S1): the
/// `d`/`along`/`across`/decide opening here is the same displacement
/// [`Incoming::lands_on_ray`] and [`PendingRunOut::rides`]'s ray
/// arm compute, each under its own predicate key. See `lands_on_ray`
/// for why the three are kept apart rather than shared.
fn tangent_arc_geom<T: Decide>(
    at: Point2<T>,
    dep: Dir<T>,
    p: Point2<T>,
    tol: Tol,
) -> Result<TangentArcGeom<T>, PathError<T>> {
    let d = p - at;
    let u = dep.unit;
    let along = u.dot(d);
    let across = u.perp_dot(d);
    // A target ON the departure line is not a carrier question —
    // the 2026-09-02 ruling took those away — but it is still a
    // GEOMETRY one, and only forward of the tip is it answerable:
    // ahead of the tip the leg is the straight segment the declaration
    // asks for, and behind it no arc spans the chord. The decision is
    // the one the emission follows: an arc built off a target decided
    // on the line would read its radius off a sub-band `across` — a
    // carrier ~1e16 across at f64, an unbounded enclosure at Interval.
    let band = linear_band(tol)?;
    // The target's side of the departure line is the arc's turn sense:
    // left of the departure is counter-clockwise.
    let (on_line, sense) = match decide("path_collinear_target", Margin::of(across), band) {
        Ok(Sign::Zero) => (true, ArcSweep::Ccw),
        Ok(Sign::Positive) => (false, ArcSweep::Ccw),
        Ok(Sign::Negative) => (false, ArcSweep::Cw),
        Err(source) => return Err(PathError::Escalated { source }),
    };
    if on_line {
        match decide("path_leg_length", Margin::of(along), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(PathError::DegenerateArcChord {
                    chord: d.norm_squared().sqrt(),
                });
            }
            Err(source) => return Err(PathError::Escalated { source }),
        }
    }
    let arc = (!on_line).then(|| tangent_arc(at, u, p).sensed(sense));
    let chord = d.norm_squared().sqrt();
    let end_ang = reflected_across(u, d);
    Ok(TangentArcGeom {
        arc,
        end_ang,
        carrier: carrier_of(arc),
        chord,
    })
}

/// **`arc_onto`'s geometry: the tangent arc, run backwards.** The arc
/// from `at` to `p` arriving along `arrival` is the tangent arc leaving
/// `p` along the REVERSED arrival and ending at `at`, traversed the
/// other way: the same carrier, the sweep negated, and the departure
/// the reverse of that arc's end tangent. So `tangent_arc_to`'s kernel
/// and refusals serve both verbs unchanged, the authored arrival is the
/// one direction that enters the carrier: the geometry's `end_ang` is
/// `arrival` itself, and the second value is the DERIVED DEPARTURE at
/// `at` (the chord reflection of `arrival`).
fn onto_arc_geom<T: Decide>(
    at: Point2<T>,
    arrival: Dir<T>,
    p: Point2<T>,
    tol: Tol,
) -> Result<(TangentArcGeom<T>, Dir<T>), PathError<T>> {
    let back = tangent_arc_geom(p, arrival.reversed(), at, tol)?;
    let arc = back.arc.map(|built| BuiltArc {
        arc: Arc2 {
            sweep: -built.arc.sweep,
            ..built.arc
        },
        sense: built.sense.map(ArcSweep::reversed),
        ..built
    });
    let geom = TangentArcGeom {
        arc,
        end_ang: arrival,
        carrier: back.carrier,
        chord: back.chord,
    };
    Ok((geom, back.end_ang.reversed()))
}

/// **A tangent arc's end direction**: the arc leaves along the unit `u`
/// and arrives along `u` reflected across its chord `d` (the
/// tangent-chord angle is the same at both ends), spelled on the ray
/// with no angle read and no root: `2(u·d)d/(d·d) − u`. A reflection
/// keeps length, so the ray is unit over the reals wherever `u` is, and
/// is not normalized again ([`Dir::from_unit`]).
fn reflected_across<T: Real>(u: Vec2<T>, d: Vec2<T>) -> Dir<T> {
    Dir::from_unit(d * (T::from_f64(2.0) * u.dot(d) / d.norm_squared()) - u)
}

/// The tip's record of a leg's carrier, read off the stored arc (none
/// for a line).
fn carrier_of<T: Real>(leg: Option<BuiltArc<T>>) -> Option<ArcData<T>> {
    leg.map(|built| ArcData {
        center: built.arc.centre,
        radius: built.arc.radius,
    })
}

/// A leg's signed sweep: its arc's, or zero for a line.
fn sweep_of<T: Real>(leg: Option<BuiltArc<T>>) -> T {
    leg.map_or_else(T::zero, |built| built.arc.sweep)
}

/// §4 item 1, one generic function: classifies the departure `dep`
/// against the incoming tangent and its reverse, levered by the shorter
/// of the arriving leg's arm and the departing leg's `leaving_arm` —
/// the two legs lie side by side only over the shorter one, and it is
/// the arm validation reads the same joint at ([`seg::joint_tangency`]).
/// An interior junction is read when its departing leg is stored
/// ([`Core::pend_junction`]); the seam's departing leg is the first,
/// stored already. `seam` says "this junction is the loop's SEAM, so classify it as
/// one" — nothing more. It is a plain flag rather than a site enum
/// because there is only one special site left: a tangent DEPARTURE on
/// a closing leg is geometrically identical to one mid-chain and now
/// refuses identically ([`PathError::JunctionTangent`]), so the only
/// thing a caller still has to say is whether the junction in hand is
/// the seam.
///
/// Every seam ARRIVAL passes `true`, whatever leg arrives. The flag is
/// the only thing left that says "this junction is the loop's seam",
/// and a seam arrival has a recourse no departure has: the arriving leg
/// is the later-authored one, so the entry cannot carry `.tangent()`
/// (§2's entry rule) and the declaration rides the TARGET instead
/// ([`Start::arrives_tangent`]). Naming
/// that fact `JunctionTangent` would send the reader to a spelling the
/// seam does not have.
fn junction_check<T: Decide>(
    inc: &Incoming<T>,
    dep: Dir<T>,
    leaving_arm: T,
    seam: bool,
    tol: Tol,
) -> Result<(), PathError<T>> {
    let band = linear_band(tol)?;
    let u_in = inc.ang.unit;
    let u_dep = dep.unit;
    let arm = inc.arm.min(leaving_arm);
    match seg::junction_turn(
        "path_junction_turn",
        u_in,
        u_dep,
        arm,
        seg::TurnSense::Signed,
        band,
    ) {
        Ok((turn, margin)) if turn.sign == Sign::Zero => {
            // Which refusal class — tangent (dep ≈ incoming) or cusp
            // (dep ≈ reverse)? `path_junction_side`, in its one home
            // (validation asks it of every tangent joint between two
            // carriers too).
            match seg::junction_reverses(u_in, u_dep, arm, band) {
                Ok(true) => Err(PathError::JunctionCusp { margin, arm }),
                Ok(false) => {
                    if seam {
                        Err(PathError::SeamTangent { margin })
                    } else {
                        Err(PathError::JunctionTangent { margin, arm })
                    }
                }
                Err(source) => Err(PathError::Escalated { source }),
            }
        }
        Ok(_) => Ok(()),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// What a closing leg's TARGET says about the seam's joint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SeamArrival {
    /// [`Start`]: the seam is classified as every junction is, and a
    /// zero turn refuses [`PathError::SeamTangent`].
    Undeclared,
    /// [`Start::arrives_tangent`] on a leg whose data fix its end
    /// direction: the arriving direction is CHECKED
    /// ([`seam_arrival_check`]) and the joint constructed.
    Checked,
    /// [`Start::arrives_tangent`] on a leg whose end direction the
    /// token SUPPLIED (`arc_onto`, the `Via` arrival): the joint is
    /// tangent by construction, and nothing is checked.
    Supplied,
}

/// Classifies the seam's joint as the target said ([`SeamArrival`]).
fn seam_joint<T: Decide>(
    core: &mut Core<T>,
    how: SeamArrival,
    arriving: &Incoming<T>,
    start_ang: Dir<T>,
    tol: Tol,
) -> Result<(), PathError<T>> {
    match how {
        SeamArrival::Undeclared => {
            junction_check(arriving, start_ang, core.first_leg_arm()?, true, tol)
        }
        SeamArrival::Checked => {
            seam_arrival_check(
                arriving.ang,
                arriving.arm,
                core.first_leg_arm()?,
                start_ang,
                tol,
            )?;
            core.construct_seam();
            Ok(())
        }
        SeamArrival::Supplied => {
            core.construct_seam();
            Ok(())
        }
    }
}

/// **The declared seam ARRIVAL's check** — the inverted twin of
/// [`junction_check`]'s tangent arm, and the mirror of
/// [`Incoming::lands_on_ray`]: there the ray is declared and the
/// TARGET checked against it, here the arrival is declared and the
/// arriving DIRECTION checked against the entry's outgoing one.
///
/// The verdict inverts with the declaration, which is the whole content
/// of the fifth-round ruling: undeclared, a zero-turn seam refuses
/// ([`PathError::SeamTangent`]); declared, a zero-turn seam is what the
/// author said and CLOSES, while a definite turn is authored data
/// disagreeing with itself and refuses
/// ([`PathError::SeamArrivalOffDirection`]).
///
/// **The token classifies the JOINT, and this check consults NOTHING
/// about the following carrier** (Ev, in-chat, 2026-09-02). The only
/// entry-side data it reads is `Start`'s own binding bits — its
/// direction — which is exactly what the interior junction check reads
/// of a directed point. The seam's other half is the FOLLOWING leg, and
/// the chain has no business seeing it. **Every zero-turn joint is a
/// tangent joint** (Ev, in-chat, 2026-09-02; D1): whether the seam's
/// two sides ride one carrier or two, a constructed zero-turn joint is
/// tangent, so there is nothing here to ask about carriers and
/// nothing to sort.
///
/// The datum is `sin` of the turn — dimensionless — so comparing it
/// against a LENGTH tolerance is a category error until an arm says
/// what it displaces. It is read as every junction's turn is
/// ([`seg::junction_turn`], under the seam's own name), LEVERED by the
/// shorter of the arriving leg's arm and the first leg's — the lever
/// §4 item 1 and validation use at this very vertex — and the product
/// is, TO FIRST ORDER, the lateral displacement the misalignment opens
/// over the stretch where the two legs lie side by side (for an arc
/// leg the exact figure is `s·sin φ + s²/2R`; the lever takes the
/// leading term). That is the deviation ε_input is defined about. The
/// first leg's extent is the one thing read of the following side; its
/// carrier is not. An arriving lever that is not definitely positive
/// carries no question at all, and refuses.
///
/// The reverse class is not this refusal's: an arrival anti-parallel to
/// the entry's outgoing direction has a near-zero turn too, and it is a
/// cusp — [`PathError::JunctionCusp`], the same name it carries at any
/// other junction. One fact, one refusal.
fn seam_arrival_check<T: Decide>(
    arriving: Dir<T>,
    arriving_arm: T,
    leaving_arm: T,
    start_ang: Dir<T>,
    tol: Tol,
) -> Result<(), PathError<T>> {
    let band = linear_band(tol)?;
    // (i) The lever.
    match decide("path_seam_arrival_lever", Margin::of(arriving_arm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(PathError::SeamArrivalLeverTooShort { arm: arriving_arm }),
        Err(source) => return Err(PathError::Escalated { source }),
    }
    // (ii) The direction the declaration asserts, read as every
    // junction's turn is ([`seg::junction_turn`]) under the seam's own
    // name.
    let arm = arriving_arm.min(leaving_arm);
    match seg::junction_turn(
        "path_seam_arrival_turn",
        arriving.unit,
        start_ang.unit,
        arm,
        seg::TurnSense::Signed,
        band,
    ) {
        Ok((turn, margin)) if turn.sign == Sign::Zero => {
            let side = decide(
                "path_seam_arrival_side",
                Margin::levered(arriving.unit.dot(start_ang.unit), arm),
                band,
            );
            match side {
                Ok(Sign::Negative) => Err(PathError::JunctionCusp { margin, arm }),
                Ok(_) => Ok(()),
                Err(source) => Err(PathError::Escalated { source }),
            }
        }
        Ok((_, margin)) => Err(PathError::SeamArrivalOffDirection { margin, arm }),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// **§2c arc extension's same-carrier decision**: whether the carrier a
/// fused `Radius` incoming derives from a directed point IS that
/// point's own incoming carrier — a d + |Δr| identity margin read as a
/// DECISION rather than as a refusal (the refusing twin,
/// `refuse_identical_carriers`, retired with the 2026-09-02 ruling;
/// this one survives because it CHOOSES A CONSTRUCTION rather than
/// judging a junction): `Zero` continues
/// the arriving leg (the vertex-move exemption), definite non-zero is
/// a new tangent carrier constructed at the tip. Both outcomes are
/// legal spellings, which is what deletes the old mismatched-r hole
/// structurally: every authored `r` names a sound construction.
///
/// It compares two WHOLE circles. [`PendingRunOut::rides`] asks
/// whether a segment lies on a circle, and measures its points'
/// deviations instead: a circle rebuilt from a short chord carries
/// ~ε·R²/chord in its centre.
fn carriers_are_identical<T: Decide>(
    a: &ArcData<T>,
    b: &ArcData<T>,
    tol: Tol,
) -> Result<bool, PathError<T>> {
    let band = linear_band(tol)?;
    let d = (a.center - b.center).norm_squared().sqrt();
    let margin = d + (a.radius - b.radius).abs();
    match decide("path_carrier_identity", Margin::of(margin), band) {
        Ok(Sign::Zero) => Ok(true),
        Ok(_) => Ok(false),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// **An arc leg's lever arm** from its carrier: [`seg::arc_lever`],
/// the one home of that choice. Three of its callers are junction
/// LEVERS, where the choice is a contract rather than an expression.
fn arc_arm<T: Real>(carrier: &ArcData<T>, chord: T) -> T {
    seg::arc_lever(carrier.radius, chord)
}

/// A leg's lever arm: its arc's [`arc_arm`], or the chord itself for a
/// line.
fn leg_arm<T: Real>(carrier: Option<ArcData<T>>, chord: T) -> T {
    carrier.map_or(chord, |carrier| arc_arm(&carrier, chord))
}

/// The straight leg's EMISSION, shared by the two `line(len)` rows —
/// the directed one (a bound departure) and the straight continuation
/// (the directed point's own tangent). Both mint the same thing and
/// must keep minting the same thing: a line vertex at `at + û·len`, a
/// tip whose carrier is None (a line leg leaves no arc carrier behind)
/// and whose lever arm is the emitted segment's own length, measured
/// head-to-end so a side squeezed between two trims measures from the
/// trim point rather than from an authored anchor.
fn emit_straight_leg<T: Decide>(
    core: &mut Core<T>,
    at: Point2<T>,
    ang: Dir<T>,
    len: T,
    run: Run<T>,
) -> Result<Tip<T>, PathError<T>> {
    emit_straight_leg_at(core, at + ang.unit * len, ang, run)
}

/// The same emission where the END is the datum rather than the extent
/// — the declared point-target continuation, whose vertex is the
/// AUTHORED target and not a length walked along the ray.
///
/// Landing the authored point rather than its projection onto the ray
/// is what §4 item 3 asks for (every authored point lies on the final
/// path, authored once), and it is what makes the closer close: `Start`
/// as the target reaches the entry vertex exactly, not to within a
/// band. The RAY is still what the tip carries out, because the ray is
/// the carrier the declaration names. The difference between the leg
/// and the line is the two vertices' accepted misses, one at each end and
/// possibly on opposite sides of it, so at most `2ε` across the leg, each
/// bounded by the check that let its target through.
///
/// **That bound is PER RUN, because the line does not move.** The
/// vertex sits up to one accepted miss off the declared line, so the end
/// point is handed `run`, the line its own target was checked against
/// ([`Incoming::continuing`]): the next continuation's `lands_on_ray`
/// reads that line again, bound at the run's first vertex, never one
/// re-anchored at this vertex. So every vertex of a declared straight
/// run lies within the band of one line, and n accepted misses are n
/// misses of it, never a sum (PATHS-DESIGN §4). A leg whose direction
/// was bound where it starts hands on the run it begins there
/// ([`Run::bound`]). `bool11_probes`'
/// `a_runs_misses_are_read_against_its_own_line_and_never_sum` pins it.
fn emit_straight_leg_at<T: Decide>(
    core: &mut Core<T>,
    end: Point2<T>,
    ang: Dir<T>,
    run: Run<T>,
) -> Result<Tip<T>, PathError<T>> {
    let head = core.head()?;
    core.push_line(end)?;
    let arm = (end - head).norm_squared().sqrt();
    Ok(directed_tip(end, Incoming::continuing(run, ang, arm)))
}

/// Maps the shared fillet closed form's refusals into the algebra's
/// vocabulary: a Negative leg fit here IS the anchor-fit refusal (the
/// helper is fed the two sides' anchoring extents); an escalation
/// stays an escalation.
///
/// A straight carrier pair derives exactly ONE corner, so the anchor-fit
/// refusal is a one-entry [`PathError::NoCornerOfPair`] — the same
/// envelope the arc-carrier channel builds, so a consumer matches one
/// shape whichever pair refused. Nothing is sorted here: one entry is
/// already nearest.
fn map_fillet_err<T: Real>(refusal: TrimRefusal<T>, corner: Point2<T>, radius: T) -> PathError<T> {
    match refusal {
        TrimRefusal::DoesNotFit {
            leg,
            setback,
            leg_length,
        } => one_corner(
            corner,
            radius,
            CornerReason::AnchorOutsideTrimmedExtent {
                side: leg,
                // The line×line seam has only straight sides by
                // construction, so its carrier kind is structural, not
                // measured — no bracket read enters `path.rs`.
                carrier: FilletLegCarrier::Line,
                setback,
                available: leg_length,
            },
        ),
        TrimRefusal::Escalated(source) => PathError::Escalated { source },
    }
}

/// The one-entry envelope: a carrier pair with a single derived corner,
/// refusing about that corner.
fn one_corner<T: Real>(at: Point2<T>, radius: T, reason: CornerReason<T>) -> PathError<T> {
    PathError::NoCornerOfPair {
        radius,
        corners: vec![CornerRefusal { at, reason }],
    }
}

/// **The fillet's one conversion**: the arc a fillet resolution built,
/// stored as built — its own carrier (the centre the resolution
/// derived, the authored radius as its magnitude `|r|`, as
/// [`circle_loop`] stores it) and Δθ = 4·atan(b), `b` the resolution's
/// quarter-tangent, algebraic in the corner's data — from
/// [`crate::sugar::split_half_tan`] of the legs' split
/// ([`line_line_fillet_trims`]'s half-angle identity), or from its first
/// arm on the centre's split (`arc_fillet`'s, a fillet sweeping under a
/// half turn) — with `facts`, which the caller derives by
/// [`fillet_facts`].
fn fillet_arc<T: Real>(carrier: ArcData<T>, bulge: T, facts: crate::Facts) -> BuiltArc<T> {
    BuiltArc::new(
        Arc2 {
            centre: carrier.center,
            radius: carrier.radius.abs(),
            sweep: T::from_f64(4.0) * bulge.atan(),
        },
        facts,
    )
}

/// **What a fillet arc's construction proves of its endpoints.** Where
/// the resolution's centre lies on both offset carriers by its algebra
/// (`centre`: the line×line centre always, an arc-carrier centre on the
/// crossing branch) and the arc starts at its own tangent point `t1`
/// (`fit_in` Positive), the endpoint facts are theorems
/// ([`crate::Facts::Registered`]): `t1` and `t2` are the feet of the
/// centre on the two carriers, so the rim at each is the radius over
/// the reals at every value of the corner's data the door's decisions
/// hold for, and `b` is tan(θ/4) of the turn from `t1` to `t2` about
/// that centre. A centre on a decided tangency of the offset carriers,
/// or an exact fit that springs the arc off the chain head instead of
/// `t1`, holds them only to that decision ([`crate::Facts::Decided`]).
/// An arc that ends on a vertex other than `t2` is its caller's to
/// downgrade ([`BuiltArc::decided`]).
fn fillet_facts(centre: crate::Facts, fit_in: Sign) -> crate::Facts {
    match (centre, fit_in) {
        (crate::Facts::Registered, Sign::Positive) => crate::Facts::Registered,
        _ => crate::Facts::Decided,
    }
}

/// **Registers the line×line fillet's incoming tangency** on the values
/// the door built: the centre, spelled from the arrival side
/// ([`fillet_arc_carrier`]) as `t2` moved `σ·r` along the arrival
/// ray's left normal, IS `t1` moved `σ·r` along the incoming ray `u1`'s
/// ([`Arc2::register_tangent_at`]) — the same radius, the same side.
///
/// **A theorem of the construction.** The fillet circle is the circle
/// of radius r tangent to both carriers on the turn side of each, `t1`
/// and `t2` are the feet of its centre on them, so the centre is `r`
/// along each carrier's turn-side normal from that carrier's foot, over
/// the reals at every value of the corner's data the door's decisions
/// hold for. The arrival-side spelling states it for `t2` by
/// construction; this states it for `t1`, which is the tangency the
/// incoming side's consumers read.
fn register_incoming_tangency<T: Real>(
    fillet: &BuiltArc<T>,
    t1: Point2<T>,
    u1: Vec2<T>,
    offset: T,
    tol: Tol,
) {
    for (fact, answer) in fillet.arc.register_tangent_at(t1, u1, offset, tol) {
        answer.handle(fact);
    }
}

/// The fillet arc's own carrier: tangent to the arrival carrier at
/// `t2`, centre r to the turn side `sgn` — the corner's decided turn
/// (`path_corner_turn`) as the literal ±1, the sign of tan(φ/2)
/// wherever the windows the door decided hold, so no sign atom enters
/// the carrier.
fn fillet_arc_carrier<T: Real>(
    trims: &LineFilletTrims<T>,
    u2: Vec2<T>,
    radius: T,
    sgn: T,
) -> ArcData<T> {
    let n_hat = Vec2::new(-u2.y, u2.x);
    ArcData {
        center: trims.t2 + n_hat * (sgn * radius),
        radius,
    }
}

impl<T: Decide> Core<T> {
    /// **The door never mints a joint the validator refuses.**
    ///
    /// A fillet's constructed tangency is a claim about the CARRIERS the
    /// loop stores, and a profile stores an arc as its carrier and sweep
    /// between two vertices. The door stores the fillet's carrier as it
    /// built it, but what a reader gets back is whatever that segment
    /// classifies into: a fillet
    /// whose sagitta `r(1 − cos(θ/2))` sits at or below ε is read as a
    /// straight segment, and a declaration against a carrier that is
    /// not there is contradicted.
    ///
    /// So before the loop leaves the door, every fillet arc in it is
    /// re-read the way validation reads it — [`seg::build_seg`] on the
    /// stored triple, [`seg::joint_tangency`] on each joint the fillet
    /// DECLARED — and the door refuses what that reading refuses. One
    /// spelling: no margin and no predicate name is written here that
    /// the classifier does not hand back.
    ///
    /// The reading happens at the close rather than at the emission
    /// because a joint is a fact about two stored segments, and the
    /// segment leaving a fillet arc is authored after it — a continuing
    /// arrival's extent, a seam's retrimmed entry vertex, a following
    /// verb that extends the leg it lands on. At the close every one of
    /// them is settled.
    ///
    /// **Two losses, two sentences.** Which refusal a failed reading
    /// gets is decided by a fact the reading itself hands back: whether
    /// the stored fillet segment came back an ARC. If it did not, the
    /// carrier is absent and the loss is flattening
    /// ([`PathError::FilletArcFlattenedInStorage`]); if it did, the
    /// carrier is there and the joint's turn is what could not be read,
    /// so the loss is the scene's resolution
    /// ([`PathError::FilletCarrierBelowSceneResolution`]). Their levers
    /// run in opposite directions, which is why they are two variants.
    ///
    /// A NEIGHBOUR the segment pass itself refuses (a degenerate leg, a
    /// near-full arc leg) is left to that pass: its stored form is
    /// refused on its own terms, which is not a tangency disagreement,
    /// and the joint it sits on is skipped here.
    ///
    /// **Only CONSTRUCTED joints are read**, and that is the whole reach.
    /// EVERY fillet arc the chain emits is in the list — the one door
    /// that used to store its arc without recording it, the exact-fit
    /// arc close, goes through [`Core::record_fillet_arc`] like the
    /// rest — so what scopes the reading is the constructed set and
    /// nothing else. A fillet's OUTGOING joint is constructed at every
    /// door but three, and at each of the three
    /// the direction leaving the arc is not the door's to claim:
    ///
    /// - an exact outgoing fit onto a far-end anchor, and an interior
    ///   arc-carrier arrival whose run the fillet consumes — the side
    ///   ends at the arc and the next direction is free;
    /// - the exact-fit arc CLOSE, where the joint leaving the arc is
    ///   the seam: its direction is the entry's own and the seam
    ///   junction check beside it requires that joint TRANSVERSAL, so
    ///   a declaration there would contradict the check rather than
    ///   record a construction.
    ///
    /// The arc's INCOMING joint is declared at all three by
    /// [`Core::emit_fillet_in`] wherever it emits a run, so a stored
    /// form that stopped being the arc the resolver computed is read
    /// at that joint even where the outgoing one is skipped.
    ///
    /// What a joint no constructor made CAN do is come back `Tangent` at
    /// validation, which records it as a tangency decided from values;
    /// that is a fact about the stored form's carriers rather than a
    /// disagreement with a construction, so it is validation's and not
    /// this reading's. The skip below is that boundary, not an
    /// oversight.
    fn fillets_carry_their_tangency(&self, tol: Tol) -> Result<(), PathError<T>> {
        let band = linear_band(tol)?;
        let n = self.verts.len();
        for &(leaving, radius) in &self.fillet_arcs {
            let stored = |i: usize| {
                let (start, segment) = self.stored_segment(i, None);
                // Every segment here is its construction's own output,
                // verified at that construction (D1), so this re-read
                // asks what validation asks of its shape and joints,
                // not its consistency.
                seg::build_constructed_seg(start, self.verts[(i + 1) % n].0, segment, band)
            };
            // A recorded index always names a vertex of the chain it was
            // recorded on: `record_fillet_arc` reads `verts.len() - 1`
            // with the chain's last vertex the arc's own start, and
            // nothing removes a vertex afterwards. A chain that has
            // neither is not a loop, and saying so is cheaper than a
            // silent skip.
            let arc_seg = leaving
                .checked_add(1)
                .filter(|_| n >= 2 && leaving < n)
                .ok_or(PathError::UnderdeterminedLeg {
                    site: "a recorded fillet arc outside the chain it was recorded on",
                })?;
            let turn = match self.verts[leaving].1 {
                Some(built) => built.arc.sweep,
                None => {
                    return Err(PathError::UnderdeterminedLeg {
                        site: "a recorded fillet arc whose segment is not an arc",
                    });
                }
            };
            let flattened =
                |predicate: &'static str, margin: T| PathError::FilletArcFlattenedInStorage {
                    turn,
                    radius,
                    arc_length: radius * turn.abs(),
                    predicate,
                    margin,
                };
            let arc = match stored(leaving) {
                Ok(arc) => arc,
                Err(seg::ShapeIssue::Escalated(source)) => {
                    return Err(PathError::Escalated { source });
                }
                Err(
                    issue @ (seg::ShapeIssue::Degenerate { margin }
                    | seg::ShapeIssue::DegenerateTurn { margin }
                    | seg::ShapeIssue::NearFull { margin }),
                ) => {
                    return Err(flattened(issue.predicate(), margin));
                }
            };
            // The stored segment came back an arc, or it did not: that
            // is the discriminator between the two losses, read off the
            // classification rather than guessed from the geometry.
            let stores_an_arc = matches!(arc.kind, seg::SegKind::Arc(_));
            // Joint v is the junction between segment v − 1 and segment
            // v, so the arc's incoming joint is its own index and its
            // outgoing joint the next one — each checked only where the
            // door declared it (see the reach, above).
            for (joint, before) in [(leaving, true), (arc_seg % n, false)] {
                if !self.tangent.contains(&joint) {
                    continue;
                }
                let other = if before {
                    (leaving + n - 1) % n
                } else {
                    arc_seg % n
                };
                let Ok(other) = stored(other) else { continue };
                let (prev, next) = if before {
                    (&other, &arc)
                } else {
                    (&arc, &other)
                };
                let reading = seg::joint_tangency(prev, next, "path_fillet_joint_turn", band)
                    .map_err(|source| PathError::Escalated { source })?;
                match reading.class {
                    // A constructed joint whose carriers are tangent is
                    // the construction verified; one whose carriers are
                    // the SAME is a tangent joint too (the directions
                    // agree), and validation accepts it.
                    seg::JointClass::Tangent | seg::JointClass::SameCarrier => {}
                    seg::JointClass::Transversal if stores_an_arc => {
                        return Err(PathError::FilletCarrierBelowSceneResolution {
                            turn,
                            radius,
                            scale: reading.scale,
                            // `f64::EPSILON` by design, not by omission: every scalar this
                            // kernel ships carries an f64 VALUE channel (a dual's value, an
                            // interval's midpoint), so this is the resolution floor at any
                            // `T`; an interval's own enclosure width is `margin`, printed
                            // beside it, not folded into this number.
                            resolution: reading.resolution,
                            predicate: reading.predicate,
                            margin: reading.margin,
                        });
                    }
                    seg::JointClass::Transversal => {
                        return Err(flattened(reading.predicate, reading.margin));
                    }
                }
            }
        }
        Ok(())
    }

    /// Finishes the loop, after re-reading every fillet arc in it the
    /// way validation will ([`Core::fillets_carry_their_tangency`]),
    /// whose refusals name a fillet's own levers, and then reading every
    /// arc back against what was authored ([`arc_reads_back`]): the
    /// door's output is a loop whose fillet declarations the stored form
    /// carries and whose arcs pass through their authored points, or it
    /// is a refusal.
    fn build(self, tol: Tol) -> Result<ClosedLoop<T>, PathError<T>> {
        self.fillets_carry_their_tangency(tol)?;
        let band = linear_band(tol)?;
        let n = self.verts.len();
        for (k, &(start, leaving)) in self.verts.iter().enumerate() {
            if let Some(built) = leaving {
                let piece =
                    self.pieces
                        .get(k)
                        .copied()
                        .flatten()
                        .ok_or(PathError::UnderdeterminedLeg {
                            site: "an arc segment of a closed chain that no emission named",
                        })?;
                let verb = self.program.get(piece.step).map(Step::verb).ok_or(
                    PathError::UnderdeterminedLeg {
                        site: "an arc segment named by a step the program does not hold",
                    },
                )?;
                arc_reads_back(built, start, self.verts[(k + 1) % n].0, band, piece, verb)?;
            }
        }
        Ok(self.finish(tol))
    }

    /// Takes the opened fillet and its chain-side bookkeeping, which
    /// are one value: the only thing left to refuse is that no fillet
    /// is open at all.
    fn take_pending(
        &mut self,
        site: &'static str,
    ) -> Result<(verbs::Pending<T>, PendingMeta<T>), PathError<T>> {
        self.pending
            .take()
            .ok_or(PathError::OverdeterminedJunction { site })
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
        let trims = (arc.resolver)(self.guide_mut(), incoming, arrival, arc.radius, tol)?;
        let run = verbs::Pending::Arc(arc).run();
        self.emit_fillet_in(&trims, meta.extends_carrier, &meta, run)?;
        let ray = ArrivalCarrier::Ray {
            anchor: arr_pos,
            dir: arr_ang,
        };
        match kind {
            ArrivalKind::Seam => {
                // The fillet arc IS the closing segment; the entry
                // vertex retrims to its end and joint 0 is the
                // constructed seam tangency (the straight seam's rule).
                let leaving = self.record_fillet_arc(arc.radius, meta.bound_at)?;
                self.set_leaving(Some(trims.fillet()), FirstSeg::Arc, trims.t2)?;
                debug_assert_eq!(leaving, self.verts.len() - 1, "{PAIRED}");
                match self.verts.first_mut() {
                    Some((v0, _)) => *v0 = trims.t2,
                    None => {
                        return Err(PathError::UnderdeterminedLeg {
                            site: "seam fillet on an empty chain",
                        });
                    }
                }
                self.tangent.push(0);
            }
            ArrivalKind::Continues => {
                self.emit_fillet_arc(&trims, Some(ray), meta.bound_at, tol)?;
            }
            ArrivalKind::EndsAtAnchor => {
                let follows = (trims.fit_out == Sign::Positive).then_some(ray);
                self.emit_fillet_arc(&trims, follows, meta.bound_at, tol)?;
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
                return self
                    .resolve_arc_pending_ray_arrival(arc, meta, arr_pos, arr_ang, kind, tol);
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
        let turn = decide("path_corner_turn", Margin::levered(cross, wn), band)
            .map_err(|source| PathError::Escalated { source })?;
        let Some((sgn, sense)) =
            crate::sugar::turn_side::<T>(turn).zip(crate::sugar::turn_sense(turn))
        else {
            return Err(PathError::NoCornerForFillet {
                reason: PathNoCornerReason::CarriersParallel,
                radius: pending.radius,
            });
        };
        // (2) the corner must lie ahead of the incoming ray's origin
        // and behind the arrival side's anchor (ray parameters, meters).
        let t_ray = w.perp_dot(u2) / cross;
        let s_arr = w.perp_dot(u1) / cross;
        // The corner is derived before the windows are read, because a
        // window refusal is ABOUT it: the sentence names the point.
        let corner = pending.origin + u1 * t_ray;
        match decide("path_corner_advance", Margin::of(t_ray), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(one_corner(
                    corner,
                    pending.radius,
                    CornerReason::OutsideAnchors(CornerWindow::BehindIncomingRay),
                ));
            }
            Err(source) => return Err(PathError::Escalated { source }),
        }
        match decide("path_corner_advance", Margin::of(-s_arr), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(one_corner(
                    corner,
                    pending.radius,
                    CornerReason::OutsideAnchors(CornerWindow::BehindArrivalAnchor),
                ));
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
        // (4) the shared line×line closed form, anchored: head = the
        // ray's origin, next = the arrival's anchor.
        let mut trims = line_line_fillet_trims(pending.origin, corner, arr_pos, pending.radius)
            .map_err(|e| map_fillet_err(e, corner, pending.radius))?;
        // A straight carrier pair derives ONE corner and its
        // construction admits one candidate, so the fit signs are this
        // resolution's whole discrete content.
        (trims.fit_in, trims.fit_out) = self
            .guide
            .line_fits(trims.fit_in, trims.fit_out)
            .map_err(PathError::Structure)?;
        let arc = fillet_arc_carrier(&trims, u2, pending.radius, sgn);
        let built = fillet_arc(
            arc,
            trims.bulge,
            fillet_facts(crate::Facts::Registered, trims.fit_in),
        )
        .sensed(sense);
        register_incoming_tangency(&built, trims.t1, u1, sgn * pending.radius, tol);
        // (5) incoming side emission: Positive fit emits the straight
        // piece + constructed joint (exactly the raw fillet's rule); Zero
        // fit springs the arc off the last vertex — if that joint is
        // constructed (a `.tangent()` ray, or a previous fillet's arc
        // end), §4 item 4 refuses carrier identity there.
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
                self.claim(meta.bound_at, crate::structure::PieceRole::RunIn);
                self.push_line(trims.t1)?;
                self.construct_last();
            }
        }
        // (6) the arc. Interior: emitted, its outgoing joint declared
        // (Positive fit: as the raw fillet; Zero fit: the
        // continuation extends the arrival carrier tangentially by
        // construction, so the algebra declares what a hand author
        // must declare manually). Seam: the arc IS the closing
        // segment; the entry vertex retrims to its end and joint 0 is
        // the constructed seam tangency.
        let leaving = self.record_fillet_arc(pending.radius, meta.bound_at)?;
        if kind == ArrivalKind::Seam {
            self.set_leaving(Some(built), FirstSeg::Arc, trims.t2)?;
            match self.verts.first_mut() {
                Some((v0, _)) => *v0 = trims.t2,
                None => {
                    return Err(PathError::UnderdeterminedLeg {
                        site: "seam fillet on an empty chain",
                    });
                }
            }
            self.tangent.push(0);
            debug_assert_eq!(leaving, self.verts.len() - 1, "{PAIRED}");
        } else {
            self.push_arc(trims.t2, built)?;
            debug_assert_eq!(leaving, self.verts.len() - 2, "{PAIRED}");
            // The outgoing joint is declared only when something
            // tangent actually follows it (see [`ArrivalKind`]): a
            // continuing arrival always rides the arrival ray, and a
            // far-end side does too WHILE it still has a straight run
            // left. On an exact fit the far-end side ends here, and the
            // next direction is free — declaring would be a claim, not
            // a construction.
            if kind == ArrivalKind::Continues || trims.fit_out == Sign::Positive {
                self.construct_last();
                self.run_out = Some(PendingRunOut {
                    step: meta.bound_at,
                    ray: Run::bound(arr_pos, arr_ang),
                    tol,
                });
            }
        }
        Ok((arc, trims.fit_out))
    }

    /// **G2**: emits the trimmed INCOMING run of an arc-carrier fillet
    /// — along that side's own carrier when it has one, straight when
    /// it does not — and declares the joint it lands on.
    ///
    /// `meta` carries the address the emitted segment is recorded at:
    /// the binder's own step, recorded only where a radius argument
    /// authored that incoming carrier (`PendingMeta::incoming_radius`).
    ///
    /// Bit-identity note: this is `fillet_corner`'s emission verbatim
    /// (`arc_to_center(t1, centre, sweep)` / `line_to(t1)`, then the
    /// joint constructed), which is what lets a migrated site reproduce
    /// the hand-authored loop to the bit. A `Zero` fit emits nothing and
    /// springs the arc off the last vertex, where §4 item 4 refuses
    /// carrier identity against an already-constructed neighbour.
    fn emit_fillet_in(
        &mut self,
        t: &arc_fillet::ArcFilletTrims<T>,
        merge: bool,
        meta: &PendingMeta<T>,
        run: Option<verbs::SideRun<T>>,
    ) -> Result<(), PathError<T>> {
        if t.fit_in == Sign::Positive {
            match t.in_arc {
                None if merge => self.extend_leg_to(t.t1)?,
                None => {
                    self.claim(meta.bound_at, crate::structure::PieceRole::RunIn);
                    self.push_line(t.t1)?;
                }
                // An EXTENSION emits no segment — the leg's own end
                // vertex moves along the carrier it already had — so
                // the segment keeps the address the leg step gave it
                // and there is nothing to record here.
                Some((_, sweep)) if merge => self.extend_arc_to(t.t1, sweep)?,
                Some((centre, sweep)) => {
                    let head = self.head()?;
                    let run = run.ok_or(PathError::UnderdeterminedLeg {
                        site: "an arc side's run without its opened arc side",
                    })?;
                    let arc = side_arc(head, t.t1, centre, run.radius, sweep, run.head_on_circle)
                        .through(run.through);
                    // The incoming carrier is the BINDER's own argument
                    // wherever the arrival finally emits it.
                    if meta.incoming_radius {
                        self.record_radius(meta.bound_at, crate::structure::RadiusRole::Carrier)?;
                    }
                    self.claim(meta.bound_at, crate::structure::PieceRole::RunIn);
                    self.push_arc(t.t1, arc)?;
                }
            }
            if !(t.in_arc.is_none() && merge) {
                self.construct_last();
            }
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
            Some((v, _)) => {
                *v = t1;
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
    /// [`Self::extend_leg_to`]) and the segment's sweep is re-derived
    /// on its stored carrier for the longer turn. The leg's authored end stays on the final
    /// path, interior to the extended segment.
    fn extend_arc_to(&mut self, t1: Point2<T>, sweep: ArcSweep) -> Result<(), PathError<T>> {
        let n = self.verts.len();
        let (from, leg) = n
            .checked_sub(2)
            .and_then(|i| self.verts.get(i))
            .and_then(|&(pos, leaving)| Some((pos, leaving?)))
            .ok_or(PathError::UnderdeterminedLeg {
                site: "arc extension without an incoming arc segment",
            })?;
        // The leg keeps its own carrier and turns further along it. Its
        // new end is on the fused side's carrier, which the door
        // decided identical to this one under ε, not proved so: the
        // extended arc's endpoint facts are that decision's, not
        // theorems.
        let arc = Arc2 {
            sweep: center_sweep(from, t1, leg.arc.centre, leg.arc.radius, sweep),
            ..leg.arc
        };
        self.verts[n - 2] = (
            from,
            Some(BuiltArc {
                arc,
                facts: crate::Facts::Decided,
                sense: Some(sweep),
                ..leg
            }),
        );
        self.verts[n - 1].0 = t1;
        Ok(())
    }

    /// **G2**: emits the fillet arc itself as a chain segment, declaring
    /// its outgoing joint when something tangent actually follows it
    /// (see [`ArrivalKind`]): `follows` is that something. A ray's run
    /// out is the next emission that rides it ([`Core::run_out`]),
    /// decided under `tol`; an authored arrival arc is claimed by the
    /// site that emits it.
    fn emit_fillet_arc(
        &mut self,
        t: &arc_fillet::ArcFilletTrims<T>,
        follows: Option<ArrivalCarrier<T>>,
        bound_at: usize,
        tol: Tol,
    ) -> Result<(), PathError<T>> {
        let leaving = self.record_fillet_arc(t.arc.radius, bound_at)?;
        self.push_arc(t.t2, t.fillet())?;
        debug_assert_eq!(leaving, self.verts.len() - 2, "{PAIRED}");
        if let Some(carrier) = follows {
            self.construct_last();
            if let ArrivalCarrier::Ray { anchor, dir } = carrier {
                self.run_out = Some(PendingRunOut {
                    step: bound_at,
                    ray: Run::bound(anchor, dir),
                    tol,
                });
            }
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

impl<T: Real, P, A> PartialPath<T, P, A> {
    /// **The steps recorded so far, in program order** — the prefix of
    /// the program this chain publishes when it closes
    /// ([`ClosedLoop::program`]).
    ///
    /// Every verb records exactly one step, before the emission it
    /// brackets (`Core::record`), so the last element is the verb
    /// that was just called and its index is that verb's step number
    /// in the published program. That is what lets a caller writing a
    /// notation beside the recording address the leg it just authored
    /// without counting the ones before it.
    ///
    /// The steps only, and no structure record: a chain's structure is
    /// taken at the close, and the recording is the part of a path
    /// that is already final at every point along it.
    #[must_use]
    pub fn recorded(&self) -> &[Step<T>] {
        &self.core.program
    }
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

/// A directed-point tip minted by a leg end whose direction was bound
/// where it ends: the bare representation, starting afresh.
fn leg_end_tip<T: Real>(at: Point2<T>, ang: Dir<T>, arm: T, carrier: Option<ArcData<T>>) -> Tip<T> {
    directed_tip(at, Incoming::fresh(ang, arm, carrier))
}

/// A directed-point tip at `at` holding `incoming`.
fn directed_tip<T: Real>(at: Point2<T>, incoming: Incoming<T>) -> Tip<T> {
    Tip {
        pos: Some(PosData {
            at,
            incoming: Some(incoming),
        }),
        ang: None,
        ang_by_tangent: false,
    }
}

impl Open {
    /// The kernel behind the table's `Open → Point` row: records the
    /// entry verb, then seeds the chain at `p`.
    ///
    /// The step is the kernel's argument rather than the row's because
    /// the core a row would record into does not exist until this
    /// function makes it, and [`Core::record`] runs BEFORE the emission
    /// it brackets everywhere ([`Core::step_spans`]).
    fn at_kernel<T: Real>(
        self,
        step: Step<T>,
        p: Point2<T>,
    ) -> PartialPath<T, HasPos<Plain>, NoAng> {
        let mut core = Core::empty();
        core.record(step);
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

    /// The kernel behind the table's `Open → Angle` rows: records the
    /// entry verb, then binds the direction. The step is the kernel's
    /// argument for the reason [`Open::at_kernel`] states.
    fn director<T: Real>(self, step: Step<T>, dir: Dir<T>) -> PartialPath<T, NoPos, HasAng> {
        let mut core = Core::empty();
        core.record(step);
        in_state(
            core,
            Tip {
                pos: None,
                ang: Some(Bound::fresh(dir)),
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
///
/// **Finiteness before sign.** Components past [`Vec2::norm`]'s
/// ~1e154 overflow band make the norm ∞, which is maximally DEFINITE
/// to the classifier: deciding the sign first answers `Positive` and
/// the two divisions below hand back `(0, 0)`. `(1e200, 0)` returned
/// `Ok(Dir { unit: (0, 0), ang: 0 })` before this question went first
/// — a stored director naming no direction, out of a decided path.
///
/// **K consequence.** The refusal precedes the funnel, so a
/// non-finite pair contributes no `path_director_norm` sample; the
/// one it used to contribute was a `+∞` margin recorded as a definite
/// `Positive`.
fn unit_from_components<T: Decide>(dx: T, dy: T, tol: Tol) -> Result<Dir<T>, PathError<T>> {
    let band = linear_band(tol)?;
    // `powi(2)`, never `dx * dx`: a director's components straddle zero
    // by construction (every axis direction has a zero component), and
    // the plain product treats its factors as independent, so an
    // interval enclosure picks up a spurious negative lower bound and
    // poisons this `sqrt`. Gated by ci.yml's "interval-square powi(2)
    // allowlist".
    let norm = (dx.powi(2) + dy.powi(2)).sqrt();
    if !is_finite_length(norm) {
        return Err(PathError::NonFiniteDirection { dx, dy });
    }
    match decide("path_director_norm", Margin::of(norm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(PathError::ZeroDirection { dx, dy }),
        Err(source) => return Err(PathError::Escalated { source }),
    }
    Ok(Dir::from_unit(Vec2::new(dx / norm, dy / norm)))
}

/// **A chord's director**: the chord's components over its length, so
/// an enclosure of the chord is an enclosure of its ray whichever way
/// it heads. An angle read off `atan2` is not: on its branch cut — a
/// chord along −x whose `y` straddles zero — the angle's enclosure is
/// the whole circle.
///
/// The length is not DECIDED here, and nothing is refused. The length
/// is the lever the junction gates ask about and the length the
/// segment's shape validation asks about, each under its own name, and
/// a chord out of the format's range is refused where its vertex is
/// validated or drawn; a question here would be a second one about the
/// same length, pre-empting the refusals that name it.
///
/// The chord is divided by its largest component before it is
/// normalized, so its squares are of order one whatever its scale: no
/// finite chord overflows them or loses them to the subnormal range, at
/// a point scalar or in an enclosure. Two chords have no length to
/// divide by. The zero chord (the authored point IS the tip) names no
/// ray and stands in `+x` for one; a component past the range (a tip
/// that overflowed) keeps `atan2`'s reading of its heading, the limit
/// along its infinite axis.
fn chord_director<T: Real>(d: Vec2<T>) -> Dir<T> {
    let witness = d.norm_witness();
    if !is_finite_length(witness) {
        return Dir::from_angle(d.y.atan2(d.x));
    }
    if geom_core::is_exact_zero(witness) {
        return Dir::from_unit(Vec2::new(T::one(), T::zero()));
    }
    let scaled = d / witness;
    Dir::from_unit(scaled / scaled.norm_squared().sqrt())
}

/// The vector `v` rotated by `by` radians — the one spelling of a
/// rotation in the plane.
fn rotated<T: Real>(v: Vec2<T>, by: T) -> Vec2<T> {
    let (s, c) = by.sin_cos();
    Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c)
}

/// The unit ray `u` turned by `by` radians: unit over the reals wherever
/// `u` is, and to its rounding in the floats ([`Dir::from_unit`]).
fn turned<T: Real>(u: Vec2<T>, by: T) -> Dir<T> {
    Dir::from_unit(rotated(u, by))
}

/// **An arc leg's tangents from its chord**: the chord's ray
/// ([`chord_director`]) turned by `∓θ/2` for the sweep `θ` — the start
/// tangent and the end tangent. A line's `θ` is zero, which leaves the
/// ray as it is.
fn chord_tangents<T: Real>(d: Vec2<T>, sweep: T) -> (Dir<T>, Dir<T>) {
    let c = chord_director(d).unit;
    let half = sweep / T::from_f64(2.0);
    (turned(c, -half), turned(c, half))
}

/// The kernel behind the table's circle row: the lowered loop (the
/// row supplies the one-step program).
fn circle_kernel<T: Decide>(
    center: Point2<T>,
    radius: T,
    tol: Tol,
) -> Result<ProfileLoop<T>, PathError<T>> {
    let band = linear_band(tol)?;
    match decide("path_circle_radius", Margin::of(radius), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(PathError::NonpositiveCircleRadius { radius }),
        Err(source) => return Err(PathError::Escalated { source }),
    }
    // One vertex per piece the role list admits a `circle`: the table
    // is typed at `CIRCLE_PIECES`, so the two cannot disagree. Each
    // half turns through 4·atan(1).
    let half = T::from_f64(4.0) * T::one().atan();
    let vertices: [Point2<T>; crate::structure::CIRCLE_PIECES as usize] = [
        Point2::new(center.x + radius, center.y),
        Point2::new(center.x - radius, center.y),
    ];
    circle_loop(
        &vertices,
        center,
        radius,
        half,
        tol,
        band,
        program::Verb::Circle,
    )
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
    if n < 2 || u32::try_from(n).is_err() {
        return Err(PathError::CircleSplitCount { n });
    }
    let n_t = T::from_f64(n as f64);
    let four = T::from_f64(4.0);
    let sweep = four * (T::pi() / (T::from_f64(2.0) * n_t)).tan().atan();
    let vertices: Vec<Point2<T>> = (0..n)
        .map(|k| {
            let theta = phase + T::from_f64(2.0) * T::pi() * T::from_f64(k as f64) / n_t;
            let (s, c) = theta.sin_cos();
            Point2::new(center.x + radius * c, center.y + radius * s)
        })
        .collect();
    circle_loop(
        &vertices,
        center,
        radius,
        sweep,
        tol,
        band,
        program::Verb::CircleSplit,
    )
}

/// **The circle forms' one conversion**: the vertices on the authored
/// carrier, each with the arc on it — the authored centre, the authored
/// radius as the magnitude `|r|` (the door gated it positive; the
/// magnitude is non-negative in the symbolic tier's own syntax, which
/// is what lets it fold the rim at a vertex onto it) and the same
/// `sweep` — to the next, the last closing back to the first.
///
/// **Its endpoint facts are theorems**, registered at `tol`: each
/// vertex is the centre plus the radius along a unit direction, and
/// the next is that direction rotated by `sweep` (a half turn for
/// `circle`; 2π/n, spelled 4·atan(tan(π/2n)), for `circle_split`), over
/// the reals at every value of the centre, radius and phase; each is
/// read back before the loop is stored ([`arc_reads_back`]), as piece
/// `k` of the one step the `verb` row records.
fn circle_loop<T: Decide>(
    vertices: &[Point2<T>],
    centre: Point2<T>,
    radius: T,
    sweep: T,
    tol: Tol,
    band: Band,
    verb: program::Verb,
) -> Result<ProfileLoop<T>, PathError<T>> {
    let n = vertices.len();
    let arc = BuiltArc::new(
        Arc2 {
            centre,
            radius: radius.abs(),
            sweep,
        },
        crate::Facts::Registered,
    )
    .sensed(ArcSweep::Ccw);
    for k in 0..n {
        let piece = crate::structure::Piece {
            step: 0,
            role: crate::structure::PieceRole::Piece(u32::try_from(k).unwrap_or(u32::MAX)),
        };
        arc_reads_back(arc, vertices[k], vertices[(k + 1) % n], band, piece, verb)?;
    }
    let chain: Vec<(Point2<T>, Segment<T>)> = (0..n)
        .map(|k| {
            let (start, end) = (vertices[k], vertices[(k + 1) % n]);
            arc.register(start, end, tol);
            (start, Segment::Arc(arc.arc))
        })
        .collect();
    Ok(ProfileLoop::from_chain(chain))
}

/// The opening of [`PathError::ArcMissesAuthoredPoint`]'s and its
/// sibling's sentences: the step, the verb, and what was built against
/// what was authored, for their `predicate`.
#[allow(clippy::too_many_arguments)]
fn read_back_sentence<T: Real>(
    f: &mut core::fmt::Formatter<'_>,
    piece: &crate::structure::Piece,
    verb: &program::Verb,
    predicate: &str,
    point: &Point2<T>,
    miss: &T,
    turn: &T,
    authored: Option<ArcSweep>,
) -> core::fmt::Result {
    let (x, y) = (num(&point.x), num(&point.y));
    if predicate == seg::BRANCH_READ {
        let sense = match authored {
            Some(ArcSweep::Ccw) => "a counter-clockwise turn",
            Some(ArcSweep::Cw) => "a clockwise turn",
            None => "the other turn",
        };
        return write!(
            f,
            "{piece} ({verb}) was authored as {sense} to ({x}, {y}) and built a turn of \
             {turn} rad, the other arc between its ends",
            turn = num(turn)
        );
    }
    let what = match predicate {
        seg::START_READ => "its start vertex",
        seg::THROUGH_POINT_READ => "the through-point it was authored by",
        _ => "its end vertex",
    };
    write!(
        f,
        "{piece} ({verb}) built an arc {miss} m off {what} ({x}, {y})",
        miss = num(miss)
    )
}

/// **A constructed arc, read back against what was authored**, before
/// the chain stores it ([`seg::read_back_arc`]): its start on its
/// carrier, its landing on `end`, a `Via` arc's carrier through `q`, and
/// its branch against the turn sense it was authored to, at a point
/// scalar. Every construction proves these over the reals, or holds
/// them to an inline decision; a miss read here is its arithmetic, or a
/// scene that cannot resolve the read, and refuses naming `piece`,
/// `verb` and the authored point the read was about.
fn arc_reads_back<T: Decide>(
    built: BuiltArc<T>,
    start: Point2<T>,
    end: Point2<T>,
    band: Band,
    piece: crate::structure::Piece,
    verb: program::Verb,
) -> Result<(), PathError<T>> {
    let point = |predicate: &str| match predicate {
        seg::START_READ => start,
        seg::THROUGH_POINT_READ => built.through.unwrap_or(end),
        seg::BRANCH_READ => built.through.unwrap_or(end),
        _ => end,
    };
    seg::read_back_arc(&built, start, end, band).map_err(|issue| match issue {
        seg::ReadBackIssue::Defect(m) => PathError::ArcMissesAuthoredPoint {
            piece,
            verb,
            predicate: m.predicate,
            point: point(m.predicate),
            miss: m.miss,
            scale: m.scale,
            resolution: m.resolution,
            turn: built.arc.sweep,
            authored: built.sense,
        },
        seg::ReadBackIssue::BelowSceneResolution(m) => PathError::ArcMissBelowSceneResolution {
            piece,
            verb,
            predicate: m.predicate,
            point: point(m.predicate),
            miss: m.miss,
            scale: m.scale,
            resolution: m.resolution,
            turn: built.arc.sweep,
            authored: built.sense,
        },
        seg::ReadBackIssue::Escalated(source) => PathError::Escalated { source },
    })
}

impl<T: Decide, A: AngMarker> PartialPath<T, NoPos, A> {
    /// The kernel behind the table's position-binding rows: resolves a
    /// bound-angle fillet arrival, or seeds the chain (recording is the
    /// row's, not the kernel's).
    fn at_kernel(
        mut self,
        p: Point2<T>,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<Plain>, A>, PathError<T>> {
        match (self.tip.ang.map(|b| b.dir), self.core.pending.is_some()) {
            (Some(theta), true) => {
                self.core
                    .resolve_fillet(p, theta, ArrivalKind::Continues, tol)?;
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
    fn director(
        mut self,
        dir: Dir<T>,
        tol: Tol,
    ) -> Result<PartialPath<T, P, HasAng>, PathError<T>> {
        if let Some(pos) = &self.tip.pos {
            if let Some(inc) = &pos.incoming {
                self.core.pend_junction(inc, dir, tol)?;
            }
            let at = pos.at;
            if self.core.pending.is_some() {
                self.core
                    .resolve_fillet(at, dir, ArrivalKind::Continues, tol)?;
            } else if self.core.start_ang.is_none() {
                self.core.start_ang = Some(dir);
            }
        }
        self.tip.ang = Some(Bound::fresh(dir));
        self.tip.ang_by_tangent = false;
        Ok(in_state(self.core, self.tip))
    }
}

impl<T: Decide> PartialPath<T, HasPos<WithIncoming>, NoAng> {
    /// The direction the tip inherits bitwise, with the run it rides
    /// (the direction-inheriting verbs: a straight leg off it continues
    /// that run), reversed together for `.cusp()`.
    fn inherited(&self, reversed: bool) -> Option<Bound<T>> {
        let pos = self.tip.pos.as_ref()?;
        let inc = pos.incoming?;
        let run = inc.run_from(pos.at);
        Some(if reversed {
            Bound::inherited(inc.ang.reversed(), run.reversed())
        } else {
            Bound::inherited(inc.ang, run)
        })
    }

    /// The kernel behind the table's tangent-inheritance row (recording
    /// is the row's, not the kernel's).
    fn tangent_kernel(mut self) -> PartialPath<T, HasPos<WithIncoming>, HasAng> {
        self.tip.ang = self.inherited(false);
        self.tip.ang_by_tangent = true;
        self.core.construct_last();
        in_state(self.core, self.tip)
    }

    /// The kernel behind the table's cusp row (recording is the row's,
    /// not the kernel's): the tangent kernel with the ray reversed.
    /// The joint it constructs is the SAME one `.tangent()` constructs —
    /// validation verifies a constructed joint by carrier tangency,
    /// which is direction-agnostic, and reads its heading apart (a
    /// cusp), so a reverse-tangent joint needs no second kind of
    /// construction.
    fn cusp_kernel(mut self) -> PartialPath<T, HasPos<WithIncoming>, HasAng> {
        self.tip.ang = self.inherited(true);
        self.tip.ang_by_tangent = true;
        self.core.construct_last();
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
        let theta = turned(inc.ang.unit, delta);
        self.core.pend_junction(&inc, theta, tol)?;
        self.tip.ang = Some(Bound::fresh(theta));
        self.tip.ang_by_tangent = false;
        Ok(in_state(self.core, self.tip))
    }

    /// The kernel behind the table's straight-continuation row
    /// (recording is the row's, not the kernel's): the leg departs
    /// along the directed point's OWN intrinsic tangent, the RAY
    /// inherited BITWISE — consecutive legs run on one ray, not on two
    /// that a round trip through the angle put a bit apart. (The ray is
    /// what is exact; the vertices it lands are ordinary sums and round
    /// like ordinary sums.) Binding bits only — the tangent is a
    /// binding bit, and nothing else about the incoming leg is read —
    /// so there is no junction to classify (no authored direction
    /// exists) and nothing is declared: the minted vertex is a
    /// structural subdivision of the one carrier the binding bits
    /// already determine.
    fn straight_continuation_kernel(
        mut self,
        len: T,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let pos = self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "straight continuation on a tip without a position",
        })?;
        let at = pos.at;
        let inc = pos.incoming.ok_or(PathError::UnderdeterminedLeg {
            site: "straight continuation on a tip without incoming data",
        })?;
        let band = linear_band(tol)?;
        match decide("path_leg_length", Margin::of(len), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => return Err(PathError::NonpositiveLeg { length: len }),
            Err(source) => return Err(PathError::Escalated { source }),
        }
        // The departure IS the incoming ray, moved wholesale: the same
        // `Dir` value, never re-derived through its angle. What that
        // buys is exact in the DIRECTION — the two legs run on one ray,
        // not on two rays a `sin_cos` round trip apart. It is not a
        // claim about the emitted coordinates: `at + û·len` rounds like
        // any other sum, so two legs of equal length lay down identical
        // displacements only while those sums are exact.
        // The continuation verbs CONSTRUCT the zero-turn joint they
        // mint (Ev, in-chat, 2026-09-02; D1: every zero-turn joint is a
        // tangent joint). The departure is the incoming ray itself, so
        // the joint at this vertex is tangent by construction, exactly
        // as `.tangent()`'s is, and validation verifies it.
        self.core.construct_last();
        let tip = emit_straight_leg(&mut self.core, at, inc.ang, len, inc.run_from(at))?;
        Ok(in_state(self.core, tip))
    }

    /// The departing directed point of a straight continuation: its
    /// position and its intrinsic data, both binding bits.
    fn departing(&self, site: &'static str) -> Result<(Point2<T>, Incoming<T>), PathError<T>> {
        let pos = self
            .tip
            .pos
            .as_ref()
            .ok_or(PathError::UnderdeterminedLeg { site })?;
        let inc = pos.incoming.ok_or(PathError::UnderdeterminedLeg { site })?;
        Ok((pos.at, inc))
    }

    /// The kernel behind the table's point-target continuation row
    /// (recording is the row's, not the kernel's): the same leg
    /// [`straight_continuation_kernel`](Self::straight_continuation_kernel)
    /// emits, with its extent given as an authored POINT instead of a
    /// length — and, because the point is authored rather than walked
    /// to, the one thing the length form has nothing to check: that the
    /// point is where the declaration says it is.
    ///
    /// The emitted vertex is the AUTHORED target, never its projection.
    /// Snapping to the ray would move an authored point (§4 item 3), and
    /// would put the closer's endpoint a hair off the entry vertex,
    /// which is the one place a hair is not allowed.
    fn continue_to_point_kernel(
        mut self,
        target: Point2<T>,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let (at, inc) = self.departing("continue_to on a tip without incoming data")?;
        inc.lands_on_ray(at, target, tol)?;
        // The continuation verbs CONSTRUCT the zero-turn joint they
        // mint (Ev, in-chat, 2026-09-02; D1: every zero-turn joint is a
        // tangent joint). The departure is the incoming ray itself, so
        // the joint at this vertex is tangent by construction, exactly
        // as `.tangent()`'s is, and validation verifies it.
        self.core.construct_last();
        let tip = emit_straight_leg_at(&mut self.core, target, inc.ang, inc.run_from(at))?;
        Ok(in_state(self.core, tip))
    }

    /// The kernel behind the table's structural CLOSER row: the
    /// point-target continuation whose target is [`Start`].
    ///
    /// One check is the point row's — the entry vertex must lie on the
    /// run's line (`lands_on_ray`) — and it replaces the closer's departure junction
    /// check outright: there is no authored direction here to classify,
    /// so §4 item 1 has nothing to say and no departure junction is
    /// classified at all. The SEAM check still runs,
    /// unchanged and un-narrowed: the junction between this leg and the
    /// entry's own departure is a real junction, the loop's, and PQ4
    /// wants it to be a corner. That is what makes a seam at a corner
    /// SUFFICIENT for an outline whose every side is subdivided — and
    /// what leaves a seam at a subdivision vertex refused as
    /// [`PathError::SeamTangent`] — a refusal only a seam can produce,
    /// so the two mechanisms are separable by TYPE rather than by
    /// reading a payload tag.
    fn continue_to_start_kernel(
        mut self,
        seam: SeamArrival,
        tol: Tol,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let (at, inc) = self.departing("continue_to(Start) on a tip without incoming data")?;
        let ang = inc.ang;
        // NO open-fillet guard here, and that is a decision rather than
        // an omission — the three straight-continuation kernels
        // (`straight_continuation_kernel`, `continue_to_point_kernel`
        // and this one) now agree, where an earlier draft of this one
        // guarded and the other two did not.
        //
        // The guarded state is UNREACHABLE from the typed surface.
        // `core.pending` is set in exactly one place, `fillet_kernel`,
        // which returns `PartialPath<T, NoPos, NoAng>` with a tip whose
        // `pos` and `ang` are both `None`. Every route from there into
        // this kernel's departing state, `HasPos<WithIncoming>`, passes
        // through a verb that RESOLVES the fillet first — `director`
        // and `line_to`'s kernel call `resolve_fillet` on the
        // `pending.is_some()` branch, and `resolve_fillet` goes through
        // `take_pending`, which `take()`s it — and `WithIncoming`
        // itself is only minted by a leg emission, which needs a bound
        // direction. So a tip in this state with a pending fillet does
        // not exist, and a guard against it is dead code asserting the
        // type system's own invariant back to it.
        //
        // The dropped guard also refused with `ArcLegOnOpenFillet`,
        // whose three live sites are all ARC arrivals and whose message
        // is written about authoring an arc with the fillet that trims
        // it. Borrowing it for a straight continuation named the wrong
        // fact even in the branch that could never run.
        inc.lands_on_ray(at, start_pos, tol)?;
        let start_ang = *self.core.start_ang.get_or_insert(ang);
        // NO vertex is minted here. `Start` is the entry vertex, which
        // the loop already carries; a closing leg is the segment BACK
        // to it, and emitting its endpoint would author the entry
        // twice (§4 item 3: authored once). That is the one place the
        // point-target row and the closer differ, and it is why the
        // closer's arm is measured head-to-entry rather than read off
        // an emitted tip.
        let head = self.core.head()?;
        let arm = (start_pos - head).norm_squared().sqrt();
        // The seam is classified with the DECLARED ray (`ang`), while
        // the segment actually emitted runs head → start_pos. Both ends
        // lie within the band's zero of the run's line (`lands_on_ray`
        // above refused anything larger, and the head was accepted the
        // same way), so the two directions differ by at most `2ε/arm`.
        //
        // The bounded consequence, stated so it is not rediscovered: a
        // seam within two band units of the boundary can be pushed from
        // a definite verdict into ESCALATION by this difference. It
        // cannot flip accept ↔ refuse while K > 3, because crossing from
        // one definite verdict to the other takes the band's whole
        // width, `(K − 1)·ε`, which passes `2ε` only there. The ambiguity
        // multiplier admits any K > 1 (`CAD_AMBIGUITY_K`), and at K ≤ 3 a
        // seam within the band's width of its boundary can flip: that is
        // the argument's limit. Escalation is the honest outcome for a
        // junction that close to the edge, so where the argument holds
        // the direction of the error is the safe one.
        //
        // `line_to(Start)` classifies from the REALIZED direction
        // instead, having computed it from the two points; that is the
        // one place the two closers' seam checks differ, and it follows
        // from the same thing everything else here follows from — this
        // verb declares its ray, `line_to` derives one. (R1 NOTE-3.)
        //
        // With the ARRIVAL declared too ([`Start::arrives_tangent`]),
        // the same comparison runs with the verdict inverted: a zero
        // turn is the declared subdivision seam and closes, a definite
        // turn is the declaration contradicted. The two declarations
        // are independent facts — this verb declares the DEPARTURE
        // (the leg continues the run), the target declares the ARRIVAL
        // (it continues the entry's first side) — and the D-shape's two
        // rotations need one each.
        seam_joint(
            &mut self.core,
            seam,
            &Incoming::fresh(ang, arm, None),
            start_ang,
            tol,
        )?;
        // The continuation verbs CONSTRUCT the zero-turn joint they
        // mint (Ev, in-chat, 2026-09-02; D1: every zero-turn joint is a
        // tangent joint). The departure is the incoming ray itself, so
        // the joint at this vertex is tangent by construction, exactly
        // as `.tangent()`'s is, and validation verifies it.
        self.core.construct_last();
        self.core.close_leaving(None, FirstSeg::Line)?;
        self.core.build(tol)
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
        Ok((pos.at, ang.dir))
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
        // The leg rides the run its direction inherited (`.tangent()`,
        // `.cusp()`), or begins one here, where its direction was bound.
        let run = self.tip.ang.map_or(Run::bound(at, ang), |b| b.run_at(at));
        let tip = emit_straight_leg(&mut self.core, at, ang, len, run)?;
        Ok(in_state(self.core, tip))
    }

    /// The kernel behind the table's corner-fillet row (recording is the
    /// row's, not the kernel's).
    fn fillet_kernel(
        mut self,
        radius: T,
        tol: Tol,
    ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>> {
        let (at, ang) = self.dep()?;
        let band = linear_band(tol)?;
        match decide("path_fillet_radius", Margin::of(radius), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => return Err(PathError::NonpositiveFilletRadius { radius }),
            Err(source) => return Err(PathError::Escalated { source }),
        }
        let meta = PendingMeta {
            bound_at: self.core.current_step()?,
            incoming_radius: false,
            by_tangent: self.tip.ang_by_tangent,
            origin_incoming: self.tip.pos.as_ref().and_then(|p| p.incoming),
            extends_carrier: false,
        };
        self.core.pending = Some((
            verbs::Pending::Ray(verbs::PendingRay {
                origin: at,
                dir: ang,
                radius,
            }),
            meta,
        ));
        Ok(in_state(
            self.core,
            Tip {
                pos: None,
                ang: None,
                ang_by_tangent: false,
            },
        ))
    }

    /// [`tangent_arc_geom`] from the bound tip.
    fn tangent_arc_geom(&self, p: Point2<T>, tol: Tol) -> Result<TangentArcGeom<T>, PathError<T>> {
        let (at, ang) = self.dep()?;
        tangent_arc_geom(at, ang, p, tol)
    }

    fn tangent_arc_to_point(
        mut self,
        p: Point2<T>,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>> {
        let g = self.tangent_arc_geom(p, tol)?;
        self.core.push_leg(p, g.arc)?;
        let arm = leg_arm(g.carrier, g.chord);
        Ok(in_state(
            self.core,
            leg_end_tip(p, g.end_ang, arm, g.carrier),
        ))
    }

    /// The tangent-arc seam. The arc is constructed from the DEPARTURE
    /// as it always was; `seam` [`SeamArrival::Checked`]
    /// ([`Start::arrives_tangent`]) says the ARRIVAL is G1 by intent,
    /// which inverts the seam junction's
    /// verdict and constructs joint 0 tangent, which validation
    /// verifies. One end constructs, the other
    /// is checked — nothing is overdetermined, and a shape no circular
    /// arc can serve refuses with the seam fillet named.
    fn tangent_arc_to_start(
        mut self,
        seam: SeamArrival,
        tol: Tol,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let start_ang = self.core.start_ang.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry direction is bound",
        })?;
        let g = self.tangent_arc_geom(start_pos, tol)?;
        let arm = leg_arm(g.carrier, g.chord);
        seam_joint(
            &mut self.core,
            seam,
            &Incoming::fresh(g.end_ang, arm, g.carrier),
            start_ang,
            tol,
        )?;
        self.core.close_leaving(g.arc, FirstSeg::Arc)?;
        self.core.build(tol)
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
        let gamma = chord_director(d);
        if self.core.pending.is_some() {
            self.core
                .resolve_fillet(at, gamma, ArrivalKind::Continues, tol)?;
        } else {
            if let Some(inc) = self.tip_pos()?.incoming {
                self.core.pend_junction(&inc, gamma, tol)?;
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

    /// The sharp straight seam. `seam` is what the TARGET said
    /// ([`seam_joint`]): declared ([`Start::arrives_tangent`]), a zero
    /// turn is what the author said and closes; undeclared, a zero turn
    /// refuses.
    ///
    /// The DEPARTURE junction is classified identically either way: the
    /// arrival declaration says nothing about it.
    fn line_to_start(mut self, seam: SeamArrival, tol: Tol) -> Result<ClosedLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let at = self.tip_pos()?.at;
        let d = start_pos - at;
        let gamma = chord_director(d);
        if self.core.pending.is_some() {
            self.core
                .resolve_fillet(at, gamma, ArrivalKind::Continues, tol)?;
        } else if let Some(inc) = self.tip_pos()?.incoming {
            self.core.pend_junction(&inc, gamma, tol)?;
        }
        let start_ang = *self.core.start_ang.get_or_insert(gamma);
        let head = self.core.head()?;
        let arm = (start_pos - head).norm_squared().sqrt();
        seam_joint(
            &mut self.core,
            seam,
            &Incoming::fresh(gamma, arm, None),
            start_ang,
            tol,
        )?;
        self.core.close_leaving(None, FirstSeg::Line)?;
        self.core.build(tol)
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

    /// The `Via` mode's arc: the collinear gate (the through-point's
    /// signed perpendicular offset from the chord LINE, meters — zero
    /// for on-chord and beyond-the-end alike, which is why one refusal
    /// covers the class), then the mode's one conversion, [`via_arc`].
    pub(super) fn arc_via(
        &self,
        via: Point2<T>,
        end: Point2<T>,
        tol: Tol,
    ) -> Result<Option<BuiltArc<T>>, PathError<T>> {
        let at = self.tip_pos()?.at;
        let chord_len = self.arc_chord(end, tol)?;
        let band = linear_band(tol)?;
        let offset = (end - at).perp_dot(via - at) / chord_len;
        // `via` left of the chord puts the arc on the chord's left, which
        // a clockwise turn from `at` traverses.
        let sense = match decide("path_arc_via_offset", Margin::of(offset), band) {
            Ok(Sign::Zero) => return Err(PathError::ArcViaCollinear { offset }),
            Ok(Sign::Positive) => ArcSweep::Cw,
            Ok(Sign::Negative) => ArcSweep::Ccw,
            Err(source) => return Err(PathError::Escalated { source }),
        };
        Ok(via_arc(at, via, end).map(|built| built.sensed(sense)))
    }

    /// The `Center` mode's arc: both radii gated definitely positive,
    /// then equidistance gated definitely ZERO (a definite mismatch
    /// refuses; an undecidable one escalates — neither is repaired),
    /// then the mode's one conversion, [`center_arc`], about the
    /// authored centre.
    pub(super) fn arc_center(
        &self,
        center: Point2<T>,
        end: Point2<T>,
        winding: ArcSweep,
        tol: Tol,
    ) -> Result<Option<BuiltArc<T>>, PathError<T>> {
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
        Ok(Some(center_arc(at, end, center, winding)))
    }

    /// The target point a closing verb resolves to (the entry pose's
    /// position — [`Start`] by reference, never re-typed).
    fn start_target(&self) -> Result<Point2<T>, PathError<T>> {
        self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })
    }

    /// The `Bulge` mode's arc: the mode's one conversion,
    /// [`crate::bulge_leg`], on the chord from the tip.
    pub(super) fn arc_bulge(
        &self,
        end: Point2<T>,
        bulge: T,
    ) -> Result<Option<BuiltArc<T>>, PathError<T>> {
        Ok(
            crate::bulge_leg(self.tip_pos()?.at, end, bulge).map(|built| BuiltArc {
                bulge_authored: true,
                ..built
            }),
        )
    }

    /// The leg's derived tangents ([`chord_tangents`] for its stored
    /// sweep) and its chord length.
    fn arc_angles(
        at: Point2<T>,
        target: Point2<T>,
        leg: Option<BuiltArc<T>>,
    ) -> (Dir<T>, Dir<T>, T) {
        let d = target - at;
        let (start, end) = chord_tangents(d, sweep_of(leg));
        (start, end, d.norm_squared().sqrt())
    }

    pub(super) fn arc_to_point(
        mut self,
        p: Point2<T>,
        leg: Option<BuiltArc<T>>,
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
        let (start_t, end_t, chord) = Self::arc_angles(at, p, leg);
        if let Some(inc) = pos.incoming {
            self.core.pend_junction(&inc, start_t, tol)?;
        }
        if self.core.start_ang.is_none() {
            self.core.start_ang = Some(start_t);
        }
        let carrier = carrier_of(leg);
        self.core.push_leg(p, leg)?;
        let arm = leg_arm(carrier, chord);
        Ok(in_state(self.core, leg_end_tip(p, end_t, arm, carrier)))
    }

    /// The SHARP arc seam. `seam` is what the spec's target said
    /// ([`seam_joint`]): every endpoint-full mode fixes the arc's end
    /// tangent, so a declared arrival is the CHECK form — one end is
    /// authored, the other is checked.
    fn arc_to_start(
        mut self,
        leg: Option<BuiltArc<T>>,
        seam: SeamArrival,
        tol: Tol,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
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
        let (start_t, end_t, chord) = Self::arc_angles(at, start_pos, leg);
        if let Some(inc) = pos.incoming {
            self.core.pend_junction(&inc, start_t, tol)?;
        }
        let start_ang = *self.core.start_ang.get_or_insert(start_t);
        let carrier = carrier_of(leg);
        let arm = leg_arm(carrier, chord);
        seam_joint(
            &mut self.core,
            seam,
            &Incoming::fresh(end_t, arm, carrier),
            start_ang,
            tol,
        )?;
        self.core.close_leaving(leg, FirstSeg::Arc)?;
        self.core.build(tol)
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
        let dir = self
            .tip
            .ang
            .ok_or(PathError::UnderdeterminedLeg {
                site: "far-end anchor on a tip without a bound direction",
            })?
            .dir;
        if self.core.pending.is_none() {
            return Err(PathError::FarEndAnchorWithoutFillet);
        }
        let (arc, fit_out) =
            self.core
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
        self.core.build(tol)
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
        path.line_to_start(SeamArrival::Undeclared, tol)
    }
}

impl<T: Decide, F: Flavor> LineTarget<T, F> for ArrivesTangent {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn line_from(mut path: PartialPath<T, HasPos<F>, NoAng>, _target: Self, tol: Tol) -> Self::Out {
        path.core.record(Step::LineTo(Target::StartArriving));
        path.line_to_start(SeamArrival::Checked, tol)
    }
}

/// A [`PartialPath::continue_to`] target: an authored absolute point,
/// or [`Start`] (the declared structural closer). Sealed.
///
/// The two are one verb because they are one construction — the same
/// ray, the same check, the same emitted vertex — differing only in
/// where the target came from: an authored point, or the chain's own
/// entry, which is emission-layer bookkeeping rather than incoming-leg
/// data. Closing is the verb's SHAPE, not a value it discovers.
pub trait ContinueTarget<T: Decide>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn continue_from(
        path: PartialPath<T, HasPos<WithIncoming>, NoAng>,
        target: Self,
        tol: Tol,
    ) -> Self::Out;
}

impl<T: Decide> ContinueTarget<T> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn continue_from(
        mut path: PartialPath<T, HasPos<WithIncoming>, NoAng>,
        target: Self,
        tol: Tol,
    ) -> Self::Out {
        path.core.record(Step::ContinueTo(Target::Point(target)));
        path.continue_to_point_kernel(target, tol)
    }
}

impl<T: Decide> ContinueTarget<T> for Start {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn continue_from(
        mut path: PartialPath<T, HasPos<WithIncoming>, NoAng>,
        _target: Self,
        tol: Tol,
    ) -> Self::Out {
        path.core.record(Step::ContinueTo(Target::Start));
        path.continue_to_start_kernel(SeamArrival::Undeclared, tol)
    }
}

impl<T: Decide> ContinueTarget<T> for ArrivesTangent {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn continue_from(
        mut path: PartialPath<T, HasPos<WithIncoming>, NoAng>,
        _target: Self,
        tol: Tol,
    ) -> Self::Out {
        path.core.record(Step::ContinueTo(Target::StartArriving));
        path.continue_to_start_kernel(SeamArrival::Checked, tol)
    }
}

/// A [`PartialPath::tangent_arc_to`] target: an authored absolute
/// point, or [`Start`] (the tangent-seam close). Sealed.
pub trait TangentArcTarget<T: Decide, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn tangent_arc_from(
        path: PartialPath<T, HasPos<F>, HasAng>,
        target: Self,
        tol: Tol,
    ) -> Self::Out;
}

impl<T: Decide, F: Flavor> TangentArcTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>;
    fn tangent_arc_from(
        mut path: PartialPath<T, HasPos<F>, HasAng>,
        target: Self,
        tol: Tol,
    ) -> Self::Out {
        path.core.record(Step::TangentArcTo(Target::Point(target)));
        path.tangent_arc_to_point(target, tol)
    }
}

impl<T: Decide, F: Flavor> TangentArcTarget<T, F> for Start {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn tangent_arc_from(
        mut path: PartialPath<T, HasPos<F>, HasAng>,
        _target: Self,
        tol: Tol,
    ) -> Self::Out {
        path.core.record(Step::TangentArcTo(Target::Start));
        path.tangent_arc_to_start(SeamArrival::Undeclared, tol)
    }
}

impl<T: Decide, F: Flavor> TangentArcTarget<T, F> for ArrivesTangent {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn tangent_arc_from(
        mut path: PartialPath<T, HasPos<F>, HasAng>,
        _target: Self,
        tol: Tol,
    ) -> Self::Out {
        path.core.record(Step::TangentArcTo(Target::StartArriving));
        path.tangent_arc_to_start(SeamArrival::Checked, tol)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **The cusp door's exactness guard.**
    ///
    /// [`Dir::reversed`] promises the departure ray is the incoming ray
    /// NEGATED, never re-derived as `θ + π` — and that promise is the
    /// reason a declared cusp is a structural fact rather than a value
    /// coincidence. Until this row existed the promise was held by
    /// prose alone: the `θ + π` mutant passed every suite in the
    /// workspace, because every downstream check reads the junction
    /// through a TOLERANCE and the two spellings differ by ulps.
    ///
    /// A tolerance can never catch that, so the guard is at the bit
    /// level, at the door itself, and it asserts both halves:
    ///
    /// 1. the reversed ray is bit-exactly the negation, and
    /// 2. the `θ + π` spelling would NOT be — on every case here,
    ///    axis-aligned and axis-oblique alike. Both parts of (2) matter:
    ///    on an axis the mutant leaks the quantization of π into the
    ///    zero component (`sin π = 1.22e-16`), and off-axis it lands
    ///    ulps away in BOTH components because the sum `θ + π` rounds
    ///    before `sin_cos` ever sees it.
    ///
    /// This is a unit row rather than an authored-path row on purpose:
    /// `Dir`'s fields are private, and the negation is exactly the fact
    /// no observable path predicate can distinguish.
    #[test]
    fn the_cusp_door_negates_the_ray_it_never_re_derives_it_from_the_angle() {
        // Unit rays: three on-axis, two oblique.
        let cases: [Vec2<f64>; 5] = [
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(-1.0, 0.0),
            Vec2::new(0.6, 0.8),
            Vec2::new(0.28, 0.96),
        ];
        for u in cases {
            let incoming = Dir::from_unit(u);
            let reversed = incoming.reversed();
            assert_eq!(
                reversed.unit.x.to_bits(),
                (-u.x).to_bits(),
                "the reversed ray's x is the negated bits ({u:?})"
            );
            assert_eq!(
                reversed.unit.y.to_bits(),
                (-u.y).to_bits(),
                "the reversed ray's y is the negated bits ({u:?})"
            );
            // The mutation this row exists to kill.
            let via_angle = Dir::from_angle(u.y.atan2(u.x) + core::f64::consts::PI);
            assert!(
                via_angle.unit.x.to_bits() != reversed.unit.x.to_bits()
                    || via_angle.unit.y.to_bits() != reversed.unit.y.to_bits(),
                "`θ + π` must NOT reproduce the negation ({u:?}) — if it does, this \
                 guard is vacuous and the door's promise is untestable here"
            );
        }
    }

    /// **Each producing door stores a unit ray** — the half of
    /// [`Dir::from_unit`]'s contract a test can reach. It does NOT prove
    /// the contract holds for every caller: nothing here can see a new
    /// caller appear, which is what the constructor's privacy and its
    /// doc are for.
    ///
    /// The two deciding doors are walked refusal and success: each
    /// classifies its own length through the funnel under its own name
    /// and refuses typed, and what it then stores is a ray of length
    /// one — exactly one on the cases below, which are chosen so the
    /// arithmetic is exact in binary and a "close enough" assertion
    /// cannot hide a door that stopped normalizing. The chord door
    /// decides nothing, and stores the exact unit ray on the same 3-4-5
    /// case (its other arms are the next row's). [`Dir::reversed`] is
    /// unit by exact negation and is pinned by the row above;
    /// [`reflected_across`] and [`turned`] are walked last, each on an
    /// exact case and on one whose rounding leaves it an ulp off.
    #[test]
    fn each_producing_door_stores_a_unit_ray() {
        let tol = Tol::witness();
        let band = linear_band::<f64>(tol).expect("the linear band");

        // The components door refuses a length decided to zero, and
        // stores the exact unit ray otherwise (3-4-5: the quotients
        // are exact).
        assert!(
            matches!(
                unit_from_components::<f64>(0.0, 0.0, tol),
                Err(PathError::ZeroDirection { .. })
            ),
            "the components door decides its own length"
        );
        let d = unit_from_components::<f64>(3.0, 4.0, tol).expect("a real direction");
        assert_eq!((d.unit.x, d.unit.y), (0.6, 0.8), "the stored ray is unit");
        let c = chord_director(Vec2::new(3.0, 4.0));
        assert_eq!(
            (c.unit.x, c.unit.y),
            (0.6, 0.8),
            "the chord door's ray is unit"
        );

        // The arc-carrier door refuses an anchor at the centre — no
        // tangent exists there — and stores the unit tangent
        // otherwise. τ·(P−O)⟂/R with P−O = (3, 0), R = 3 is (0, 1).
        let centre = Point2::new(1.0, 2.0);
        assert!(
            matches!(
                arc_fillet::carrier_tangent::<f64>(centre, centre, ArcSweep::Ccw, band),
                Err(PathError::DegenerateArcCenter { .. })
            ),
            "the carrier door decides its own radius"
        );
        let t =
            arc_fillet::carrier_tangent::<f64>(Point2::new(4.0, 2.0), centre, ArcSweep::Ccw, band)
                .expect("a real tangent");
        assert_eq!((t.unit.x, t.unit.y), (0.0, 1.0), "the stored ray is unit");

        // The tangent arc's end ray: +x reflected across the chord
        // (1, 1) is +y, exactly; across (3, 4) it is unit to an ulp.
        let r = reflected_across(Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0));
        assert_eq!((r.unit.x, r.unit.y), (0.0, 1.0), "an exact reflection");
        let r = reflected_across(Vec2::new(1.0, 0.0), Vec2::new(3.0, 4.0));
        assert!(
            (r.unit.norm() - 1.0).abs() <= 2.0 * f64::EPSILON,
            "a reflection keeps the length: {:?}",
            r.unit
        );

        // The turned ray: +x turned by zero is +x, exactly; (0.6, 0.8)
        // turned by one radian is unit to an ulp.
        let t = turned(Vec2::new(1.0, 0.0), 0.0);
        assert_eq!((t.unit.x, t.unit.y), (1.0, 0.0), "an exact turn");
        let t = turned(Vec2::new(0.6, 0.8), 1.0);
        assert!(
            (t.unit.norm() - 1.0).abs() <= 2.0 * f64::EPSILON,
            "a rotation keeps the length: {:?}",
            t.unit
        );
    }

    /// **The chord door divides; it decides and refuses nothing.** Its
    /// ray is exact where the quotients are (3-4-5, and an axis chord
    /// along −x, whose angle round-trip lands `1.2e-16` off the axis); a
    /// chord far below ε still names its ray, because the lever gates
    /// and the shape validation own that length; a finite chord whose
    /// length over- or underflows still divides; the zero chord stands
    /// in `+x`; and a component past the range keeps its heading.
    #[test]
    fn the_chord_director_divides_without_deciding_its_length() {
        let ray = |x: f64, y: f64| {
            let d = chord_director(Vec2::new(x, y));
            (d.unit.x, d.unit.y)
        };
        assert_eq!(ray(-3.0, 4.0), (-0.6, 0.8), "a 3-4-5 chord");
        assert_eq!(ray(-1.5, 0.0), (-1.0, 0.0), "an axis chord");
        assert_eq!(ray(0.0, 1e-13), (0.0, 1.0), "a sub-ε chord");
        assert_eq!(ray(0.0, 0.0), (1.0, 0.0), "the zero chord");
        assert_eq!(ray(-3e300, 4e300), (-0.6, 0.8), "an overflowed length");
        assert_eq!(ray(3e-300, -4e-300), (0.6, -0.8), "an underflowed length");
        assert_eq!(
            ray(f64::NEG_INFINITY, 1e307).0,
            -1.0,
            "a component past the range"
        );
        let (x, y) = ray(f64::NAN, 1.0);
        assert!(x.is_nan() && y.is_nan(), "a component that is not a number");
        // The gradual-underflow band: the squares are subnormal and the
        // length is not zero, so a plain `d / |d|` is not unit there.
        let (x, y) = ray(1.134e-163, 1.683e-162);
        let truth = 1.683_f64.atan2(0.1134);
        assert!(
            ((x * x + y * y).sqrt() - 1.0).abs() <= 2.0 * f64::EPSILON
                && (y.atan2(x) - truth).abs() <= 4.0 * f64::EPSILON,
            "a chord in the subnormal band of its squares: ({x}, {y})"
        );
    }

    /// **The chord door at the interval scalar is sound, and narrow at
    /// both ends of the range.** The prescale by the largest component
    /// is the same arithmetic in an enclosure, so an overflowed or an
    /// underflowed chord's ray encloses `(−1, 0)` and is as narrow as
    /// one along −x of ordinary length; and a chord on `atan2`'s branch
    /// cut (`x < 0`, `y` straddling zero) reads a narrow ray.
    #[test]
    fn the_chord_director_encloses_a_narrow_ray_at_the_interval_scalar() {
        use geom_core::{Bounds, Interval};
        let iv = |lo: f64, hi: f64| Interval::from_bounds(lo, hi);
        let cases = [
            ("an ordinary chord", iv(-1.0, -1.0), iv(-1e-12, 1e-12)),
            ("an overflowed chord", iv(-1e300, -1e300), iv(-1e288, 1e288)),
            (
                "an underflowed chord",
                iv(-1e-200, -1e-200),
                iv(-1e-212, 1e-212),
            ),
        ];
        for (what, x, y) in cases {
            let u = chord_director(Vec2::new(x, y)).unit;
            assert!(
                u.x.lo() <= -1.0 && -1.0 <= u.x.hi() && u.y.lo() <= 0.0 && 0.0 <= u.y.hi(),
                "{what}: the ray encloses (−1, 0): {u:?}"
            );
            assert!(
                u.x.hi() - u.x.lo() < 1e-11 && u.y.hi() - u.y.lo() < 1e-11,
                "{what}: and is narrow: {u:?}"
            );
        }
    }

    /// **The overflow end of both 2-D director doors.** Components
    /// past `Vec2::norm`'s ~1e154 band make the norm ∞, which is
    /// maximally DEFINITE to the classifier, and the division that
    /// follows collapses the ray to zero. Measured at the merge base,
    /// both doors reported SUCCESS:
    ///
    /// - `unit_from_components(1e200, 0.0, witness)` →
    ///   `Ok(Dir { unit: (0, 0), ang: 0 })`;
    /// - `carrier_tangent((1e200, 0), origin, Ccw, band)` →
    ///   `Ok(Dir { unit: (-0, 0), ang: π })` — an angle asserted over
    ///   a ray of nothing. This door had never been executed at the
    ///   overflow end before this row.
    ///
    /// Both now refuse [`PathError::NonFiniteDirection`], whose
    /// sentence is NOT [`PathError::ZeroDirection`]'s: the direction
    /// is not zero, and "scale the components up" is the wrong way
    /// round.
    #[test]
    fn both_director_doors_refuse_a_length_that_is_not_a_number() {
        let tol = Tol::witness();
        let band = linear_band::<f64>(tol).expect("the linear band");
        for (dx, dy) in [(1e200, 0.0), (0.0, 1e200), (1e200, 1e200), (f64::NAN, 1.0)] {
            assert!(
                matches!(
                    unit_from_components::<f64>(dx, dy, tol),
                    Err(PathError::NonFiniteDirection { .. })
                ),
                "components ({dx}, {dy})"
            );
            let got = arc_fillet::carrier_tangent(
                Point2::new(dx, dy),
                Point2::origin(),
                crate::ArcSweep::Ccw,
                band,
            );
            assert!(
                matches!(got, Err(PathError::NonFiniteDirection { .. })),
                "carrier anchor ({dx}, {dy}): {got:?}"
            );
        }
        // A finite pair still climbs: the rows above cannot be passing
        // because the doors refuse everything.
        assert!(unit_from_components::<f64>(3.0, 4.0, tol).is_ok());
        assert!(
            arc_fillet::carrier_tangent(
                Point2::new(3.0, 4.0),
                Point2::origin(),
                crate::ArcSweep::Ccw,
                band,
            )
            .is_ok()
        );
        // The sentence names the cause and a recourse that can work
        // AT BOTH DOORS, and is not the zero-direction sentence.
        let s = PathError::NonFiniteDirection {
            dx: 1e200_f64,
            dy: 0.0,
        }
        .to_string();
        assert!(s.contains("no finite length"), "{s}");
        // The arm is shared by a door whose components are SPELLED
        // (`toward`) and one whose vector is DERIVED (the arc
        // carrier's tangent). Only the ratio is read at either, so the
        // free recourse — divide the pair through — must be named
        // first; sending a `toward` caller to move geometry instead
        // would be a refusal naming the wrong recourse.
        assert!(
            s.contains("only the ratio of the components is read"),
            "{s}"
        );
        assert!(s.contains("divide them through by a common factor"), "{s}");
        assert!(s.contains(geom_core::RANGE_RECOURSE), "{s}");
        assert!(!s.contains("scaling them up costs nothing"), "{s}");
        assert_eq!(
            PathError::NonFiniteDirection {
                dx: 1e200_f64,
                dy: 0.0
            }
            .kind(),
            PathErrorKind::NonFiniteDirection
        );
    }

    /// **The underflow end of the same two doors, which they answer
    /// DIFFERENTLY — deliberately.** Components below ~1e-162 square to
    /// zero, so the norm is exactly zero and the classifier answers
    /// `Zero` definitely at both.
    ///
    /// The arc carrier's tangent refuses
    /// [`PathError::UnderflowedDirection`]: its vector is DERIVED (the
    /// anchor's displacement from the centre), the caller does not hold
    /// those components, and its old refusal —
    /// `DegenerateArcCenter { radius: 0 }`, "the authored centre is
    /// within tolerance of an endpoint" — named a coincidence that is
    /// not there and a tolerance lever that cannot reach it.
    ///
    /// The components director does NOT, and this row is the pin on
    /// that decision rather than an omission. Its pair is SPELLED by the
    /// caller, so `ZeroDirection`'s sentence ("whose norm is within
    /// tolerance of zero") is true of it and its recourse ("scaling them
    /// up costs nothing") is already the only one that works. A second
    /// arm here would split two inputs that want the same answer. A lane
    /// adding one for symmetry breaks this row, which is the point.
    #[test]
    fn the_two_director_doors_split_at_the_underflow_end() {
        let tol = Tol::witness();
        let band = linear_band::<f64>(tol).expect("the linear band");
        for (dx, dy) in [(1e-200, 0.0), (0.0, 1e-200), (1e-200, 1e-200)] {
            // The premise: the norm flushed to zero, and the direction
            // survives in the witness the question is asked against.
            let v = Vec2::new(dx, dy);
            assert_eq!(v.norm_squared().sqrt(), 0.0, "({dx}, {dy})");
            assert_ne!(v.norm_witness(), 0.0, "({dx}, {dy})");

            let got = arc_fillet::carrier_tangent(
                Point2::new(dx, dy),
                Point2::origin(),
                crate::ArcSweep::Ccw,
                band,
            );
            assert!(
                matches!(got, Err(PathError::UnderflowedDirection { .. })),
                "carrier anchor ({dx}, {dy}): {got:?}"
            );
            assert!(
                matches!(
                    unit_from_components::<f64>(dx, dy, tol),
                    Err(PathError::ZeroDirection { .. })
                ),
                "components ({dx}, {dy}) keep the zero-direction arm"
            );
        }
        // A finite pair still climbs at both doors: the rows above
        // cannot be passing because everything refuses.
        assert!(unit_from_components::<f64>(3.0, 4.0, tol).is_ok());
        assert!(
            arc_fillet::carrier_tangent(
                Point2::new(3.0, 4.0),
                Point2::origin(),
                crate::ArcSweep::Ccw,
                band,
            )
            .is_ok()
        );
        // The sentence names the end of the format it is, and a
        // recourse that can work — not a coincidence at this tolerance.
        let s = PathError::UnderflowedDirection {
            dx: 0.0_f64,
            dy: 1e-200,
        }
        .to_string();
        assert!(s.contains("underflowed out of the format"), "{s}");
        assert!(
            s.contains("multiply them through by a common factor"),
            "{s}"
        );
        assert!(s.contains(geom_core::RANGE_RECOURSE), "{s}");
        assert!(!s.contains("within tolerance of zero"), "{s}");
        assert!(!s.contains("no finite length"), "{s}");
        // The component the format lost is RENDERED, never the zero it
        // measured to.
        assert!(s.contains("1e-200"), "{s}");
        assert_eq!(
            PathError::UnderflowedDirection {
                dx: 0.0_f64,
                dy: 1e-200
            }
            .kind(),
            PathErrorKind::UnderflowedDirection
        );
    }

    /// **A refusal never renders a number it did not measure.**
    ///
    /// [`num`] shortens a scalar payload to the shortest spelling that
    /// still names it to a relative 1e-9. The tolerance being RELATIVE
    /// is the whole of that promise: with an absolute floor under it,
    /// every payload below the floor rounds to the floor's own zero —
    /// and the kernel's unit is the metre, so the picometre and
    /// nanometre margins a junction refusal exists to report are
    /// exactly the values erased.
    ///
    /// Each row names a magnitude a refusal can carry. A floor at `f`
    /// makes every row with `|x| < f` read `0` (and every negative one
    /// `-0`, which reports the sign of a magnitude it just erased), so
    /// a 1e-9 floor breaks rows 3 onward and a 1e-30 floor still
    /// breaks the last four.
    #[test]
    fn num_renders_a_sub_nanometre_payload_at_its_own_magnitude() {
        let rows: [(f64, &str); 11] = [
            (1e-8, "1e-8"),
            (2e-9, "2e-9"),
            (1e-9, "1e-9"),
            (1e-10, "1e-10"),
            (1e-12, "1e-12"),
            (3.7e-12, "3.7e-12"),
            (1e-30, "1e-30"),
            (-1e-30, "-1e-30"),
            (1e-180, "1e-180"),
            (5e-324, "5e-324"),
            (-5e-324, "-5e-324"),
        ];
        for (x, want) in rows {
            let got = num(&x);
            assert_eq!(got, want, "num({x:?})");
            // The two spellings this row exists to forbid, stated
            // apart from the equality above so that re-baselining a
            // spelling cannot quietly re-admit them.
            assert_ne!(got, "0", "num({x:?}) erased the magnitude");
            assert_ne!(got, "-0", "num({x:?}) kept the sign of an erased magnitude");
        }
    }

    /// **A difference the kernel can certify is a difference the
    /// sentence spells.** ε is a LENGTH, so a purely relative 1e-9
    /// crosses it at one metre and is coarser above — at a kilometre it
    /// is a micron, a thousand ε. Two lengths the kernel decided were
    /// different then reach the reader as one number, which is how a
    /// refusal comes to read *"margin 1234.5 m exceeds the 1234.5 m the
    /// anchor pins"*.
    ///
    /// Each row is a pair at a magnitude at or above the crossover,
    /// separated by the multiple of ε named beside it, and the guard
    /// above each comparison is what makes the row mean anything: a
    /// pair closer than ε is one the kernel could not certify apart
    /// either, so `num` would owe nothing. The first row is the
    /// control — one metre is where a relative 1e-9 still equals ε, and
    /// it is distinct under either grid. The three above it collide
    /// under a purely relative tolerance.
    #[test]
    fn num_separates_two_lengths_the_kernel_can_certify_apart() {
        let rows: [(f64, f64, &str); 4] = [
            (1.0, 1.000_000_01, "1 m, 10 ε"),
            (100.0, 100.000_000_01, "100 m, 10 ε"),
            (1_234.5, 1_234.500_000_1, "1234.5 m, 100 ε"),
            (10_000.0, 10_000.000_001, "10 km, 1000 ε"),
        ];
        for (base, other, label) in rows {
            assert!(
                (other - base).abs() >= DEFAULT_EPS,
                "{label}: the pair is closer than ε, so the row proves nothing"
            );
            assert_ne!(num(&base), num(&other), "{label}: num rendered both alike");
        }
        assert_eq!(num(&100.000_000_01_f64), "100.00000001");
    }

    /// **The floor's absence is visible at the door, not only in the
    /// helper.** [`PathError::JunctionTangent`]'s sentence is *"turn
    /// margin {margin} m on a {arm} m arm"*, and a picometre margin
    /// rendered `0 m` reads as a claim that the margin IS zero — the
    /// one thing it is not, since a decided-zero margin takes a
    /// different arm entirely.
    ///
    /// Breaks if `num` regains any absolute floor above 3.7e-12, or if
    /// an arm stops routing its scalars through it.
    #[test]
    fn a_picometre_turn_margin_reaches_the_sentence_as_a_picometre() {
        let s = PathError::JunctionTangent {
            margin: 3.7e-12_f64,
            arm: 2.5e-6_f64,
        }
        .to_string();
        assert!(s.contains("turn margin 3.7e-12 m on a 2.5e-6 m arm"), "{s}");
        assert!(!s.contains("margin 0 m"), "{s}");
        let cusp = PathError::JunctionCusp {
            margin: -4e-11_f64,
            arm: 1.0_f64,
        }
        .to_string();
        assert!(cusp.contains("turn margin -4e-11 m on a 1 m arm"), "{cusp}");
        assert!(!cusp.contains("margin -0 m"), "{cusp}");
    }

    /// **A carrier clause carries no noise its sentence does not.**
    /// [`CornerReason::AnchorOutsideTrimmedExtent`] interpolates
    /// [`FilletLegCarrier`]'s `Display` as `{carrier}`, beside a
    /// `{setback}` and an `{available}` that `num` already shortened —
    /// so an unshortened carrier puts the arithmetic's noise inside a
    /// sentence otherwise free of it.
    ///
    /// The two scalars here are SUBTRACTED, not typed: an 8 mm carrier
    /// and a 3.5 mrad margin are each exactly representable as a
    /// literal, and a literal would therefore render `0.008` and
    /// `0.0035` with no helper at all. `0.017 - 0.009` and
    /// `0.0135 - 0.01` are the values arithmetic actually lands on, and
    /// they are what makes this row state the defect rather than
    /// illustrate it: without `num` it reads *"carrier radius
    /// 0.008000000000000002 m, angular margin 0.0034999999999999996
    /// rad"*.
    ///
    /// The payload is untouched — the shortening is a DISPLAY choice,
    /// and the field still holds the bits the subtraction produced.
    #[test]
    fn a_subtracted_carrier_radius_reaches_the_sentence_shortened() {
        let radius = 0.017_f64 - 0.009_f64;
        let angular_margin = 0.0135_f64 - 0.01_f64;
        // The premise: these are the noisy values, not the exact ones.
        assert_ne!(radius, 0.008_f64);
        assert_ne!(angular_margin, 0.0035_f64);

        let carrier = FilletLegCarrier::Arc {
            radius,
            angular_margin,
        };
        assert_eq!(
            carrier.to_string(),
            "circular (carrier radius 0.008 m, angular margin 0.0035 rad)"
        );

        // And through the corner sentence, whose setback and available
        // length are subtractions too and render shortened the same way.
        let s = CornerRefusal {
            at: Point2::new(0.25_f64, 0.5_f64),
            reason: CornerReason::AnchorOutsideTrimmedExtent {
                side: FilletLeg::Incoming,
                carrier,
                setback: 0.017_f64 - 0.009_f64,
                available: 0.0135_f64 - 0.01_f64,
            },
        }
        .to_string();
        assert!(s.contains("(setback 0.008 m, 0.0035 m available)"), "{s}");
        assert!(!s.contains("0.008000000000000002"), "{s}");
        assert!(!s.contains("0.0034999999999999996"), "{s}");

        // Display only: the payload keeps what the subtraction gave it.
        let FilletLegCarrier::Arc {
            radius: kept_radius,
            angular_margin: kept_margin,
        } = carrier
        else {
            panic!("an arc carrier");
        };
        assert_eq!(kept_radius.to_bits(), (0.017_f64 - 0.009_f64).to_bits());
        assert_eq!(kept_margin.to_bits(), (0.0135_f64 - 0.01_f64).to_bits());
    }

    /// **The cap is visible at the door too.**
    /// [`PathError::ArcCenterNotEquidistant`] fires precisely BECAUSE
    /// the kernel decided two radii differ, and its sentence prints
    /// both of them. Under a purely relative 1e-9 a 100 m arc whose
    /// radii differ by 10 ε renders *"|tip - centre| = 100 m, |end -
    /// centre| = 100 m"* — a refusal that contradicts itself in its own
    /// sentence, telling the reader the centre is not equidistant while
    /// printing one number twice.
    #[test]
    fn a_hundred_metre_refusal_does_not_print_its_two_radii_alike() {
        let s = PathError::ArcCenterNotEquidistant {
            tip_radius: 100.0_f64,
            end_radius: 100.000_000_01_f64,
        }
        .to_string();
        assert!(
            s.contains("|tip - centre| = 100 m, |end - centre| = 100.00000001 m"),
            "{s}"
        );
    }

    /// **Zero renders as zero.** A relative tolerance at `0.0` is
    /// `0.0`, so only a spelling that parses back to the payload is
    /// accepted — which `0` is, at the first precision tried. The row
    /// exists because the obvious failure of a relative tolerance is
    /// the one value it cannot scale: were the loop to reject every
    /// precision here it would fall through to the `{:?}` form and a
    /// refusal would read `margin 0.0 m`.
    ///
    /// `-0.0` keeps its sign, and that is not the defect above: no
    /// magnitude was erased, the payload IS negative zero, and `-0` is
    /// the only spelling that names it.
    #[test]
    fn num_renders_zero_as_zero_and_negative_zero_as_negative_zero() {
        assert_eq!(num(&0.0_f64), "0");
        assert_eq!(num(&-0.0_f64), "-0");
    }

    /// **The `{:?}` fallback is reached by the non-finite payloads and
    /// by nothing finite.** The loop tries 18 significant figures in
    /// the notation `{:?}` itself chose, which names every finite
    /// `f64` exactly; only `NaN` and the infinities fail every
    /// comparison — each is false against a `NaN` or infinite
    /// tolerance — and fall through.
    ///
    /// The finite rows below are asserted SHORTENED, which the fallback
    /// cannot produce: a fall-through returns the full
    /// 17-significant-figure `Debug` spelling.
    ///
    /// **The largest magnitude is not one of them, and cannot be.** The
    /// display grid is absolute above a decimetre, and `f64`'s own
    /// spacing near [`f64::MAX`] is some 10³⁰⁰ metres — coarser than the
    /// grid by every order there is. The shortest spelling inside the
    /// grid is therefore the exact one, which is what `{:?}` already
    /// prints, so `num` and `{:?}` necessarily agree and no shortening
    /// is available to observe. That equality is itself the assertion: a
    /// tolerance that went relative again shortens the extreme to
    /// `1.79769313486e308` and reds the row.
    #[test]
    fn num_falls_through_to_debug_for_the_non_finite_payloads_only() {
        assert_eq!(num(&f64::NAN), "NaN");
        assert_eq!(num(&f64::INFINITY), "inf");
        assert_eq!(num(&f64::NEG_INFINITY), "-inf");
        for x in [f64::MAX, -f64::MAX] {
            assert_eq!(num(&x), format!("{x:?}"), "num({x:?}) dropped a digit");
        }
        for x in [
            f64::MIN_POSITIVE,
            -f64::MIN_POSITIVE,
            1_234.567_890_123_456_7,
            -1_234.567_890_123_456_7,
        ] {
            let got = num(&x);
            assert_ne!(got, format!("{x:?}"), "num({x:?}) fell through");
            assert!(
                (got.parse::<f64>().unwrap() - x).abs() <= 1e-9 * x.abs(),
                "num({x:?}) = {got} does not name the payload"
            );
        }
        // A payload that is not a scalar at all — an enclosure, a dual
        // — has no `f64` spelling to shorten and passes through whole.
        assert_eq!(num(&(1.0_f64, 2.0_f64)), "(1.0, 2.0)");
    }

    /// **The reason [`num`] exists survives the fix.** `{:?}` on an
    /// `f64` is the shortest round-tripping spelling, which at
    /// ordinary scales is the arithmetic's own noise: an 8 mm setback
    /// that a subtraction produced is `0.008000000000000002`. Every
    /// row here is a value whose `Debug` form carries that noise, and
    /// each regresses to it if the shortening search is dropped.
    ///
    /// The last two rows straddle the decimetre where the grid stops
    /// being relative and becomes an absolute ε/10: `1/3` keeps ten
    /// decimal places because a tenth of a nanometre is what the grid
    /// is worth there, while `0.0035` keeps two significant figures
    /// because below the crossover the relative arm is the finer one.
    #[test]
    fn num_still_shortens_arithmetic_noise_at_metre_scale() {
        let rows: [(f64, &str); 6] = [
            (0.1 + 0.2, "0.3"),
            (0.008_000_000_000_000_002, "0.008"),
            (0.003_499_999_999_999_999_6, "0.0035"),
            (-0.008_000_000_000_000_002, "-0.008"),
            (1.0 / 3.0, "0.3333333333"),
            (2.5, "2.5"),
        ];
        for (x, want) in rows {
            assert_eq!(num(&x), want, "num({x:?})");
        }
    }

    /// **Notation is the `Debug` form's, never re-chosen.** A
    /// metre-scale number keeps its decimal point and a far-from-unity
    /// one keeps its exponent, so the shortening never turns a
    /// readable `0.0035` into `3.5e-3`, nor a `1e300` into the 301
    /// digits its fixed-point spelling needs.
    ///
    /// The length assertion is the one a re-chosen notation breaks:
    /// with fixed-point forced, `num(&1e300)` is 301 characters of
    /// refusal sentence.
    #[test]
    fn num_keeps_the_notation_the_debug_form_chose() {
        assert_eq!(num(&1e300_f64), "1e300");
        assert_eq!(num(&1e17_f64), "1e17");
        for x in [0.0035_f64, 0.001, 0.0001, 123_456.789, 1.0, -1.0, 8e-3] {
            let got = num(&x);
            assert!(!got.contains('e'), "num({x:?}) = {got} left metre scale");
        }
    }

    /// **The spelling names the payload to the finer of a relative 1e-9
    /// and an absolute ε/10, at every magnitude.** This is the property
    /// the per-value rows above sample; here it is asserted as the
    /// invariant, over a ladder that spans the exponent range in both
    /// signs.
    ///
    /// Each arm catches a different mistake. A returning absolute FLOOR
    /// breaks the relative arm at the bottom of the ladder — the
    /// rendered `0` has relative error 1, not 1e-9. A tolerance that
    /// drops the CAP and goes purely relative breaks the absolute arm
    /// at every rung above a decimetre, where a relative 1e-9 exceeds
    /// ε and the search stops a digit early on a difference the kernel
    /// can certify.
    #[test]
    fn num_names_its_payload_to_the_finer_of_a_relative_1e_9_and_an_absolute_grid() {
        let mut x = 1.234_567_890_123_456_7e-300_f64;
        let mut rungs = 0;
        while x.is_finite() && x != 0.0 {
            for signed in [x, -x] {
                let got = num(&signed);
                let back: f64 = got
                    .parse()
                    .unwrap_or_else(|e| panic!("num({signed:?}) = {got} does not parse: {e}"));
                assert!(
                    (back - signed).abs() <= 1e-9 * signed.abs(),
                    "num({signed:?}) = {got} is not within a relative 1e-9"
                );
                assert!(
                    (back - signed).abs() <= DEFAULT_EPS * 0.1,
                    "num({signed:?}) = {got} is coarser than a tenth of ε"
                );
                assert!(
                    got != "0" && got != "-0",
                    "num({signed:?}) = {got} renders a non-zero payload as zero"
                );
            }
            rungs += 1;
            x *= 1e17;
        }
        // The ladder ran: a `while` whose first test failed would pass
        // every assertion above vacuously.
        assert!(rungs >= 30, "the magnitude ladder covered {rungs} rungs");
    }

    /// **A pending run out claims only an emission on its ray.** Step 0
    /// is the fillet's, with its run out pending; step 1 draws one
    /// segment from the fillet arc's end. On the ray that segment is
    /// step 0's run out, since the earlier step outranks; off it — a
    /// line elsewhere, a line backward, or an arc — it is step 1's own
    /// leg.
    ///
    /// The lattice cannot reach the off-ray rows: it refuses every
    /// straight emission from a directed tip that leaves the arrival
    /// ray. So the state is built directly, which is the only way to
    /// show the naming reads the geometry rather than the kind.
    #[test]
    fn a_pending_run_out_claims_only_an_emission_on_its_ray() {
        use crate::structure::{Piece, PieceRole};
        let tol = Tol::witness();
        let head = Point2::new(1.0, 0.0);
        let named = |to: Point2<f64>, bulge: f64| -> Piece {
            let mut core = Core::<f64>::empty();
            core.seed(Point2::new(0.0, -1.0));
            core.record(Step::LineTo(Target::Point(head)));
            core.push_line(head).expect("step 0's segment");
            // The arrival ray: along +x through (0.5, 0).
            core.run_out = Some(PendingRunOut {
                step: 0,
                ray: Run::bound(Point2::new(0.5, 0.0), Dir::from_unit(Vec2::new(1.0, 0.0))),
                tol,
            });
            core.record(Step::LineTo(Target::Point(to)));
            if bulge == 0.0 {
                core.push_line(to).expect("step 1's line");
            } else {
                core.push_arc(to, BuiltArc::lowered(head, to, bulge))
                    .expect("step 1's arc");
            }
            core.pieces[1].expect("step 1's segment is named")
        };
        let run_out = Piece {
            step: 0,
            role: PieceRole::RunOut,
        };
        let own_leg = Piece {
            step: 1,
            role: PieceRole::Leg,
        };
        assert_eq!(named(Point2::new(3.0, 0.0), 0.0), run_out);
        assert_eq!(named(Point2::new(3.0, 1.0), 0.0), own_leg);
        // On the ray's line but behind the chain head: the ray is a
        // half-line, and a segment leaving backward along it is not on it.
        assert_eq!(named(Point2::new(0.0, 0.0), 0.0), own_leg);
        // An arc never rides a ray, whatever its ends.
        assert_eq!(named(Point2::new(3.0, 0.0), 0.3), own_leg);
    }

    /// **The tangent arc's and the centre arc's sweeps against a
    /// 60-digit oracle**: one JSON line per case — the tip `a`, the
    /// director's components, the target `b`, and the arc [`tangent_arc`]
    /// stores at `f64` and at `Interval`; then, keyed `C`, the ends, the
    /// centre, [`crate::bulge_from_center`] and the [`center_sweep`] a
    /// `Center` leg stores from it, at both scalars — over
    /// turns from 1e-12 to 2π − 1e-12 in both windings, at scales 1e-6 to
    /// 1e6. `scripts/oracles/tangent_arc_sweep_mpmath.py` re-derives
    /// each sweep from the dumped inputs and checks the `f64` error, the
    /// stored arc's miss of `b`, and the enclosure. Run:
    /// `cargo nextest run -p profile --lib --run-ignored only
    /// dump_tangent_arcs_for_the_mpmath_oracle --no-capture | sed -n
    /// 's/^TADUMP //p' > /tmp/ta.jsonl`, then
    /// `python3 scripts/oracles/tangent_arc_sweep_mpmath.py /tmp/ta.jsonl`.
    #[test]
    #[ignore = "an oracle dump; run command in the docs"]
    #[allow(clippy::print_stdout)]
    fn dump_tangent_arcs_for_the_mpmath_oracle() {
        use geom_core::{Bounds, Interval};
        let tau = core::f64::consts::TAU;
        let mut turns: Vec<f64> = (0..=24)
            .map(|k| 10f64.powf(-12.0 + 0.5 * f64::from(k)))
            .collect();
        turns.retain(|&t| t < core::f64::consts::PI);
        let near_full: Vec<f64> = turns.iter().map(|&g| tau - g).collect();
        turns.extend(near_full);
        turns.push(core::f64::consts::PI);
        let directors = [(1.0, 0.0), (3.0, -4.0), (1.234, -0.567)];
        for &(cx, cy) in &directors {
            for scale in [1e-6, 1e-4, 1e-2, 1.0, 1e2, 1e4, 1e6] {
                for at in [
                    Point2::new(0.0, 0.0),
                    Point2::new(0.7 * scale, -1.3 * scale),
                ] {
                    for &theta in &turns {
                        for w in [1.0, -1.0] {
                            let n = (cx * cx + cy * cy).sqrt();
                            let (ux, uy) = (cx / n, cy / n);
                            let (s, c1) = (theta.sin(), 1.0 - theta.cos());
                            let b = Point2::new(
                                at.x + scale * (s * ux - w * c1 * uy),
                                at.y + scale * (s * uy + w * c1 * ux),
                            );
                            // The departure-line gate's floor: the
                            // smallest band of any gated ε row.
                            let u = Vec2::new(cx, cy) / n;
                            if u.perp_dot(b - at).abs() <= 1e-11 {
                                continue;
                            }
                            let f = tangent_arc(at, u, b).arc;
                            let lift = Interval::from_f64;
                            let (ci, cj) = (lift(cx), lift(cy));
                            let ni = (ci * ci + cj * cj).sqrt();
                            let i = tangent_arc(
                                Point2::new(lift(at.x), lift(at.y)),
                                Vec2::new(ci, cj) / ni,
                                Point2::new(lift(b.x), lift(b.y)),
                            )
                            .arc;
                            println!(
                                "TADUMP {{\"a\":[{:e},{:e}],\"c\":[{:e},{:e}],\"b\":[{:e},{:e}],\
                                 \"turn\":{:e},\"w\":{w},\"f\":[{:e},{:e},{:e},{:e}],\
                                 \"i\":[{:e},{:e},{:e},{:e}]}}",
                                at.x,
                                at.y,
                                cx,
                                cy,
                                b.x,
                                b.y,
                                theta,
                                f.centre.x,
                                f.centre.y,
                                f.radius,
                                f.sweep,
                                i.sweep.lo(),
                                i.sweep.hi(),
                                i.radius.lo(),
                                i.radius.hi(),
                            );
                        }
                    }
                }
            }
        }
        // The arc about a known centre: ends on a carrier of radius
        // `scale` about `c`, turning `theta` in either sense — its
        // quarter-tangent `bulge_from_center`, the `Center` leg's, and
        // the sweep `center_sweep` stores from it.
        for scale in [1e-6, 1e-4, 1e-2, 1.0, 1e2, 1e4, 1e6, 1.37e7] {
            for c in [
                Point2::new(0.0, 0.0),
                Point2::new(0.7 * scale, -1.3 * scale),
            ] {
                for &theta in &turns {
                    for w in [1.0, -1.0] {
                        let start = 0.4;
                        let on = |t: f64| Point2::new(c.x + scale * t.cos(), c.y + scale * t.sin());
                        let (a, b) = (on(start), on(start + w * theta));
                        let winding = if w > 0.0 { ArcSweep::Ccw } else { ArcSweep::Cw };
                        let x = crate::bulge_from_center(a, b, c, winding);
                        let f = center_sweep(a, b, c, a.distance(c), winding);
                        let lift = |p: Point2<f64>| {
                            Point2::new(Interval::from_f64(p.x), Interval::from_f64(p.y))
                        };
                        let (ia, ib, ic) = (lift(a), lift(b), lift(c));
                        let xi = crate::bulge_from_center(ia, ib, ic, winding);
                        let i = center_sweep(ia, ib, ic, (ia - ic).norm(), winding);
                        println!(
                            "TADUMP {{\"k\":\"C\",\"a\":[{:e},{:e}],\"o\":[{:e},{:e}],\"b\":[{:e},{:e}],\
                             \"turn\":{:e},\"w\":{w},\"f\":{:e},\"i\":[{:e},{:e}],\"x\":{:e},\
                             \"xi\":[{:e},{:e}]}}",
                            a.x,
                            a.y,
                            c.x,
                            c.y,
                            b.x,
                            b.y,
                            theta,
                            f,
                            i.lo(),
                            i.hi(),
                            x,
                            xi.lo(),
                            xi.hi(),
                        );
                    }
                }
            }
        }
    }
}

/// **What a stored fillet arc can carry of the tangency the door
/// computed** — the measurement behind the door's own stored-form
/// check, over four decades of corner turn at whatever ε the run
/// commits.
///
/// A fillet arc is stored as its carrier and sweep between two
/// vertices. Its sagitta, `L·b/2 = r(1 − cos(θ/2))` with `b` the
/// sweep's quarter-tangent, is the margin `segment_straightness`
/// classifies, so a fillet whose turn is small enough that
/// `r·θ²/8 ≤ ε` is stored as a segment the validator reads as a
/// **line**: the carrier the door computed is not in the stored form at
/// all. The door's own carrier, by contrast, is tangent to both legs to
/// within a few ulps at every turn — which is what
/// [`the_door_computes_its_tangency_to_the_ulp`] asserts and what
/// decides that the door, not the classifier, is the side that has to
/// ask before it emits.
#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]
mod fillet_stored_form {
    use super::*;
    use crate::path::verbs::Center;
    use crate::seg::{self, JointClass, Seg, SegIssue, SegKind};
    use crate::sugar::ArcSweep;
    use crate::{ConstructedProfile, SketchPlane};
    use geom_core::Arc2;

    /// The fillet radius every corner below is rounded with.
    const R: f64 = 0.2;

    /// The arrival leg's carrier. The door's fillet arc is tangent to it
    /// at `t2`, so the carrier's unit tangent there plus the turn sense
    /// reconstructs the centre the door computed — `fillet_arc_carrier`'s
    /// `t2 + n̂·σr`, read from the stored end point.
    #[derive(Clone, Copy)]
    enum Arrival {
        Ray(Vec2<f64>),
        Circle { c: Point2<f64>, ccw: bool },
    }

    impl Arrival {
        /// The direction of travel along the carrier at `p`.
        fn tangent_at(self, p: Point2<f64>) -> Vec2<f64> {
            match self {
                Arrival::Ray(u) => u,
                Arrival::Circle { c, ccw } => {
                    let radial = p - c;
                    let t = Vec2::new(-radial.y, radial.x) / radial.norm_squared().sqrt();
                    if ccw { t } else { -t }
                }
            }
        }
    }

    /// One corner of the sweep: the door that rounds it, the arrival
    /// carrier its fillet arc ends tangent to, and a name for the table.
    struct Corner {
        name: &'static str,
        build: fn(f64) -> Result<ConstructedLoop<f64>, PathError<f64>>,
        arrival: fn(f64) -> Arrival,
    }

    /// The same segment, re-carried onto the circle (`centre`,
    /// `radius`) — the carrier the DOOR computed, in place of the
    /// stored one. Everything else about the segment
    /// is kept, so [`seg::joint_tangency`] classifies the door's carrier
    /// through exactly the arms it classifies the stored one through:
    /// the measurement is a substitution, not a second expression.
    fn on_the_doors_carrier(arc: &Seg<f64>, centre: Point2<f64>, radius: f64) -> Seg<f64> {
        let toward_apex = (arc.a.lerp(arc.b, 0.5) - centre).normalize();
        let apex = centre + toward_apex * radius;
        let (sweep, turn) = match &arc.kind {
            SegKind::Arc(g) => (g.arc.sweep, g.turn),
            SegKind::Line => (0.0, Sign::Zero),
        };
        Seg {
            kind: SegKind::Arc(seg::ArcGeom {
                arc: Arc2 {
                    centre,
                    radius,
                    sweep,
                },
                span: seg::ArcSpan::Chord {
                    apex,
                    span_chord: arc.a.distance(apex),
                },
                turn,
            }),
            ..*arc
        }
    }

    /// A joint's reading as one printable cell: the class, the predicate
    /// the funnel stopped at and the margin it classified — all three
    /// from [`seg::joint_tangency`], none of them re-derived here.
    fn reading(prev: &Seg<f64>, next: &Seg<f64>, band: Band) -> (String, f64) {
        match seg::joint_tangency(prev, next, "path_fillet_joint_turn", band) {
            Ok(j) => (
                format!(
                    "{} ({} {:e})",
                    match j.class {
                        JointClass::Tangent => "tangent",
                        JointClass::Transversal => "TRANSVERSAL",
                        JointClass::SameCarrier => "same-carrier",
                    },
                    j.predicate,
                    j.margin
                ),
                j.margin,
            ),
            Err(i) => (
                format!("escalated ({})", i.predicate.unwrap_or("?")),
                f64::NAN,
            ),
        }
    }

    fn short(s: &str) -> String {
        s.chars().take(72).collect()
    }

    /// The stored fillet arc of a door-built loop: the segment whose two
    /// joints the door both constructed tangent.
    fn fillet_index(lp: &ConstructedLoop<f64>) -> Option<usize> {
        let vs = lp.vertices();
        let n = vs.len();
        let declared = lp.constructed_joints();
        (0..n).find(|&i| {
            declared.contains(&i)
                && declared.contains(&((i + 1) % n))
                && matches!(lp.segments()[i], crate::Segment::Arc(..))
        })
    }

    /// What the door emitted for one turn and what the validator reads
    /// back out of it: one table row, and the door's own tangency
    /// margins at both joints for the assertion above the table.
    fn row(
        lp: &ConstructedLoop<f64>,
        arrival: Arrival,
        theta: f64,
        tol: Tol,
    ) -> (String, [f64; 2]) {
        let band = match Band::linear(tol) {
            Ok(b) => b,
            Err(e) => return (format!("| {theta:e} | band: {e} |"), [0.0; 2]),
        };
        let vs = lp.vertices();
        let n = vs.len();
        let Some(s) = fillet_index(lp) else {
            return (
                format!(
                    "| {theta:e} | no joint-declared arc: n = {n}, declared = {:?}, segments = {:?} |",
                    lp.constructed_joints(),
                    lp.segments()
                ),
                [0.0; 2],
            );
        };
        let built: Vec<Result<Seg<f64>, SegIssue<f64>>> = (0..n)
            .map(|i| {
                seg::build_seg(
                    vs[i],
                    vs[(i + 1) % n],
                    lp.segments()[i],
                    seg::Consistency::Decide,
                    band,
                )
            })
            .collect();
        let validates =
            match ConstructedProfile::new(SketchPlane::xy(), vec![lp.clone()]).validate(tol) {
                Ok(_) => "ok".to_string(),
                Err(e) => format!("REFUSED: {}", short(&e.to_string())),
            };
        // The stored sweep's quarter tangent: the sagitta's lever, as
        // validation reads it.
        let bulge = match lp.segments()[s] {
            crate::Segment::Arc(arc) => arc.quarter_tan(),
            crate::Segment::Line => 0.0,
        };
        let chord = vs[s].distance(vs[(s + 1) % n]);
        let head = format!(
            "| {theta:e} | {:e} | {chord:e} | {bulge:e} | {:e} |",
            R * theta,
            (chord * bulge * 0.5).abs()
        );
        let (prev, next) = ((s + n - 1) % n, (s + 1) % n);
        let (Ok(arc), Ok(p), Ok(nx)) = (&built[s], &built[prev], &built[next]) else {
            let tag = |r: &Result<Seg<f64>, SegIssue<f64>>| match r {
                Ok(Seg {
                    kind: SegKind::Line,
                    ..
                }) => "line".to_string(),
                Ok(_) => "arc".to_string(),
                Err(e) => format!("refused: {}", e.predicate()),
            };
            return (
                format!(
                    "{head} prev {}, fillet {}, next {} | - | - | - | - | - | - | - | - | built | \
                     {validates} |",
                    tag(&built[prev]),
                    tag(&built[s]),
                    tag(&built[next])
                ),
                [0.0; 2],
            );
        };
        let t2 = arc.b;
        let u2 = arrival.tangent_at(t2);
        let sigma = if bulge >= 0.0 { R } else { -R };
        let door_centre = t2 + Vec2::new(-u2.y, u2.x) * sigma;
        let carried = on_the_doors_carrier(arc, door_centre, R);
        let (door_in, m_door_in) = reading(p, &carried, band);
        let (door_out, m_door_out) = reading(&carried, nx, band);
        let (joint_in, _) = reading(p, arc, band);
        let (joint_out, _) = reading(arc, nx, band);
        let (d_centre, d_radius, kind) = match &arc.kind {
            SegKind::Arc(g) => (
                format!("{:e}", g.arc.centre.distance(door_centre)),
                format!("{:e}", (g.arc.radius - R).abs()),
                "arc",
            ),
            SegKind::Line => ("stored as a line".to_string(), "-".to_string(), "line"),
        };
        (
            format!(
                "{head} {kind} | {d_centre} | {d_radius} | {joint_in} | {joint_out} | \
                 {door_in} | {door_out} | built | {validates} |"
            ),
            [m_door_in, m_door_out],
        )
    }

    /// The item's line × line bend: the incoming ray runs east from the
    /// origin, the corner sits at (4, 0), the arrival leaves it at
    /// `theta`, anchored three units along.
    fn line_line(theta: f64) -> Result<ConstructedLoop<f64>, PathError<f64>> {
        let anchor = Point2::new(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin());
        Open.at(Point2::new(0.0, 0.0))
            .angle(0.0, Tol::witness())?
            .fillet(R, Tol::witness())?
            .at(anchor, Tol::witness())?
            .angle(theta, Tol::witness())?
            .line(1.0, Tol::witness())?
            .line_to(Start, Tol::witness())
            .map(|c| c.loop_)
    }

    /// The line × arc corner's arrival circle: radius 2, counterclockwise
    /// tangent (cos θ, sin θ) at the corner (4, 0).
    fn line_arc_centre(theta: f64) -> Point2<f64> {
        Point2::new(4.0 - 2.0 * theta.sin(), 2.0 * theta.cos())
    }

    /// A line × arc corner turning by `theta`: the east ray from the
    /// origin meets that circle at (4, 0) and the fillet closes along it.
    fn line_arc(theta: f64) -> Result<ConstructedLoop<f64>, PathError<f64>> {
        let c = line_arc_centre(theta);
        let start = c + Vec2::new(2.0 * theta.cos(), 2.0 * theta.sin());
        Open.at(start)
            .line_to(Point2::new(0.0, 0.0), Tol::witness())?
            .toward(1.0, 0.0, Tol::witness())?
            .fillet_arc(
                R,
                Center {
                    c,
                    winding: ArcSweep::Ccw,
                    p: Start,
                },
                Tol::witness(),
            )
            .map(|c| c.loop_)
    }

    /// An arc × arc corner turning by `theta`: two radius-2 circles about
    /// (−θ, 0) and (θ, 0) cross at (0, √(4 − θ²)), where their tangents
    /// are an angle θ apart — the vesica of the arc × arc fixtures, with
    /// its corner opened out to a shallow turn.
    fn arc_arc(theta: f64) -> Result<ConstructedLoop<f64>, PathError<f64>> {
        Open.arc_fillet_arc(
            Center {
                c: Point2::new(-theta, 0.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(2.0 - theta, 0.0),
            },
            R,
            Center {
                c: Point2::new(theta, 0.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(theta - 2.0, 0.0),
            },
            Tol::witness(),
        )?
        .line_to(Start, Tol::witness())
        .map(|c| c.loop_)
    }

    fn corners() -> [Corner; 3] {
        [
            Corner {
                name: "line x line",
                build: line_line,
                arrival: |t| Arrival::Ray(Vec2::new(t.cos(), t.sin())),
            },
            Corner {
                name: "line x arc",
                build: line_arc,
                arrival: |t| Arrival::Circle {
                    c: line_arc_centre(t),
                    ccw: true,
                },
            },
            Corner {
                name: "arc x arc",
                build: arc_arc,
                arrival: |t| Arrival::Circle {
                    c: Point2::new(t, 0.0),
                    ccw: true,
                },
            },
        ]
    }

    /// The swept turns: `c·10^k`, `c ∈ {1, 2, 5}`, `k ∈ {−9 … −2}`.
    fn turns() -> impl Iterator<Item = f64> {
        (-9i32..=-2).flat_map(|k| [1.0, 2.0, 5.0].map(|c| c * 10f64.powi(k)))
    }

    /// **The door computes the tangency exactly; the stored form is what
    /// loses it.** At every turn the door builds, the carrier the door
    /// computed — radius `r`, centred `r` off the arrival carrier at the
    /// arc's end — turns from both legs by a few ulps, levered by the
    /// joint's arm, whatever the stored segment reads back as. The table
    /// this prints (`--nocapture`) is the per-decade measurement: the
    /// stored kind, the two carrier errors, the margins both sides
    /// decide on, and both verdicts.
    #[test]
    fn the_door_computes_its_tangency_to_the_ulp() {
        let tol = Tol::witness();
        // What the sweep MEASURES, with stated headroom rather than a
        // round number well above it: the largest door-carrier margin
        // anywhere in these three tables at any of the three epsilon
        // rows is 8.9e-15, a heading rounded by an ulp or two and
        // levered by the corners' 4 m legs, and this pins 4.4e-14 —
        // five times the measured worst case. A pin an order of
        // magnitude looser would pass a door whose carrier had stopped
        // being exact.
        let ulps = 200.0 * f64::EPSILON;
        for corner in corners() {
            println!(
                "\n### {}   eps = {:e}  K = {}  band = ({:e}, {:e})  r = {R}",
                corner.name,
                tol.eps(),
                tol.k(),
                tol.eps(),
                tol.eps() * tol.k()
            );
            println!(
                "| turn | r*turn | chord | bulge | sagitta | stored kind | |dcentre| | |dradius| \
                 | joint in | joint out | door in | door out | door | validate |"
            );
            println!("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
            for theta in turns() {
                match (corner.build)(theta) {
                    Err(e) => println!(
                        "| {theta:e} | {:e} | - | - | - | - | - | - | - | - | - | - | \
                         REFUSED: {} | - |",
                        R * theta,
                        short(&e.to_string())
                    ),
                    Ok(lp) => {
                        let (line, door) = row(&lp, (corner.arrival)(theta), theta, tol);
                        println!("{line}");
                        for (joint, margin) in door.iter().enumerate() {
                            assert!(
                                margin.abs() <= ulps,
                                "{}, turn {theta:e}: the door's own carrier misses tangency at \
                                 joint {joint} by {margin:e}",
                                corner.name
                            );
                        }
                        // The door's promise: nothing it builds carries
                        // a constructed tangency validation refuses.
                        if let Err(e) =
                            ConstructedProfile::new(SketchPlane::xy(), vec![lp]).validate(tol)
                        {
                            assert!(
                                !matches!(e, crate::ProfileError::TangencyContradicted { .. }),
                                "{}, turn {theta:e}: the door built a loop validation refuses \
                                 for its constructed tangency: {e}",
                                corner.name
                            );
                        }
                    }
                }
            }
        }
    }
}

/// **A constructed arc is read back against what was authored, at the
/// door** ([`arc_reads_back`]). #5403's three silent misses, spelled
/// here as they stood before it: the tangent arc's, the `Center` arc's
/// and the `Via` arc's quarter-tangent through
/// [`crate::sugar::split_half_tan`]'s first arm alone,
/// `across/(len + along)`, which cancels past a half turn. Each probe
/// is #5403's: a target 2.7 m of arc short of the full turn.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod stored_arc_landing {
    use super::*;
    use crate::structure::{Piece, PieceRole};
    use crate::sugar::{ArcSweep, centre_split, split_half_tan_near};
    use core::f64::consts::{FRAC_PI_2, TAU};
    use geom_core::{Bounds, Interval};

    const CHORD: f64 = 2.7;

    const PIECE: Piece = Piece {
        step: 1,
        role: PieceRole::Leg,
    };

    /// The tangent probe: the tip at the origin heading `+x`, the target
    /// at chord 2.7 behind it on the carrier of radius `r`.
    fn tangent_probe(r: f64) -> (Point2<f64>, Vec2<f64>, Point2<f64>) {
        (
            Point2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Point2::new(-CHORD, CHORD * CHORD / (2.0 * r)),
        )
    }

    /// The `Center` probe: the tip at the origin, the centre at `(0, r)`,
    /// the end 2.7 m of arc short of the full counter-clockwise turn.
    fn centre_probe(r: f64) -> (Point2<f64>, Point2<f64>, Point2<f64>) {
        let centre = Point2::new(0.0, r);
        let (s, c) = (-FRAC_PI_2 + TAU - CHORD / r).sin_cos();
        (
            Point2::new(0.0, 0.0),
            centre,
            Point2::new(centre.x + r * c, centre.y + r * s),
        )
    }

    /// The `Via` probe: the carrier tangent to `+x` at the origin with
    /// radius `r`, through its top `q = (0, 2r)`, to the end 2.7 m of
    /// arc short of the full turn.
    fn via_probe(r: f64) -> (Point2<f64>, Point2<f64>, Point2<f64>) {
        let turn = TAU - CHORD / r;
        (
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 2.0 * r),
            Point2::new(r * turn.sin(), r * (1.0 - turn.cos())),
        )
    }

    /// [`tangent_arc`] with the cancelling quarter-tangent.
    fn cancelling_tangent<T: Real>(a: Point2<T>, u: Vec2<T>, b: Point2<T>) -> Arc2<T> {
        let d = b - a;
        let (along, across) = (u.dot(d), u.perp_dot(d));
        let rho = d.norm_squared() / (T::from_f64(2.0) * across);
        Arc2 {
            centre: a + Vec2::new(-u.y, u.x) * rho,
            radius: rho.abs(),
            sweep: T::from_f64(4.0) * split_half_tan_near(across, along, d.norm()).atan(),
        }
    }

    /// [`center_arc`], counter-clockwise, with the cancelling
    /// quarter-tangent.
    fn cancelling_centre<T: Real>(a: Point2<T>, b: Point2<T>, centre: Point2<T>) -> Arc2<T> {
        let radius = (a - centre).norm();
        let (across, along) = centre_split(a, b, centre, T::one());
        Arc2 {
            centre,
            radius,
            sweep: T::from_f64(4.0) * split_half_tan_near(across, along, radius).atan(),
        }
    }

    /// [`via_arc`] with the cancelling quarter-tangent: lowered from its
    /// chord, so it lands on `b` whatever X is, and X moves its carrier
    /// off `q`.
    fn cancelling_via(a: Point2<f64>, q: Point2<f64>, b: Point2<f64>) -> BuiltArc<f64> {
        let (d1, d2) = (q - a, b - q);
        let x = split_half_tan_near(d1.perp_dot(d2), d1.dot(d2), d1.norm() * d2.norm());
        BuiltArc::lowered(a, b, x).through(Some(q))
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The largest magnitude at which a read of `ulps` rounding stays
    /// inside the band ([`seg::resolution_headroom`] at zero).
    fn resolvable(ulps: f64) -> f64 {
        seg::resolution_headroom(0.0, ulps, band())
    }

    /// The probe radius at this run's ε: the first of a ladder at which
    /// the cancelling arc misses past twice the band, on a scene that
    /// resolves twice that magnitude.
    fn probe_radius(ulps: f64, miss: impl Fn(f64) -> f64) -> f64 {
        let ladder = (4..=16).map(|k| 10f64.powf(f64::from(k) / 2.0));
        ladder
            .clone()
            .find(|&r| miss(r) > 2.0 * band().escalate() && 2.0 * r < resolvable(ulps))
            .unwrap_or_else(|| {
                let misses: Vec<(f64, f64)> = ladder.map(|r| (r, miss(r))).collect();
                panic!(
                    "no ladder radius gives a resolvable miss past the band ({:e}): {misses:?}",
                    band().escalate()
                )
            })
    }

    fn tangent_miss(r: f64) -> f64 {
        let (a, u, b) = tangent_probe(r);
        cancelling_tangent(a, u, b).landing(a).distance(b)
    }

    fn centre_miss(r: f64) -> f64 {
        let (a, c, b) = centre_probe(r);
        cancelling_centre(a, b, c).landing(a).distance(b)
    }

    fn via_miss(r: f64) -> f64 {
        let (a, q, b) = via_probe(r);
        let arc = cancelling_via(a, q, b).arc;
        (arc.rim(q) - arc.radius).abs()
    }

    fn built<T: Real>(arc: Arc2<T>) -> BuiltArc<T> {
        BuiltArc::new(arc, crate::Facts::Registered)
    }

    fn read<T: Decide>(
        arc: BuiltArc<T>,
        a: Point2<T>,
        b: Point2<T>,
        verb: program::Verb,
    ) -> Result<(), PathError<T>> {
        arc_reads_back(arc, a, b, band(), PIECE, verb)
    }

    /// The refusal's miss, where the read refused as a kernel defect
    /// naming `predicate`, the step and the verb.
    fn defect(
        result: Result<(), PathError<f64>>,
        predicate: &str,
        verb: program::Verb,
        what: &str,
    ) -> f64 {
        match result {
            Err(PathError::ArcMissesAuthoredPoint {
                piece,
                verb: said,
                predicate: read,
                miss,
                ..
            }) => {
                assert_eq!(
                    (piece, said, read),
                    (PIECE, verb, predicate),
                    "{what}: the refusal names its step, verb and read"
                );
                miss
            }
            other => panic!("{what}: the read-back answered {other:?}"),
        }
    }

    /// The cancelling tangent arc refuses at `f64` as a kernel defect;
    /// the door's own arc, on the same probe, passes.
    #[test]
    fn a_cancelling_tangent_arc_refuses_at_f64() {
        let r = probe_radius(seg::LANDING_ULPS, tangent_miss);
        let (a, u, b) = tangent_probe(r);
        let verb = program::Verb::TangentArcTo;
        let miss = defect(
            read(built(cancelling_tangent(a, u, b)), a, b, verb),
            seg::LANDING_READ,
            verb,
            "the cancelling tangent arc",
        );
        assert_eq!(
            miss,
            tangent_miss(r),
            "the refusal reports the miss it read"
        );
        assert!(
            read(tangent_arc(a, u, b), a, b, verb).is_ok(),
            "the door's tangent arc at r = {r:e} passes"
        );
    }

    /// The cancelling `Center` arc refuses at `f64` as a kernel defect;
    /// the door's own arc about the same centre passes.
    #[test]
    fn a_cancelling_centre_arc_refuses_at_f64() {
        let r = probe_radius(seg::LANDING_ULPS, centre_miss);
        let (a, c, b) = centre_probe(r);
        let verb = program::Verb::ArcTo;
        // The door's own `Center` construction (decided, wound Ccw) with
        // only its sweep spelled the cancelling way.
        let cancelling = BuiltArc {
            arc: cancelling_centre(a, b, c),
            ..center_arc(a, b, c, ArcSweep::Ccw)
        };
        assert_eq!(
            cancelling.facts,
            crate::Facts::Decided,
            "premise: a decided arc"
        );
        let miss = defect(
            read(cancelling, a, b, verb),
            seg::LANDING_READ,
            verb,
            "the cancelling centre arc",
        );
        let arc = cancelling.arc;
        let towards = b - arc.centre;
        let onto = arc.centre + towards * (arc.rim(a) / towards.norm());
        assert_eq!(
            miss,
            arc.landing(a).distance(onto),
            "the refusal reports the sweep's miss it read: the landing against the end vertex \
             moved onto its circle"
        );
        assert!(
            read(center_arc(a, b, c, ArcSweep::Ccw), a, b, verb).is_ok(),
            "the door's centre arc at r = {r:e} passes"
        );
    }

    /// The cancelling `Via` arc lands on its end vertex, so only its
    /// through-point read sees it: it refuses at `f64` as a kernel
    /// defect off `q`, and the door's own `Via` arc passes.
    #[test]
    fn a_cancelling_via_arc_refuses_off_its_through_point_at_f64() {
        let r = probe_radius(seg::READ_BACK_RIM_ULPS, via_miss);
        let (a, q, b) = via_probe(r);
        let verb = program::Verb::ArcTo;
        let arc = cancelling_via(a, q, b);
        assert!(
            read(arc.through(None), a, b, verb).is_ok(),
            "premise: the cancelling Via arc at r = {r:e} lands on its end vertex"
        );
        let miss = defect(
            read(arc, a, b, verb),
            seg::THROUGH_POINT_READ,
            verb,
            "the cancelling Via arc",
        );
        assert_eq!(miss, via_miss(r), "the refusal reports the miss it read");
        let door = via_arc(a, q, b).unwrap();
        assert_eq!(
            door.through.map(|p| (p.x, p.y)),
            Some((q.x, q.y)),
            "the Via leg carries its q"
        );
        assert!(
            read(door, a, b, verb).is_ok(),
            "the door's Via arc at r = {r:e} passes"
        );
    }

    /// **The `Via` arc near a full turn holds its `q`**: the read-back
    /// sweep's (`tests/arc_read_back_sweep.rs`) first counterexample, an
    /// arc 4.4e-5 rad short of the full turn at R = 1.4e4 m, which sat
    /// 4.8e-8 m off its `q` while `bulge_from_via` read its turn's
    /// cross term off the two long, nearly opposed chords at `q`. The
    /// door's arc now holds `q` to 8 ulps of the read's magnitude, and
    /// the read passes wherever those 8 ulps stay under ε.
    #[test]
    fn the_sweeps_via_arc_near_a_full_turn_holds_its_through_point() {
        let (centre, r) = (
            Point2::new(1103.8818785097276, 1390.3359303537966),
            1.4351494718012413e4,
        );
        let (start, turn, through) = (5.8899133812414, -6.283141957025729, 0.6992541876608326);
        let at = |t: f64| {
            let (s, c) = t.sin_cos();
            Point2::new(centre.x + r * c, centre.y + r * s)
        };
        let (a, b, q) = (at(start), at(start + turn), at(start + through * turn));
        let door = via_arc(a, q, b).unwrap();
        let scale = door
            .arc
            .radius
            .max(seg::reach(door.arc.centre))
            .max(seg::reach(q));
        let off = (door.arc.rim(q) - door.arc.radius).abs();
        assert!(
            off <= 8.0 * scale * f64::EPSILON,
            "the Via arc is {off:e} m off its q, past 8 ulps of {scale:e}"
        );
        let answer = read(door, a, b, program::Verb::ArcTo);
        if 8.0 * scale * f64::EPSILON <= band().zero() {
            assert!(
                answer.is_ok(),
                "the door's Via arc passes its read-back: {answer:?}"
            );
        } else {
            assert!(
                matches!(
                    answer,
                    Ok(())
                        | Err(PathError::ArcMissBelowSceneResolution { .. }
                            | PathError::Escalated { .. })
                ),
                "past the read's resolution the door's Via arc refuses only as the scene's: \
                 {answer:?}"
            );
        }
    }

    /// At `Interval` the cancelling arcs are wide, not wrong: each
    /// stored enclosure lands around the end vertex, so the read, which
    /// an exact scalar does not run, has no miss to find there.
    #[test]
    fn a_cancelling_arc_at_interval_encloses_its_end_vertex() {
        let iv = |p: Point2<f64>| Point2::new(Interval::from_f64(p.x), Interval::from_f64(p.y));
        let holds = |end: Point2<Interval>, b: Point2<f64>, what: &str| {
            for (e, t) in [(end.x, b.x), (end.y, b.y)] {
                assert!(
                    e.lo() <= t && t <= e.hi(),
                    "{what}: the landing [{:e}, {:e}] does not hold {t:e}",
                    e.lo(),
                    e.hi()
                );
            }
        };
        let verb = program::Verb::TangentArcTo;
        let r = probe_radius(seg::LANDING_ULPS, tangent_miss);
        let (a, u, b) = tangent_probe(r);
        let (ai, bi) = (iv(a), iv(b));
        let ui = Vec2::new(Interval::from_f64(u.x), Interval::from_f64(u.y));
        let arc = cancelling_tangent(ai, ui, bi);
        assert!(
            read(built(arc), ai, bi, verb).is_ok(),
            "exact scalar: no read"
        );
        holds(arc.landing(ai), b, "the cancelling tangent arc");
        let r = probe_radius(seg::LANDING_ULPS, centre_miss);
        let (a, c, b) = centre_probe(r);
        let (ai, bi) = (iv(a), iv(b));
        let arc = cancelling_centre(ai, bi, iv(c));
        assert!(
            read(built(arc), ai, bi, verb).is_ok(),
            "exact scalar: no read"
        );
        holds(arc.landing(ai), b, "the cancelling centre arc");
    }

    /// A miss the scene cannot tell from its own rounding still
    /// refuses, as the scene-resolution sibling, naming its step, read
    /// and recourse: the door's tangent arc on a carrier at four times
    /// the magnitude whose landing rounding reaches the band, read
    /// against an end vertex moved 1.5·Kε off it, inside that rounding.
    /// The cancelling arcs' misses stand clear of their rounding and
    /// refuse as defects ([`a_cancelling_tangent_arc_refuses_at_f64`]),
    /// so the two together pin which side of the gate each answer is
    /// on.
    #[test]
    fn a_miss_the_scene_cannot_resolve_refuses_as_the_scene_resolution_sibling() {
        let r = 4.0 * resolvable(seg::LANDING_ULPS);
        let (a, u, b) = tangent_probe(r);
        let arc = tangent_arc(a, u, b).arc;
        let verb = program::Verb::TangentArcTo;
        let off = Point2::new(b.x, b.y + 1.5 * band().escalate());
        let b = off;
        assert!(
            arc.landing(a).distance(b) > band().escalate(),
            "premise: the arc at r = {r:e} misses past the band"
        );
        let refusal = read(built(arc), a, b, verb).unwrap_err();
        match &refusal {
            PathError::ArcMissBelowSceneResolution {
                piece,
                predicate,
                scale,
                resolution,
                ..
            } => {
                assert_eq!(
                    (*piece, *predicate),
                    (PIECE, seg::LANDING_READ),
                    "the sibling names its step and read"
                );
                assert!(
                    *scale >= r && *resolution > band().escalate(),
                    "the sibling reports a magnitude ({scale:e}) whose rounding ({resolution:e}) \
                     reaches the band"
                );
            }
            other => panic!("a miss at r = {r:e} answered {other:?}"),
        }
        let said = refusal.to_string();
        assert!(
            said.contains("step 1's leg (tangent_arc_to)")
                && said.contains("its end vertex")
                && said.contains(crate::validate::ARC_SCENE_RESOLUTION_RECOURSE),
            "the sibling's sentence names the step, the point and the recourse: {said}"
        );
    }

    /// **An authored bulge that reads as a line is not read back**: a
    /// `Bulge` leg of 1e-320 on a centimetre chord, whose radius
    /// overflows, so its carrier is not a circle; its sagitta, 5e-323 m,
    /// reads Zero, and the bulge is the authored data, so validation
    /// reads the segment as the line it was authored as. The same
    /// stored arc from a construction that authored an arc is read, and
    /// its poisoned carrier escalates.
    #[test]
    fn an_authored_bulge_that_reads_as_a_line_is_not_read_back() {
        let (a, b) = (Point2::new(0.0, 0.0), Point2::new(0.01, 0.0));
        let flat = BuiltArc::lowered(a, b, 1e-320);
        let authored = BuiltArc {
            bulge_authored: true,
            ..flat
        };
        let verb = program::Verb::ArcTo;
        assert!(
            read(authored, a, b, verb).is_ok(),
            "an authored line-like bulge passes"
        );
        assert!(
            matches!(read(flat, a, b, verb), Err(PathError::Escalated { .. })),
            "a constructed arc stored flat and poisoned is read and escalates"
        );
    }

    /// **A `Via` arc a defect flattened is read against its `q`**: a
    /// 10 m chord stored with bulge 1e-12, so its stored form reads as a
    /// line, while its authored `q` sits 1 m off the chord. The door
    /// reads it whatever its stored form looks like, and it refuses off
    /// `q`.
    #[test]
    fn a_flattened_via_arc_is_read_against_its_through_point() {
        let (a, b, q) = (
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
            Point2::new(5.0, 1.0),
        );
        let flat = BuiltArc::lowered(a, b, 1e-12)
            .through(Some(q))
            .sensed(ArcSweep::Cw);
        let verb = program::Verb::ArcTo;
        defect(
            read(flat, a, b, verb),
            seg::THROUGH_POINT_READ,
            verb,
            "the flattened Via arc",
        );
        let door = via_arc(a, q, b).unwrap().sensed(ArcSweep::Cw);
        assert!(read(door, a, b, verb).is_ok(), "the door's Via arc passes");
    }

    /// **A stored radius off its start vertex refuses**: the door's
    /// quarter arc with its radius 1 mm long, both ends still on the
    /// circle about its centre at the stored sweep's angles, so the
    /// landing reads Zero and only the start read sees the radius.
    #[test]
    fn a_stored_radius_off_its_start_vertex_refuses() {
        let (a, b, c) = (
            Point2::new(10.0, 0.0),
            Point2::new(0.0, 10.0),
            Point2::new(0.0, 0.0),
        );
        let door = center_arc(a, b, c, ArcSweep::Ccw);
        let verb = program::Verb::ArcTo;
        assert!(
            read(door, a, b, verb).is_ok(),
            "the door's centre arc passes"
        );
        let long = BuiltArc {
            arc: Arc2 {
                radius: door.arc.radius + 1e-3,
                ..door.arc
            },
            ..door
        };
        let miss = defect(
            read(long, a, b, verb),
            seg::START_READ,
            verb,
            "the long radius",
        );
        assert!(
            (miss - 1e-3).abs() < 1e-12,
            "the start read reports the 1 mm: {miss:e}"
        );
    }

    /// **The complementary arc refuses on its branch**: the arc on the
    /// same carrier between the same ends, the other way round, lands
    /// exactly, and is what a quarter-tangent X spelled −1/X builds
    /// (4·atan(−1/X) = 4·atan(X) ∓ 2π). Each construction authors its
    /// turn sense from data other than X, and the door reads the stored
    /// sweep's branch against it: the `Via` leg (the side of the chord
    /// `q` lies on), the `Center` leg (its winding) and the tangent arc
    /// (the side of the departure its target lies on), each spelled with
    /// X → −1/X, refuse `path_arc_branch` at `f64`.
    #[test]
    fn a_complementary_arc_refuses_on_its_branch() {
        let verb = program::Verb::ArcTo;
        let flip = |built: BuiltArc<f64>| {
            let x = built.arc.quarter_tan();
            BuiltArc {
                arc: Arc2 {
                    sweep: 4.0 * (-1.0 / x).atan(),
                    ..built.arc
                },
                ..built
            }
        };
        let (a, q, b) = (
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 1.0),
            Point2::new(2.0, 0.0),
        );
        let via = via_arc(a, q, b).unwrap().sensed(ArcSweep::Cw);
        assert!(read(via, a, b, verb).is_ok(), "the door's Via arc passes");
        assert!(
            flip(via).arc.landing(a).distance(b) < 1e-12,
            "premise: the complementary arc lands"
        );
        defect(
            read(flip(via), a, b, verb),
            seg::BRANCH_READ,
            verb,
            "the flipped Via arc",
        );
        let c = Point2::new(1.0, 0.0);
        let centre = center_arc(a, b, c, ArcSweep::Ccw);
        assert!(
            read(centre, a, b, verb).is_ok(),
            "the door's centre arc passes"
        );
        defect(
            read(flip(centre), a, b, verb),
            seg::BRANCH_READ,
            verb,
            "the flipped centre arc",
        );
        let u = Vec2::new(0.0, 1.0);
        let geom = tangent_arc_geom(a, Dir::from_unit(u), b, Tol::witness()).unwrap();
        let tangent = geom.arc.unwrap();
        let verb = program::Verb::TangentArcTo;
        assert!(
            read(tangent, a, b, verb).is_ok(),
            "the door's tangent arc passes"
        );
        defect(
            read(flip(tangent), a, b, verb),
            seg::BRANCH_READ,
            verb,
            "the flipped tangent arc",
        );
    }

    /// **A decided arc reads its sweep, not the decision's slack**: a
    /// `Center`-shaped arc (`Facts::Decided`) whose end vertex lies on
    /// its carrier, with its sweep off by a defect that moves its
    /// landing along the carrier. Inside the band it escalates, past it
    /// it refuses, exactly as a registered arc does; only the radial
    /// part a decision admitted (the end off the carrier by under ε,
    /// with the sweep exact) passes.
    #[test]
    fn a_decided_arcs_sweep_is_read_like_any_other() {
        let verb = program::Verb::ArcTo;
        let (eps, esc) = (band().zero(), band().escalate());
        let (c, r) = (Point2::new(0.0, 0.0), 10.0);
        let at = |t: f64, rho: f64| Point2::new(c.x + rho * t.cos(), c.y + rho * t.sin());
        let a = at(0.0, r);
        let decided = |sweep: f64| {
            BuiltArc::new(
                Arc2 {
                    centre: c,
                    radius: r,
                    sweep,
                },
                crate::Facts::Decided,
            )
            .sensed(ArcSweep::Ccw)
        };
        let b = at(1.0, r);
        for frac in [0.1, 0.5, 0.9] {
            let miss = eps + frac * (esc - eps);
            let off = decided(1.0 + miss / r);
            assert!(
                matches!(read(off, a, b, verb), Err(PathError::Escalated { .. })),
                "a decided arc whose sweep misses by {miss:e}, inside the band, escalates"
            );
        }
        defect(
            read(decided(1.0 + 2.0 * esc / r), a, b, verb),
            seg::LANDING_READ,
            verb,
            "a decided arc past the band",
        );
        let admitted = at(1.0, r + 0.9 * eps);
        assert!(
            read(decided(1.0), a, admitted, verb).is_ok(),
            "an end off the carrier by what the decision admitted passes"
        );
        // The decision's slack and a sweep error together: the end up to
        // 0.99ε off the carrier on either side, the sweep off by 0.5ε of
        // arc (passes) or by 1.2ε to 2.5ε (escalates). The radial part
        // hides none of the sweep's.
        for rho in [0.0, 0.5 * eps, 0.99 * eps, -0.5 * eps, -0.99 * eps] {
            let end = at(1.0, r - rho);
            assert!(
                read(decided(1.0 + 0.5 * eps / r), a, end, verb).is_ok(),
                "a decided end {rho:e} inside the carrier with a 0.5ε sweep error passes"
            );
            for sweep in [1.2 * eps, 1.5 * eps, 1.9 * eps, 2.5 * eps] {
                let off = decided(1.0 + sweep / r);
                assert!(
                    matches!(read(off, a, end, verb), Err(PathError::Escalated { .. })),
                    "a decided end {rho:e} inside the carrier with a {sweep:e} sweep defect \
                     escalates"
                );
            }
        }
    }

    /// **A read that overflows is never a defect**: a centimetre chord
    /// whose stored carrier sits 1e160 m away, so the start read's
    /// `‖a − centre‖` squares past the range and reads infinite. The
    /// authored points resolve, but an infinite miss is no measurement,
    /// so it refuses as the scene's.
    #[test]
    fn a_read_that_overflows_is_never_a_defect() {
        let (a, b) = (Point2::new(0.0, 0.0), Point2::new(0.01, 0.0));
        let far = BuiltArc::new(
            Arc2 {
                centre: Point2::new(0.005, -1e160),
                radius: 1e160,
                sweep: 1e-162,
            },
            crate::Facts::Registered,
        );
        assert!(
            !(far.arc.rim(a) - far.arc.radius).is_finite(),
            "premise: the start read overflows"
        );
        assert!(
            matches!(
                read(far, a, b, program::Verb::ArcTo),
                Err(PathError::ArcMissBelowSceneResolution { .. })
            ),
            "an overflowing read refuses as the scene's"
        );
    }

    /// **An arc fillet's complementary arc refuses on its branch**: a
    /// fillet between a ray and an arc carrier, resolved by the door's
    /// own resolution, carries the corner's decided turn as its sense;
    /// the same fillet spelled with X → −1/X lands on the same tangent
    /// point and refuses `path_arc_branch`.
    #[test]
    fn an_arc_fillets_complementary_arc_refuses_on_its_branch() {
        use arc_fillet::{FilletSide, SideCarrier};
        let tol = Tol::witness();
        let trims = arc_fillet::resolve::<f64>(
            &mut crate::structure::Guide::recording(),
            FilletSide {
                anchor: Point2::new(-5.0, 0.0),
                carrier: SideCarrier::Ray(Vec2::new(1.0, 0.0)),
            },
            FilletSide {
                anchor: Point2::new(10.0, 3.0),
                carrier: SideCarrier::Circle {
                    centre: Point2::new(5.0, 3.0),
                    winding: ArcSweep::Ccw,
                },
            },
            0.5,
            tol,
        )
        .expect("the corner resolves");
        let fillet = trims.fillet();
        assert!(
            fillet.sense.is_some(),
            "the arc fillet carries its corner's turn"
        );
        let verb = program::Verb::FilletArc;
        let (a, b) = (trims.t1, trims.t2);
        assert!(
            read(fillet, a, b, verb).is_ok(),
            "the door's arc fillet passes"
        );
        let x = fillet.arc.quarter_tan();
        let flipped = BuiltArc {
            arc: Arc2 {
                sweep: 4.0 * (-1.0 / x).atan(),
                ..fillet.arc
            },
            ..fillet
        };
        defect(
            read(flipped, a, b, verb),
            seg::BRANCH_READ,
            verb,
            "the flipped arc fillet",
        );
    }

    /// A miss inside the band escalates, and so does a poisoned arc;
    /// an exact landing passes. The door's tangent arc, with its end
    /// vertex moved off it by half again ε along the chord's normal,
    /// is the in-band reading.
    #[test]
    fn an_in_band_or_poisoned_read_escalates_and_zero_passes() {
        let verb = program::Verb::TangentArcTo;
        let (a, u, b) = tangent_probe(10.0);
        let arc = tangent_arc(a, u, b);
        assert!(read(arc, a, b, verb).is_ok(), "the door's arc passes");
        let gap = 0.5 * (band().zero() + band().escalate());
        let off = Point2::new(b.x, b.y + gap);
        assert!(
            matches!(read(arc, a, off, verb), Err(PathError::Escalated { .. })),
            "a landing {gap:e} m off, inside the band, escalates"
        );
        let poisoned = Arc2 {
            sweep: f64::NAN,
            ..arc.arc
        };
        assert!(
            matches!(
                read(built(poisoned), a, b, verb),
                Err(PathError::Escalated { .. })
            ),
            "a poisoned arc escalates"
        );
    }
}
