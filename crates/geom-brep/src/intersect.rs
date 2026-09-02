//! **THE C5 dispatch table** (M5 PR 5, C12.1): one total
//! `(SurfaceKind, SurfaceKind)` routing table for surface×surface
//! intersection, plus the rung-2 closed forms this PR lands.
//!
//! # The table rules (C5, verbatim-binding)
//!
//! - **Compile-time routing, no runtime fallback.** [`route`] is an
//!   exhaustive match with **no wildcard arms**: adding a
//!   [`SurfaceKind`] breaks the build (D3), and every arm is a
//!   documented decision into C1's rung 1 (closed form), rung 2
//!   (conic), or rung 3 (march + fit). "Try closed-form, else march"
//!   is a silent semantic downgrade and does not exist here: an
//!   unimplemented arm refuses **typed**, naming its routing.
//! - **Within-pair degeneracy trileans run BEFORE any rung**: each
//!   implemented arm's section function classifies its configuration
//!   invariants through named Q1 predicates (axis parallelism at
//!   derived angular thresholds with named lever arms, center/axis
//!   distances vs radii, all K-funnel registered) — definitely-generic
//!   ⇒ the arm's rung; exactly-degenerate ⇒ the degenerate closed
//!   form; in-band ⇒ the F6 escalated typed error
//!   ([`SectionError::Escalated`] — an ill-conditioned operand pair at
//!   this ε), whose Display composes the shared two-tolerance recourse
//!   through [`geom_core::Indeterminate`]'s own Display.
//! - **The M2 pairs enter unchanged**: plane×plane stays the existing
//!   splitting/boolean seam (rung 1, implemented — the table names it,
//!   the pipelines execute it bit-identically); plane×cylinder's rim
//!   case stays the rung-1 `Circle`.
//! - **R1 is permanent until a PR moves it**: plane×cone generic tilt
//!   routes to rung 3 *explicitly and permanently* — the conic trio
//!   (parabola/hyperbola) does NOT land in M5, so the arm's generic
//!   verdict is [`SectionError::RoutesToGeneralRung`], a documented
//!   decision, not a TODO.
//!
//! # What executes in this PR (spec §3)
//!
//! 1. [`plane_cylinder_section`] — tilted ⇒ exact `Ellipse`
//!    (zero-residual-by-construction: every constructed point satisfies
//!    both implicit forms exactly in ℝ); axis ∥ normal ⇒ the M2 rim
//!    `Circle`; axis in-plane ⇒ line pair / tangent line / empty.
//! 2. [`cylinder_cylinder_section`] — equal radii (**structural or
//!    declared ONLY, never inferred from values** — the caller passes
//!    [`RadiusEvidence`] resolved through the coincidence ladder; the
//!    declaration is then *verified*, D5-style) with intersecting axes
//!    ⇒ two `Ellipse` carriers in the two axis-bisector planes;
//!    parallel axes ⇒ line pair / tangent line / empty; skew or
//!    undeclared ⇒ typed rung-3 refusal.
//! 3. [`plane_cone_section`] — exact-degenerate cases only (R1):
//!    apex-through plane (two generator lines / tangent line / apex
//!    point), axis-normal cut (`Circle`); generic tilt refuses typed as
//!    permanently routed to rung 3.
//!
//! # What M5 PR 7 added (rung 3 becomes real)
//!
//! Two arms retired their refusals — per-arm, never wholesale (C12.1),
//! each with its trace shape as a **compile-time** decision documented
//! at the arm (C5: no runtime fallback, so an arm's shape is not
//! something a caller can influence):
//!
//! - **plane × NURBS** — the ℝ⁴ parametric×parametric trace (3×4 SVD).
//! - **cylinder × sphere** — the ℝ³ implicit-pair march (2×3 SVD).
//!
//! Both go through `geom_brep::ssi`, which marches (untrusted), fits
//! (PR 4), certifies all three C2 limbs, and proves the domain
//! exhausted or refuses typed. Every other rung-3 arm keeps its typed
//! refusal and now **cites the trace shape it would use** and what is
//! actually missing — which is, for most of them, not the trace but the
//! exact meters conversion of their C9 composite.
//!
//! Tangential outcomes (`TangentLine` variants) are **classification
//! data**, not constructible edges: a pair whose transversality margin
//! dies along the locus is C7 (`TangentIntersection`) territory — M5
//! PR 9 builds those; consumers here must refuse to construct from
//! them (the split/boolean lanes do, typed).

use geom::Surface;
use geom::{Curve3, EllipseInvalid};
use geom_core::{Band, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::dihedral::decide;
use geom_core::Decide;

// ---------------------------------------------------------------------
// Kinds, rungs, routing
// ---------------------------------------------------------------------

/// The closed kind tag of a [`Surface`] variant — the table's index
/// set. Mirrors the enum exactly (D3: closed, compiler-enumerated).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceKind {
    /// [`Surface::Plane`].
    Plane,
    /// [`Surface::Cylinder`].
    Cylinder,
    /// [`Surface::Cone`].
    Cone,
    /// [`Surface::Sphere`].
    Sphere,
    /// [`Surface::Torus`].
    Torus,
    /// [`Surface::Nurbs`] — the universal fallback kind.
    Nurbs,
    /// [`Surface::Approx`] — a fitted stand-in for a description.
    ///
    /// **Its own kind, not `Nurbs`.** The payload is a NURBS and every
    /// evaluator delegates to it, but a pair table indexed by kind is
    /// deciding what a *locus claim* about the pair means, and a claim
    /// about an approximating surface is a claim about the fit, not
    /// about the surface the modeller asked for. Collapsing the two
    /// tags would let every such table answer for `Approx` silently —
    /// the exact failure the closed enum exists to prevent.
    Approx,
}

impl SurfaceKind {
    /// The kind of a surface value.
    pub fn of<T: Real>(s: &Surface<T>) -> Self {
        match s {
            Surface::Plane { .. } => Self::Plane,
            Surface::Cylinder { .. } => Self::Cylinder,
            Surface::Cone { .. } => Self::Cone,
            Surface::Sphere { .. } => Self::Sphere,
            Surface::Torus { .. } => Self::Torus,
            Surface::Nurbs(_) => Self::Nurbs,
            Surface::Approx(_) => Self::Approx,
        }
    }

    /// The kind's display name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Plane => "plane",
            Self::Cylinder => "cylinder",
            Self::Cone => "cone",
            Self::Sphere => "sphere",
            Self::Torus => "torus",
            Self::Nurbs => "nurbs",
            Self::Approx => "approx",
        }
    }
}

/// C1's three-rung intersection-locus ladder — where a pair's locus
/// representation lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rung {
    /// Rung 1: closed-form `Line`/`Circle` carriers (the M2 pairs).
    Closed,
    /// Rung 2: exact conic carriers (`Ellipse`, M5 PR 5).
    Conic,
    /// Rung 3: march + fit (SSI, M5 PR 7) — the general rung.
    General,
}

impl Rung {
    /// The rung's display name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Closed => "rung 1 (closed form)",
            Self::Conic => "rung 2 (exact conic)",
            Self::General => "rung 3 (march + fit / SSI)",
        }
    }
}

/// One arm of the table: the pair's **documented routing decision**.
/// `implemented` is per-arm retirement state (C12.1: the curved-boolean
/// refusal retires arm by arm, never wholesale); an unimplemented arm's
/// consumers refuse typed with [`PairRoute::refusal`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PairRoute {
    /// The rung this pair's locus is routed to.
    pub rung: Rung,
    /// Whether this build executes the arm (closed forms exist and are
    /// wired). `false` arms refuse typed — never a runtime fallback.
    pub implemented: bool,
    /// The arm's decision note (cited by refusals).
    pub note: &'static str,
}

impl PairRoute {
    /// The typed-refusal sentence for an unimplemented arm — names the
    /// routing per C5 ("this pair routes to …") and what that arm is
    /// still missing.
    pub fn refusal(&self, a: SurfaceKind, b: SurfaceKind) -> String {
        format!(
            "{}×{} routes to {} — {}",
            a.name(),
            b.name(),
            self.rung.name(),
            self.note
        )
    }
}

