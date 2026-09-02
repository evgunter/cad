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
//! [`turn`](PartialPath::turn), the junction check and the straight
//! continuation, settable by nothing.
//!
//! # Closure
//!
//! [`Start`] is a first-class directed-point value — the bound entry.
//! Using it is closing, structurally: `line_to(Start)`,
//! `arc_to(Bulge { p: Start, b })`, `.tangent().tangent_arc_to(Start)`,
//! and the seam fillet `.angle(θ).fillet(r).to(Start)`. There is
//! deliberately no `close()` alias. The entry authors the first side;
//! the seam is authored once, at the back, by the verb that targets
//! `Start` — a leading `.fillet`/`.tangent()` is ill-typed (they need
//! bits the entry Open lacks).
//!
//! The seam's own junction is the one declaration that cannot ride a
//! departing leg, because the arriving leg is authored LAST. It rides
//! the TARGET, and it is CHECKED, never inferred:
//! [`Start::arrives_straight`] on the straight closers declares the
//! seam a SUBDIVISION point of the entry's first side (which must
//! therefore be a line), and [`Start::arrives_tangent`] on the arc
//! closers declares it a G1 joint. Undeclared, a tangent seam refuses
//! [`PathError::SeamTangent`] from every closing verb, exactly as
//! before.
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
//! use geom_core::{Point2, Tol};
//! use profile::{Open, Profile, SketchPlane, Start};
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
//! assert_eq!(square.loop_.tangent_joints().len(), 8);
//! // The chain also RECORDED itself: the program replays to the same
//! // loop, bit for bit (profiles-as-programs v2 — see [`program`]).
//! assert_eq!(square.program.len(), 13);
//! Profile::new(SketchPlane::xy(), vec![square.loop_]).validate(tol)?;
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

use crate::RawLoop;
use core::marker::PhantomData;

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point2, Real, Sign, Tol, Vec2};

use crate::path::program::{Arrival, ClosedLoop, Step, Target};
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

impl Start {
    /// The entry, targeted with a DECLARED STRAIGHT ARRIVAL: the
    /// closing leg arrives straight into the entry's first side, so the
    /// seam is a declared SUBDIVISION point of one carrier.
    ///
    /// The declaration is the target, because the seam is the one
    /// junction whose arriving leg is the later-authored one — every
    /// declaration that rides the DEPARTING leg elsewhere
    /// (`line(len)`/[`continue_to`](PartialPath::continue_to) for a
    /// straight continuation, [`tangent`](PartialPath::tangent) for a
    /// G1 joint) has no departing leg to ride here: the entry's first
    /// side is already authored and cannot carry the seam's content
    /// from the front (§2's entry rule).
    ///
    /// Nothing is inferred. The kernel CHECKS that the entry's outgoing
    /// direction continues the arriving one, banded through the funnel
    /// exactly as [`continue_to`](PartialPath::continue_to) checks its
    /// target; a seam past the band refuses
    /// [`PathError::SeamArrivalOffDirection`] as inconsistent authored
    /// data. Without the declaration the same seam keeps refusing
    /// [`PathError::SeamTangent`].
    #[must_use]
    pub fn arrives_straight(self) -> ArrivesStraight {
        ArrivesStraight
    }

    /// The entry, targeted with a DECLARED G1 ARRIVAL: the closing ARC
    /// arrives tangent to the entry's outgoing direction, so the seam
    /// is a declared tangent joint (and the lowered loop carries the
    /// flag at joint 0, which the verify layer re-checks).
    ///
    /// The same declaration [`tangent`](PartialPath::tangent) makes on
    /// a departure, moved to the arrival for the reason above. The arc
    /// itself is constructed from the DEPARTURE as it always was — one
    /// end constructs, the other is checked, so nothing is
    /// overdetermined. Where no circular arc can carry both tangencies
    /// the check refuses and names the seam fillet, which constructs
    /// both.
    #[must_use]
    pub fn arrives_tangent(self) -> ArrivesTangent {
        ArrivesTangent
    }
}

/// [`Start`] with the closing leg's STRAIGHT arrival declared — the
/// target of the straight closers ([`line_to`](PartialPath::line_to),
/// [`continue_to`](PartialPath::continue_to)). Built by
/// [`Start::arrives_straight`].
#[derive(Clone, Copy, Debug)]
pub struct ArrivesStraight;

impl sealed::Sealed for ArrivesStraight {}

