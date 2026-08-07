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

use core::marker::PhantomData;

use geom_core::{Band, Bounds, Decide, Indeterminate, Length, Point2, Real, Sign, Tolerance, Vec2};

use crate::k_stats::decide;
use crate::sugar::{LineFilletTrims, line_line_fillet_trims};
use crate::validate::{FilletLeg, ProfileError};
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
/// behind the ray start, refuse typed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoCornerReason {
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
pub enum PathError {
    /// §4 item 1: the authored departure is within ε_input of the
    /// incoming TANGENT direction — one refusal, one recourse, for any
    /// sub-ε_input margin: if the tangency is intended, author it
    /// structurally (`.tangent()`, or the tangent-arc / seam-fillet
    /// close at the seam), which makes it exact by construction;
    /// otherwise move the geometry (or lower the tolerance). The
    /// margin rides along as data; the message never forks on
    /// exactly-on vs in-band.
    JunctionTangent {
        /// The classified turn margin sin φ · arm, meters (diagnostic
        /// channel).
        margin: f64,
        /// The lever arm (the incoming leg's extent, capped by its
        /// carrier radius), meters.
        arm: f64,
    },
    /// §4 item 1, reverse class: the departure is within ε_input of
    /// the REVERSE of the incoming tangent — a cusp. No declaration
    /// door exists: the kernel's material-wedge invariant refuses cusp
    /// wedges in any solid built from such a profile; #131 is the
    /// tabled front door that does not exist yet.
    JunctionCusp {
        /// The classified turn margin sin φ · arm, meters.
        margin: f64,
        /// The lever arm, meters.
        arm: f64,
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
        margin: f64,
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
        margin: f64,
    },
    /// The fillet's virtual corner does not exist (see
    /// [`NoCornerReason`]).
    NoCornerForFillet {
        /// Which structural condition failed.
        reason: NoCornerReason,
        /// The requested radius, meters (diagnostic).
        radius: f64,
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
        setback: f64,
        /// The anchored extent available to the trim, meters.
        available: f64,
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

impl core::fmt::Display for PathError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::JunctionTangent { margin, arm } => write!(
                f,
                "this junction is tangent at any precision you could care about \
                 (turn margin {margin:.3e} m on a {arm:.3e} m arm) — if intended, author it \
                 structurally: .tangent() at an interior junction (exact by construction), or \
                 the tangent-arc / seam-fillet close at the seam; otherwise move the geometry \
                 (or lower the tolerance)"
            ),
            Self::JunctionCusp { margin, arm } => write!(
                f,
                "this junction reverses onto the incoming direction (turn margin {margin:.3e} m \
                 on a {arm:.3e} m arm): a cusp, which the material-wedge invariant refuses in \
                 any solid built from such a profile — there is no declaration door for cusps \
                 (#131 is the tabled front door that does not exist yet); move the geometry"
            ),
            Self::TangentLineClose { margin } => write!(
                f,
                "a tangent LINE close is overdetermined — direction inherited AND through Start \
                 (margin {margin:.3e} m) — and refuses always, exact collinearity included: \
                 close with the tangent ARC instead (.tangent().tangent_arc_to(Start)), or \
                 rotate the loop's authoring origin so the straight run is authored forward as \
                 side 1 and the arc becomes the closer"
            ),
            Self::SameCarrierJunction { margin } => write!(
                f,
                "this junction joins two pieces of the SAME carrier (identity margin \
                 {margin:.3e} m): carrier identity is not tangency (#101) — extend the leg \
                 instead of minting a collinear/cocircular neighbor"
            ),
            Self::NoCornerForFillet { reason, radius } => {
                let what = match reason {
                    NoCornerReason::CarriersParallel => {
                        "the incoming ray and the arrival carrier are parallel at tolerance — \
                         no corner exists (if they are meant to run tangentially, author the \
                         tangency: .tangent(), or the seam fillet at the seam)"
                    }
                    NoCornerReason::BehindIncomingRay => {
                        "the carrier intersection lies behind the incoming ray's start"
                    }
                    NoCornerReason::BehindArrivalAnchor => {
                        "the carrier intersection does not lie behind the arrival side's anchor"
                    }
                };
                write!(f, "no corner for a radius-{radius:.3e} m fillet: {what}")
            }
            Self::AnchorOutsideTrimmedExtent {
                side,
                setback,
                available,
            } => write!(
                f,
                "the fillet trim would eat the {side} side's anchoring on-path point: tangent \
                 setback {setback:.3e} m exceeds the {available:.3e} m the anchor pins — reduce \
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

impl std::error::Error for PathError {}

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

/// A directed point's intrinsic incoming data: the arriving leg's end
/// tangent (angle, radians), the leg's lever arm (extent capped by its
/// carrier radius — the junction check's meters lever), and its
/// carrier kind.
#[derive(Clone, Copy, Debug)]
struct Incoming<T: Real> {
    ang: T,
    arm: T,
    carrier: Option<ArcData<T>>,
}

/// The one-struct tip of PATHS-DESIGN §5: `pos: Option<PosData>`,
/// `ang: Option<T>`; the position data's optional incoming tangent is
/// what the junction check reads at runtime (one generic function).
#[derive(Clone, Debug)]
struct Tip<T: Real> {
    pos: Option<PosData<T>>,
    ang: Option<T>,
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
    ang: T,
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
    start_ang: Option<T>,
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
    fn set_leaving(&mut self, bulge: T, kind: FirstSeg) -> Result<(), PathError> {
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
    fn push_line(&mut self, p: Point2<T>) -> Result<(), PathError> {
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
    fn push_arc(&mut self, p: Point2<T>, bulge: T, carrier: ArcData<T>) -> Result<(), PathError> {
        self.set_leaving(bulge, FirstSeg::Arc)?;
        self.verts.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self.last_arc = Some(carrier);
        Ok(())
    }

    /// The current chain end.
    fn head(&self) -> Result<Point2<T>, PathError> {
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
fn linear_band() -> Result<Band, PathError> {
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
fn junction_check<T: Real + Decide + Bounds>(
    inc: &Incoming<T>,
    dep: T,
    line_close: bool,
) -> Result<(), PathError> {
    let band = linear_band()?;
    let u_in = unit(inc.ang);
    let u_dep = unit(dep);
    let turn = u_in.perp_dot(u_dep);
    match decide("path_junction_turn", Length::levered(turn, inc.arm), band) {
        Ok(Sign::Zero) => {
            let margin = (turn * inc.arm).lo();
            // Diagnostic-channel comparison (naming the refusal class),
            // not a geometric decision: the geometric decision was the
            // Zero above.
            if u_in.dot(u_dep).lo() < 0.0 {
                Err(PathError::JunctionCusp {
                    margin,
                    arm: inc.arm.lo(),
                })
            } else if line_close {
                Err(PathError::TangentLineClose { margin })
            } else {
                Err(PathError::JunctionTangent {
                    margin,
                    arm: inc.arm.lo(),
                })
            }
        }
        Ok(_) => Ok(()),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// §4 item 4: refuses a declared continuation whose constructed
/// carrier is the incoming carrier itself (cocircular arcs) — the
/// `carrier_circles_identity` margin d + |Δr| on the linear band.
fn refuse_identical_carriers<T: Real + Decide + Bounds>(
    a: &ArcData<T>,
    b: &ArcData<T>,
) -> Result<(), PathError> {
    let band = linear_band()?;
    let d = (a.center - b.center).norm_squared().sqrt();
    let margin = d + (a.radius - b.radius).abs();
    match decide("path_carrier_identity", Length::of(margin), band) {
        Ok(Sign::Zero) => Err(PathError::SameCarrierJunction {
            margin: margin.lo(),
        }),
        Ok(_) => Ok(()),
        Err(source) => Err(PathError::Escalated { source }),
    }
}

/// Maps the shared fillet closed form's refusals into the algebra's
/// vocabulary: a Negative leg fit here IS the anchor-fit refusal (the
/// helper is fed the two sides' anchoring extents), an escalation
/// stays an escalation; anything else the algebra's own gates should
/// have refused first (backstop).
fn map_fillet_err(e: ProfileError) -> PathError {
    match e {
        ProfileError::FilletDoesNotFit {
            leg,
            setback,
            leg_length,
            ..
        } => PathError::AnchorOutsideTrimmedExtent {
            side: leg,
            setback,
            available: leg_length,
        },
        ProfileError::Escalated { source, .. } => PathError::Escalated { source },
        ProfileError::Band(b) => PathError::Band(b),
        _ => PathError::OverdeterminedJunction {
            site: "line_line_fillet_trims",
        },
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

impl<T: Real + Decide + Bounds> Core<T> {
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
        arr_ang: T,
        seam: bool,
    ) -> Result<(), PathError> {
        let pending = self
            .pending
            .take()
            .ok_or(PathError::OverdeterminedJunction {
                site: "fillet resolution without an opened fillet",
            })?;
        let band = linear_band()?;
        let u1 = unit(pending.ang);
        let u2 = unit(arr_ang);
        let w = arr_pos - pending.origin;
        let wn = w.norm_squared().sqrt();
        // (1) parallel/tangent carriers admit no corner: the turn
        // margin sin φ levered by the anchor separation.
        let cross = u1.perp_dot(u2);
        match decide("path_corner_turn", Length::levered(cross, wn), band) {
            Ok(Sign::Zero) => {
                return Err(PathError::NoCornerForFillet {
                    reason: NoCornerReason::CarriersParallel,
                    radius: pending.radius.lo(),
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
                    reason: NoCornerReason::BehindIncomingRay,
                    radius: pending.radius.lo(),
                });
            }
            Err(source) => return Err(PathError::Escalated { source }),
        }
        match decide("path_corner_advance", Length::of(-s_arr), band) {
            Ok(Sign::Positive) => {}
            Ok(_) => {
                return Err(PathError::NoCornerForFillet {
                    reason: NoCornerReason::BehindArrivalAnchor,
                    radius: pending.radius.lo(),
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
                pending
                    .origin_incoming
                    .as_ref()
                    .and_then(|inc| inc.carrier)
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
/// use-after-close is ill-typed. Repeated motifs are builder
/// functions over the one chain
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
fn leg_end_tip<T: Real>(at: Point2<T>, ang: T, arm: T, carrier: Option<ArcData<T>>) -> Tip<T> {
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
        in_state(
            Core::empty(),
            Tip {
                pos: None,
                ang: Some(theta),
                ang_by_tangent: false,
            },
        )
    }
}

impl<T: Real + Decide + Bounds, A: AngMarker> PartialPath<T, NoPos, A> {
    /// Adds the position bit (`Open → Point`, `Angle → Directed`) —
    /// written once, generic over the angle slot it does not touch.
    ///
    /// On a fillet arrival whose angle is already bound, completing
    /// the position resolves the fillet (both carriers fixed): the
    /// corner construction and anchor-fit gates run here — see
    /// [`PathError`]. On the angle-first entry, this seeds the chain.
    /// `p` is absolute (profile frame), a real on-path point (the
    /// side's anchor).
    pub fn at(mut self, p: Point2<T>) -> Result<PartialPath<T, HasPos<Plain>, A>, PathError> {
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

impl<T: Real + Decide + Bounds, P: PosMarker> PartialPath<T, P, NoAng> {
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
    pub fn angle(mut self, theta: T) -> Result<PartialPath<T, P, HasAng>, PathError> {
        if let Some(pos) = &self.tip.pos {
            if let Some(inc) = &pos.incoming {
                junction_check(inc, theta, false)?;
            }
            let at = pos.at;
            if self.core.pending.is_some() {
                self.core.resolve_fillet(at, theta, false)?;
            } else if self.core.start_ang.is_none() {
                self.core.start_ang = Some(theta);
            }
        }
        self.tip.ang = Some(theta);
        self.tip.ang_by_tangent = false;
        Ok(in_state(self.core, self.tip))
    }
}

impl<T: Real + Decide + Bounds> PartialPath<T, HasPos<WithIncoming>, NoAng> {
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
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, HasAng>, PathError> {
        let inc = self
            .tip
            .pos
            .as_ref()
            .and_then(|p| p.incoming)
            .ok_or(PathError::UnderdeterminedLeg {
                site: "turn on a tip without incoming data",
            })?;
        let theta = inc.ang + delta;
        junction_check(&inc, theta, false)?;
        self.tip.ang = Some(theta);
        self.tip.ang_by_tangent = false;
        Ok(in_state(self.core, self.tip))
    }
}

// ------------------------------------------------------------------
// Legs (direction-consuming, from Directed) and the fillet.
// ------------------------------------------------------------------

impl<T: Real + Decide + Bounds, F: Flavor> PartialPath<T, HasPos<F>, HasAng> {
    /// The bound tip pose (backstopped: unreachable-missing data is a
    /// typed error, never a panic).
    fn dep(&self) -> Result<(Point2<T>, T), PathError> {
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
    pub fn line(mut self, len: T) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError> {
        let (at, ang) = self.dep()?;
        if self.tip.ang_by_tangent
            && let Some(inc) = self.tip.pos.as_ref().and_then(|p| p.incoming.as_ref())
                && inc.carrier.is_none() {
                    return Err(PathError::SameCarrierJunction { margin: 0.0 });
                }
        let end = at + unit(ang) * len;
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
    pub fn fillet(mut self, radius: T) -> Result<PartialPath<T, NoPos, NoAng>, PathError> {
        let (at, ang) = self.dep()?;
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
    ) -> Result<(T, T, ArcData<T>, T), PathError> {
        let (at, ang) = self.dep()?;
        let d = p - at;
        let u = unit(ang);
        let along = u.dot(d);
        let across = u.perp_dot(d);
        let delta = across.atan2(along);
        let bulge = (delta / T::from_f64(2.0)).tan();
        let carrier = arc_carrier(at, p, bulge);
        if self.tip.ang_by_tangent
            && let Some(inc) = self.tip.pos.as_ref().and_then(|pd| pd.incoming.as_ref()) {
                match &inc.carrier {
                    None => {
                        let band = linear_band()?;
                        match decide("path_collinear_target", Length::of(across), band) {
                            Ok(Sign::Zero) => {
                                return Err(if closing {
                                    PathError::TangentLineClose {
                                        margin: across.lo(),
                                    }
                                } else {
                                    PathError::SameCarrierJunction {
                                        margin: across.lo(),
                                    }
                                });
                            }
                            Ok(_) => {}
                            Err(source) => return Err(PathError::Escalated { source }),
                        }
                    }
                    Some(prev) => refuse_identical_carriers(prev, &carrier)?,
                }
            }
        let end_ang = ang + delta + delta;
        let chord = d.norm_squared().sqrt();
        Ok((bulge, end_ang, carrier, chord))
    }

    fn tangent_arc_to_point(
        mut self,
        p: Point2<T>,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError> {
        let (bulge, end_ang, carrier, chord) = self.tangent_arc_geom(p, false)?;
        self.core.push_arc(p, bulge, carrier)?;
        let arm = carrier.radius.min(chord);
        Ok(in_state(
            self.core,
            leg_end_tip(p, end_ang, arm, Some(carrier)),
        ))
    }

    fn tangent_arc_to_start(mut self) -> Result<ProfileLoop<T>, PathError> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let start_ang = self.core.start_ang.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry direction is bound",
        })?;
        let (bulge, end_ang, carrier, chord) = self.tangent_arc_geom(start_pos, true)?;
        let arm = carrier.radius.min(chord);
        junction_check(
            &Incoming {
                ang: end_ang,
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

// ------------------------------------------------------------------
// Point-state verbs (sugar tier: one call each, expands to core).
// ------------------------------------------------------------------

impl<T: Real + Decide + Bounds, F: Flavor> PartialPath<T, HasPos<F>, NoAng> {
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

    fn tip_pos(&self) -> Result<&PosData<T>, PathError> {
        self.tip.pos.as_ref().ok_or(PathError::UnderdeterminedLeg {
            site: "point tip without a position",
        })
    }

    fn line_to_point(
        mut self,
        p: Point2<T>,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError> {
        let pos = self.tip_pos()?;
        let at = pos.at;
        let d = p - at;
        let gamma = d.y.atan2(d.x);
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

    fn line_to_start(mut self) -> Result<ProfileLoop<T>, PathError> {
        let start_pos = self.core.start_pos.ok_or(PathError::UnderdeterminedLeg {
            site: "close before the entry position is bound",
        })?;
        let at = self.tip_pos()?.at;
        let d = start_pos - at;
        let gamma = d.y.atan2(d.x);
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

    /// The arc's derived angles: chord direction γ, included angle
    /// θ = 4·atan(b), start tangent γ − θ/2, end tangent γ + θ/2.
    fn arc_angles(at: Point2<T>, target: Point2<T>, bulge: T) -> (T, T, T) {
        let d = target - at;
        let gamma = d.y.atan2(d.x);
        let theta = bulge.atan() * T::from_f64(4.0);
        let half = theta / T::from_f64(2.0);
        (gamma - half, gamma + half, d.norm_squared().sqrt())
    }

    fn arc_to_point(
        mut self,
        p: Point2<T>,
        bulge: T,
    ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError> {
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

    fn arc_to_start(mut self, bulge: T) -> Result<ProfileLoop<T>, PathError> {
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

impl<T: Real + Decide + Bounds> PartialPath<T, NoPos, NoAng> {
    /// The combined binder consuming a directed-point VALUE
    /// (`Open → Directed` in one step). [`Start`] is its canonical
    /// argument, and using it is closing: `.angle(θ).fillet(r)
    /// .to(Start)` is the seam fillet — both carriers bound, nothing
    /// pending, loop closed. (Curve-pose arguments — `c.start()` /
    /// `c.end()` — arrive with NURBS legs in v2; see the module docs.)
    pub fn to(mut self, target: Start) -> Result<ProfileLoop<T>, PathError> {
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
pub trait LineTarget<T: Real + Decide + Bounds, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn line_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self) -> Self::Out;
}

impl<T: Real + Decide + Bounds, F: Flavor> LineTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError>;
    fn line_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self) -> Self::Out {
        path.line_to_point(target)
    }
}

impl<T: Real + Decide + Bounds, F: Flavor> LineTarget<T, F> for Start {
    type Out = Result<ProfileLoop<T>, PathError>;
    fn line_from(path: PartialPath<T, HasPos<F>, NoAng>, _target: Self) -> Self::Out {
        path.line_to_start()
    }
}

/// A [`PartialPath::arc_to`] target: an authored absolute point, or
/// [`Start`] (the sharp arc seam). Sealed.
pub trait ArcTarget<T: Real + Decide + Bounds, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn arc_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self, bulge: T) -> Self::Out;
}

impl<T: Real + Decide + Bounds, F: Flavor> ArcTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError>;
    fn arc_from(path: PartialPath<T, HasPos<F>, NoAng>, target: Self, bulge: T) -> Self::Out {
        path.arc_to_point(target, bulge)
    }
}

impl<T: Real + Decide + Bounds, F: Flavor> ArcTarget<T, F> for Start {
    type Out = Result<ProfileLoop<T>, PathError>;
    fn arc_from(path: PartialPath<T, HasPos<F>, NoAng>, _target: Self, bulge: T) -> Self::Out {
        path.arc_to_start(bulge)
    }
}

/// A [`PartialPath::tangent_arc_to`] target: an authored absolute
/// point, or [`Start`] (the tangent-seam close). Sealed.
pub trait TangentArcTarget<T: Real + Decide + Bounds, F: Flavor>: sealed::Sealed {
    /// A directed point for an interior target; the closed loop for
    /// [`Start`].
    type Out;
    #[doc(hidden)]
    fn tangent_arc_from(path: PartialPath<T, HasPos<F>, HasAng>, target: Self) -> Self::Out;
}

impl<T: Real + Decide + Bounds, F: Flavor> TangentArcTarget<T, F> for Point2<T> {
    type Out = Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError>;
    fn tangent_arc_from(path: PartialPath<T, HasPos<F>, HasAng>, target: Self) -> Self::Out {
        path.tangent_arc_to_point(target)
    }
}

impl<T: Real + Decide + Bounds, F: Flavor> TangentArcTarget<T, F> for Start {
    type Out = Result<ProfileLoop<T>, PathError>;
    fn tangent_arc_from(path: PartialPath<T, HasPos<F>, HasAng>, _target: Self) -> Self::Out {
        path.tangent_arc_to_start()
    }
}