/// **THE table** (C5): every unordered kind pair's routing, written as
/// the exhaustive ordered match — no wildcard arm anywhere, so adding a
/// `SurfaceKind` breaks this build at compile time (D3). Symmetric: the
/// two orders of a pair share one arm via explicit `|` alternation.
///
/// **Compile-break note (spec §6's doc-note, deliberately not a
/// committed test)**: verified at spec time by adding a scratch
/// seventh `SurfaceKind` variant — this match (and `SurfaceKind::of`
/// / `name`) fail with E0004 non-exhaustive-patterns before anything
/// else in the workspace; the no-wildcard grep row in
/// `tests/pcurve_conic.rs` keeps the property pinned in CI.
pub fn route(a: SurfaceKind, b: SurfaceKind) -> PairRoute {
    use SurfaceKind::{Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
    match (a, b) {
        // ---- Rung 1, implemented: the M2 pair, executed by the
        // existing splitting/boolean seam bit-identically. ----
        (Plane, Plane) => PairRoute {
            rung: Rung::Closed,
            implemented: true,
            note: "the planar seam (lines; splitting + boolean execute it)",
        },
        // ---- Rung 2, implemented HERE (M5 PR 5 §3.1): tilted ⇒
        // Ellipse; the rim case stays the rung-1 Circle; axis-parallel
        // degenerates are closed forms. ----
        (Plane, Cylinder) | (Cylinder, Plane) => PairRoute {
            rung: Rung::Conic,
            implemented: true,
            note: "tilted cut is the exact Ellipse (plane_cylinder_section); the \
                   perpendicular cut stays the rung-1 rim Circle",
        },
        // ---- Rung 2, exact-degenerates only (R1, PERMANENT): generic
        // tilt routes to rung 3 until a future PR adds the conic trio.
        // The routing itself is the decision — not a TODO. ----
        (Plane, Cone) | (Cone, Plane) => PairRoute {
            rung: Rung::Conic,
            implemented: true,
            note: "exact-degenerate cases only (apex-through lines/tangent/point, \
                   axis-normal Circle); generic tilt routes to the general rung \
                   PERMANENTLY (parabola and hyperbola are outside the conic \
                   inventory by decision, not by omission) — a routing that no \
                   general-rung arm retires",
        },
        // ---- Rung 1, implemented (M5 S13): the closed-form Circle —
        // never a fitted chord (the die-pips premise). ----
        (Plane, Sphere) | (Sphere, Plane) => PairRoute {
            rung: Rung::Closed,
            implemented: true,
            note: "a plane×sphere cut is the closed-form Circle \
                   (plane_sphere_section); the tangent gap is a POINT — \
                   classification data, refused as a carrier",
        },
        (Sphere, Sphere) => PairRoute {
            rung: Rung::Closed,
            implemented: true,
            note: "distinct spheres cut in the closed-form radical-plane Circle \
                   (sphere_sphere_section); either tangency is a POINT — \
                   classification data, refused as a carrier — and one sphere \
                   given twice is a COINCIDENCE to declare, never a section",
        },
        // ---- Rung 2, implemented HERE (M5 PR 5 §3.2) for the
        // equal-radius intersecting-axes configuration; everything else
        // in the pair routes to rung 3. ----
        (Cylinder, Cylinder) => PairRoute {
            rung: Rung::Conic,
            implemented: true,
            note: "equal radii (structural/declared ONLY — never inferred from \
                   values) with intersecting axes split into two Ellipses \
                   (cylinder_cylinder_section); unequal, undeclared, or skew routes \
                   to the general rung, whose cylinder×cylinder arm has not retired \
                   (arms retire one at a time, each with its proof)",
        },
        // ---- Rung 3: quartic-and-worse loci. The general rung is
        // implemented, but it retires per arm (C12.1), so these still
        // refuse typed, naming the routing AND what each one lacks.
        (Plane, Torus) | (Torus, Plane) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "a plane×torus section is quartic (special Villarceau/profile \
                   circles are not classified here); this pair routes to the \
                   general rung with the ℝ³ IMPLICIT-PAIR trace shape, which \
                   exists — but the torus's exact-arithmetic composite is quartic (m⁴) \
                   and its exact conversion back to meters needs a certified root \
                   the ring does not have, so the arm stays refused until that \
                   conversion lands (arms retire one at a time, each with its \
                   proof)",
        },
        (Cylinder, Cone) | (Cone, Cylinder) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher); the cone's meters composite \
                   needs a certified root the exact-arithmetic ring lacks, so its \
                   certificate — \
                   not its trace — is what is missing",
        },
        // ---- Rung 3, IMPLEMENTED (M5 PR 7): the ℝ³ implicit-pair
        // march. Both operands' C9 composites convert to meters
        // exactly, so all three C2 limbs certify without an invented
        // scale factor — which is why this is the pair the milestone's
        // planted small-loop fixture is built on. ----
        (Cylinder, Sphere) | (Sphere, Cylinder) => PairRoute {
            rung: Rung::General,
            implemented: true,
            note: "marched in ℝ³ on the IMPLICIT PAIR (2×3 SVD, Hoffmann §6.2) and \
                   fitted, with the full three-limb certificate and in-op \
                   exhaustiveness (geom_brep::ssi::cylinder_sphere_ssi); the \
                   DECLARED-coaxial special case is classified exactly \
                   (cylinder_sphere_section: two circles, the tangent circle as \
                   classification data, or empty), and everything else — every \
                   transversal pose, and every coaxial pose without ladder evidence, \
                   because coaxiality is never inferred from a measured distance — \
                   still marches",
        },
        (Cylinder, Torus) | (Torus, Cylinder) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher); blocked on the torus's exact \
                   meters conversion, as plane×torus is",
        },
        (Cone, Cone) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher), blocked on the cone's exact \
                   meters conversion; the common-apex line-pair special case is \
                   not classified here",
        },
        (Cone, Sphere) | (Sphere, Cone) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher), blocked on the cone's exact \
                   meters conversion",
        },
        (Cone, Torus) | (Torus, Cone) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher), blocked on both operands' \
                   exact meters conversions",
        },
        (Sphere, Torus) | (Torus, Sphere) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher), blocked on the torus's exact \
                   meters conversion",
        },
        (Torus, Torus) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "this pair routes to the general rung with the ℝ³ IMPLICIT-PAIR \
                   trace shape (the general-rung marcher), blocked on the torus's exact \
                   meters conversion",
        },
        // ---- Rung 3, IMPLEMENTED (M5 PR 7): the ℝ⁴ trace. The
        // plane and the wall are both charts, so the state is
        // (u₁,v₁,u₂,v₂) on G₁ − G₂ = 0 (3×4 SVD, Hoffmann §6.3.2) and
        // BOTH pcurves fall out as coordinate projections of the one
        // traced object — the shared parameter PR 6's cache contract
        // wants, which is how OQ4 discharged. ----
        // ---- Rung 3, RETIRED 2026-07-31 (M5 PR 7b): the ℝ⁴ arm's
        // last limb landed. PR 7 shipped the trace, the
        // shared-parameter fit of the carrier and BOTH pcurves, the
        // certified foot points, the chart uniqueness tube and the
        // UV-domain exhaustiveness, and refused at C2.2's
        // between-samples sup bound against the NURBS operand (the
        // per-span first-order enclosure was sound but scaled like
        // the span width). PR 7b landed the tensor-product Bernstein
        // composition (`geom_core::spline::compose::tensor`): the
        // residual S(P(t)) − C(t) is enclosed as ONE composite so the
        // cancellation survives, limb 2 flipped to the tight bound,
        // and the arm retired by deleting nothing (C12.1: per-arm,
        // WITH its proof). ----
        (Plane, Nurbs) | (Nurbs, Plane) => PairRoute {
            rung: Rung::General,
            implemented: true,
            note: "traced in ℝ⁴ on the PARAMETRIC PAIR (3×4 SVD, Hoffmann §6.3.2) by \
                   geom_brep::ssi::plane_nurbs_ssi, which certifies the whole chain: \
                   the trace, the shared-parameter fit of the carrier and BOTH \
                   pcurves, the certified foot points, the chart uniqueness tube, \
                   the UV-domain exhaustiveness, and the between-samples SUP bound \
                   against the NURBS operand. That last bound is the \
                   tensor-product Bernstein composition of the surface with the \
                   pcurve (geom_core::spline::compose::tensor): the residual \
                   S(P(t)) − C(t) is enclosed as a SINGLE composite, so the \
                   cancellation a per-span first-order enclosure throws away \
                   survives into the bound. Practical breadth: gentle single-cell \
                   walls — an interior-knot wall refuses at the march/fit limb, \
                   and multi-cell/rational span windows hull neighbor-cell \
                   extensions into the bound or poison — loud and typed, never \
                   silent",
        },
        // ---- Nurbs × the rest: the universal general-rung route. ----
        (Nurbs, Cylinder | Cone | Sphere | Torus | Nurbs)
        | (Cylinder | Cone | Sphere | Torus, Nurbs) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "a NURBS operand routes to the general rung with the ℝ⁴ \
                   PARAMETRIC-PAIR trace shape; the general-rung marcher traces \
                   it and the tensor-composite sup bound that certifies \
                   plane×NURBS is the substrate these arms reuse. What is \
                   missing is the rest of the CERTIFICATE: the analytic partner \
                   needs its chart-form uniqueness tube (written for the plane), \
                   and NURBS×NURBS needs both charts' tube plus its own \
                   exhaustiveness/seeding story — arms retire one at a time, each \
                   with its proof",
        },
        // ---- Approx × everything: refused, and deliberately NOT as
        // the fitted kind would be. An intersection locus is a claim
        // about the surfaces the modeller asked for; against an
        // approximating surface it is a claim about the FIT, off the
        // intended locus by up to the fit's own ε. Certifying it means
        // composing that ε with the SSI's three limbs, and no rule for
        // that composition is ratified. Routing `Approx` to its fitted
        // kind's arm would silently make the weaker claim. ----
        (Approx, Plane | Cylinder | Cone | Sphere | Torus | Nurbs | Approx)
        | (Plane | Cylinder | Cone | Sphere | Torus | Nurbs, Approx) => PairRoute {
            rung: Rung::General,
            implemented: false,
            note: "an approximating operand routes to the general rung with the ℝ⁴ \
                   PARAMETRIC-PAIR trace shape of its FIT, and refuses there: the \
                   locus the trace would certify is the fit's, not the described \
                   surface's, and composing the fit's precision claim with the \
                   SSI certificate's limbs is not a ratified rule. The refusal is \
                   the honest answer, not a missing marcher",
        },
    }
}