/// [`Start`] with the closing arc's G1 arrival declared — the target
/// of [`tangent_arc_to`](PartialPath::tangent_arc_to). Built by
/// [`Start::arrives_tangent`].
///
/// It is not a [`LineTarget`]: a straight leg's arrival direction IS
/// its own direction, so "arrives tangent" and "arrives straight" name
/// one fact there and the straight token is that fact's spelling. The
/// missing impl is the §2c matrix discipline — unrepresentable rather
/// than refused.
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
    /// §4 item 1: the AUTHORED departure is within ε_input of the
    /// incoming TANGENT direction — one refusal, one recourse, for any
    /// sub-ε_input margin: if the tangency is intended onto a new
    /// carrier, author it structurally (`.tangent()`, or the
    /// tangent-arc / seam-fillet close at the seam), which makes it
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
    /// the joint; otherwise move the geometry. The declaration is what
    /// the kernel's material-wedge invariant asks for at rest (D1's
    /// tier-3 arm), and it is never inferred from a margin — which is
    /// why an authored near-reverse still refuses here.
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
    /// The recourse is to DECLARE the arrival, or to move the seam.
    /// The arriving leg is the later-authored one, so the declaration
    /// rides the target: [`Start::arrives_straight`] for a straight
    /// leg continuing the entry's first side (the seam becomes a
    /// declared subdivision point), [`Start::arrives_tangent`] for a
    /// closing ARC meeting it G1. Both are CHECKED, not inferred.
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
    /// leg targeted [`Start::arrives_straight`] / [`Start::arrives_tangent`]
    /// — declaring that it arrives continuing the entry's outgoing
    /// direction — and it definitely does not.
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
        /// Which member declared, so the recourse names that member's
        /// spellings rather than both.
        arrival: Arrival,
    },
    /// **A STRAIGHT arrival declared at a seam whose entry side is an
    /// ARC.** [`Start::arrives_straight`] declares that the seam is a
    /// SUBDIVISION point — one carrier continuing through it — which is
    /// the ruling's own words, and that is only true when the entry's
    /// first side is a line. A straight leg arriving along an arc's
    /// start tangent is a G1 joint between DISTINCT carriers, which
    /// #101 says must be declared, and this declaration declares no
    /// tangency.
    ///
    /// Refused HERE rather than left to the data gate: without it the
    /// lattice mints a loop whose only recourse is the raw door's
    /// `tangent_joints`, which is the door issue 433 is demoting. The
    /// authoring layer is the insurance, so it has to speak first.
    ///
    /// The shape is [`SeamRetrimsArcFirstSide`](Self::SeamRetrimsArcFirstSide)'s:
    /// a structural fact about the entry's first side, with a
    /// structural recourse and no measured payload.
    SeamArrivalNeedsStraightFirstSide,
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
    /// **The declared point-target continuation's consistency
    /// refusal**: [`continue_to(target)`](PartialPath::continue_to)
    /// declares the leg to be the straight continuation of the run
    /// LANDING on `target`, and the target does not lie on the
    /// departing point's ray.
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
    /// The requested radius demands the **enclosing** tangency at this
    /// corner: on the named side the signed offset radius ρ = R − σ·τ·r
    /// is negative (σ·τ = +1 with r > R), so every circle of that radius
    /// tangent to that side's carrier with this corner's turn sense
    /// contains the carrier whole — and the corner with it, the corner
    /// being a point of that carrier. An arc that cannot touch the corner
    /// is not a fillet OF that corner, so no fillet of this corner exists
    /// at this radius, and none ever will: the class is permanently out
    /// of reach by design (`docs/ENCLOSING-TANGENCY-DESIGN.md`), which is
    /// why it is its own refusal rather than one of the "no corner"
    /// ones — those would send the author looking for a corner that is
    /// right there. The bound is the named side's carrier radius; the
    /// recourse is a smaller radius.
    FilletEnclosesLegCarrier {
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
        /// The requested radius, meters.
        radius: T,
        /// The EXISTENCE bound, meters, when the corner's two circular
        /// carriers define one: the largest radius that can be tangent to
        /// both of them here, (R₁ + R₂ − d)/2. This is the quantity the
        /// message endorses, because it is the one below which a tangent
        /// circle actually exists; the class bound alone would send an
        /// author to radii that refuse again for a different reason.
        /// `None` on the degenerate corners where the quantity is not
        /// defined at the gate (a straight partner, or a partner whose
        /// own ρ is positive), and there the message endorses no number.
        largest_tangent_radius: Option<T>,
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
    /// leg (or a tip with no incoming leg data) has nothing to split.
    /// The straight case is not missing vocabulary and never needed a
    /// verb of its own: `line(len)` off the directed point IS the
    /// straight continuation, because the binding bits determine a line
    /// carrier completely — subdivide a straight run by chaining it.
    /// (Nothing about a line has to be learned from the incoming leg,
    /// which is exactly the asymmetry with an arc.)
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
    /// [`PathError::SeamArrivalNeedsStraightFirstSide`].
    SeamArrivalNeedsStraightFirstSide,
    /// [`PathError::SeamArrivalLeverTooShort`].
    SeamArrivalLeverTooShort,
    /// [`PathError::SameCarrierJunction`].
    SameCarrierJunction,
    /// [`PathError::ContinuationTargetOffRay`].
    ContinuationTargetOffRay,
    /// [`PathError::NoCornerForFillet`].
    NoCornerForFillet,
    /// [`PathError::AnchorOutsideTrimmedExtent`].
    AnchorOutsideTrimmedExtent,
    /// [`PathError::FilletOffsetLeverTooShort`].
    FilletOffsetLeverTooShort,
    /// [`PathError::FilletEnclosesLegCarrier`].
    FilletEnclosesLegCarrier,
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
    /// [`PathError::CircleSplitCount`].
    CircleSplitCount,
    /// [`PathError::ArcContinueNeedsArcCarrier`].
    ArcContinueNeedsArcCarrier,
    /// [`PathError::ArcContinueOffCarrier`].
    ArcContinueOffCarrier,
    /// [`PathError::ZeroDirection`].
    ZeroDirection,
    /// [`PathError::ArcViaCollinear`].
    ArcViaCollinear,
    /// [`PathError::DegenerateArcChord`].
    DegenerateArcChord,
    /// [`PathError::ArcCenterNotEquidistant`].
    ArcCenterNotEquidistant,
    /// [`PathError::DegenerateArcCenter`].
    DegenerateArcCenter,
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
            Self::SeamArrivalNeedsStraightFirstSide => {
                PathErrorKind::SeamArrivalNeedsStraightFirstSide
            }
            Self::SeamArrivalLeverTooShort { .. } => PathErrorKind::SeamArrivalLeverTooShort,
            Self::SameCarrierJunction { .. } => PathErrorKind::SameCarrierJunction,
            Self::ContinuationTargetOffRay { .. } => PathErrorKind::ContinuationTargetOffRay,
            Self::NoCornerForFillet { .. } => PathErrorKind::NoCornerForFillet,
            Self::AnchorOutsideTrimmedExtent { .. } => PathErrorKind::AnchorOutsideTrimmedExtent,
            Self::FilletOffsetLeverTooShort { .. } => PathErrorKind::FilletOffsetLeverTooShort,
            Self::FilletEnclosesLegCarrier { .. } => PathErrorKind::FilletEnclosesLegCarrier,
            Self::ArcLegOnOpenFillet { .. } => PathErrorKind::ArcLegOnOpenFillet,
            Self::SeamRetrimsArcFirstSide => PathErrorKind::SeamRetrimsArcFirstSide,
            Self::NonpositiveLeg { .. } => PathErrorKind::NonpositiveLeg,
            Self::NonpositiveFilletRadius { .. } => PathErrorKind::NonpositiveFilletRadius,
            Self::NonpositiveCircleRadius { .. } => PathErrorKind::NonpositiveCircleRadius,
            Self::DegenerateArcSpec { .. } => PathErrorKind::DegenerateArcSpec,
            Self::CircleSplitCount { .. } => PathErrorKind::CircleSplitCount,
            Self::ArcContinueNeedsArcCarrier => PathErrorKind::ArcContinueNeedsArcCarrier,
            Self::ArcContinueOffCarrier { .. } => PathErrorKind::ArcContinueOffCarrier,
            Self::ZeroDirection { .. } => PathErrorKind::ZeroDirection,
            Self::ArcViaCollinear { .. } => PathErrorKind::ArcViaCollinear,
            Self::DegenerateArcChord { .. } => PathErrorKind::DegenerateArcChord,
            Self::ArcCenterNotEquidistant { .. } => PathErrorKind::ArcCenterNotEquidistant,
            Self::DegenerateArcCenter { .. } => PathErrorKind::DegenerateArcCenter,
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
/// parses back as an `f64` this renders the shortest decimal that still
/// names the same number to a relative 1e-9; anything else (an interval,
/// a dual) passes through untouched. A DISPLAY choice only — the payload
/// keeps the exact scalar, and every claim a caller branches on reads
/// the field, never this string.
///
/// Every arm below renders its scalars through here. Non-scalar payloads
/// — a side, a carrier, an index, a `&'static str` site — are not this
/// helper's business and reach the sentence through their own `Display`.
fn num<T: core::fmt::Debug>(v: &T) -> String {
    let raw = format!("{v:?}");
    let Ok(x) = raw.parse::<f64>() else {
        return raw;
    };
    let tol = 1e-9 * x.abs().max(1.0);
    for prec in 0..=17 {
        let short = format!("{x:.prec$}");
        if short
            .parse::<f64>()
            .is_ok_and(|back| (back - x).abs() <= tol)
        {
            let trimmed = if short.contains('.') {
                short.trim_end_matches('0').trim_end_matches('.')
            } else {
                &short
            };
            return trimmed.to_string();
        }
    }
    raw
}

impl<T: Real> core::fmt::Display for PathError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::JunctionTangent { margin, arm } => write!(
                f,
                "this junction is tangent at any precision you could care about \
                 (turn margin {margin} m on a {arm} m arm) — if intended as tangency onto a \
                 new carrier, use .tangent(), which makes it exact by construction (or the \
                 tangent-arc / seam-fillet close at the seam); if intended as a straight \
                 continuation of the same line, spell it line(len) off the directed point — \
                 no junction exists there; otherwise move the geometry (or lower the \
                 tolerance)",
                margin = num(margin),
                arm = num(arm)
            ),
            Self::JunctionCusp { margin, arm } => write!(
                f,
                "this junction reverses onto the incoming direction at any precision you \
                 could care about (turn margin {margin} m on a {arm} m arm): a cusp, and \
                 the material-wedge invariant admits one only where it is DECLARED — if \
                 intended, author it structurally: .cusp() at an interior junction (exact by \
                 construction, and it emits the declaration); otherwise move the geometry. AT \
                 THE SEAM this is the closing leg arriving REVERSED into the entry\'s outgoing \
                 direction, and no declared arrival makes it anything else: rotate the loop\'s \
                 authoring origin, or cut the seam at a corner",
                margin = num(margin),
                arm = num(arm)
            ),
            Self::SeamTangent { margin } => write!(
                f,
                "the SEAM arrives tangent to the entry\'s first side (margin {margin} m) and \
                 nothing declared it: the loop closes on one carrier, and an UNDECLARED one is \
                 refused however the closing leg is spelled — `continue_to(Start)` does not \
                 reach it either, because the junction in band is the entry\'s, not the \
                 closer\'s. DECLARE the arrival on the target — Start.arrives_straight() for a \
                 straight leg continuing that side, Start.arrives_tangent() for a closing arc \
                 meeting it G1; both are checked, never inferred. Or cut the loop at a CORNER \
                 instead (author the seam where the outline actually turns)",
                margin = num(margin)
            ),
            Self::SeamArrivalOffDirection {
                margin,
                arm,
                arrival,
            } => {
                let (declared, alternative) = match arrival {
                    Arrival::Straight => (
                        "Start.arrives_straight()",
                        "or drop the declaration and author the seam at a CORNER",
                    ),
                    Arrival::Tangent => (
                        "Start.arrives_tangent()",
                        "or, if BOTH ends of the closing arc must be tangent, use the seam \
                         FILLET (.angle(theta).fillet(r).to(Start)) — no circular arc \
                         generically carries both, and the fillet CONSTRUCTS them",
                    ),
                };
                write!(
                    f,
                    "the declared seam arrival does not continue the entry\'s outgoing \
                     direction: it misses by {margin} m over the arriving leg\'s {arm} m arm. \
                     The TARGET declares the arrival ({declared}), so the two directions must \
                     agree to within the input tolerance — this is authored data disagreeing \
                     with itself, not a tangency judgement. Move the geometry so the leg does \
                     arrive continuing that side; a LARGER input tolerance admits a miss \
                     inside its own band, but this margin is definite and raising K only moves \
                     where definite starts; {alternative}",
                    margin = num(margin),
                    arm = num(arm)
                )
            }
            Self::SeamArrivalNeedsStraightFirstSide => write!(
                f,
                "Start.arrives_straight() declares the seam a SUBDIVISION point — one carrier \
                 continuing through it — and the entry\'s first side is an ARC, so there is no \
                 one carrier: a straight leg arriving along an arc\'s start tangent is a G1 \
                 joint between two carriers, which is a tangency and must be declared as one. \
                 Rotate the loop so the seam sits where the outline TURNS, or make the \
                 entry\'s first side the straight one (the loop is cyclic, so either side can \
                 be side 1). Whether a straight leg should be able to declare a TANGENT \
                 arrival here is an open question in PATHS-DESIGN §6"
            ),
            Self::SeamArrivalLeverTooShort { arm } => write!(
                f,
                "the declared seam arrival has no lever: the closing leg\'s arm is {arm} m, \
                 which is not definitely positive, so the levered turn cannot tell one \
                 arriving direction from another and the declaration would be accepted \
                 whatever the geometry says. A leg this short is a degenerate segment however \
                 it is spelled — move the geometry (lengthen the closing leg, or move the \
                 entry off it), or drop the vertex if it was not wanted",
                arm = num(arm)
            ),
            Self::ContinuationTargetOffRay { across, along } => write!(
                f,
                "the declared straight continuation\'s target is not on the departing ray: it \
                 misses by {across} m across the ray, {along} m along it. The verb DECLARES \
                 the leg to continue the run onto that point, so the point must lie on the ray \
                 to within the input tolerance — this is authored data disagreeing with \
                 itself, not a tangency judgement. Move the target onto the ray (the other \
                 direction — a LARGER input tolerance — would admit this miss, which is the \
                 opposite of the tangency refusals\' recourse: there closeness is what \
                 refuses, here distance is); if a TURN was meant here, author the direction \
                 (.turn(delta)/.angle(theta)) and use line_to",
                across = num(across),
                along = num(along)
            ),
            Self::SameCarrierJunction { margin } => write!(
                f,
                "this junction joins two pieces of the SAME carrier (identity margin \
                 {margin} m): carrier identity is not tangency — extend the leg \
                 instead of minting a collinear/cocircular neighbor, or, where the extra \
                 vertex is the point, subdivide the carrier structurally: line(len) off \
                 the directed point, which declares nothing",
                margin = num(margin)
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
                write!(
                    f,
                    "no corner for a radius-{radius} m fillet: {what}",
                    radius = num(radius)
                )
            }
            Self::AnchorOutsideTrimmedExtent {
                side,
                carrier,
                setback,
                available,
            } => write!(
                f,
                "the fillet trim would eat the {side} side's anchoring on-path point on its \
                 {carrier} carrier: tangent setback {setback} m exceeds the {available} m \
                 the anchor pins — reduce the radius or move the anchor",
                setback = num(setback),
                available = num(available)
            ),
            Self::FilletOffsetLeverTooShort {
                side,
                carrier_radius,
                offset_radius,
                least_lever,
                margin,
            } => write!(
                f,
                "the {side} side's offset lever rho {offset_radius} m (carrier radius \
                 {carrier_radius} m) is shorter than the {least_lever} m this corner's \
                 scale needs at the run's tolerance (margin {margin} m): the fillet's \
                 tangent point is recovered by projecting its centre back onto that \
                 carrier, and dividing by a lever that short cannot place the point within \
                 tolerance — move the fillet radius away from that side's carrier radius, \
                 or bring the corner's carriers closer together",
                offset_radius = num(offset_radius),
                carrier_radius = num(carrier_radius),
                least_lever = num(least_lever),
                margin = num(margin)
            ),
            Self::FilletEnclosesLegCarrier {
                side,
                carrier_radius,
                offset_radius,
                radius,
                largest_tangent_radius,
            } => {
                let whose = match side {
                    Some(side) => &format!("the {side} side's carrier"),
                    None => "both sides' carriers",
                };
                // The deixis is deliberately "a corner of these carriers"
                // rather than "this corner": where both crossings of the
                // pair sit inside the anchors' windows the refusal
                // reported is the first corner enumerated, which need not
                // be the one the author bracketed (issue #1281).
                write!(
                    f,
                    "a radius-{radius} m fillet cannot round a corner of these carriers: it \
                     would SWALLOW {whose} (radius {carrier_radius} m). The offset radius \
                     rho = R - sigma*tau*r is {offset_radius} m, and a negative rho means \
                     every circle of that radius tangent \
                     to that carrier on the corner's turn side contains the carrier \
                     whole — the corner with it, since the corner sits on that carrier — so \
                     the arc could never touch the corner it was asked to round. That is not \
                     a fillet of the corner, and no door builds it",
                    radius = num(radius),
                    carrier_radius = num(carrier_radius),
                    offset_radius = num(offset_radius)
                )?;
                match largest_tangent_radius {
                    // The endorsable number: a circle of this radius IS
                    // tangent to both carriers at the corner. Anchored
                    // extents can still require less, and refuse in their
                    // own words when they do.
                    Some(bound) => write!(
                        f,
                        " — the largest circle tangent to both carriers here has radius \
                         {bound} m, so try a radius below that (a short anchored leg can \
                         need less still)",
                        bound = num(bound)
                    ),
                    // Nothing endorsable at this site: the class bound is
                    // necessary and not sufficient, and naming a radius
                    // below it would be a promise this gate cannot keep.
                    None => write!(
                        f,
                        " — any fillet of this corner needs a radius below {carrier_radius} m, \
                         which is a necessary bound and not a sufficient one: these carriers \
                         may admit no fillet at all at this corner",
                        carrier_radius = num(carrier_radius)
                    ),
                }
            }
            Self::ArcLegOnOpenFillet { site } => write!(f, "{site}"),
            Self::DegenerateArcSpec { value } => write!(
                f,
                "this arc spec's authored datum ({value}) names no arc: a zero bulge \
                 degenerates to the chord (author a line), and a sweep angle or arc length \
                 must be definitely positive",
                value = num(value)
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
                 {offset} m): a subdivision vertex is ON the carrier by definition — fix the \
                 authored point rather than expecting a re-projection",
                offset = num(offset)
            ),
            Self::ZeroDirection { dx, dy } => write!(
                f,
                "a director spelled as components must name a direction (got \
                 ({dx}, {dy}), whose norm is within tolerance of zero): only the ratio \
                 of the components is read, so scaling them up costs nothing",
                dx = num(dx),
                dy = num(dy)
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
                "an arc leg's endpoints are within tolerance of each other (chord {chord} \
                 m): a leg spans a chord, and a closed carrier is a circle primitive, not a \
                 chain leg",
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
            Self::FarEndAnchorWithoutFillet => write!(
                f,
                "the far-end-anchor form ends an ARRIVAL side at its own anchor, and no \
                 fillet is open here: the entry authors its first side with .at(p), and the \
                 seam is authored at the back by the verb that targets Start \
                 (PATHS-DESIGN §2's entry rule)"
            ),
            // The prefix is computed from the predicate, not hard-coded.
            // Three of the four keys that reach this arm are NOT junction
            // classifications — `path_leg_length` meters an authored
            // extent and `path_continuation_target_offset` meters a
            // declared target's lateral miss — and calling those "junction
            // classification" told the reader the opposite of what the
            // margin means. R1 and R2 both found this; the leg-length case
            // was already wrong before this unit.
            //
            // The two non-junction keys also compose their OWN recourse
            // from `source.payload()` (D4 (iv)): the shared
            // `COINCIDENCE_RECOURSE` tail on the bare `Indeterminate`
            // Display says "declare the coincidence", which is meaningless
            // at these sites — for the continuation the declaration IS the
            // verb, and for a leg length there is no coincidence to
            // declare, only a number to change.
            Self::Escalated { source } => match source.predicate {
                Some("path_continuation_target_offset") => write!(
                    f,
                    "the declared straight continuation's target is neither on the \
                     departing ray nor definitely off it: {payload}. The verb DECLARES the \
                     leg, so there is no coincidence to declare here — the declaration is \
                     the verb. Move the target onto the ray, or widen the input tolerance \
                     (K·ε) so this miss is admissible",
                    payload = source.payload()
                ),
                Some("path_seam_arrival_turn" | "path_seam_arrival_side") => write!(
                    f,
                    "the declared seam arrival is neither continuing the entry\'s outgoing \
                     direction nor definitely off it: {payload}. The TARGET declares the \
                     arrival, so there is no coincidence left to declare here — the \
                     declaration is the target. Move the geometry so the two directions agree, \
                     or widen the input tolerance (K·ε) so a miss this size is admissible. \
                     LOWERING the tolerance is the wrong direction at this site: closeness is \
                     what is being ASSERTED here, not what is refusing",
                    payload = source.payload()
                ),
                Some("path_leg_length") => write!(
                    f,
                    "an authored leg extent could not be told from zero: {payload}. \
                     Author a longer leg, or widen the input tolerance (K·ε)",
                    payload = source.payload()
                ),
                // The junction keys (`path_junction_turn`,
                // `path_junction_side`) keep the full `Indeterminate`
                // Display, shared recourse and all: at a junction
                // "declare the coincidence" is exactly the right advice,
                // and `.tangent()` is what declaring it means.
                _ => write!(f, "path junction classification: {source}"),
            },
            Self::Band(e) => write!(f, "path tolerance band: {e}"),
            Self::Structure(r) => write!(f, "guided elaboration: {r}"),
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

    /// The exact REVERSE of this director — the cusp door's departure.
    ///
    /// The ray is NEGATED, never re-derived as `ang + π`: negation is
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
            pending_meta: None,
            last_arc: None,
            program: Vec::new(),
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

    /// The entry's FIRST SIDE — the seam's other half. Structural: the
    /// kind comes from `first_seg`, which the first emission pins, and
    /// the carrier from the first vertex's own authored bulge and the
    /// two points it spans. Nothing is measured and nothing is
    /// re-derived from a value.
    fn first_side(&self) -> Result<FirstSide<T>, PathError<T>> {
        let a = self.verts.first().ok_or(PathError::UnderdeterminedLeg {
            site: "the entry's first side on an empty chain",
        })?;
        match self.first_seg {
            FirstSeg::Line => Ok(FirstSide::Line),
            FirstSeg::Arc => {
                let b = self.verts.get(1).ok_or(PathError::UnderdeterminedLeg {
                    site: "an arc first side with no far end",
                })?;
                Ok(FirstSide::Arc(arc_carrier(a.pos, b.pos, a.bulge)))
            }
            // The closing leg IS the first segment: a one-segment loop,
            // which has no seam to classify because it has no second
            // side to meet.
            FirstSeg::NotYet => Err(PathError::UnderdeterminedLeg {
                site: "a seam arrival on a chain with no first side",
            }),
        }
    }

    /// Declares the SEAM joint — joint 0, the entry vertex — tangent.
    /// The seam fillet does the same thing inline when its arc IS the
    /// closing segment; a declared G1 arrival is the other way a
    /// closing leg can make joint 0 a tangency.
    fn declare_seam(&mut self) {
        if !self.tangent.contains(&0) {
            self.tangent.push(0);
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
    fn build(mut self) -> ClosedLoop<T> {
        let structure = self.take_structure();
        ClosedLoop {
            loop_: ProfileLoop {
                vertices: self.verts,
                tangent_joints: self.tangent,
            },
            program: self.program,
            structure: structure.into_record(),
        }
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
/// The label matters and was previously wrong ("(ε_input, K·ε_input)",
/// which under this repo's role naming reads as (K·ε, K²·ε) — a band
/// this function has never built). D4's two-tolerance principle gives
/// ε_input its meaning as a ROLE: ε_input IS K·ε, the escalating edge,
/// not a third dial. So the two edges are ε and ε_input, and naming
/// them that way is what lets a reader check the body against the doc.
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
/// lever arm. `seam` says "this junction is the loop's SEAM, so classify it as
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
/// ([`Start::arrives_straight`] / [`Start::arrives_tangent`]). Naming
/// that fact `JunctionTangent` would send the reader to a spelling the
/// seam does not have.
fn junction_check<T: Decide>(
    inc: &Incoming<T>,
    dep: Dir<T>,
    seam: bool,
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
                Ok(_) => {
                    if seam {
                        Err(PathError::SeamTangent { margin })
                    } else {
                        Err(PathError::JunctionTangent {
                            margin,
                            arm: inc.arm,
                        })
                    }
                }
                Err(source) => Err(PathError::Escalated { source }),
            }
        }
        Ok(_) => Ok(()),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// The entry's FIRST SIDE, as the seam's other half — read off the
/// chain's own emission-layer bookkeeping (§2c's parenthetical homes
/// identity data there), never re-derived from a value.
enum FirstSide<T: Real> {
    /// The entry departs on a straight carrier.
    Line,
    /// The entry departs on this arc carrier.
    Arc(ArcData<T>),
}

/// **The declared seam ARRIVAL's check** — the inverted twin of
/// [`junction_check`]'s tangent arm, and the mirror of
/// [`PartialPath::on_ray_extent`]: there the ray is declared and the
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
/// **The seam's CLASS follows the declaration AND the entry's first
/// side, never the arriving direction alone.** Two directions agreeing
/// is not yet a subdivision: whether the seam joins one carrier to
/// itself or two carriers tangentially depends on what the entry
/// departs on, which is why this takes the chain rather than a pair of
/// angles. A STRAIGHT arrival onto an arc first side is not the ruled
/// case at all (the ruling's words are "a declared SUBDIVISION point"),
/// and a TANGENT arrival onto the SAME carrier is declared tangency
/// onto identity, which §4 item 4 refuses at every junction.
///
/// The datum is `sin` of the turn — dimensionless — so comparing it
/// against a LENGTH tolerance is a category error until an arm says
/// what it displaces. It is LEVERED by the arriving leg's own arm, the
/// same lever §4 item 1 uses for the junction at this very vertex, and
/// the product is, TO FIRST ORDER, the lateral displacement the
/// misalignment opens at the seam (for an arc leg the exact figure is
/// `s·sin φ + s²/2R`; the lever takes the leading term, as §4 item 1
/// does). That is the deviation ε_input is defined about, so the
/// threshold is on the DISPLACEMENT and does not drift with leg length.
/// A lever that is not definitely positive carries no question at all,
/// and refuses.
///
/// The reverse class is not this refusal's: an arrival anti-parallel to
/// the entry's outgoing direction has a near-zero turn too, and it is a
/// cusp — [`PathError::JunctionCusp`], the same name it carries at any
/// other junction. One fact, one refusal.
fn seam_arrival_check<T: Decide>(
    core: &Core<T>,
    arrival: Arrival,
    arriving: Dir<T>,
    arriving_carrier: Option<&ArcData<T>>,
    arm: T,
    start_ang: Dir<T>,
    tol: Tol,
) -> Result<(), PathError<T>> {
    let band = linear_band(tol)?;
    // (i) The seam's CLASS, before any measurement.
    match (arrival, core.first_side()?) {
        (Arrival::Straight, FirstSide::Line) => {}
        (Arrival::Straight, FirstSide::Arc(_)) => {
            return Err(PathError::SeamArrivalNeedsStraightFirstSide);
        }
        (Arrival::Tangent, FirstSide::Line) => {}
        (Arrival::Tangent, FirstSide::Arc(first)) => {
            // Carrier identity is not tangency, at the seam as anywhere
            // else. `tangent_arc_geom` runs the same test against the
            // PREVIOUS segment's carrier; the entry's is a different
            // neighbour, and a straight leg in between hides it.
            if let Some(closing) = arriving_carrier {
                refuse_identical_carriers(&first, closing, tol)?;
            }
        }
    }
    // (ii) The lever.
    match decide("path_seam_arrival_lever", Margin::of(arm), band) {
        Ok(Sign::Positive) => {}
        Ok(_) => return Err(PathError::SeamArrivalLeverTooShort { arm }),
        Err(source) => return Err(PathError::Escalated { source }),
    }
    // (iii) The direction the declaration asserts.
    let turn = arriving.unit.perp_dot(start_ang.unit);
    let margin = turn * arm;
    match decide("path_seam_arrival_turn", Margin::levered(turn, arm), band) {
        Ok(Sign::Zero) => {
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
        Ok(_) => Err(PathError::SeamArrivalOffDirection {
            margin,
            arm,
            arrival,
        }),
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

/// **An arc leg's lever arm**, in one place: the smaller of its
/// carrier's radius and its chord.
///
/// The radius is what an angular margin displaces over; the chord bounds
/// it for an arc shorter than its own radius, where the radius would
/// overstate how far the leg actually reaches. Named because several
/// sites spell it and three of them are junction LEVERS, where the
/// choice is a contract rather than an expression.
fn arc_arm<T: Real>(carrier: &ArcData<T>, chord: T) -> T {
    carrier.radius.min(chord)
}

/// The straight leg's EMISSION, shared by the two `line(len)` rows —
/// the directed one (a bound departure) and the straight continuation
/// (the directed point's own tangent). Both mint the same thing and
/// must keep minting the same thing: a line vertex at `at + û·len`, a
/// tip whose carrier is None (a line leg leaves no arc carrier behind)
/// and whose lever arm is the emitted segment's own length, measured
/// head-to-end so a side squeezed between two trims measures from the
/// trim point rather than from an authored anchor.
fn emit_straight_leg<T: Real>(
    core: &mut Core<T>,
    at: Point2<T>,
    ang: Dir<T>,
    len: T,
) -> Result<Tip<T>, PathError<T>> {
    emit_straight_leg_at(core, at + ang.unit * len, ang)
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
/// the carrier the declaration names; the accepted lateral miss is the
/// whole of the difference between the two, and it is bounded by the
/// check that let the target through.
///
/// **That bound is PER LEG, and this is where to read it.** The tip
/// leaves on the declared ray while the vertex sits up to one accepted
/// miss off it, so a RUN of declared continuations can accumulate: n
/// legs each accepting a same-side miss below ε put the run's end up to
/// n·ε off the ray it started on, and every per-leg check is green,
/// correctly. R1's review measured it — forty legs at 0.5·ε reach 20·ε,
/// two full ε_input.
///
/// This is a recorded LIMIT, not a hole, because the run-level
/// certifier exists and is loud: the data gate sees the accumulated bow
/// that no per-leg check can and ESCALATES on `chord_side` rather than
/// accepting it. Escalation, not silence and not a guess.
/// `the_per_leg_band_composes_and_the_data_gate_catches_the_sum` pins
/// that verdict; PATHS-DESIGN §4 records it beside the band decisions.
/// Tightening the per-leg band would not change the shape of this — any
/// per-step tolerance composes — so the answer is the gate, which is
/// already the design's answer for run-level facts.
fn emit_straight_leg_at<T: Real>(
    core: &mut Core<T>,
    end: Point2<T>,
    ang: Dir<T>,
) -> Result<Tip<T>, PathError<T>> {
    let head = core.head()?;
    core.push_line(end)?;
    let arm = (end - head).norm_squared().sqrt();
    Ok(leg_end_tip(end, ang, arm, None))
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
        let trims = (arc.resolver)(self.guide_mut(), incoming, arrival, arc.radius, tol)?;
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
        let mut trims = line_line_fillet_trims(pending.origin, corner, arr_pos, pending.radius)
            .map_err(map_fillet_err)?;
        // A straight carrier pair derives ONE corner and its
        // construction admits one candidate, so the fit signs are this
        // resolution's whole discrete content.
        (trims.fit_in, trims.fit_out) = self
            .guide
            .line_fits(trims.fit_in, trims.fit_out)
            .map_err(PathError::Structure)?;
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
    fn at_kernel(
        mut self,
        p: Point2<T>,
        tol: Tol,
    ) -> Result<PartialPath<T, HasPos<Plain>, A>, PathError<T>> {
        match (self.tip.ang, self.core.pending.is_some()) {
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
                junction_check(inc, dir, false, tol)?;
            }
            let at = pos.at;
            if self.core.pending.is_some() {
                self.core
                    .resolve_fillet(at, dir, ArrivalKind::Continues, tol)?;
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

    /// The kernel behind the table's cusp row (recording is the row's,
    /// not the kernel's): the tangent kernel with the ray reversed.
    /// The declaration it emits is the SAME one `.tangent()` emits —
    /// the profile data gate judges declared joints by carrier
    /// tangency, which is direction-agnostic, so a reverse-tangent
    /// joint needs no second flag to be accepted there.
    fn cusp_kernel(mut self) -> PartialPath<T, HasPos<WithIncoming>, HasAng> {
        self.tip.ang = self
            .tip
            .pos
            .as_ref()
            .and_then(|p| p.incoming.as_ref())
            .map(|inc| inc.ang.reversed());
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
        let tip = emit_straight_leg(&mut self.core, at, inc.ang, len)?;
        Ok(in_state(self.core, tip))
    }

    /// The departing RAY of a straight continuation: the point it
    /// leaves from and the tangent it leaves along, both binding bits.
    fn continuation_ray(&self, site: &'static str) -> Result<(Point2<T>, Dir<T>), PathError<T>> {
        let pos = self
            .tip
            .pos
            .as_ref()
            .ok_or(PathError::UnderdeterminedLeg { site })?;
        let inc = pos.incoming.ok_or(PathError::UnderdeterminedLeg { site })?;
        Ok((pos.at, inc.ang))
    }

    /// **The declared point-target continuation's one decision**: is the
    /// authored target ON the departing ray? Returns how far ALONG it
    /// the target sits — the leg length the declaration implies — or
    /// refuses.
    ///
    /// Two facts are gated, and they are separate facts, so they get
    /// separate refusals. The LATERAL miss decides whether the target is
    /// on the ray's LINE, and it is classified on the same linear band
    /// every other decision in this kernel uses, with the same reading:
    /// below ε_precision the target and the ray are the same place at
    /// the precision anything here represents, so the declaration is
    /// consistent; above ε_input (= K·ε) they are definitely different
    /// places, and the authored data contradicts itself; between them
    /// nothing is decidable, so the band escalates rather than guesses.
    /// The refusal edge IS ε_input, which is the band the input-quality
    /// role names — this is a question about authored input, not about
    /// what the kernel can build — and it is reached through the funnel
    /// rather than by comparing against K·ε directly, because a bare
    /// comparison would swallow the escalation band and decide where the
    /// numbers cannot.
    ///
    /// The margin is metered with NO lever: `across` is already the
    /// target's own displacement from the ray in meters — the distance
    /// the authored point would have to move — so [`Margin::of`] is the
    /// honest door. (§4 item 1 levers its turn margin because ITS datum
    /// is an angle; an angle is a pure number until an arm says what it
    /// displaces. Levering here would mean dividing this length by the
    /// leg to get an angle and multiplying it back, which can only lose
    /// bits and would make the threshold depend on how far away the
    /// author put the point.)
    ///
    /// The ALONG component is the ray's half-line-ness, and it is the
    /// same fact `line(len)` gates on its authored length: a target
    /// behind the departure, or on top of it, is a leg of non-positive
    /// length ([`PathError::NonpositiveLeg`]), not a target that misses.
    /// **Sibling measurement, cross-declared** (R1 S1): this and
    /// [`tangent_arc_geom`](Self::tangent_arc_geom) compute the SAME
    /// four lines — `d = target − at`, `along = û·d`, `across = û⊥·d`,
    /// then a banded decision — under two different predicate keys, and
    /// neither used to admit the other existed.
    ///
    /// They are kept separate deliberately rather than shared, because
    /// the keys are the point: this site classifies `across` as a
    /// declared target's MISS (`path_continuation_target_offset`, an
    /// authored-data disagreement), while the tangent-arc site
    /// classifies the same number as a degenerate-arc condition. The
    /// funnel key is what tells a margin telemetry reader which question
    /// was being answered, so collapsing them would lose the
    /// distinction that makes the funnel worth having. What was missing
    /// was the cross-reference, not the sharing.
    ///
    /// `arc_continue_kernel` is the third member of the family; it
    /// retires with BOOL-10.
    fn on_ray_extent(
        at: Point2<T>,
        ang: Dir<T>,
        target: Point2<T>,
        tol: Tol,
    ) -> Result<T, PathError<T>> {
        let d = target - at;
        let along = ang.unit.dot(d);
        let across = ang.unit.perp_dot(d);
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
        let (at, ang) = self.continuation_ray("continue_to on a tip without incoming data")?;
        Self::on_ray_extent(at, ang, target, tol)?;
        let tip = emit_straight_leg_at(&mut self.core, target, ang)?;
        Ok(in_state(self.core, tip))
    }

    /// The kernel behind the table's structural CLOSER row: the
    /// point-target continuation whose target is [`Start`].
    ///
    /// One check is the point row's — the entry vertex must lie on the
    /// departing ray — and it replaces the closer's departure junction
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
        arrival: Option<Arrival>,
        tol: Tol,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let (at, ang) =
            self.continuation_ray("continue_to(Start) on a tip without incoming data")?;
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
        Self::on_ray_extent(at, ang, start_pos, tol)?;
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
        // the segment actually emitted runs head → start_pos. Those two
        // directions differ by the accepted lateral miss over the arm,
        // at most `across/arm` — one band unit by construction, because
        // `on_ray_extent` above refused anything larger.
        //
        // The bounded consequence, stated so it is not rediscovered: a
        // seam within one band unit of the boundary can be pushed from a
        // definite verdict into ESCALATION by this difference. It cannot
        // flip accept ↔ refuse, because crossing from one definite
        // verdict to the other would take more than the band's whole
        // width. Escalation is the honest outcome for a junction that
        // close to the edge, so the direction of the error is the safe
        // one.
        //
        // `line_to(Start)` classifies from the REALIZED direction
        // instead, having computed it from the two points; that is the
        // one place the two closers' seam checks differ, and it follows
        // from the same thing everything else here follows from — this
        // verb declares its ray, `line_to` derives one. (R1 NOTE-3.)
        //
        // With the ARRIVAL declared too ([`Start::arrives_straight`]),
        // the same comparison runs with the verdict inverted: a zero
        // turn is the declared subdivision seam and closes, a definite
        // turn is the declaration contradicted. The two declarations
        // are independent facts — this verb declares the DEPARTURE
        // (the leg continues the run), the target declares the ARRIVAL
        // (it continues the entry's first side) — and the D-shape's two
        // rotations need one each.
        if let Some(arrival) = arrival {
            seam_arrival_check(&self.core, arrival, ang, None, arm, start_ang, tol)?;
        } else {
            junction_check(
                &Incoming {
                    ang,
                    arm,
                    carrier: None,
                },
                start_ang,
                true,
                tol,
            )?;
        }
        self.core.set_leaving(T::zero(), FirstSeg::Line)?;
        Ok(self.core.build())
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
        let arm = arc_arm(&carrier, chord);
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
        let tip = emit_straight_leg(&mut self.core, at, ang, len)?;
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
    ///
    /// **Sibling measurement, cross-declared** (R1 S1): the
    /// `d`/`along`/`across`/decide opening here is the same four lines
    /// [`on_ray_extent`](Self::on_ray_extent) computes, under a
    /// different predicate key. See that function for why the two are
    /// kept apart rather than shared.
    fn tangent_arc_geom(&self, p: Point2<T>, tol: Tol) -> Result<TangentArcGeom<T>, PathError<T>> {
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
                            // The closing and non-closing cases refuse
                            // IDENTICALLY. A collinear target under a
                            // declared tangency degenerates the arc onto
                            // the incoming straight carrier, which is
                            // carrier identity, and that fact does not
                            // change because the target happens to be
                            // `Start`. This used to fork to the
                            // departure half of the close-only refusal —
                            // the same second-naming the seam-wall
                            // collapse removed everywhere else.
                            return Err(PathError::SameCarrierJunction { margin: across });
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
        let g = self.tangent_arc_geom(p, tol)?;
        self.core.push_arc(p, g.bulge, g.carrier)?;
        let arm = arc_arm(&g.carrier, g.chord);
        Ok(in_state(
            self.core,
            leg_end_tip(p, g.end_ang, arm, Some(g.carrier)),
        ))
    }

    /// The tangent-arc seam. The arc is constructed from the DEPARTURE
    /// as it always was; `declared` ([`Start::arrives_tangent`]) says
    /// the ARRIVAL is G1 by intent, which inverts the seam junction's
    /// verdict and declares joint 0 tangent so the verify layer
    /// re-checks the flag it now carries. One end constructs, the other
    /// is checked — nothing is overdetermined, and a shape no circular
    /// arc can serve refuses with the seam fillet named.
    fn tangent_arc_to_start(
        mut self,
        arrival: Option<Arrival>,
        tol: Tol,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let start_ang = self.core.start_ang.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry direction is bound",
        })?;
        let g = self.tangent_arc_geom(start_pos, tol)?;
        let arm = arc_arm(&g.carrier, g.chord);
        if let Some(arrival) = arrival {
            seam_arrival_check(
                &self.core,
                arrival,
                g.end_ang,
                Some(&g.carrier),
                arm,
                start_ang,
                tol,
            )?;
            self.core.declare_seam();
        } else {
            junction_check(
                &Incoming {
                    ang: g.end_ang,
                    arm,
                    carrier: Some(g.carrier),
                },
                start_ang,
                true,
                tol,
            )?;
        }
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

    /// The sharp straight seam. `arrival` is the arrival-side
    /// declaration the TARGET carried ([`Start::arrives_straight`]):
    /// with it the seam junction goes through [`seam_arrival_check`] —
    /// a zero turn is what the author said and closes — and without it
    /// through [`junction_check`]'s seam arm, where a zero turn
    /// refuses. It is an `Option<Arrival>` rather than a flag because
    /// the refusals name the member that declared, so the member has to
    /// travel with it.
    ///
    /// The DEPARTURE junction is classified identically either way: the
    /// arrival declaration says nothing about it.
    fn line_to_start(
        mut self,
        arrival: Option<Arrival>,
        tol: Tol,
    ) -> Result<ClosedLoop<T>, PathError<T>> {
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
            junction_check(inc, gamma, false, tol)?;
        }
        let start_ang = *self.core.start_ang.get_or_insert(gamma);
        let head = self.core.head()?;
        let arm = (start_pos - head).norm_squared().sqrt();
        if let Some(arrival) = arrival {
            seam_arrival_check(&self.core, arrival, gamma, None, arm, start_ang, tol)?;
        } else {
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
        }
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
        let arm = arc_arm(&carrier, chord);
        Ok(in_state(
            self.core,
            leg_end_tip(p, end_t, arm, Some(carrier)),
        ))
    }

    /// The SHARP arc seam. `arrival` is [`Start::arrives_tangent`]
    /// carried by the `Bulge` spec's target: the arc's end tangent is
    /// already fixed by the authored bulge, so the CHECK form applies
    /// unchanged — one end is authored, the other is checked.
    fn arc_to_start(
        mut self,
        bulge: T,
        arrival: Option<Arrival>,
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
        let (start_t, end_t, chord) = Self::arc_angles(at, start_pos, bulge);
        if let Some(inc) = &pos.incoming {
            junction_check(inc, start_t, false, tol)?;
        }
        let start_ang = *self.core.start_ang.get_or_insert(start_t);
        let carrier = arc_carrier(at, start_pos, bulge);
        let arm = arc_arm(&carrier, chord);
        if let Some(arrival) = arrival {
            seam_arrival_check(
                &self.core,
                arrival,
                end_t,
                Some(&carrier),
                arm,
                start_ang,
                tol,
            )?;
            self.core.declare_seam();
        } else {
            junction_check(
                &Incoming {
                    ang: end_t,
                    arm,
                    carrier: Some(carrier),
                },
                start_ang,
                true,
                tol,
            )?;
        }
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
        path.line_to_start(None, tol)
    }
}

impl<T: Decide, F: Flavor> LineTarget<T, F> for ArrivesStraight {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn line_from(mut path: PartialPath<T, HasPos<F>, NoAng>, _target: Self, tol: Tol) -> Self::Out {
        path.core
            .record(Step::LineTo(Target::StartArriving(Arrival::Straight)));
        path.line_to_start(Some(Arrival::Straight), tol)
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
        path.continue_to_start_kernel(None, tol)
    }
}

impl<T: Decide> ContinueTarget<T> for ArrivesStraight {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn continue_from(
        mut path: PartialPath<T, HasPos<WithIncoming>, NoAng>,
        _target: Self,
        tol: Tol,
    ) -> Self::Out {
        path.core
            .record(Step::ContinueTo(Target::StartArriving(Arrival::Straight)));
        path.continue_to_start_kernel(Some(Arrival::Straight), tol)
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
        path.tangent_arc_to_start(None, tol)
    }
}

impl<T: Decide, F: Flavor> TangentArcTarget<T, F> for ArrivesTangent {
    type Out = Result<ClosedLoop<T>, PathError<T>>;
    fn tangent_arc_from(
        mut path: PartialPath<T, HasPos<F>, HasAng>,
        _target: Self,
        tol: Tol,
    ) -> Self::Out {
        path.core
            .record(Step::TangentArcTo(Target::StartArriving(Arrival::Tangent)));
        path.tangent_arc_to_start(Some(Arrival::Tangent), tol)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **The cusp door's exactness guard.**
    ///
    /// [`Dir::reversed`] promises the departure ray is the incoming ray
    /// NEGATED, never re-derived as `ang + π` — and that promise is the
    /// reason a declared cusp is a structural fact rather than a value
    /// coincidence. Until this row existed the promise was held by
    /// prose alone: the `ang + π` mutant passed every suite in the
    /// workspace, because every downstream check reads the junction
    /// through a TOLERANCE and the two spellings differ by ulps.
    ///
    /// A tolerance can never catch that, so the guard is at the bit
    /// level, at the door itself, and it asserts both halves:
    ///
    /// 1. the reversed ray is bit-exactly the negation, and
    /// 2. the `ang + π` spelling would NOT be — on every case here,
    ///    axis-aligned and axis-oblique alike. Both parts of (2) matter:
    ///    on an axis the mutant leaks the quantization of π into the
    ///    zero component (`sin π = 1.22e-16`), and off-axis it lands
    ///    ulps away in BOTH components because the sum `ang + π` rounds
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
            let via_angle = Dir::from_angle(incoming.ang + core::f64::consts::PI);
            assert!(
                via_angle.unit.x.to_bits() != reversed.unit.x.to_bits()
                    || via_angle.unit.y.to_bits() != reversed.unit.y.to_bits(),
                "`ang + π` must NOT reproduce the negation ({u:?}) — if it does, this \
                 guard is vacuous and the door's promise is untestable here"
            );
        }
    }
}