// ---------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------

/// Typed refusal of a section function — D4 ¶3: actionable, closed.
#[derive(Clone, Debug, PartialEq)]
pub enum SectionError {
    /// The caller routed a pair to the wrong arm (kind mismatch) — a
    /// caller bug, loudly typed rather than assumed away.
    WrongLane {
        /// What the arm expected.
        expected: &'static str,
    },
    /// A within-pair degeneracy trilean landed in the ambiguity band
    /// or poisoned (F6): the operand pair is ill-conditioned at this ε.
    Escalated(Indeterminate),
    /// The configuration routes to the general rung — a documented arm
    /// decision (no runtime fallback exists; C5). The general rung is
    /// implemented; its arms retire one at a time, so a pair reaching
    /// here is one whose arm has not retired — or one routed there
    /// permanently (plane×cone generic tilt, R1).
    RoutesToGeneralRung {
        /// The pair, for the message.
        pair: &'static str,
        /// The routing grounds.
        why: &'static str,
    },
    /// The declared radius equality is contradicted by the geometry
    /// (|r₁ − r₂| definitely nonzero): declarations are verified, never
    /// trusted (the M3 verified-at-use posture).
    RadiusDeclarationContradicted,
    /// The declared COAXIALITY is contradicted by the geometry (the
    /// axis-to-centre distance definitely nonzero): declarations are
    /// verified, never trusted (the M3 verified-at-use posture).
    CoaxialDeclarationContradicted,
    /// An operand violates the surface convention its arm is written
    /// against — a radius that is not definitely positive. Asked per
    /// QUESTION rather than through the relation between two radii: a
    /// relation-only guard admits `r = 0` and `r = R = 0` and mints a
    /// radius-zero locus from them.
    DegenerateOperand {
        /// Which clause of the convention failed.
        what: &'static str,
    },
    /// The two surfaces are coincident (coaxial equal-radius cylinders):
    /// a same-surface locus is a coincidence to declare/merge, never an
    /// intersection curve.
    CoincidentSurfaces,
    /// The conic carrier constructor refused (near-circular tilt or a
    /// degenerate axis) — the constructor is the one deciding door for
    /// axis ordering (spec §1) and its verdict stands.
    Carrier(EllipseInvalid),
}

impl From<EllipseInvalid> for SectionError {
    fn from(e: EllipseInvalid) -> Self {
        Self::Carrier(e)
    }
}

impl core::fmt::Display for SectionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WrongLane { expected } => {
                write!(
                    f,
                    "section: wrong dispatch lane — this arm classifies {expected} (caller bug)"
                )
            }
            // The Indeterminate Display carries the shared two-tolerance
            // recourse (S6) exactly once.
            Self::Escalated(diag) => write!(
                f,
                "section: configuration trilean escalated — an ill-conditioned \
                 operand pair at this tolerance: {diag}"
            ),
            Self::RoutesToGeneralRung { pair, why } => {
                write!(f, "section: {pair}: {why}")
            }
            Self::RadiusDeclarationContradicted => write!(
                f,
                "section: the declared equal-radius coincidence is contradicted by the \
                 geometry (|r1 - r2| definitely nonzero) — declarations are verified at \
                 use, never trusted"
            ),
            Self::CoaxialDeclarationContradicted => write!(
                f,
                "section: the declared coaxiality is contradicted by the geometry (the \
                 sphere's centre is definitely off the cylinder's axis) — declarations \
                 are verified at use, never trusted"
            ),
            Self::DegenerateOperand { what } => write!(
                f,
                "section: {what} — the arm is written against the convention that both \
                 radii are definitely positive, and it asks each clause its own \
                 question rather than reading the relation between them"
            ),
            Self::CoincidentSurfaces => write!(
                f,
                "section: the surfaces are coincident (coaxial equal-radius cylinders) — \
                 a same-surface locus is not an intersection; {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::Carrier(e) => write!(f, "section: {e}"),
        }
    }
}

impl std::error::Error for SectionError {}

// ---------------------------------------------------------------------
// plane × cylinder (spec §3.1)
// ---------------------------------------------------------------------

/// The classified plane×cylinder section — every configuration's
/// closed form (the trileans run before any rung, C5).
#[derive(Clone, Debug)]
pub enum PlaneCylinderSection<T: Real> {
    /// Generic tilt: the exact `Ellipse` (rung 2) — semi-major
    /// `r/|cos φ|` along the axis' in-plane shadow, semi-minor `r`
    /// along `axis × normal`, centered where the axis pierces the
    /// plane. Zero-residual-by-construction (both implicit forms
    /// vanish identically in ℝ on the constructed locus).
    TiltedEllipse(Curve3<T>),
    /// Axis ∥ normal: the rung-1 rim `Circle` (the M2 case, unchanged —
    /// carrier axis is the CYLINDER axis, u_ref the cylinder's seam).
    Rim(Curve3<T>),
    /// Axis in-plane, axis-to-plane gap definitely < r: two rulings.
    ParallelLines {
        /// The ruling on the `+axis×normal`-ish side.
        l1: Curve3<T>,
        /// The other ruling.
        l2: Curve3<T>,
    },
    /// Axis in-plane, gap coincident with r: the tangency ruling —
    /// **classification data, not a constructible edge** (C7: tangent
    /// loci are `TangentIntersection` territory, M5 PR 9; consumers
    /// refuse typed).
    TangentLine(Curve3<T>),
    /// Axis in-plane, gap definitely > r: no intersection.
    Empty,
}

/// Classifies and constructs the plane×cylinder section (spec §3.1).
///
/// Trileans, in order (named lever arms per D4 ¶1):
///
/// 1. `pc_axis_plane_parallel` — margin `(axis·normal)·extent` (the
///    axis' angle off the plane, metered at the operand extent
///    `extent`): Zero ⇒ the axis lies in the plane (the parallel
///    degenerate lane, step 2); definite ⇒ a bounded cut (step 3).
/// 2. `pc_parallel_gap` — margin `r − |signed axis-to-plane gap|`
///    (meters): Positive ⇒ [`PlaneCylinderSection::ParallelLines`],
///    Zero ⇒ [`PlaneCylinderSection::TangentLine`], Negative ⇒
///    [`PlaneCylinderSection::Empty`].
/// 3. `pc_rim_alignment` — margin `‖axis×normal‖·r` (the tilt angle's
///    sine, metered at the rim radius): Zero ⇒
///    [`PlaneCylinderSection::Rim`]; definite ⇒
///    [`PlaneCylinderSection::TiltedEllipse`] through the ellipse
///    constructor (whose own `ellipse_axes_distinct` gate is the final
///    word on near-circular tilts — the documented double gate: the
///    constructor is the one deciding door for the kind).
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, in-band escalations (F6), or a
/// carrier-constructor refusal.
pub fn plane_cylinder_section<T: Decide>(
    plane: &Surface<T>,
    cylinder: &Surface<T>,
    extent: T,
    band: Band,
) -> Result<PlaneCylinderSection<T>, SectionError> {
    let &Surface::Plane {
        origin: q,
        normal: n,
        ..
    } = plane
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×cylinder",
        });
    };
    let &Surface::Cylinder {
        origin: o,
        axis: a,
        radius: r,
        u_ref: cyl_u,
    } = cylinder
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×cylinder",
        });
    };

    let c = a.dot(n);
    match decide("pc_axis_plane_parallel", Margin::levered(c, extent), band)
        .map_err(SectionError::Escalated)?
    {
        Sign::Zero => {
            // The axis lies in the plane: line pair / tangent / empty
            // by the axis-to-plane gap vs the radius.
            let gap_signed = (o - q).dot(n);
            let margin = Margin::of(r - gap_signed.abs());
            match decide("pc_parallel_gap", margin, band).map_err(SectionError::Escalated)? {
                Sign::Positive => {
                    // Cross-section chord: the plane cuts the circle at
                    // foot ± w·half, foot the axis' plane projection.
                    let foot = o - n * gap_signed;
                    let half = (r.powi(2) - gap_signed.powi(2)).sqrt();
                    let w = a.cross(n).normalize();
                    Ok(PlaneCylinderSection::ParallelLines {
                        l1: Curve3::Line {
                            origin: foot + w * half,
                            dir: a,
                        },
                        l2: Curve3::Line {
                            origin: foot - w * half,
                            dir: a,
                        },
                    })
                }
                Sign::Zero => Ok(PlaneCylinderSection::TangentLine(Curve3::Line {
                    origin: o - n * gap_signed,
                    dir: a,
                })),
                Sign::Negative => Ok(PlaneCylinderSection::Empty),
            }
        }
        Sign::Positive | Sign::Negative => {
            // Bounded cut: rim circle vs tilted ellipse.
            let sin_vec = a.cross(n);
            let sin_norm = sin_vec.norm();
            let t_star = (q - o).dot(n) / c;
            let center = o + a * t_star;
            match decide("pc_rim_alignment", Margin::levered(sin_norm, r), band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Zero => Ok(PlaneCylinderSection::Rim(Curve3::Circle {
                    center,
                    axis: a,
                    radius: r,
                    u_ref: cyl_u,
                })),
                // The margin is a norm: Negative is unreachable; both
                // definite verdicts take the tilted lane.
                Sign::Positive | Sign::Negative => {
                    let v_minor = sin_vec / sin_norm;
                    let u_major = v_minor.cross(n);
                    let e = Curve3::ellipse(center, n, r / c.abs(), r, u_major, band)?;
                    Ok(PlaneCylinderSection::TiltedEllipse(e))
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// plane × sphere (M5 S13: the die-pips join lane's C5 row)
// ---------------------------------------------------------------------

/// The classified plane×sphere section — every configuration's closed
/// form (rung 1: the trileans run before any rung, C5; **no fitted
/// chord anywhere in this pair** — the locus is an exact `Circle`).
#[derive(Clone, Debug)]
pub enum PlaneSphereSection<T: Real> {
    /// Definite cut: the exact `Circle` — centered at the sphere
    /// center's foot on the plane, radius `√((r−|s|)(r+|s|))` for `s`
    /// the signed center-to-plane gap, carrier axis the plane normal,
    /// `u_ref` the plane's own `u_ref` (in-plane by construction).
    /// Zero-residual-by-construction against both surfaces.
    Circle(Curve3<T>),
    /// Gap coincident with r: the tangency **point** — classification
    /// data, not a constructible edge (C7 lineage: tangent loci are
    /// not minted as section carriers; consumers refuse typed).
    TangentPoint(Point3<T>),
    /// Gap definitely > r: no intersection.
    Empty,
}

/// Classifies and constructs the plane×sphere section (C5 rung 1,
/// M5 S13).
///
/// One trilean: `ps_center_gap` — margin `r − |s|` (meters), `s` the
/// signed sphere-center-to-plane gap: Positive ⇒
/// [`PlaneSphereSection::Circle`], Zero ⇒
/// [`PlaneSphereSection::TangentPoint`], Negative ⇒
/// [`PlaneSphereSection::Empty`]. The in-band twin of both definite
/// verdicts escalates through the same named predicate (F6) — the
/// two-tolerance shape.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds or the in-band escalation.
pub fn plane_sphere_section<T: Decide>(
    plane: &Surface<T>,
    sphere: &Surface<T>,
    band: Band,
) -> Result<PlaneSphereSection<T>, SectionError> {
    let &Surface::Plane {
        origin: q,
        normal: n,
        ..
    } = plane
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×sphere",
        });
    };
    let &Surface::Sphere {
        center: c,
        radius: r,
        axis: sph_axis,
        u_ref: sph_u,
    } = sphere
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×sphere",
        });
    };
    let s = (c - q).dot(n);
    let foot = c - n * s;
    // The circle's `u_ref` is a PLACEMENT convention (D2): every
    // downstream margin is a frame DIFFERENCE, so any in-plane unit
    // vector serves — but it must be genuinely in-plane. The plane
    // operand's own `u_ref` is deliberately NOT consulted: the
    // splitting/boolean lanes hand transient classification planes
    // whose `u_ref` is a placeholder (sometimes the normal itself,
    // which would degenerate the frame and collapse every section
    // angle to zero). Derived instead from the sphere's chart frame:
    // `n̂ × û` unless that sine is small, then `n̂ × â` — with
    // `û ⊥ â` the two sines satisfy sin²(û,n̂) + sin²(â,n̂) ≥ 1, so
    // the second candidate is definitely nonzero whenever the first
    // is not chosen. The selection trilean's degenerate and in-band
    // arms both take the second candidate: near the threshold BOTH
    // are valid placements, so the arm is a deterministic tie-break
    // (D9), not a verdict — no downstream VERDICT moves with it
    // (derived f64 parameter bits may differ across code versions;
    // within-run replay and strategy identity are what D9 pins).
    let seam_cand = n.cross(sph_u);
    let u_ref = match decide(
        "ps_frame_seam",
        Margin::levered(seam_cand.norm() - T::from_f64(0.5), r),
        band,
    ) {
        Ok(Sign::Positive) => seam_cand / seam_cand.norm(),
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {
            let polar_cand = n.cross(sph_axis);
            polar_cand / polar_cand.norm()
        }
    };
    match decide("ps_center_gap", Margin::of(r - s.abs()), band).map_err(SectionError::Escalated)? {
        Sign::Positive => Ok(PlaneSphereSection::Circle(Curve3::Circle {
            center: foot,
            axis: n,
            // The interval-square tripwire does not bite: both factors
            // are definitely positive after the trilean (never a
            // spuriously negative bracket under a sqrt).
            radius: ((r - s.abs()) * (r + s.abs())).sqrt(),
            u_ref,
        })),
        Sign::Zero => Ok(PlaneSphereSection::TangentPoint(foot)),
        Sign::Negative => Ok(PlaneSphereSection::Empty),
    }
}

// ---------------------------------------------------------------------
// sphere × sphere
// ---------------------------------------------------------------------

/// The classified sphere×sphere section (rung 1: the trileans run
/// before any rung, C5; **no fitted chord anywhere in this pair** — the
/// locus is an exact `Circle`).
#[derive(Clone, Debug)]
pub enum SphereSphereSection<T: Real> {
    /// Definite cut: the exact radical-plane `Circle`, centred on the
    /// centre line at `a = (d² + r₁² − r₂²)/2d` from sphere 1, radius
    /// `√((r₁−a)(r₁+a))`, carrier axis the unit centre direction.
    /// Zero-residual-by-construction against both surfaces.
    Circle(Curve3<T>),
    /// The spheres touch — externally (`d = r₁ + r₂`) or internally
    /// (`d = |r₁ − r₂|`) — at the one **point** `c₁ + n̂·a`: the
    /// degenerate section circle, at the SAME signed offset
    /// `a = (d² + r₁² − r₂²)/2d` the [`SphereSphereSection::Circle`]
    /// arm uses, with radius zero. One formula, three branches.
    ///
    /// The sign is load-bearing and is why `a` is not written `r₁`.
    /// External tangency and internal tangency with `r₁ > r₂` both give
    /// `a = +r₁`; internal tangency with `r₂ > r₁` — sphere A strictly
    /// inside sphere B — gives `a = −r₁`, and the contact is
    /// diametrically opposite, on the `−n̂` side. Substituting `d` into
    /// `a` gives `+r₁` for two of the three configurations and `−r₁`
    /// for the third; `|a| = r₁` is what all three share.
    ///
    /// Classification data, not a constructible edge (C7 lineage:
    /// tangent loci are not minted as section carriers; consumers
    /// refuse typed).
    TangentPoint(Point3<T>),
    /// Definitely separated (`d > r₁ + r₂`) or definitely strictly
    /// nested (`d < |r₁ − r₂|`): no intersection.
    Empty,
}

/// Classifies and constructs the sphere×sphere section (C5 rung 1).
///
/// The 2-D skeleton is `profile`'s arc/arc carrier gate, and the three
/// trileans are its three, in the same gate order and with the same
/// per-branch division argument — the algebra is identical, only the
/// perpendicular direction differs (a 2-D normal there, a whole circle
/// here). All margins are LENGTHS in metres:
///
/// - **`ss_carrier_identity`** — margin `d + |r₁ − r₂|`, the carrier
///   distance in "same sphere" terms. Zero ⇒ the two surfaces are the
///   same sphere, refused as [`SectionError::CoincidentSurfaces`]: a
///   same-surface locus is a whole-sphere coincidence to declare or
///   merge, never a section circle, and CONTACT-DESIGN forbids
///   inferring the gluing at any ε.
/// - **`ss_carrier_external`** — margin `d − (r₁ + r₂)`. Positive ⇒
///   [`SphereSphereSection::Empty`] (separated); Zero ⇒
///   [`SphereSphereSection::TangentPoint`] (external tangency).
/// - **`ss_carrier_internal`** — margin `d − |r₁ − r₂|`. Negative ⇒
///   [`SphereSphereSection::Empty`] (strictly nested); Zero ⇒
///   [`SphereSphereSection::TangentPoint`] (internal tangency);
///   Positive ⇒ [`SphereSphereSection::Circle`]. The radicand
///   `(r₁ − a)(r₁ + a)` factors through both clearances — `|a| < r₁`
///   reduces exactly to `|d − r₁| < r₂` and `d + r₁ > r₂`, which are
///   the two clearances this branch has already pinned definite — so
///   it is definitely positive here and the interval-square tripwire
///   cannot bite.
///
/// Divisions by `d` occur only where a preceding trilean bounds it away
/// from zero, per branch:
/// the secant branch has `d > |r₁ − r₂| ≥ 0` definitely; the internal
/// tangency has `d = |r₁ − r₂|` with the identity margin `2d` definite;
/// the external tangency has `d = r₁ + r₂ ≥` the identity margin.
///
/// The in-band twin of every definite verdict escalates through the
/// same named predicate (F6) — the two-tolerance shape.
///
/// The form is `atan2`-free and branch-cut-free by construction, so the
/// `Interval` lane takes it unchanged: there is no lane fork here.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, coincident surfaces, or the
/// in-band escalation.
pub fn sphere_sphere_section<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
    band: Band,
) -> Result<SphereSphereSection<T>, SectionError> {
    let wrong = || SectionError::WrongLane {
        expected: "sphere×sphere",
    };
    let &Surface::Sphere {
        center: c1,
        radius: r1,
        axis: axis1,
        u_ref: u1,
    } = a
    else {
        return Err(wrong());
    };
    let &Surface::Sphere {
        center: c2,
        radius: r2,
        ..
    } = b
    else {
        return Err(wrong());
    };
    let delta = c2 - c1;
    let d = delta.norm();
    let dr = (r1 - r2).abs();
    // The SIGNED radical-plane offset, one formula for all three
    // non-empty branches (the 2-D skeleton's shape, `seg.rs:604-616`).
    // Every branch that reads it has already decided `d` definitely
    // positive, so the division is safe wherever it is called.
    let offset = || (d.powi(2) + r1.powi(2) - r2.powi(2)) / (d + d);
    match decide("ss_carrier_identity", Margin::of(d + dr), band)
        .map_err(SectionError::Escalated)?
    {
        Sign::Zero | Sign::Negative => Err(SectionError::CoincidentSurfaces),
        Sign::Positive => {
            match decide("ss_carrier_external", Margin::of(d - (r1 + r2)), band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Positive => Ok(SphereSphereSection::Empty),
                // Externally tangent: `d = r₁ + r₂ ≥` the definite
                // identity margin, so the division is safe. The contact
                // is the degenerate section circle, `c₁ + n̂·a` at
                // radius zero — and `a` must be the SIGNED offset, not
                // `r₁`. Substituting `d = r₁ + r₂` does give `a = +r₁`
                // here; the internal branch below is where the sign
                // matters, and one formula serves both.
                Sign::Zero => Ok(SphereSphereSection::TangentPoint(
                    c1 + delta * (offset() / d),
                )),
                Sign::Negative => {
                    match decide("ss_carrier_internal", Margin::of(d - dr), band)
                        .map_err(SectionError::Escalated)?
                    {
                        Sign::Negative => Ok(SphereSphereSection::Empty),
                        // Internally tangent: `d = |Δr|` and the
                        // identity margin `d + |Δr| = 2d` is definite,
                        // so `d` is bounded away from zero. Here the
                        // SIGN of `a` is the whole answer: substituting
                        // `d = |r₁ − r₂|` gives `a = +r₁` when `r₁ > r₂`
                        // (B inside A, contact on the `+n̂` side) and
                        // `a = −r₁` when `r₂ > r₁` (A inside B, contact
                        // on the `−n̂` side, diametrically opposite).
                        // Writing `+r₁` in both orders puts the contact
                        // point INSIDE the larger ball — for the unit
                        // sphere at the origin against radius 3 at
                        // `x = 2` it returns `(1,0,0)`, which is B's own
                        // centre, and the true contact is `(−1,0,0)`.
                        Sign::Zero => Ok(SphereSphereSection::TangentPoint(
                            c1 + delta * (offset() / d),
                        )),
                        // Proper secant: `d > |Δr|` definitely, so
                        // `d > 0`. The radical-plane closed form.
                        Sign::Positive => {
                            let n = delta / d;
                            let a_off = offset();
                            Ok(SphereSphereSection::Circle(Curve3::Circle {
                                center: c1 + n * a_off,
                                axis: n,
                                radius: ((r1 - a_off) * (r1 + a_off)).sqrt(),
                                u_ref: ss_frame_seam(n, axis1, u1, r1, band),
                            }))
                        }
                    }
                }
            }
        }
    }
}

/// The section circle's `u_ref` PLACEMENT (D2/D9), the sphere sibling
/// of the plane×sphere lane's identical tie-break.
///
/// The section axis is the centre direction `n̂`, and the candidate
/// `n̂ × û` collapses exactly when the centre line runs along the
/// operand's own seam direction — the same degeneracy, reached by a
/// different route. Derived from operand A's chart: `n̂ × û` unless
/// that sine is small, then `n̂ × â` — with `û ⊥ â` the two sines
/// satisfy sin²(û,n̂) + sin²(â,n̂) ≥ 1, so the second candidate is
/// definitely nonzero whenever the first is not chosen. The selection
/// trilean's degenerate and in-band arms both take the second
/// candidate: near the threshold BOTH are valid placements, so the arm
/// is a deterministic tie-break, not a verdict — **no downstream
/// VERDICT moves with it**.
fn ss_frame_seam<T: Decide>(
    n: Vec3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    r: T,
    band: Band,
) -> Vec3<T> {
    let seam_cand = n.cross(u_ref);
    match decide(
        "ss_frame_seam",
        Margin::levered(seam_cand.norm() - T::from_f64(0.5), r),
        band,
    ) {
        Ok(Sign::Positive) => seam_cand / seam_cand.norm(),
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {
            let polar_cand = n.cross(axis);
            polar_cand / polar_cand.norm()
        }
    }
}

// ---------------------------------------------------------------------
// cylinder × cylinder, equal radii (spec §3.2)
// ---------------------------------------------------------------------

/// The coincidence-ladder evidence for radius equality (C5: structural
/// or declared ONLY — **never inferred from values**). The caller —
/// who owns provenance/declaration data — resolves the ladder; this
/// module only consumes its verdict, then *verifies* a declaration
/// against the geometry (declared ≠ unchecked).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadiusEvidence {
    /// Radius equality is structural or declared through the ladder.
    Declared,
    /// No ladder evidence: the pair routes to the general rung even if
    /// the radius VALUES happen to coincide (the never-infer rule).
    None,
}

/// The classified equal-radius cylinder×cylinder section.
#[derive(Clone, Debug)]
pub enum EqualCylinderSection<T: Real> {
    /// Intersecting axes: the two exact `Ellipse` carriers, one in each
    /// axis-bisector plane through the axes' intersection point
    /// (normals ∝ `a1 − a2` and `a1 + a2`). Each is exactly the tilted
    /// plane×cylinder ellipse of cylinder 1 with that bisector plane —
    /// on cylinder 2 by the mirror symmetry, so the residual is
    /// identically zero in ℝ against both.
    TwoEllipses {
        /// The ellipse in the `a1 − a2`-normal bisector plane.
        e1: Curve3<T>,
        /// The ellipse in the `a1 + a2`-normal bisector plane.
        e2: Curve3<T>,
    },
    /// Parallel axes, gap definitely < 2r: two parallel rulings.
    ParallelLines {
        /// One ruling.
        l1: Curve3<T>,
        /// The other.
        l2: Curve3<T>,
    },
    /// Parallel axes, gap coincident with 2r: the tangency ruling —
    /// classification data, not a constructible edge (C7 / M5 PR 9).
    TangentLine(Curve3<T>),
    /// Parallel axes, gap definitely > 2r: no intersection.
    Empty,
}

/// Classifies and constructs the equal-radius cylinder×cylinder
/// section (spec §3.2).
///
/// Trileans, in order:
///
/// 1. [`RadiusEvidence`] gate — **structural, not numeric**: without
///    ladder evidence the pair routes to the general rung
///    ([`SectionError::RoutesToGeneralRung`]), radius values never
///    consulted.
/// 2. `cc_declared_radius_equality` — margin `r₁ − r₂` (meters):
///    the declaration is verified — Zero required; definite ⇒
///    [`SectionError::RadiusDeclarationContradicted`]; in-band ⇒
///    escalated.
/// 3. `cc_axes_parallel` — margin `‖a1×a2‖·extent`: Zero ⇒ the
///    parallel lane (step 4); definite ⇒ the crossing lane (step 5).
/// 4. `cc_coaxial` / `cc_parallel_gap` — axis-to-axis distance `d`:
///    coincident-with-zero ⇒ [`SectionError::CoincidentSurfaces`];
///    then margin `2r − d`: Positive ⇒ two rulings, Zero ⇒ tangent
///    ruling, Negative ⇒ empty.
/// 5. `cc_axes_coplanar` — margin the signed axis-to-axis gap
///    `(o2−o1)·(a1×a2)/‖a1×a2‖` (meters): Zero ⇒ intersecting axes ⇒
///    the two bisector-plane ellipses; definite ⇒ skew ⇒ typed rung-3
///    refusal.
///
/// # Errors
///
/// [`SectionError`] — see the trilean list.
pub fn cylinder_cylinder_section<T: Decide>(
    c1: &Surface<T>,
    c2: &Surface<T>,
    evidence: RadiusEvidence,
    extent: T,
    band: Band,
) -> Result<EqualCylinderSection<T>, SectionError> {
    let &Surface::Cylinder {
        origin: o1,
        axis: a1,
        radius: r1,
        ..
    } = c1
    else {
        return Err(SectionError::WrongLane {
            expected: "cylinder×cylinder",
        });
    };
    let &Surface::Cylinder {
        origin: o2,
        axis: a2,
        radius: r2,
        ..
    } = c2
    else {
        return Err(SectionError::WrongLane {
            expected: "cylinder×cylinder",
        });
    };

    // 1. The ladder gate: never inferred from values.
    if evidence == RadiusEvidence::None {
        return Err(SectionError::RoutesToGeneralRung {
            pair: "cylinder×cylinder",
            why: "radius equality is not structural/declared — never inferred from \
                  values (the coincidence ladder); the undeclared pair routes to the \
                  general rung, whose cylinder×cylinder arm has not retired",
        });
    }
    // 2. Verify the declaration (declared ≠ unchecked).
    match decide("cc_declared_radius_equality", Margin::of(r1 - r2), band)
        .map_err(SectionError::Escalated)?
    {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            return Err(SectionError::RadiusDeclarationContradicted);
        }
    }

    let cross = a1.cross(a2);
    let cross_norm = cross.norm();
    match decide(
        "cc_axes_parallel",
        Margin::levered(cross_norm, extent),
        band,
    )
    .map_err(SectionError::Escalated)?
    {
        Sign::Zero => {
            // Parallel axes: the cross-section is two equal circles at
            // center distance d.
            let w0 = o2 - o1;
            let d_vec = w0 - a1 * w0.dot(a1);
            let d = d_vec.norm();
            match decide("cc_coaxial", Margin::of(d), band).map_err(SectionError::Escalated)? {
                Sign::Zero => return Err(SectionError::CoincidentSurfaces),
                Sign::Positive | Sign::Negative => {}
            }
            let two = T::from_f64(2.0);
            match decide("cc_parallel_gap", Margin::of(two * r1 - d), band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Positive => {
                    let mid = o1 + d_vec * T::from_f64(0.5);
                    let half = (r1.powi(2) - (d / two).powi(2)).sqrt();
                    let h = a1.cross(d_vec / d);
                    Ok(EqualCylinderSection::ParallelLines {
                        l1: Curve3::Line {
                            origin: mid + h * half,
                            dir: a1,
                        },
                        l2: Curve3::Line {
                            origin: mid - h * half,
                            dir: a1,
                        },
                    })
                }
                Sign::Zero => Ok(EqualCylinderSection::TangentLine(Curve3::Line {
                    origin: o1 + d_vec * T::from_f64(0.5),
                    dir: a1,
                })),
                Sign::Negative => Ok(EqualCylinderSection::Empty),
            }
        }
        Sign::Positive | Sign::Negative => {
            // Crossing lane: coplanarity (intersecting vs skew).
            let w0 = o2 - o1;
            let gap = w0.dot(cross) / cross_norm;
            match decide("cc_axes_coplanar", Margin::of(gap), band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Zero => {}
                Sign::Positive | Sign::Negative => {
                    return Err(SectionError::RoutesToGeneralRung {
                        pair: "cylinder×cylinder",
                        why: "skew axes have no conic section; this configuration routes \
                              to the general rung, whose cylinder×cylinder arm has not \
                              retired",
                    });
                }
            }
            // The axes' intersection point (closest point on axis 1;
            // the coplanarity verdict bounds the residual by ε).
            let t1 = w0.cross(a2).dot(cross) / cross_norm.powi(2);
            let p = o1 + a1 * t1;
            // The two bisector planes through p. Each ellipse is the
            // tilted plane×cylinder cut of cylinder 1 (by symmetry it
            // lies on cylinder 2 as well).
            let n1 = (a1 - a2).normalize();
            let n2 = (a1 + a2).normalize();
            let mk = |n: Vec3<T>| -> Result<Curve3<T>, SectionError> {
                let c = a1.dot(n);
                let sin_vec = a1.cross(n);
                let v_minor = sin_vec.normalize();
                let u_major = v_minor.cross(n);
                Ok(Curve3::ellipse(p, n, r1 / c.abs(), r1, u_major, band)?)
            };
            Ok(EqualCylinderSection::TwoEllipses {
                e1: mk(n1)?,
                e2: mk(n2)?,
            })
        }
    }
}

// ---------------------------------------------------------------------
// cylinder × sphere, DECLARED coaxial
// ---------------------------------------------------------------------

/// The coincidence-ladder evidence for a cylinder×sphere pair being
/// COAXIAL — the sphere's centre lying on the cylinder's axis. The
/// [`RadiusEvidence`] sibling, and structural or declared ONLY:
/// **never inferred from a measured axis-to-centre distance**, at any
/// tolerance. The caller — who owns provenance/declaration data —
/// resolves the ladder; this module consumes the verdict, then
/// *verifies* it against the geometry (declared ≠ unchecked).
///
/// **No production caller can supply `Declared` today**, and that is
/// stated rather than papered over: the honest carrier for a
/// parameter-level identity between a cylinder's axis and a sphere's
/// centre is the parameter-identity channel (#1372), which does not
/// exist. Until it does, every in-tree consumer passes [`Self::None`]
/// and the pair routes to the general rung — the arm below is reached
/// only by direct tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoaxialEvidence {
    /// Coaxiality is structural or declared through the ladder.
    Declared,
    /// No ladder evidence: the pair routes to the general rung even if
    /// the axis-to-centre distance happens to measure zero (the
    /// never-infer rule).
    None,
}

/// The classified DECLARED-coaxial cylinder×sphere section.
///
/// Every variant is stated in the cylinder's frame: the shared axis is
/// the cylinder's, and `center` is the sphere's centre — which lies on
/// that axis by the verified declaration, so it doubles as the
/// stations' origin.
#[derive(Clone, Debug)]
pub enum CylinderSphereSection<T: Real> {
    /// `R > r`: the sphere's wall crosses the cylinder's in TWO
    /// circles, both of the cylinder's radius `r`, both centred on the
    /// shared axis at `center ± axis·station`.
    ///
    /// The carriers are given as centre/axis/radius/station rather
    /// than as two [`Curve3::Circle`]s deliberately: a `Circle` carries
    /// a `u_ref` PLACEMENT, and nothing in this classification decides
    /// one. A consumer that needs a seam picks it at its own door with
    /// its own tie-break (the [`ss_frame_seam`] shape), which keeps the
    /// placement out of the verdict.
    TwoCircles {
        /// The sphere centre — on the shared axis, the stations' origin.
        center: Point3<T>,
        /// The shared axis (the cylinder's, unit).
        axis: Vec3<T>,
        /// Both circles' radius: the CYLINDER's `r`, exactly.
        radius: T,
        /// The half-separation `√((R−r)(R+r))`, in the FACTORED form.
        station: T,
    },
    /// `R = r`: one circle at the equator station — CLASSIFICATION
    /// DATA, never a constructible edge (C7 / M5 PR 9), and the same
    /// pose `ssi::cylinder_sphere_ssi`'s own `ssi_cs_tangency` trilean
    /// refuses toward C7.
    TangentCircle {
        /// The sphere centre — the equator station itself.
        center: Point3<T>,
        /// The shared axis (the cylinder's, unit).
        axis: Vec3<T>,
        /// The common radius `R = r`.
        radius: T,
    },
    /// `R < r`: the sphere never reaches the wall.
    Empty,
}

/// Classifies and constructs the DECLARED-coaxial cylinder×sphere
/// section.
///
/// **This arm classifies the coaxial configuration ONLY.** Everything
/// else in the pair — every transversal pose, and every coaxial pose
/// without ladder evidence — routes to the general rung, which for
/// this pair is IMPLEMENTED (`ssi::cylinder_sphere_ssi`, marched and
/// fitted). So a refusal here is a routing, not a frontier: the pair
/// still has an answer, one rung down.
///
/// Trileans, in order — one margin per question:
///
/// 1. [`CoaxialEvidence`] gate — **structural, not numeric**: without
///    ladder evidence no value is consulted at all and the pair routes
///    to the general rung ([`SectionError::RoutesToGeneralRung`]).
/// 2. `cs_cylinder_radius` — margin `r` (meters). The degeneracy guard
///    runs FIRST among the numeric rows and it guards the **FULL**
///    convention `R > 0` and `r > 0`, not merely the relation between
///    them: a guard that decides only `R − r` lets `r = 0` through as
///    `Positive` and mints two radius-zero "circles", and lets
///    `r = R = 0` through as `Zero` and mints a point wearing a
///    tangent circle's name. Definite-positive required; anything else
///    is [`SectionError::DegenerateOperand`].
/// 3. `cs_sphere_radius` — margin `R` (meters). Not implied by row 2
///    plus row 5: a `Surface::Sphere` whose stored radius is NEGATIVE
///    denotes the same point set as its absolute value, so `R = −r`
///    reaches row 5 as `Negative` and answers `Empty` — a FALSE
///    NEGATIVE on a pose whose true section is the tangent circle.
///    Definite-positive required.
/// 4. `cs_declared_coaxial` — margin `d`, the axis-to-centre distance
///    (meters). The declaration is VERIFIED, never trusted: Zero
///    required; definite ⇒ [`SectionError::CoaxialDeclarationContradicted`].
///    This row never runs without row 1's evidence, which is what
///    keeps `d ≈ 0` from ever being read as a declaration.
/// 5. `cs_wall_reach` — margin `R − r` (a length): Positive ⇒
///    [`CylinderSphereSection::TwoCircles`], Zero ⇒
///    [`CylinderSphereSection::TangentCircle`], Negative ⇒
///    [`CylinderSphereSection::Empty`].
///
/// **Consistency with the SSI's own tangency door, not a second
/// adjudication.** At `d = 0` the marcher's `ssi_cs_tangency` margin
/// `min(||d − r| − R|, |d + r − R|)` collapses to `|r − R|` — the
/// absolute value of row 5's margin, on the same band. So the two
/// doors partition the coaxial poses identically: where this arm says
/// `TangentCircle`, the marcher says `TransversalityBand` and refuses
/// toward C7; where this arm says `TwoCircles` or `Empty`, the
/// marcher's trilean is definite. Same margin shape, same verdict
/// class — a tangency is classification data at both doors, and
/// neither constructs a carrier from it.
///
/// # Errors
///
/// [`SectionError`] — see the trilean list.
pub fn cylinder_sphere_section<T: Decide>(
    cyl: &Surface<T>,
    sph: &Surface<T>,
    evidence: CoaxialEvidence,
    band: Band,
) -> Result<CylinderSphereSection<T>, SectionError> {
    let &Surface::Cylinder {
        origin,
        axis,
        radius: r,
        ..
    } = cyl
    else {
        return Err(SectionError::WrongLane {
            expected: "cylinder×sphere (cylinder first)",
        });
    };
    let &Surface::Sphere {
        center,
        radius: big_r,
        ..
    } = sph
    else {
        return Err(SectionError::WrongLane {
            expected: "cylinder×sphere (sphere second)",
        });
    };

    // 1. The ladder gate: never inferred from values.
    if evidence == CoaxialEvidence::None {
        return Err(SectionError::RoutesToGeneralRung {
            pair: "cylinder×sphere",
            why: "coaxiality is not structural/declared — never inferred from a \
                  measured axis-to-centre distance (the coincidence ladder); the \
                  undeclared pair routes to the general rung, whose cylinder×sphere \
                  arm IS implemented (marched and fitted), so this is a routing and \
                  not a frontier",
        });
    }

    // 2-3. The degeneracy guard, on the FULL convention: two questions,
    // two margins. Neither is implied by the reach trilean below.
    for (name, margin, what) in [
        ("cs_cylinder_radius", r, "the cylinder's radius is not definitely positive"),
        ("cs_sphere_radius", big_r, "the sphere's radius is not definitely positive"),
    ] {
        match decide(name, Margin::of(margin), band).map_err(SectionError::Escalated)? {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => {
                return Err(SectionError::DegenerateOperand { what });
            }
        }
    }

    // 4. Verify the declaration (declared ≠ unchecked). The rejection
    // of the axial component is the standard point-to-line distance;
    // `axis` is unit by the surface's own invariant, so no division
    // enters here.
    let q = center - origin;
    let d = (q - axis * q.dot(axis)).norm();
    match decide("cs_declared_coaxial", Margin::of(d), band).map_err(SectionError::Escalated)? {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            return Err(SectionError::CoaxialDeclarationContradicted);
        }
    }

    // 5. Reach: does the sphere's wall get out to the cylinder's?
    match decide("cs_wall_reach", Margin::of(big_r - r), band).map_err(SectionError::Escalated)? {
        // Both factors are DEFINITELY POSITIVE here, so the sqrt's
        // argument is: `R − r` by this trilean's own `Positive`, and
        // `R + r` as the sum of two definitely-positive radii by rows
        // 2 and 3. The FACTORED form is what keeps an interval
        // evaluation tight — `R² − r²` widens both squares before
        // cancelling them, exactly the `sphere_sphere_section`
        // precedent.
        Sign::Positive => Ok(CylinderSphereSection::TwoCircles {
            center,
            axis,
            radius: r,
            station: ((big_r - r) * (big_r + r)).sqrt(),
        }),
        Sign::Zero => Ok(CylinderSphereSection::TangentCircle {
            center,
            axis,
            radius: r,
        }),
        Sign::Negative => Ok(CylinderSphereSection::Empty),
    }
}

// ---------------------------------------------------------------------
// plane × cone, exact-degenerates only (spec §3.3, R1)
// ---------------------------------------------------------------------

/// The classified plane×cone exact-degenerate section. Generic tilt is
/// deliberately NOT a variant: it refuses typed
/// ([`SectionError::RoutesToGeneralRung`]) — R1's permanent routing.
#[derive(Clone, Debug)]
pub enum PlaneConeSection<T: Real> {
    /// Apex on the plane, plane cutting inside the cone: two generator
    /// lines through the apex.
    ApexLinePair {
        /// The generator at azimuth `φ + δ`.
        l1: Curve3<T>,
        /// The generator at azimuth `φ − δ`.
        l2: Curve3<T>,
    },
    /// Apex on the plane, plane tangent along one generator —
    /// classification data, not a constructible edge (C7 / M5 PR 9).
    ApexTangentLine(Curve3<T>),
    /// Apex on the plane, no other contact: the apex point alone.
    ApexPoint(Point3<T>),
    /// Axis ∥ normal, apex off the plane: the rung-1 `Circle` cut.
    AxisNormalCircle(Curve3<T>),
}

/// Classifies and constructs the plane×cone exact-degenerate sections
/// (spec §3.3). Generic tilt refuses typed — **permanently routed to
/// rung 3** (R1: the conic trio does not land in M5; a future PR that
/// adds parabola/hyperbola moves the arm, nothing else does).
///
/// Trileans, in order:
///
/// 1. `pn_apex_on_plane` — margin `(apex − q)·normal` (meters): Zero ⇒
///    the apex lane (step 2); definite ⇒ step 3.
/// 2. `pn_apex_section` — margin `sin α·‖axis×normal‖ −
///    cos α·|axis·normal|` metered at `extent` (the two-generator
///    discriminant: positive exactly when the plane dips inside the
///    cone): Positive ⇒ [`PlaneConeSection::ApexLinePair`], Zero ⇒
///    [`PlaneConeSection::ApexTangentLine`], Negative ⇒
///    [`PlaneConeSection::ApexPoint`].
/// 3. `pn_axis_normal` — margin `‖axis×normal‖·arm`, arm the would-be
///    circle radius `|h|·tan α` (h the apex-to-plane distance along
///    the axis): Zero ⇒ [`PlaneConeSection::AxisNormalCircle`];
///    definite ⇒ the R1 refusal.
///
/// # Errors
///
/// [`SectionError`] — wrong-lane kinds, escalations (F6), or the R1
/// generic-tilt routing refusal.
pub fn plane_cone_section<T: Decide>(
    plane: &Surface<T>,
    cone: &Surface<T>,
    extent: T,
    band: Band,
) -> Result<PlaneConeSection<T>, SectionError> {
    let &Surface::Plane {
        origin: q,
        normal: n,
        ..
    } = plane
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×cone",
        });
    };
    let &Surface::Cone {
        apex,
        axis: a,
        half_angle,
        u_ref: cone_u,
    } = cone
    else {
        return Err(SectionError::WrongLane {
            expected: "plane×cone",
        });
    };

    let (sin_a, cos_a) = half_angle.sin_cos();
    let c = a.dot(n);
    let s_vec = a.cross(n);
    let s = s_vec.norm();

    let apex_gap = (apex - q).dot(n);
    match decide("pn_apex_on_plane", Margin::of(apex_gap), band).map_err(SectionError::Escalated)? {
        Sign::Zero => {
            // Apex lane: generators g(u) = a·cosα + radial(u)·sinα with
            // g·n = 0 ⇔ cos(u − φ) = −cosα·c / (sinα·s).
            let discr = sin_a * s - cos_a * c.abs();
            let verdict = decide("pn_apex_section", Margin::levered(discr, extent), band)
                .map_err(SectionError::Escalated)?;
            match verdict {
                Sign::Positive | Sign::Zero => {
                    let v_ref = a.cross(cone_u);
                    let (nu, nv) = (cone_u.dot(n), v_ref.dot(n));
                    let phi = nv.atan2(nu);
                    // Clamped acos argument (outward-rounding can push
                    // it a hair past ±1 at the tangency boundary —
                    // min/max are Real lattice ops, evaluation-legal).
                    let arg = ((T::zero() - cos_a * c) / (sin_a * s))
                        .min(T::one())
                        .max(T::zero() - T::one());
                    let delta = arg.acos();
                    let gen_at = |u: T| -> Curve3<T> {
                        let (su, cu) = u.sin_cos();
                        Curve3::Line {
                            origin: apex,
                            dir: a * cos_a + (cone_u * cu + v_ref * su) * sin_a,
                        }
                    };
                    if verdict == Sign::Zero {
                        Ok(PlaneConeSection::ApexTangentLine(gen_at(phi + delta)))
                    } else {
                        Ok(PlaneConeSection::ApexLinePair {
                            l1: gen_at(phi + delta),
                            l2: gen_at(phi - delta),
                        })
                    }
                }
                Sign::Negative => Ok(PlaneConeSection::ApexPoint(apex)),
            }
        }
        Sign::Positive | Sign::Negative => {
            // Apex definitely off the plane: axis-normal circle or the
            // R1 permanent routing.
            let h = (q - apex).dot(a);
            let rim_r = h.abs() * (sin_a / cos_a);
            match decide("pn_axis_normal", Margin::levered(s, rim_r), band)
                .map_err(SectionError::Escalated)?
            {
                Sign::Zero => Ok(PlaneConeSection::AxisNormalCircle(Curve3::Circle {
                    center: apex + a * h,
                    axis: a,
                    radius: rim_r,
                    u_ref: cone_u,
                })),
                Sign::Positive | Sign::Negative => Err(SectionError::RoutesToGeneralRung {
                    pair: "plane×cone",
                    why: "generic tilt routes to the general rung PERMANENTLY — the \
                          conic trio is outside the closed-form inventory by \
                          decision, and only an arm that adds parabola/hyperbola \
                          moves it. The general rung is implemented; this routing is \
                          not waiting on it",
                }),
            }
        }
    }
}
